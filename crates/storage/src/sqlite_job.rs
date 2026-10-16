use std::sync::{Arc, Mutex};

use core_domain::{
    JobErrorCode, JobId, JobRecord, JobState, JobType, NotebookId, ProductError, ProductErrorCode,
};
use rusqlite::{params, Connection};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

#[derive(Clone)]
pub struct SqliteJobRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteJobRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    fn now_rfc3339() -> Result<String, ProductError> {
        OffsetDateTime::now_utc().format(&Rfc3339).map_err(|_| {
            ProductError::new(ProductErrorCode::LocalUnavailable, "timestamp")
        })
    }

    pub fn insert(&self, notebook_id: &NotebookId, record: &JobRecord) -> Result<(), ProductError> {
        let updated_at = Self::now_rfc3339()?;
        let conn = self.conn.lock().expect("db lock");
        conn.execute(
            "INSERT INTO jobs (id, notebook_id, type, state, progress, error_code, updated_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                record.job_id.0.to_string(),
                notebook_id.0,
                job_type_to_db(&record.job_type),
                job_state_to_db(&record.state),
                f64::from(record.progress),
                record.error_code.as_ref().map(job_error_code_to_db),
                updated_at,
            ],
        )
        .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "insert job"))?;
        Ok(())
    }

    pub fn get(&self, job_id: &JobId) -> Result<Option<JobRecord>, ProductError> {
        let conn = self.conn.lock().expect("db lock");
        let mut stmt = conn
            .prepare(
                "SELECT type, state, progress, error_code FROM jobs WHERE id = ?1",
            )
            .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "prepare job"))?;
        let mut rows = stmt
            .query([job_id.0.to_string()])
            .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "query job"))?;
        if let Some(row) = rows
            .next()
            .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "fetch job"))?
        {
            return Ok(Some(row_to_job_record(job_id, row)?));
        }
        Ok(None)
    }

    pub fn update(&self, record: &JobRecord) -> Result<(), ProductError> {
        let updated_at = Self::now_rfc3339()?;
        let conn = self.conn.lock().expect("db lock");
        let changed = conn
            .execute(
                "UPDATE jobs SET type = ?2, state = ?3, progress = ?4, error_code = ?5, updated_at = ?6 \
                 WHERE id = ?1",
                params![
                    record.job_id.0.to_string(),
                    job_type_to_db(&record.job_type),
                    job_state_to_db(&record.state),
                    f64::from(record.progress),
                    record.error_code.as_ref().map(job_error_code_to_db),
                    updated_at,
                ],
            )
            .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "update job"))?;
        if changed == 0 {
            return Err(ProductError::not_found("job"));
        }
        Ok(())
    }

    pub fn mark_cancelled(&self, job_id: &JobId) -> Result<JobRecord, ProductError> {
        let Some(mut job) = self.get(job_id)? else {
            return Err(ProductError::not_found("job"));
        };
        if matches!(
            job.state,
            JobState::Completed | JobState::Failed | JobState::Cancelled
        ) {
            return Ok(job);
        }
        job.state = JobState::Cancelled;
        job.error_code = Some(JobErrorCode::Cancelled);
        job.progress = job.progress.min(99);
        self.update(&job)?;
        Ok(job)
    }
}

fn row_to_job_record(job_id: &JobId, row: &rusqlite::Row<'_>) -> Result<JobRecord, ProductError> {
    let type_raw: String = row.get(0).map_err(map_row_err)?;
    let state_raw: String = row.get(1).map_err(map_row_err)?;
    let progress_f: f64 = row.get(2).map_err(map_row_err)?;
    let error_raw: Option<String> = row.get(3).map_err(map_row_err)?;

    Ok(JobRecord {
        job_id: *job_id,
        job_type: job_type_from_db(&type_raw)?,
        state: job_state_from_db(&state_raw)?,
        progress: progress_f.round().clamp(0.0, 100.0) as u8,
        error_code: error_raw
            .as_deref()
            .map(job_error_code_from_db)
            .transpose()?,
        dedupe_key: None,
    })
}

fn map_row_err(_: rusqlite::Error) -> ProductError {
    ProductError::new(ProductErrorCode::LocalUnavailable, "read job row")
}

fn job_type_to_db(t: &JobType) -> &'static str {
    match t {
        JobType::ImportBatch => "import_batch",
        JobType::Extract => "extract",
        JobType::EmbedIndex => "embed_index",
        JobType::GenerateArtifact => "generate_artifact",
    }
}

fn job_type_from_db(s: &str) -> Result<JobType, ProductError> {
    match s {
        "import_batch" => Ok(JobType::ImportBatch),
        "extract" => Ok(JobType::Extract),
        "embed_index" => Ok(JobType::EmbedIndex),
        "generate_artifact" => Ok(JobType::GenerateArtifact),
        _ => Err(ProductError::new(
            ProductErrorCode::LocalUnavailable,
            "job type desconocido",
        )),
    }
}

fn job_state_to_db(s: &JobState) -> &'static str {
    match s {
        JobState::Queued => "queued",
        JobState::Running => "running",
        JobState::Completed => "completed",
        JobState::Failed => "failed",
        JobState::Cancelled => "cancelled",
    }
}

fn job_state_from_db(s: &str) -> Result<JobState, ProductError> {
    match s {
        "queued" => Ok(JobState::Queued),
        "running" => Ok(JobState::Running),
        "completed" => Ok(JobState::Completed),
        "failed" => Ok(JobState::Failed),
        "cancelled" => Ok(JobState::Cancelled),
        _ => Err(ProductError::new(
            ProductErrorCode::LocalUnavailable,
            "job state desconocido",
        )),
    }
}

fn job_error_code_to_db(c: &JobErrorCode) -> &'static str {
    match c {
        JobErrorCode::ExtractionCorrupt => "EXTRACTION_CORRUPT",
        JobErrorCode::PolicyDenied => "POLICY_DENIED",
        JobErrorCode::Cancelled => "CANCELLED",
    }
}

fn job_error_code_from_db(s: &str) -> Result<JobErrorCode, ProductError> {
    match s {
        "EXTRACTION_CORRUPT" => Ok(JobErrorCode::ExtractionCorrupt),
        "POLICY_DENIED" => Ok(JobErrorCode::PolicyDenied),
        "CANCELLED" => Ok(JobErrorCode::Cancelled),
        _ => Err(ProductError::new(
            ProductErrorCode::LocalUnavailable,
            "job error_code desconocido",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::migrate::migrate_to_latest;
    use core_domain::NotebookId;
    use rusqlite::Connection;
    use std::fs;
    use std::sync::{Arc, Mutex};

    fn temp_db_path() -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "sanctumlm-jobs-test-{}",
            uuid::Uuid::new_v4()
        ));
        fs::create_dir_all(&dir).expect("temp dir");
        dir.join("jobs.db")
    }

    fn open_migrated(path: &std::path::Path) -> Connection {
        let conn = Connection::open(path).expect("open db");
        migrate_to_latest(&conn).expect("migrate");
        conn
    }

    fn seed_notebook(conn: &Connection) -> NotebookId {
        let nb = NotebookId(uuid::Uuid::new_v4().to_string());
        let now = OffsetDateTime::now_utc()
            .format(&Rfc3339)
            .expect("rfc3339");
        conn.execute(
            "INSERT INTO notebooks (id, title, created_at, updated_at) VALUES (?1, ?2, ?3, ?3)",
            params![nb.0, "Jobs test", now.as_str()],
        )
        .expect("notebook");
        nb
    }

    #[test]
    fn migration_0003_creates_jobs_table() {
        let path = temp_db_path();
        let conn = open_migrated(&path);
        let count: i64 = conn
            .query_row(
                "SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name = 'jobs'",
                [],
                |row| row.get(0),
            )
            .expect("jobs table");
        assert_eq!(count, 1);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn job_survives_new_connection_simulating_restart() {
        let path = temp_db_path();
        let nb;
        let job_id;
        {
            let conn = open_migrated(&path);
            nb = seed_notebook(&conn);
            let arc = Arc::new(Mutex::new(conn));
            let repo = SqliteJobRepository::new(Arc::clone(&arc));
            let mut job = JobRecord::new(JobType::ImportBatch);
            job.state = JobState::Running;
            job.progress = 42;
            job_id = job.job_id;
            repo.insert(&nb, &job).expect("insert");
        }

        let conn2 = open_migrated(&path);
        let repo2 = SqliteJobRepository::new(Arc::new(Mutex::new(conn2)));
        let loaded = repo2.get(&job_id).expect("get").expect("job exists");
        assert_eq!(loaded.state, JobState::Running);
        assert_eq!(loaded.progress, 42);

        let _ = fs::remove_file(&path);
    }

    #[test]
    fn cancel_marks_terminal_state_in_sqlite() {
        let conn = Connection::open_in_memory().expect("mem");
        migrate_to_latest(&conn).expect("migrate");
        let nb = seed_notebook(&conn);
        let arc = Arc::new(Mutex::new(conn));
        let repo = SqliteJobRepository::new(arc);
        let mut job = JobRecord::new(JobType::ImportBatch);
        job.state = JobState::Running;
        job.progress = 40;
        let id = job.job_id;
        repo.insert(&nb, &job).expect("insert");

        let cancelled = repo.mark_cancelled(&id).unwrap();
        assert_eq!(cancelled.state, JobState::Cancelled);
        assert_eq!(cancelled.error_code, Some(JobErrorCode::Cancelled));

        let again = repo.mark_cancelled(&id).unwrap();
        assert_eq!(again.state, JobState::Cancelled);
    }

    /// Retry de import con mismo SHA-256 no duplica filas `ready`; el job puede reconsultarse tras reconexión.
    #[test]
    fn retry_same_hash_single_ready_job_status_stable() {
        use crate::sqlite_source::{PersistedChunk, SqliteSourceRepository};

        let conn = Connection::open_in_memory().expect("mem");
        migrate_to_latest(&conn).expect("migrate");
        let nb = seed_notebook(&conn);
        let arc = Arc::new(Mutex::new(conn));
        let sources = SqliteSourceRepository::new(Arc::clone(&arc));
        let jobs = SqliteJobRepository::new(Arc::clone(&arc));

        let hash = "feedface".repeat(8);
        let id1 = sources
            .insert_processing_source(&nb, "a.txt", Some("text/plain"), &hash, core_domain::DataClassification::Public)
            .unwrap();
        sources
            .persist_ready_version(
                &id1,
                &hash,
                "staging/a.txt",
                &[PersistedChunk {
                    text: "contenido".into(),
                    page: None,
                    char_start: Some(0),
                    char_end: Some(9),
                }],
            )
            .unwrap();

        let mut job = JobRecord::new(JobType::ImportBatch);
        job.state = JobState::Completed;
        job.progress = 100;
        let job_id = job.job_id;
        jobs.insert(&nb, &job).expect("insert job");

        let dup = sources.find_ready_by_sha256(&nb, &hash).unwrap();
        assert_eq!(dup, Some(id1));
        assert_eq!(
            sources
                .count_sources_by_sha256(&nb, &hash, crate::source_meta::SourceStatus::Ready)
                .unwrap(),
            1
        );

        drop(sources);
        let jobs2 = SqliteJobRepository::new(arc);
        let loaded = jobs2.get(&job_id).unwrap().unwrap();
        assert_eq!(loaded.state, JobState::Completed);
    }
}
