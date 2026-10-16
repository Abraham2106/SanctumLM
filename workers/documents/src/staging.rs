//! Validación de rutas confinadas al directorio de staging (ADR-0009).

use core_domain::{ProductError, ProductErrorCode};
use std::path::{Component, Path, PathBuf};

/// Comprueba que `file_path` quede bajo `staging_root` sin traversal (`..`).
pub fn validate_staging_path(
    staging_root: &str,
    file_path: &str,
) -> Result<PathBuf, ProductError> {
    if staging_root.trim().is_empty() {
        return Err(ProductError::new(
            ProductErrorCode::ValidationFailed,
            "staging_root requerido",
        ));
    }
    if file_path.trim().is_empty() {
        return Err(ProductError::new(
            ProductErrorCode::ValidationFailed,
            "staging_path requerido",
        ));
    }

    let root = Path::new(staging_root);
    let file = Path::new(file_path);

    if has_parent_traversal(file) {
        return Err(ProductError::new(
            ProductErrorCode::PolicyDenied,
            "ruta fuera de staging: componentes parent",
        ));
    }

    let root_abs = normalize_prefix(root);
    let file_abs = normalize_prefix(file);

    if !file_abs.starts_with(&root_abs) {
        return Err(ProductError::new(
            ProductErrorCode::PolicyDenied,
            "ruta fuera de staging acordado",
        ));
    }

    Ok(file_abs)
}

fn has_parent_traversal(path: &Path) -> bool {
    path.components().any(|c| matches!(c, Component::ParentDir))
}

/// Normaliza separadores y elimina prefijos `./` para comparación por prefijo.
fn normalize_prefix(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => {
                out.push(other.as_os_str());
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_path_under_staging_root() {
        let root = std::env::temp_dir().join("sanctum-staging-test");
        let file = root.join("batch").join("doc.txt");
        validate_staging_path(
            root.to_str().unwrap(),
            file.to_str().unwrap(),
        )
        .expect("debe aceptar ruta hija");
    }

    #[test]
    fn rejects_path_outside_staging_root() {
        let err = validate_staging_path("/var/staging", "/etc/passwd").unwrap_err();
        assert_eq!(err.code, ProductErrorCode::PolicyDenied);
    }

    #[test]
    fn rejects_parent_dir_in_path() {
        let err = validate_staging_path("/var/staging", "/var/staging/../secret").unwrap_err();
        assert_eq!(err.code, ProductErrorCode::PolicyDenied);
    }

    #[test]
    fn rejects_empty_staging_path() {
        let err = validate_staging_path("/var/staging", "").unwrap_err();
        assert_eq!(err.code, ProductErrorCode::ValidationFailed);
    }
}
