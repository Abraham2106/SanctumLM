//! Cola de importación en hilo de fondo con cancelación **cooperativa**.
//!
//! `job_cancel` activa [`CancelFlag`]; el worker comprueba la señal **entre ítems**
//! del lote (no interrumpe extracción/escritura SQLite a mitad de un ítem).
//! Fuentes ya persistidas como `ready` en ítems anteriores no se revierten (R-PROD-014).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

use core_domain::{
    DataClassification, JobId, JobRecord, JobState, JobType, NotebookId, ProductError,
    ProductErrorCode, SourceId,
};
use rusqlite::Connection;
use storage::{
    chunk_inline_text, sha256_hex, validate_inline_size, CancelFlag, SqliteJobStore,
    SqliteSourceRepository,
};
use uuid::Uuid;

use crate::ipc_types::{ImportSourceInlineItem, ImportSourceItemResultDto, ImportSourcesResponse};
use crate::sidecar::spawn_documents_worker;

const INLINE_MAX_BYTES: u64 = 8 * 1024 * 1024;

#[derive(Clone)]
pub struct ImportRuntime {
    conn: Arc<Mutex<Connection>>,
    staging_root: PathBuf,
    documents_worker: Option<PathBuf>,
    cancel_flags: Arc<Mutex<HashMap<JobId, CancelFlag>>>,
}

impl ImportRuntime {
    pub fn new(
        conn: Arc<Mutex<Connection>>,
        staging_root: PathBuf,
        documents_worker: Option<PathBuf>,
    ) -> Self {
        let _ = std::fs::create_dir_all(&staging_root);
        Self {
            conn,
            staging_root,
            documents_worker,
            cancel_flags: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn enqueue_batch(
        &self,
        jobs: Arc<SqliteJobStore>,
        notebook_id: NotebookId,
        items: Vec<ImportSourceInlineItem>,
    ) -> ImportSourcesResponse {
        let mut job = JobRecord::new(JobType::ImportBatch);
        job.state = if items.is_empty() {
            JobState::Completed
        } else {
            JobState::Queued
        };
        job.progress = if items.is_empty() { 100 } else { 0 };
        let job_id = job.job_id;
        jobs.insert(job.clone(), Some(&notebook_id));

        let cancel = CancelFlag::new();
        self.cancel_flags
            .lock()
            .expect("cancel map")
            .insert(job_id, cancel.clone());

        let mut item_results = Vec::new();
        for (item_index, item) in items.iter().enumerate() {
            if item.kind != "inline_text" {
                item_results.push(ImportSourceItemResultDto {
                    item_index,
                    source_id: None,
                    error: Some(ProductError::new(
                        ProductErrorCode::ValidationFailed,
                        "solo inline_text en TASK-0027",
                    )),
                });
                continue;
            }
            if item.text.trim().is_empty() {
                item_results.push(ImportSourceItemResultDto {
                    item_index,
                    source_id: None,
                    error: Some(ProductError::new(
                        ProductErrorCode::ValidationFailed,
                        "El archivo está vacío.",
                    )),
                });
                continue;
            }
            item_results.push(ImportSourceItemResultDto {
                item_index,
                source_id: None,
                error: None,
            });
        }

        let response = ImportSourcesResponse {
            job_id: job_id.0.to_string(),
            item_results: item_results.clone(),
        };

        if items.is_empty() {
            return response;
        }

        let runtime = self.clone();
        let items_owned = items;
        thread::spawn(move || {
            runtime.run_batch(jobs, job_id, notebook_id, items_owned, item_results, cancel);
        });

        response
    }

    pub fn cancel_job(&self, job_id: JobId) {
        if let Some(flag) = self
            .cancel_flags
            .lock()
            .expect("cancel map")
            .get(&job_id)
            .cloned()
        {
            flag.cancel();
        }
    }

    fn run_batch(
        &self,
        jobs: Arc<SqliteJobStore>,
        job_id: JobId,
        notebook_id: NotebookId,
        items: Vec<ImportSourceInlineItem>,
        mut item_results: Vec<ImportSourceItemResultDto>,
        cancel: CancelFlag,
    ) {
        let sources = SqliteSourceRepository::new(Arc::clone(&self.conn));
        let total = items.len().max(1);
        let Some(mut job) = jobs.get(&job_id) else {
            return;
        };
        job.state = JobState::Running;
        jobs.update(job.clone(), Some(&notebook_id));

        for (item_index, item) in items.iter().enumerate() {
            #[cfg(test)]
            if item_index > 0 {
                std::thread::sleep(std::time::Duration::from_millis(25));
            }

            if cancel.is_cancelled() {
                job.state = JobState::Cancelled;
                job.error_code = Some(core_domain::JobErrorCode::Cancelled);
                jobs.update(job, Some(&notebook_id));
                return;
            }

            if item_results[item_index].error.is_some() {
                continue;
            }

            let classification = item.classification.unwrap_or(DataClassification::Private);
            let mime = mime_str(&item.mime);
            let hash = sha256_hex(item.text.as_bytes());

            if let Ok(Some(existing)) = sources.find_ready_by_sha256(&notebook_id, &hash) {
                item_results[item_index].source_id = Some(existing.0.clone());
                job.progress = ((item_index + 1) * 100 / total).min(99) as u8;
                jobs.update(job.clone(), Some(&notebook_id));
                continue;
            }

            match self.import_one(&sources, &notebook_id, item, &hash, classification, &mime) {
                Ok(source_id) => {
                    item_results[item_index].source_id = Some(source_id.0);
                }
                Err(err) => {
                    item_results[item_index].error = Some(err);
                }
            }

            job.progress = ((item_index + 1) * 100 / total).min(99) as u8;
            jobs.update(job.clone(), Some(&notebook_id));
        }

        if cancel.is_cancelled() {
            job.state = JobState::Cancelled;
            job.error_code = Some(core_domain::JobErrorCode::Cancelled);
        } else {
            job.state = JobState::Completed;
            job.progress = 100;
        }
        jobs.update(job, Some(&notebook_id));
    }

    fn import_one(
        &self,
        sources: &SqliteSourceRepository,
        notebook_id: &NotebookId,
        item: &ImportSourceInlineItem,
        hash: &str,
        classification: DataClassification,
        mime: &str,
    ) -> Result<SourceId, ProductError> {
        validate_inline_size(&item.text, INLINE_MAX_BYTES)?;

        let source_id = sources.insert_processing_source(
            notebook_id,
            &item.display_name,
            Some(mime),
            hash,
            classification,
        )?;

        let staging_name = staging_file_name(&item.display_name, mime);
        let staging_path = self.staging_root.join(staging_name);
        std::fs::write(&staging_path, item.text.as_bytes()).map_err(|_| {
            ProductError::new(ProductErrorCode::LocalUnavailable, "staging write")
        })?;

        let chunks = self.extract_chunks(&staging_path, mime)?;
        if chunks.is_empty() {
            sources.mark_failed(&source_id)?;
            return Err(ProductError::new(
                ProductErrorCode::ValidationFailed,
                "sin chunks extraíbles",
            ));
        }

        let storage_path = format!(
            "staging/{}",
            staging_path.file_name().and_then(|s| s.to_str()).unwrap_or("inline.txt")
        );
        sources.persist_ready_version(&source_id, hash, &storage_path, &chunks)?;
        Ok(source_id)
    }

    fn extract_chunks(&self, staging_path: &Path, mime: &str) -> Result<Vec<storage::PersistedChunk>, ProductError> {
        if matches!(mime, "text/plain" | "text/markdown") {
            let text = std::fs::read_to_string(staging_path).map_err(|_| {
                ProductError::new(ProductErrorCode::LocalUnavailable, "read staging")
            })?;
            return Ok(chunk_inline_text(&text));
        }

        if let Some(worker) = self.documents_worker.as_ref() {
            if let Ok(chunks) = spawn_documents_worker(
                worker,
                &self.staging_root,
                &Uuid::new_v4().to_string(),
                staging_path,
                mime,
            ) {
                return Ok(chunks);
            }
        }

        Err(ProductError::new(
            ProductErrorCode::LocalUnavailable,
            "extracción no disponible",
        ))
    }
}

fn mime_str(mime: &crate::ipc_types::ImportInlineMime) -> &'static str {
    match mime {
        crate::ipc_types::ImportInlineMime::TextPlain => "text/plain",
        crate::ipc_types::ImportInlineMime::TextMarkdown => "text/markdown",
    }
}

fn staging_file_name(display_name: &str, mime: &str) -> String {
    let safe: String = display_name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' { c } else { '_' })
        .collect();
    if safe.contains('.') {
        format!("{}-{}", Uuid::new_v4(), safe)
    } else {
        let ext = if mime == "text/markdown" { "md" } else { "txt" };
        format!("{}-{}.{}", Uuid::new_v4(), safe, ext)
    }
}
