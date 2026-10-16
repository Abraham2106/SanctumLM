use core_domain::{ProductError, ProductErrorCode};

use crate::{DocumentExtractor, ExtractionInput, ExtractionOutput, ExtractedChunk};

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
        Ok(ExtractionOutput {
            normalized_text: text.clone(),
            chunks: vec![ExtractedChunk {
                text,
                page: Some(1),
                char_start: 0,
                char_end: 32,
            }],
            warnings: vec![],
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
    }
}
