use core_domain::ProductError;
use sha2::{Digest, Sha256};

use crate::sqlite_source::PersistedChunk;

pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

/// Puente documentado para TXT/MD cuando el sidecar no está disponible (TASK-0026 pendiente).
pub fn chunk_inline_text(text: &str) -> Vec<PersistedChunk> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    let parts: Vec<&str> = trimmed
        .split("\n\n")
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .collect();

    let segments: Vec<&str> = if parts.is_empty() {
        vec![trimmed]
    } else {
        parts
    };

    let mut offset = 0usize;
    let mut chunks = Vec::new();
    for part in segments {
        let start = text.find(part).unwrap_or(offset);
        let char_start = start as i32;
        let char_end = (start + part.len()) as i32;
        offset = start + part.len();
        chunks.push(PersistedChunk {
            text: part.to_string(),
            page: None,
            char_start: Some(char_start),
            char_end: Some(char_end),
        });
    }
    chunks
}

pub fn validate_inline_size(text: &str, max_bytes: u64) -> Result<(), ProductError> {
    let len = text.len() as u64;
    if len > max_bytes {
        return Err(core_domain::ProductError::new(
            core_domain::ProductErrorCode::ValidationFailed,
            "texto inline excede max_bytes",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_is_stable() {
        let a = sha256_hex(b"hola");
        let b = sha256_hex(b"hola");
        assert_eq!(a, b);
        assert_eq!(a.len(), 64);
    }

    #[test]
    fn chunks_split_on_blank_lines() {
        let chunks = chunk_inline_text("uno\n\n dos tres ");
        assert_eq!(chunks.len(), 2);
        assert!(chunks[0].text.contains("uno"));
    }
}
