use std::sync::{Arc, Mutex};

use core_domain::{
    JobErrorCode, JobId, JobRecord, JobState, JobType, NotebookId, ProductError, ProductErrorCode,
};
use rusqlite::{Connection, OptionalExtension};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// Persistencia SQLite de jobs (TASK-0035; ADR-0007 §jobs).
#[derive(Debug)]
pub struct SqliteJobStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteJobStore {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    fn now_rfc3339() -> Result<String, ProductError> {
        OffsetDateTime::now_utc().format(&Rfc3339).map_err(|_| {
            ProductError::new(ProductErrorCode::LocalUnavailable, "timestamp")
        })
    }

    pub fn insert(&self, record: JobRecord, notebook_id: Option<&NotebookId>) {
        let updated_at = Self::now_rfc3339().expect("timestamp");
        let conn = self.conn.lock().expect("sqlite lock");
        conn.execute(
            "INSERT INTO jobs (id, notebook_id, type, state, progress, error_code, updated_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                record.job_id.0.to_string(),
                notebook_id.map(|n| n.0.as_str()),
                job_type_to_str(&record.job_type),
                job_state_to_str(&record.state),
                f64::from(record.progress),
                record
                    .error_code
                    .as_ref()
                    .map(job_error_code_to_str),
                updated_at.as_str(),
            ),
        )
        .expect("insert job");
    }

    pub fn get(&self, job_id: &JobId) -> Option<JobRecord> {
        let conn = self.conn.lock().expect("sqlite lock");
        conn.query_row(
            "SELECT type, state, progress, error_code FROM jobs WHERE id = ?1",
            [job_id.0.to_string()],
            |row| {
                let job_type: String = row.get(0)?;
                let state: String = row.get(1)?;
                let progress: f64 = row.get(2)?;
                let error_code: Option<String> = row.get(3)?;
                Ok(JobRecord {
                    job_id: *job_id,
                    job_type: parse_job_type(&job_type)?,
                    state: parse_job_state(&state)?,
                    progress: progress.round().clamp(0.0, 100.0) as u8,
                    error_code: error_code
                        .as_deref()
                        .map(parse_job_error_code)
                        .transpose()?,
                    dedupe_key: None,
                })
            },
        )
        .optional()
        .expect("select job")
    }

    pub fn update(&self, record: JobRecord) {
        let updated_at = Self::now_rfc3339().expect("timestamp");
        let conn = self.conn.lock().expect("sqlite lock");
        let rows = conn
            .execute(
                "UPDATE jobs SET type = ?2, state = ?3, progress = ?4, error_code = ?5, updated_at = ?6 \
                 WHERE id = ?1",
                (
                    record.job_id.0.to_string(),
                    job_type_to_str(&record.job_type),
                    job_state_to_str(&record.state),
                    f64::from(record.progress),
                    record
                        .error_code
                        .as_ref()
                        .map(job_error_code_to_str),
                    updated_at.as_str(),
                ),
            )
            .expect("update job");
        if rows == 0 {
            self.insert(record, None);
        }
    }

    pub fn mark_cancelled(&self, job_id: &JobId) -> Result<JobRecord, ProductError> {
        let mut job = self.get(job_id).ok_or_else(|| ProductError::not_found("job"))?;
        if matches!(
            job.state,
            JobState::Completed | JobState::Failed | JobState::Cancelled
        ) {
            return Ok(job);
        }
        job.state = JobState::Cancelled;
        job.error_code = Some(JobErrorCode::Cancelled);
        job.progress = job.progress.min(99);
        self.update(job.clone());
        Ok(job)
    }
}

fn job_type_to_str(t: &JobType) -> &'static str {
    match t {
        JobType::ImportBatch => "import_batch",
        JobType::Extract => "extract",
        JobType::EmbedIndex => "embed_index",
        JobType::GenerateArtifact => "generate_artifact",
    }
}

fn parse_job_type(raw: &str) -> rusqlite::Result<JobType> {
    Ok(match raw {
        "import_batch" => JobType::ImportBatch,
        "extract" => JobType::Extract,
        "embed_index" => JobType::EmbedIndex,
        "generate_artifact" => JobType::GenerateArtifact,
        _ => return Err(rusqlite::Error::InvalidQuery),
    })
}

fn job_state_to_str(s: &JobState) -> &'static str {
    match s {
        JobState::Queued => "queued",
        JobState::Running => "running",
        JobState::Completed => "completed",
        JobState::Failed => "failed",
        JobState::Cancelled => "cancelled",
    }
}

fn parse_job_state(raw: &str) -> rusqlite::Result<JobState> {
    Ok(match raw {
        "queued" => JobState::Queued,
        "running" => JobState::Running,
        "completed" => JobState::Completed,
        "failed" => JobState::Failed,
        "cancelled" => JobState::Cancelled,
        _ => return Err(rusqlite::Error::InvalidQuery),
    })
}

fn job_error_code_to_str(c: &JobErrorCode) -> &'static str {
    match c {
        JobErrorCode::ExtractionCorrupt => "EXTRACTION_CORRUPT",
        JobErrorCode::PolicyDenied => "POLICY_DENIED",
        JobErrorCode::Cancelled => "CANCELLED",
    }
}

fn parse_job_error_code(raw: &str) -> Result<JobErrorCode, rusqlite::Error> {
    Ok(match raw {
        "EXTRACTION_CORRUPT" => JobErrorCode::ExtractionCorrupt,
        "POLICY_DENIED" => JobErrorCode::PolicyDenied,
        "CANCELLED" => JobErrorCode::Cancelled,
        _ => return Err(rusqlite::Error::InvalidQuery),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::open_memory_connection;

    fn store_with_migrations() -> (SqliteJobStore, Arc<Mutex<Connection>>) {
        let conn = open_memory_connection().expect("open db");
        let arc = Arc::new(Mutex::new(conn));
        (SqliteJobStore::new(Arc::clone(&arc)), arc)
    }

    #[test]
    fn job_survives_new_connection_on_same_db_file() {
        let dir = std::env::temp_dir().join(format!(
            "sanctumlm-jobs-test-{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&dir).expect("temp dir");
        let path = dir.join("jobs.db");

        let conn1 = crate::open_connection(&path).expect("open");
        let arc1 = Arc::new(Mutex::new(conn1));
        let store1 = SqliteJobStore::new(Arc::clone(&arc1));
        let mut job = JobRecord::new(JobType::ImportBatch);
        job.state = JobState::Running;
        job.progress = 42;
        let id = job.job_id;
        store1.insert(job, None);

        drop(arc1);

        let conn2 = crate::open_connection(&path).expect("reopen");
        let store2 = SqliteJobStore::new(Arc::new(Mutex::new(conn2)));
        let loaded = store2.get(&id).expect("job persisted");
        assert_eq!(loaded.state, JobState::Running);
        assert_eq!(loaded.progress, 42);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn cancel_marks_terminal_state() {
        let (store, _) = store_with_migrations();
        let mut job = JobRecord::new(JobType::ImportBatch);
        job.state = JobState::Running;
        job.progress = 40;
        let id = job.job_id;
        store.insert(job, None);

        let cancelled = store.mark_cancelled(&id).unwrap();
        assert_eq!(cancelled.state, JobState::Cancelled);
        assert_eq!(cancelled.error_code, Some(JobErrorCode::Cancelled));
    }
}
