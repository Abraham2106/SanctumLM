use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use core_domain::{JobErrorCode, JobId, JobRecord, JobState, ProductError, ProductErrorCode};

/// Cola de jobs en memoria hasta migración SQL `jobs` (ADR-0007; sin migraciones en TASK-0027).
#[derive(Debug, Default)]
pub struct InMemoryJobStore {
    inner: Mutex<HashMap<JobId, JobRecord>>,
}

impl InMemoryJobStore {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
        }
    }

    pub fn insert(&self, record: JobRecord) {
        self.inner
            .lock()
            .expect("job lock")
            .insert(record.job_id, record);
    }

    pub fn get(&self, job_id: &JobId) -> Option<JobRecord> {
        self.inner.lock().expect("job lock").get(job_id).cloned()
    }

    pub fn update(&self, record: JobRecord) {
        self.inner
            .lock()
            .expect("job lock")
            .insert(record.job_id, record);
    }

    pub fn mark_cancelled(&self, job_id: &JobId) -> Result<JobRecord, ProductError> {
        let mut guard = self.inner.lock().expect("job lock");
        let Some(mut job) = guard.get(job_id).cloned() else {
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
        guard.insert(*job_id, job.clone());
        Ok(job)
    }
}

/// Señal cooperativa compartida con el hilo de importación.
#[derive(Clone, Default)]
pub struct CancelFlag(Arc<Mutex<bool>>);

impl CancelFlag {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(false)))
    }

    pub fn cancel(&self) {
        if let Ok(mut v) = self.0.lock() {
            *v = true;
        }
    }

    pub fn is_cancelled(&self) -> bool {
        self.0.lock().map(|v| *v).unwrap_or(false)
    }
}

pub fn parse_job_id(raw: &str) -> Result<JobId, ProductError> {
    let uuid = uuid::Uuid::parse_str(raw).map_err(|_| {
        ProductError::new(ProductErrorCode::ValidationFailed, "job_id inválido")
    })?;
    Ok(JobId(uuid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancel_marks_terminal_state() {
        let store = InMemoryJobStore::new();
        let mut job = JobRecord::new(core_domain::JobType::ImportBatch);
        job.state = JobState::Running;
        job.progress = 40;
        let id = job.job_id;
        store.insert(job);

        let cancelled = store.mark_cancelled(&id).unwrap();
        assert_eq!(cancelled.state, JobState::Cancelled);
        assert_eq!(cancelled.error_code, Some(JobErrorCode::Cancelled));
    }
}
