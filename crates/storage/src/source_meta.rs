use core_domain::{DataClassification, NotebookId, SourceId};
use serde::{Deserialize, Serialize};

/// Metadatos de fuente expuestos al núcleo / IPC (mapeo sobre filas `sources`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    File,
    Paste,
    Url,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceStatus {
    Pending,
    Processing,
    Ready,
    Failed,
    Partial,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceMeta {
    pub source_id: SourceId,
    pub notebook_id: NotebookId,
    pub display_name: String,
    pub kind: SourceKind,
    pub mime: Option<String>,
    pub status: SourceStatus,
    pub classification: DataClassification,
    pub sha256_prefix: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct SourceUriPayload {
    display_name: String,
    mime: Option<String>,
}

pub fn encode_source_uri(display_name: &str, mime: Option<&str>) -> String {
    let payload = SourceUriPayload {
        display_name: display_name.to_string(),
        mime: mime.map(str::to_string),
    };
    serde_json::to_string(&payload).unwrap_or_else(|_| display_name.to_string())
}

pub fn decode_source_uri(uri: Option<&str>, fallback_id: &str) -> (String, Option<String>) {
    let Some(raw) = uri.filter(|s| !s.is_empty()) else {
        return (fallback_id.to_string(), None);
    };
    if let Ok(payload) = serde_json::from_str::<SourceUriPayload>(raw) {
        return (payload.display_name, payload.mime);
    }
    (raw.to_string(), None)
}

pub fn kind_from_db(raw: &str) -> SourceKind {
    match raw {
        "paste" => SourceKind::Paste,
        "url" => SourceKind::Url,
        _ => SourceKind::File,
    }
}

pub fn kind_to_db(kind: SourceKind) -> &'static str {
    match kind {
        SourceKind::File => "file",
        SourceKind::Paste => "paste",
        SourceKind::Url => "url",
    }
}

pub fn status_from_db(raw: &str) -> SourceStatus {
    match raw {
        "pending" => SourceStatus::Pending,
        "processing" => SourceStatus::Processing,
        "ready" => SourceStatus::Ready,
        "failed" => SourceStatus::Failed,
        "partial" => SourceStatus::Partial,
        _ => SourceStatus::Failed,
    }
}

pub fn status_to_db(status: SourceStatus) -> &'static str {
    match status {
        SourceStatus::Pending => "pending",
        SourceStatus::Processing => "processing",
        SourceStatus::Ready => "ready",
        SourceStatus::Failed => "failed",
        SourceStatus::Partial => "partial",
    }
}

pub fn classification_to_db(c: DataClassification) -> &'static str {
    match c {
        DataClassification::Public => "public",
        DataClassification::Private => "private",
    }
}

pub fn classification_from_db(raw: &str) -> DataClassification {
    match raw {
        "public" => DataClassification::Public,
        _ => DataClassification::Private,
    }
}

pub fn sha256_prefix(full: &str) -> String {
    full.chars().take(8).collect()
}
