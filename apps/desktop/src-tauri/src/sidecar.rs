//! Spawn del sidecar `documents-worker` (ADR-0009). Sin shell arbitrario.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use core_domain::{IpcResult, ProductError, ProductErrorCode};
use documents_worker::ipc::{
    ExtractionItemRequest, IngestSidecarRequest, IngestSidecarResponse, ItemOutcomeState,
};
use documents_worker::ExtractedChunk;

use storage::PersistedChunk;

const DEFAULT_MAX_BYTES: u64 = 8 * 1024 * 1024;

pub fn worker_binary_from_env() -> Option<PathBuf> {
    std::env::var_os("SANCTUMLM_DOCUMENTS_WORKER")
        .map(PathBuf::from)
        .filter(|p| p.is_file())
}

pub fn spawn_documents_worker(
    worker: &Path,
    staging_root: &Path,
    item_id: &str,
    staging_path: &Path,
    mime: &str,
) -> Result<Vec<PersistedChunk>, ProductError> {
    let request = IngestSidecarRequest {
        staging_root: staging_root.to_string_lossy().into_owned(),
        items: vec![ExtractionItemRequest {
            item_id: item_id.into(),
            staging_path: staging_path.to_string_lossy().into_owned(),
            mime: mime.into(),
            max_bytes: DEFAULT_MAX_BYTES,
            adapter_hint: None,
        }],
    };

    let input = serde_json::to_string(&request)
        .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "sidecar json"))?;

    let mut child = Command::new(worker)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| {
            ProductError::new(
                ProductErrorCode::LocalUnavailable,
                "no se pudo iniciar documents-worker",
            )
        })?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes()).map_err(|_| {
            ProductError::new(ProductErrorCode::LocalUnavailable, "sidecar stdin")
        })?;
    }

    let output = child.wait_with_output().map_err(|_| {
        ProductError::new(ProductErrorCode::LocalUnavailable, "sidecar wait")
    })?;

    if !output.status.success() {
        return Err(ProductError::new(
            ProductErrorCode::LocalUnavailable,
            "sidecar terminó con error",
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let envelope: IpcResult<IngestSidecarResponse> =
        serde_json::from_str(stdout.trim()).map_err(|_| {
            ProductError::new(ProductErrorCode::LocalUnavailable, "sidecar respuesta")
        })?;

    if !envelope.ok {
        return Err(envelope.error.unwrap_or_else(|| {
            ProductError::new(ProductErrorCode::LocalUnavailable, "sidecar falló")
        }));
    }

    let data = envelope
        .data
        .ok_or_else(|| ProductError::new(ProductErrorCode::LocalUnavailable, "sidecar vacío"))?;

    let item = data.items.into_iter().next().ok_or_else(|| {
        ProductError::new(ProductErrorCode::LocalUnavailable, "sidecar sin items")
    })?;

    if item.state != ItemOutcomeState::Completed {
        return Err(ProductError::new(
            ProductErrorCode::LocalUnavailable,
            item.error_message.unwrap_or_else(|| "extracción fallida".into()),
        ));
    }

    let output = item.output.ok_or_else(|| {
        ProductError::new(ProductErrorCode::LocalUnavailable, "sidecar sin output")
    })?;

    Ok(output
        .chunks
        .into_iter()
        .map(chunk_from_sidecar)
        .collect())
}

fn chunk_from_sidecar(chunk: ExtractedChunk) -> PersistedChunk {
    PersistedChunk {
        text: chunk.text,
        page: chunk.page.map(|p| p as i32),
        char_start: Some(chunk.char_start as i32),
        char_end: Some(chunk.char_end as i32),
    }
}
