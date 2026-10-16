//! Adaptador nativo text/plain y text/markdown (ADR-0009).

use core_domain::{ProductError, ProductErrorCode};
use std::path::Path;
use std::time::Instant;

use crate::secure_io::read_staging_file;
use crate::{
    DocumentExtractor, ExtractionInput, ExtractionMetadata, ExtractionOutput, ExtractedChunk,
};

pub const NATIVE_TEXT_ADAPTER_ID: &str = "native-text";
pub const NATIVE_TEXT_ADAPTER_VERSION: &str = "0.1.0";
const DEFAULT_MAX_CHUNK_CHARS: usize = 4096;

pub struct NativeTextExtractor;

pub struct SidecarExtractor;

impl DocumentExtractor for SidecarExtractor {
    fn extract(&self, input: ExtractionInput) -> Result<ExtractionOutput, ProductError> {
        if supports_native_text(&input.mime, &input.staging_path) {
            NativeTextExtractor.extract(input)
        } else {
            Err(ProductError::new(
                ProductErrorCode::ValidationFailed,
                format!("tipo no soportado por sidecar: {}", input.mime),
            ))
        }
    }
}

impl DocumentExtractor for NativeTextExtractor {
    fn extract(&self, input: ExtractionInput) -> Result<ExtractionOutput, ProductError> {
        let started = Instant::now();
        let bytes = read_staging_file(
            &input.staging_root,
            &input.staging_path,
            input.max_bytes,
        )?;
        let raw = String::from_utf8(bytes).map_err(|_| {
            ProductError::new(
                ProductErrorCode::ValidationFailed,
                "contenido no es UTF-8 válido",
            )
        })?;
        let normalized = normalize_text(&raw);
        if normalized.is_empty() {
            return Err(ProductError::new(
                ProductErrorCode::ValidationFailed,
                "archivo vacío tras normalización",
            ));
        }
        let chunks = chunk_by_paragraphs(&normalized, DEFAULT_MAX_CHUNK_CHARS);
        if chunks.is_empty() {
            return Err(ProductError::new(
                ProductErrorCode::ValidationFailed,
                "no se produjeron chunks",
            ));
        }
        let duration_ms = started.elapsed().as_millis() as u64;
        Ok(ExtractionOutput {
            normalized_text: normalized,
            chunks,
            warnings: vec![],
            extraction_metadata: ExtractionMetadata {
                adapter_id: NATIVE_TEXT_ADAPTER_ID.into(),
                adapter_version: NATIVE_TEXT_ADAPTER_VERSION.into(),
                duration_ms,
                sidecar_exit_code: 0,
            },
        })
    }
}

pub fn supports_native_text(mime: &str, path: &Path) -> bool {
    let mime = mime.trim().to_ascii_lowercase();
    if matches!(
        mime.as_str(),
        "text/plain" | "text/markdown" | "text/x-markdown"
    ) {
        return true;
    }
    path.extension()
        .and_then(|e| e.to_str())
        .map(|ext| {
            let ext = ext.to_ascii_lowercase();
            ext == "txt" || ext == "md" || ext == "markdown"
        })
        .unwrap_or(false)
}

fn normalize_text(raw: &str) -> String {
    let unified = raw.replace("\r\n", "\n").replace('\r', "\n");
    unified.trim_end().to_string()
}

/// Párrafos no vacíos con `char_start` en `text` (tras trim de bordes del segmento).
fn paragraph_spans(text: &str) -> Vec<(u32, &str)> {
    let parts: Vec<&str> = text.split("\n\n").collect();
    let mut spans = Vec::new();
    let mut char_offset = 0u32;

    for (i, part) in parts.iter().enumerate() {
        let trimmed = part.trim();
        if !trimmed.is_empty() {
            let lead_ws = part.chars().take_while(|c| c.is_whitespace()).count() as u32;
            spans.push((char_offset + lead_ws, trimmed));
        }
        char_offset += part.chars().count() as u32;
        if i + 1 < parts.len() {
            char_offset += 2;
        }
    }
    spans
}

fn chunk_by_paragraphs(text: &str, max_chars: usize) -> Vec<ExtractedChunk> {
    let max_chars = max_chars.max(1);
    let spans = paragraph_spans(text);

    let mut chunks = Vec::new();
    if spans.is_empty() {
        push_sized_chunks(text, 0, max_chars, &mut chunks);
        return chunks;
    }

    let mut buf = String::new();
    let mut buf_start: Option<u32> = None;

    let flush_buf = |chunks: &mut Vec<ExtractedChunk>, buf: &mut String, start: &mut Option<u32>| {
        if buf.is_empty() {
            return;
        }
        let char_start = start.expect("buf no vacío implica char_start");
        let len = buf.chars().count() as u32;
        chunks.push(ExtractedChunk {
            text: std::mem::take(buf),
            page: None,
            char_start,
            char_end: char_start + len,
        });
        *start = None;
    };

    for (para_start, para_text) in spans {
        let para_chars = para_text.chars().count();
        if para_chars > max_chars {
            flush_buf(&mut chunks, &mut buf, &mut buf_start);
            push_sized_chunks(para_text, para_start, max_chars, &mut chunks);
            continue;
        }

        let sep = if buf.is_empty() { 0 } else { 2 };
        if !buf.is_empty() && buf.chars().count() + sep + para_chars > max_chars {
            flush_buf(&mut chunks, &mut buf, &mut buf_start);
            buf_start = Some(para_start);
            buf.push_str(para_text);
            continue;
        }

        if buf.is_empty() {
            buf_start = Some(para_start);
            buf.push_str(para_text);
        } else {
            buf.push_str("\n\n");
            buf.push_str(para_text);
        }
    }
    flush_buf(&mut chunks, &mut buf, &mut buf_start);
    chunks
}

fn push_sized_chunks(
    text: &str,
    base_char_offset: u32,
    max_chars: usize,
    chunks: &mut Vec<ExtractedChunk>,
) {
    let mut char_start = base_char_offset;
    let mut chunk_buf = String::new();
    for ch in text.chars() {
        chunk_buf.push(ch);
        if chunk_buf.chars().count() >= max_chars {
            let len = chunk_buf.chars().count() as u32;
            chunks.push(ExtractedChunk {
                text: chunk_buf.clone(),
                page: None,
                char_start,
                char_end: char_start + len,
            });
            char_start += len;
            chunk_buf.clear();
        }
    }
    if !chunk_buf.is_empty() {
        let len = chunk_buf.chars().count() as u32;
        chunks.push(ExtractedChunk {
            text: chunk_buf,
            page: None,
            char_start,
            char_end: char_start + len,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::staging::{validate_staging_path, validate_staging_root};
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_DIR_SEQ: AtomicU64 = AtomicU64::new(0);

    fn staging_with(file_name: &str, contents: &[u8]) -> (PathBuf, PathBuf) {
        let seq = TEST_DIR_SEQ.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("documents-worker-native-{file_name}-{seq}"));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        let file = root.join(file_name);
        fs::write(&file, contents).unwrap();
        let validated_root = validate_staging_root(root.to_str().unwrap()).unwrap();
        let validated_path =
            validate_staging_path(&validated_root, file.to_str().unwrap()).unwrap();
        (validated_root, validated_path)
    }

    fn input(root: &PathBuf, path: PathBuf, mime: &str) -> ExtractionInput {
        ExtractionInput {
            staging_root: root.clone(),
            staging_path: path,
            mime: mime.into(),
            max_bytes: 1_000_000,
        }
    }

    #[test]
    fn extracts_txt_with_chunks() {
        let (root, path) = staging_with("note.txt", b"Linea uno.\n\nLinea dos.");
        let out = NativeTextExtractor
            .extract(input(&root, path, "text/plain"))
            .unwrap();
        assert!(!out.chunks.is_empty());
        assert!(out.normalized_text.contains("Linea uno"));
        assert_eq!(out.extraction_metadata.adapter_id, NATIVE_TEXT_ADAPTER_ID);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn extracts_md_by_extension() {
        let (root, path) = staging_with("readme.md", b"# Titulo\n\nCuerpo.");
        let out = NativeTextExtractor
            .extract(input(&root, path, "application/octet-stream"))
            .unwrap();
        assert_eq!(out.chunks.len(), 1);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn invalid_utf8_is_per_item_error() {
        let (root, path) = staging_with("bad.txt", &[0xff, 0xfe, 0x00]);
        let err = NativeTextExtractor
            .extract(input(&root, path, "text/plain"))
            .unwrap_err();
        assert_eq!(err.code, ProductErrorCode::ValidationFailed);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn empty_file_is_per_item_error() {
        let (root, path) = staging_with("empty.txt", b"   \n  ");
        let err = NativeTextExtractor
            .extract(input(&root, path, "text/plain"))
            .unwrap_err();
        assert_eq!(err.code, ProductErrorCode::ValidationFailed);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn chunking_splits_long_paragraphs() {
        let long: String = "a".repeat(5000);
        let chunks = chunk_by_paragraphs(&long, 1000);
        assert!(chunks.len() >= 5);
        assert_eq!(chunks.first().unwrap().char_start, 0);
    }

    #[test]
    fn chunk_offsets_include_paragraph_separators_in_normalized_text() {
        let text = "12345\n\n67890";
        let chunks = chunk_by_paragraphs(text, 10);
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].char_start, 0);
        assert_eq!(chunks[0].char_end, 5);
        assert_eq!(chunks[1].char_start, 7);
        assert_eq!(chunks[1].char_end, 12);
        assert_eq!(chunks[0].text, "12345");
        assert_eq!(chunks[1].text, "67890");
        assert_eq!(
            chars_in_range(text, chunks[1].char_start, chunks[1].char_end),
            chunks[1].text
        );
    }

    fn chars_in_range(text: &str, start: u32, end: u32) -> String {
        text.chars()
            .skip(start as usize)
            .take((end - start) as usize)
            .collect()
    }
}
