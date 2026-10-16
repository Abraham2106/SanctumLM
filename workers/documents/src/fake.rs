use core_domain::{ProductError, ProductErrorCode};

use crate::{
    DocumentExtractor, ExtractionInput, ExtractionMetadata, ExtractionOutput, ExtractedChunk,
};

const STUB_ADAPTER_ID: &str = "stub-native";
const STUB_ADAPTER_VERSION: &str = "0.0.0-skeleton";

pub struct FakeDocumentExtractor;

impl DocumentExtractor for FakeDocumentExtractor {
    fn extract(&self, input: ExtractionInput) -> Result<ExtractionOutput, ProductError> {
        if input.staging_path.is_empty() {
            return Err(ProductError::new(
                ProductErrorCode::ValidationFailed,
                "staging_path requerido",
            ));
        }
        let text = format!("contenido-stub:{}", input.mime);
        let char_end = text.len() as u32;
        Ok(ExtractionOutput {
            normalized_text: text.clone(),
            chunks: vec![ExtractedChunk {
                text,
                page: Some(1),
                char_start: 0,
                char_end,
            }],
            warnings: vec![],
            extraction_metadata: ExtractionMetadata {
                adapter_id: STUB_ADAPTER_ID.into(),
                adapter_version: STUB_ADAPTER_VERSION.into(),
                duration_ms: 0,
                sidecar_exit_code: 0,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fake_extractor_returns_one_chunk() {
        let ex = FakeDocumentExtractor;
        let out = ex
            .extract(ExtractionInput {
                staging_path: "/staging/doc.txt".into(),
                mime: "text/plain".into(),
                max_bytes: 1_000_000,
            })
            .unwrap();
        assert_eq!(out.chunks.len(), 1);
        assert_eq!(out.chunks[0].char_end, out.normalized_text.len() as u32);
    }
}
