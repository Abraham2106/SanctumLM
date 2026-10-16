use core_domain::SourceId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UrlImportRequest {
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UrlImportPreview {
    pub source_id: SourceId,
    pub title: String,
    pub byte_length_estimate: u64,
}
