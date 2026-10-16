//! Contrato del sidecar de ingesta (ADR-0009); sin red ni parsers reales.

use core_domain::ProductError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractionInput {
    /// Ruta ya validada y normalizada bajo `staging_root` (no usar JSON crudo).
    pub staging_path: PathBuf,
    pub mime: String,
    pub max_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractedChunk {
    pub text: String,
    pub page: Option<u32>,
    pub char_start: u32,
    pub char_end: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtractionMetadata {
    pub adapter_id: String,
    pub adapter_version: String,
    pub duration_ms: u64,
    pub sidecar_exit_code: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtractionOutput {
    pub normalized_text: String,
    pub chunks: Vec<ExtractedChunk>,
    pub warnings: Vec<String>,
    pub extraction_metadata: ExtractionMetadata,
}

pub trait DocumentExtractor: Send + Sync {
    fn extract(&self, input: ExtractionInput) -> Result<ExtractionOutput, ProductError>;
}

pub mod fake;
pub mod ipc;
pub mod staging;
