use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JobId(pub Uuid);

impl JobId {
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobType {
    ImportBatch,
    Extract,
    EmbedIndex,
    GenerateArtifact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobState {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobErrorCode {
    ExtractionCorrupt,
    PolicyDenied,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JobRecord {
    pub job_id: JobId,
    pub job_type: JobType,
    pub state: JobState,
    pub progress: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<JobErrorCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedupe_key: Option<String>,
}

impl JobRecord {
    pub fn new(job_type: JobType) -> Self {
        Self {
            job_id: JobId::new_v4(),
            job_type,
            state: JobState::Queued,
            progress: 0,
            error_code: None,
            dedupe_key: None,
        }
    }
}
