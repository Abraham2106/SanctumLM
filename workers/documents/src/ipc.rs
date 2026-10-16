//! IPC stdin/stdout JSON entre núcleo y sidecar (ADR-0009, DATA_AND_INTERFACES § sidecar).

use core_domain::{IpcResult, JobErrorCode, ProductError, ProductErrorCode};
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::time::Instant;

use crate::fake::FakeDocumentExtractor;
use crate::staging::{validate_staging_path, validate_staging_root};
use crate::{DocumentExtractor, ExtractionInput, ExtractionMetadata, ExtractionOutput};

const STUB_ADAPTER_ID: &str = "stub-native";
const STUB_ADAPTER_VERSION: &str = "0.0.0-skeleton";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractionItemRequest {
    pub item_id: String,
    pub staging_path: String,
    pub mime: String,
    pub max_bytes: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adapter_hint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IngestSidecarRequest {
    pub staging_root: String,
    pub items: Vec<ExtractionItemRequest>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemOutcomeState {
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtractionItemResponse {
    pub item_id: String,
    pub state: ItemOutcomeState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<ExtractionOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<JobErrorCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IngestSidecarResponse {
    pub items: Vec<ExtractionItemResponse>,
}

pub fn process_ingest_request(
    request: &IngestSidecarRequest,
    staging_root: &PathBuf,
) -> IngestSidecarResponse {
    let extractor = FakeDocumentExtractor;
    let mut items = Vec::with_capacity(request.items.len());

    for item in &request.items {
        items.push(process_one_item(&extractor, staging_root, item));
    }

    IngestSidecarResponse { items }
}

fn process_one_item(
    extractor: &impl DocumentExtractor,
    staging_root: &PathBuf,
    item: &ExtractionItemRequest,
) -> ExtractionItemResponse {
    let started = Instant::now();

    match validate_staging_path(staging_root, &item.staging_path) {
        Ok(validated_path) => match extractor.extract(ExtractionInput {
            staging_path: validated_path,
            mime: item.mime.clone(),
            max_bytes: item.max_bytes,
        }) {
            Ok(mut output) => {
                let duration_ms = started.elapsed().as_millis() as u64;
                output.extraction_metadata = stub_metadata(duration_ms, 0);
                ExtractionItemResponse {
                    item_id: item.item_id.clone(),
                    state: ItemOutcomeState::Completed,
                    output: Some(output),
                    error_code: None,
                    error_message: None,
                }
            }
            Err(err) => failed_item(
                &item.item_id,
                job_code_for_product(&err),
                err.message,
                started,
            ),
        },
        Err(err) => failed_item(
            &item.item_id,
            job_code_for_product(&err),
            err.message,
            started,
        ),
    }
}

fn failed_item(
    item_id: &str,
    error_code: JobErrorCode,
    error_message: String,
    started: Instant,
) -> ExtractionItemResponse {
    let _duration_ms = started.elapsed().as_millis() as u64;
    ExtractionItemResponse {
        item_id: item_id.to_string(),
        state: ItemOutcomeState::Failed,
        output: None,
        error_code: Some(error_code),
        error_message: Some(error_message),
    }
}

fn job_code_for_product(err: &ProductError) -> JobErrorCode {
    match err.code {
        ProductErrorCode::PolicyDenied => JobErrorCode::PolicyDenied,
        _ => JobErrorCode::ExtractionCorrupt,
    }
}

fn stub_metadata(duration_ms: u64, sidecar_exit_code: i32) -> ExtractionMetadata {
    ExtractionMetadata {
        adapter_id: STUB_ADAPTER_ID.into(),
        adapter_version: STUB_ADAPTER_VERSION.into(),
        duration_ms,
        sidecar_exit_code,
    }
}

/// Lee un JSON de stdin, escribe `IpcResult<IngestSidecarResponse>` en stdout.
pub fn run_stdio_sidecar() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let envelope = match serde_json::from_str::<IngestSidecarRequest>(input.trim()) {
        Ok(request) => {
            if request.items.is_empty() {
                IpcResult::err(ProductError::new(
                    ProductErrorCode::ValidationFailed,
                    "items no puede estar vacío",
                ))
            } else {
                match validate_staging_root(&request.staging_root) {
                    Ok(root) => IpcResult::ok(process_ingest_request(&request, &root)),
                    Err(err) => IpcResult::err(err),
                }
            }
        }
        Err(parse_err) => IpcResult::err(ProductError::new(
            ProductErrorCode::ValidationFailed,
            format!("JSON de job inválido: {parse_err}"),
        )),
    };

    let line = serde_json::to_string(&envelope).map_err(|e| io::Error::other(e.to_string()))?;
    let mut stdout = io::stdout().lock();
    stdout.write_all(line.as_bytes())?;
    stdout.write_all(b"\n")?;
    stdout.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ExtractedChunk;
    use core_domain::{ProductError, ProductErrorCode};
    use std::sync::{Arc, Mutex};

    fn sample_request(staging_root: &str, paths: &[(&str, &str)]) -> IngestSidecarRequest {
        IngestSidecarRequest {
            staging_root: staging_root.into(),
            items: paths
                .iter()
                .map(|(id, path)| ExtractionItemRequest {
                    item_id: (*id).into(),
                    staging_path: (*path).into(),
                    mime: "text/plain".into(),
                    max_bytes: 1_000_000,
                    adapter_hint: None,
                })
                .collect(),
        }
    }

    fn normalized_root(staging_root: &str) -> PathBuf {
        validate_staging_root(staging_root).expect("root de prueba válido")
    }

    struct PathCaptureExtractor {
        last_path: Arc<Mutex<Option<PathBuf>>>,
    }

    impl DocumentExtractor for PathCaptureExtractor {
        fn extract(&self, input: ExtractionInput) -> Result<ExtractionOutput, ProductError> {
            *self.last_path.lock().unwrap() = Some(input.staging_path.clone());
            FakeDocumentExtractor.extract(input)
        }
    }

    #[test]
    fn batch_continues_when_one_item_fails() {
        let root = normalized_root("/var/sanctum/staging");
        let response = process_ingest_request(
            &sample_request(
                "/var/sanctum/staging",
                &[
                    ("ok", "/var/sanctum/staging/a.txt"),
                    ("bad", "/tmp/outside.txt"),
                ],
            ),
            &root,
        );

        assert_eq!(response.items.len(), 2);
        assert_eq!(response.items[0].state, ItemOutcomeState::Completed);
        assert_eq!(response.items[1].state, ItemOutcomeState::Failed);
        assert_eq!(
            response.items[1].error_code,
            Some(JobErrorCode::PolicyDenied)
        );
    }

    #[test]
    fn ipc_envelope_ok_shape() {
        let root = normalized_root("/staging");
        let response = process_ingest_request(
            &sample_request("/staging", &[("1", "/staging/x.md")]),
            &root,
        );
        let envelope = IpcResult::ok(response);
        let raw = serde_json::to_value(&envelope).unwrap();
        assert_eq!(raw["ok"], true);
        assert!(raw["data"]["items"].is_array());
    }

    #[test]
    fn stub_output_includes_metadata_and_chunks() {
        let root = normalized_root("/data/staging");
        let response = process_ingest_request(
            &sample_request("/data/staging", &[("doc", "/data/staging/file.txt")]),
            &root,
        );
        let out = response.items[0].output.as_ref().unwrap();
        assert_eq!(out.extraction_metadata.adapter_id, STUB_ADAPTER_ID);
        assert!(!out.chunks.is_empty());
        assert_eq!(out.chunks[0], ExtractedChunk {
            text: out.normalized_text.clone(),
            page: Some(1),
            char_start: 0,
            char_end: out.normalized_text.len() as u32,
        });
    }

    #[test]
    fn extractor_receives_validated_path_not_raw_json() {
        let root = normalized_root("/var/staging");
        let capture = PathCaptureExtractor {
            last_path: Arc::new(Mutex::new(None)),
        };
        let last = Arc::clone(&capture.last_path);
        let item = ExtractionItemRequest {
            item_id: "x".into(),
            staging_path: "/var/staging/./nested/doc.txt".into(),
            mime: "text/plain".into(),
            max_bytes: 1,
            adapter_hint: None,
        };
        let expected = validate_staging_path(&root, &item.staging_path).unwrap();
        assert!(item.staging_path.contains("./"));
        process_one_item(&capture, &root, &item);
        let captured = last.lock().unwrap().take().expect("debe invocar extractor");
        assert_eq!(captured, expected);
    }

    #[test]
    fn invalid_staging_root_rejects_via_validate() {
        let err = validate_staging_root("/bad/../root").unwrap_err();
        assert_eq!(err.code, ProductErrorCode::PolicyDenied);
    }
}
