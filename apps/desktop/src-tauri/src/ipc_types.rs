use core_domain::{DataClassification, IpcResult, JobRecord, ProductError, ProductErrorCode};
use serde::{Deserialize, Serialize};

pub const IPC_API_VERSION: i64 = 1;

pub fn check_api_version(version: i64) -> Result<(), ProductError> {
    if version != IPC_API_VERSION {
        return Err(ProductError::new(
            ProductErrorCode::UnsupportedApiVersion,
            format!("api_version {version} no soportada"),
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiVersioned {
    pub api_version: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NotebookCreateRequest {
    pub api_version: i64,
    pub title: String,
    pub classification: Option<DataClassification>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NotebookSummaryDto {
    pub notebook_id: String,
    pub title: String,
    pub classification: DataClassification,
}

#[derive(Debug, Clone, Serialize)]
pub struct NotebookCreateResponse {
    pub notebook: NotebookSummaryDto,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NotebookListRequest {
    pub api_version: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct NotebookListResponse {
    pub notebooks: Vec<NotebookSummaryDto>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListSourcesRequest {
    pub api_version: i64,
    pub notebook_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceRecordDto {
    pub source_id: String,
    pub notebook_id: String,
    pub display_name: String,
    pub kind: String,
    pub mime: Option<String>,
    pub status: String,
    pub classification: DataClassification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256_prefix: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListSourcesResponse {
    pub sources: Vec<SourceRecordDto>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportInlineMime {
    #[serde(rename = "text/plain")]
    TextPlain,
    #[serde(rename = "text/markdown")]
    TextMarkdown,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImportSourceInlineItem {
    pub kind: String,
    pub display_name: String,
    pub mime: ImportInlineMime,
    pub text: String,
    pub classification: Option<DataClassification>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImportSourcesRequest {
    pub api_version: i64,
    pub notebook_id: String,
    pub items: Vec<ImportSourceInlineItem>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ImportSourceItemResultDto {
    pub item_index: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ProductError>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ImportSourcesResponse {
    pub job_id: String,
    pub item_results: Vec<ImportSourceItemResultDto>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JobStatusRequest {
    pub api_version: i64,
    pub job_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JobCancelRequest {
    pub api_version: i64,
    pub job_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct JobRecordDto {
    pub job_id: String,
    pub job_type: String,
    pub state: String,
    pub progress: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedupe_key: Option<String>,
}

impl From<JobRecord> for JobRecordDto {
    fn from(job: JobRecord) -> Self {
        Self {
            job_id: job.job_id.0.to_string(),
            job_type: match job.job_type {
                core_domain::JobType::ImportBatch => "import_batch".into(),
                core_domain::JobType::Extract => "extract".into(),
                core_domain::JobType::EmbedIndex => "embed_index".into(),
                core_domain::JobType::GenerateArtifact => "generate_artifact".into(),
            },
            state: match job.state {
                core_domain::JobState::Queued => "queued".into(),
                core_domain::JobState::Running => "running".into(),
                core_domain::JobState::Completed => "completed".into(),
                core_domain::JobState::Failed => "failed".into(),
                core_domain::JobState::Cancelled => "cancelled".into(),
            },
            progress: job.progress,
            error_code: job.error_code.map(|c| match c {
                core_domain::JobErrorCode::ExtractionCorrupt => "EXTRACTION_CORRUPT".into(),
                core_domain::JobErrorCode::PolicyDenied => "POLICY_DENIED".into(),
                core_domain::JobErrorCode::Cancelled => "CANCELLED".into(),
            }),
            dedupe_key: job.dedupe_key,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchSourcesRequest {
    pub api_version: i64,
    pub notebook_id: String,
    pub query: String,
    pub limit: Option<u32>,
    pub selected_source_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchHitDto {
    pub chunk_id: String,
    pub source_id: String,
    pub source_display_name: String,
    pub snippet: String,
    pub page: Option<i32>,
    pub char_start: i32,
    pub char_end: i32,
    pub rank: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchSourcesResponse {
    pub query: String,
    pub hits: Vec<SearchHitDto>,
}

pub type IpcEnvelope<T> = IpcResult<T>;
