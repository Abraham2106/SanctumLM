//! Contrato del sidecar de ingesta (ADR-0009); sin red ni parsers reales.

use core_domain::ProductError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractionInput {
    pub staging_path: String,
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
pub struct ExtractionOutput {
    pub normalized_text: String,
    pub chunks: Vec<ExtractedChunk>,
    pub warnings: Vec<String>,
}

pub trait DocumentExtractor: Send + Sync {
    fn extract(&self, input: ExtractionInput) -> Result<ExtractionOutput, ProductError>;
}

pub mod fake;
