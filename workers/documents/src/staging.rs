//! Validación de rutas confinadas al directorio de staging (ADR-0009).

use core_domain::{ProductError, ProductErrorCode};
use std::path::{Component, Path, PathBuf};

/// Normaliza y valida `staging_root` (sin componentes `..`).
pub fn validate_staging_root(staging_root: &str) -> Result<PathBuf, ProductError> {
    require_non_empty(staging_root, "staging_root requerido")?;
    let root = Path::new(staging_root);
    if has_parent_traversal(root) {
        return Err(ProductError::new(
            ProductErrorCode::PolicyDenied,
            "staging_root inválido: componentes parent",
        ));
    }
    Ok(normalize_prefix(root))
}

/// Comprueba que `file_path` quede bajo `staging_root` normalizado, sin traversal (`..`).
pub fn validate_staging_path(
    staging_root: &PathBuf,
    file_path: &str,
) -> Result<PathBuf, ProductError> {
    require_non_empty(file_path, "staging_path requerido")?;
    let file = Path::new(file_path);
    if has_parent_traversal(file) {
        return Err(ProductError::new(
            ProductErrorCode::PolicyDenied,
            "ruta fuera de staging: componentes parent",
        ));
    }

    let file_abs = normalize_prefix(file);

    if !file_abs.starts_with(staging_root) {
        return Err(ProductError::new(
            ProductErrorCode::PolicyDenied,
            "ruta fuera de staging acordado",
        ));
    }

    Ok(file_abs)
}

fn require_non_empty(value: &str, message: &str) -> Result<(), ProductError> {
    if value.trim().is_empty() {
        return Err(ProductError::new(
            ProductErrorCode::ValidationFailed,
            message,
        ));
    }
    Ok(())
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
    fn accepts_valid_staging_root() {
        let root = validate_staging_root("/var/sanctum/staging").unwrap();
        assert_eq!(root, PathBuf::from("/var/sanctum/staging"));
    }

    #[test]
    fn rejects_staging_root_with_parent_dir() {
        let err = validate_staging_root("/var/staging/../escape").unwrap_err();
        assert_eq!(err.code, ProductErrorCode::PolicyDenied);
    }

    #[test]
    fn accepts_path_under_staging_root() {
        let root = validate_staging_root(std::env::temp_dir().to_str().unwrap()).unwrap();
        let file = root.join("batch").join("doc.txt");
        validate_staging_path(&root, file.to_str().unwrap()).expect("debe aceptar ruta hija");
    }

    #[test]
    fn rejects_path_outside_staging_root() {
        let root = validate_staging_root("/var/staging").unwrap();
        let err = validate_staging_path(&root, "/etc/passwd").unwrap_err();
        assert_eq!(err.code, ProductErrorCode::PolicyDenied);
    }

    #[test]
    fn rejects_parent_dir_in_item_path() {
        let root = validate_staging_root("/var/staging").unwrap();
        let err = validate_staging_path(&root, "/var/staging/../secret").unwrap_err();
        assert_eq!(err.code, ProductErrorCode::PolicyDenied);
    }

    #[test]
    fn rejects_empty_staging_path() {
        let root = validate_staging_root("/var/staging").unwrap();
        let err = validate_staging_path(&root, "").unwrap_err();
        assert_eq!(err.code, ProductErrorCode::ValidationFailed);
    }

    #[test]
    fn item_path_normalized_before_prefix_check() {
        let root = validate_staging_root("/var/staging").unwrap();
        let validated =
            validate_staging_path(&root, "/var/staging/./nested/doc.txt").unwrap();
        assert_eq!(validated, PathBuf::from("/var/staging/nested/doc.txt"));
    }
}
