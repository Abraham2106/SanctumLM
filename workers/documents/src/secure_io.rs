//! Lectura acotada de archivos bajo staging (sin follow implícito vía canonicalize).

use core_domain::{ProductError, ProductErrorCode};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Lee bytes de un archivo ya validado lógicamente bajo `staging_root`.
///
/// Si el directorio de staging existe en disco, canonicaliza root y archivo y
/// rechaza rutas que queden fuera del root (symlinks/junctions fuera de staging).
pub fn read_staging_file(
    staging_root: &Path,
    validated_path: &Path,
    max_bytes: u64,
) -> Result<Vec<u8>, ProductError> {
    let read_path = resolve_read_path(staging_root, validated_path)?;
    read_limited(&read_path, max_bytes)
}

fn resolve_read_path(
    staging_root: &Path,
    validated_path: &Path,
) -> Result<PathBuf, ProductError> {
    if staging_root.exists() {
        let canon_root = staging_root.canonicalize().map_err(|e| io_policy_err(e, "staging"))?;
        let canon_file = validated_path.canonicalize().map_err(|e| io_policy_err(e, "archivo"))?;
        if !canon_file.starts_with(&canon_root) {
            return Err(ProductError::new(
                ProductErrorCode::PolicyDenied,
                "ruta resuelta fuera de staging acordado",
            ));
        }
        return Ok(canon_file);
    }

    if !validated_path.starts_with(staging_root) {
        return Err(ProductError::new(
            ProductErrorCode::PolicyDenied,
            "ruta fuera de staging acordado",
        ));
    }
    Ok(validated_path.to_path_buf())
}

fn read_limited(path: &Path, max_bytes: u64) -> Result<Vec<u8>, ProductError> {
    let meta = fs::metadata(path).map_err(|e| io_corrupt_err(e, "no se pudo leer metadatos"))?;
    if !meta.is_file() {
        return Err(ProductError::new(
            ProductErrorCode::ValidationFailed,
            "staging_path no es un archivo regular",
        ));
    }
    if meta.len() > max_bytes {
        return Err(ProductError::new(
            ProductErrorCode::PolicyDenied,
            format!("archivo excede max_bytes ({})", meta.len()),
        ));
    }

    let file = fs::File::open(path).map_err(|e| io_corrupt_err(e, "no se pudo abrir archivo"))?;
    let mut buf = Vec::with_capacity(meta.len() as usize);
    file.take(max_bytes)
        .read_to_end(&mut buf)
        .map_err(|e| io_corrupt_err(e, "lectura incompleta"))?;
    Ok(buf)
}

fn io_policy_err(err: std::io::Error, ctx: &str) -> ProductError {
    ProductError::new(
        ProductErrorCode::PolicyDenied,
        format!("{ctx}: {err}"),
    )
}

fn io_corrupt_err(err: std::io::Error, ctx: &str) -> ProductError {
    ProductError::new(
        ProductErrorCode::ValidationFailed,
        format!("{ctx}: {err}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_DIR_SEQ: AtomicU64 = AtomicU64::new(0);

    fn temp_staging() -> PathBuf {
        let seq = TEST_DIR_SEQ.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("documents-worker-secure-io-{seq}"));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        root
    }

    #[test]
    fn reads_file_under_canonical_staging() {
        let root = temp_staging();
        let file = root.join("doc.txt");
        fs::write(&file, b"hola").unwrap();
        let bytes = read_staging_file(&root, &file, 1024).unwrap();
        assert_eq!(bytes, b"hola");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn rejects_oversized_file() {
        let root = temp_staging();
        let file = root.join("big.txt");
        fs::write(&file, vec![0u8; 16]).unwrap();
        let err = read_staging_file(&root, &file, 8).unwrap_err();
        assert_eq!(err.code, ProductErrorCode::PolicyDenied);
        let _ = fs::remove_dir_all(&root);
    }
}
