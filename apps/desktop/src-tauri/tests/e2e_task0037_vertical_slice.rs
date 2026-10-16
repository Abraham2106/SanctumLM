//! Evidencia E2E reproducible (TASK-0037): núcleo Tauri vía IPC interno.
//!
//! Flujo: notebook → import inline → job terminal → FTS → chat notebook con citas.
//! Cobertura **parcial** documentada: R-PROD-001, 002, 006, 105 (no sustituye TC-PROD formal ni G2).

use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use ai_router::adapter::ModelAdapter;
use ai_router::types::{ChatRequest, ChatResponse, ChatRole};
use ai_router::{FakeModelAdapter, ModelRouter};
use core_domain::{DataClassification, IpcResult};
use retrieval::search::{ChunkHit, ChunkSearch, SearchQuery};
use sanctumlm_desktop_lib::chat_service::{chat_list_inner, list_models_inner};
use sanctumlm_desktop_lib::commands::{
    chat_send_inner_state, import_sources_inner, job_status_inner, list_sources_inner,
    notebook_create_inner, search_sources_inner,
};
use sanctumlm_desktop_lib::ipc_types::{
    ChatListRequest, ChatSendRequest, ChatTurnDto, ImportInlineMime, ImportSourceInlineItem,
    ImportSourcesRequest, JobStatusRequest, ListModelsRequest, ListSourcesRequest,
    NotebookCreateRequest, SearchSourcesRequest,
};
use sanctumlm_desktop_lib::chunk_search::StorageChunkSearch;
use sanctumlm_desktop_lib::state::AppState;

const CORPUS_REL: &str =
    "benchmarks/es-mvp-rag/sources/01-politica-vacaciones.md";
const NB_A_REL: &str =
    "benchmarks/es-mvp-rag/sources/notebook-a/10-concepto-alfa-capacidad.md";
const NB_B_REL: &str =
    "benchmarks/es-mvp-rag/sources/notebook-b/10-concepto-alfa-capacidad.md";

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("resolver raíz del workspace")
}

fn read_corpus(rel: &str) -> String {
    let path = workspace_root().join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!("no se pudo leer corpus de benchmark {}: {e}", path.display())
    })
}

struct CitationEchoAdapter;

impl ModelAdapter for CitationEchoAdapter {
    fn adapter_id(&self) -> &str {
        "citation-echo-e2e"
    }

    fn list_models(
        &self,
    ) -> Result<Vec<ai_router::types::ModelDescriptor>, core_domain::AdapterError> {
        FakeModelAdapter::ollama_loopback_stub().list_models()
    }

    fn describe_capabilities(
        &self,
        model_id: &str,
    ) -> Result<ai_router::types::ModelCapabilities, core_domain::AdapterError> {
        FakeModelAdapter::ollama_loopback_stub().describe_capabilities(model_id)
    }

    fn chat(&self, request: ChatRequest) -> Result<ChatResponse, core_domain::AdapterError> {
        let chunk_id = request
            .messages
            .iter()
            .find(|m| matches!(m.role, ChatRole::System))
            .and_then(|m| {
                m.content
                    .split("chunk_id=")
                    .nth(1)
                    .and_then(|rest| rest.split_whitespace().next())
            })
            .unwrap_or("1");
        Ok(ChatResponse {
            content: format!(
                "E2E: cita validada {{\"chunk_id\":\"{chunk_id}\",\"quote\":\"antigüedad\"}}"
            ),
            model_id: request.model_id,
            adapter_id: self.adapter_id().into(),
        })
    }

    fn embed(
        &self,
        request: ai_router::types::EmbedRequest,
    ) -> Result<ai_router::types::EmbedResponse, core_domain::AdapterError> {
        FakeModelAdapter::ollama_loopback_stub().embed(request)
    }
}

fn e2e_state() -> AppState {
    let conn = storage::open_memory_connection().expect("sqlite memoria");
    let arc = Arc::new(Mutex::new(conn));
    let staging = std::env::temp_dir().join("sanctumlm-e2e-task0037");
    let _ = std::fs::create_dir_all(&staging);
    AppState::from_connection_with_router(
        Arc::clone(&arc),
        staging,
        None,
        ModelRouter::local_only(CitationEchoAdapter),
        Arc::new(StorageChunkSearch::new(arc)),
    )
}

fn wait_job_terminal(state: &AppState, job_id: &str) {
    for _ in 0..80 {
        let status = job_status_inner(
            state,
            JobStatusRequest {
                api_version: 1,
                job_id: job_id.to_string(),
            },
        );
        if let IpcResult {
            ok: true,
            data: Some(job),
            ..
        } = status
        {
            if matches!(job.state.as_str(), "completed" | "failed" | "cancelled") {
                assert_eq!(job.state, "completed", "job import debe completar: {:?}", job);
                return;
            }
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    panic!("timeout esperando job {job_id}");
}

fn import_inline(
    state: &AppState,
    notebook_id: &str,
    display_name: &str,
    text: String,
) -> String {
    let imported = import_sources_inner(
        state,
        ImportSourcesRequest {
            api_version: 1,
            notebook_id: notebook_id.to_string(),
            items: vec![ImportSourceInlineItem {
                kind: "inline_text".into(),
                display_name: display_name.into(),
                mime: ImportInlineMime::TextMarkdown,
                text,
                classification: Some(DataClassification::Private),
            }],
        },
    );
    assert!(imported.ok, "{:?}", imported.error);
    let job_id = imported.data.unwrap().job_id;
    wait_job_terminal(state, &job_id);
    job_id
}

/// R-PROD-001 (crear notebook), 002 (import ready), FTS, 006 (chat notebook + citas).
#[test]
fn e2e_task0037_import_fts_notebook_chat() {
    let state = e2e_state();
    let corpus = read_corpus(CORPUS_REL);
    assert!(
        corpus.contains("SRC-01-politica-vacaciones"),
        "corpus benchmark esperado"
    );

    let nb = notebook_create_inner(
        &state,
        NotebookCreateRequest {
            api_version: 1,
            title: "E2E TASK-0037".into(),
            classification: Some(DataClassification::Private),
        },
    );
    assert!(nb.ok);
    let nb_id = nb.data.unwrap().notebook.notebook_id;
    eprintln!("[e2e-0037] notebook_id={}", nb_id);

    import_inline(&state, &nb_id, "01-politica-vacaciones.md", corpus);

    let listed = list_sources_inner(
        &state,
        ListSourcesRequest {
            api_version: 1,
            notebook_id: nb_id.clone(),
        },
    )
    .data
    .unwrap();
    assert_eq!(listed.sources.len(), 1);
    let src = &listed.sources[0];
    assert_eq!(src.status, "ready");
    assert!(src.sha256_prefix.is_some());
    eprintln!(
        "[e2e-0037] source_id={} sha256_prefix={}",
        src.source_id,
        src.sha256_prefix.as_deref().unwrap_or("?")
    );

    let search = search_sources_inner(
        &state,
        SearchSourcesRequest {
            api_version: 1,
            notebook_id: nb_id.clone(),
            query: "antigüedad".into(),
            limit: Some(5),
            selected_source_ids: None,
        },
    );
    assert!(search.ok);
    let hits = search.data.unwrap().hits;
    assert!(!hits.is_empty(), "FTS debe devolver al menos un hit");
    eprintln!(
        "[e2e-0037] fts_hits={} top_snippet={}",
        hits.len(),
        hits[0].snippet.chars().take(80).collect::<String>()
    );
    assert!(
        hits[0]
            .snippet
            .to_lowercase()
            .contains("antig")
            || hits[0].snippet.contains("antigüedad")
    );

    let models = list_models_inner(&state.chat, ListModelsRequest { api_version: 1 });
    assert!(models.ok);
    let model_id = models.data.unwrap().models[0].model_id.clone();

    let chat = chat_send_inner_state(
        &state,
        ChatSendRequest {
            api_version: 1,
            chat_id: None,
            mode: "notebook".into(),
            notebook_id: Some(nb_id.clone()),
            model_id,
            messages: vec![ChatTurnDto {
                role: "user".into(),
                content: "antigüedad".into(),
            }],
            selected_source_ids: None,
        },
    );
    assert!(chat.ok, "{:?}", chat.error);
    let turn = chat.data.unwrap();
    assert!(
        !turn.citations.is_empty(),
        "chat notebook debe citar FTS; content={}",
        turn.content
    );
    eprintln!(
        "[e2e-0037] chat_id={} citations={}",
        turn.chat_id,
        turn.citations.len()
    );

    let history = chat_list_inner(
        &state.chat,
        ChatListRequest {
            api_version: 1,
            mode: Some("notebook".into()),
            notebook_id: Some(nb_id),
        },
    );
    let list = history.data.unwrap();
    let msgs = list
        .messages_by_chat
        .get(&turn.chat_id)
        .expect("mensajes persistidos");
    assert_eq!(msgs.len(), 2);
    assert!(!msgs[1].citations.is_empty());
}

/// R-PROD-105 (parcial): búsqueda acotada al notebook activo.
#[test]
fn e2e_task0037_notebook_fts_isolation() {
    let state = e2e_state();
    let text_a = read_corpus(NB_A_REL);
    let text_b = read_corpus(NB_B_REL);

    let nb_a = notebook_create_inner(
        &state,
        NotebookCreateRequest {
            api_version: 1,
            title: "Notebook A".into(),
            classification: Some(DataClassification::Private),
        },
    )
    .data
    .unwrap()
    .notebook
    .notebook_id;
    let nb_b = notebook_create_inner(
        &state,
        NotebookCreateRequest {
            api_version: 1,
            title: "Notebook B".into(),
            classification: Some(DataClassification::Private),
        },
    )
    .data
    .unwrap()
    .notebook
    .notebook_id;

    import_inline(&state, &nb_a, "10a.md", text_a);
    import_inline(&state, &nb_b, "10b.md", text_b);

    let hits_a = search_sources_inner(
        &state,
        SearchSourcesRequest {
            api_version: 1,
            notebook_id: nb_a.clone(),
            query: "ALFA-A".into(),
            limit: Some(10),
            selected_source_ids: None,
        },
    )
    .data
    .unwrap()
    .hits;
    assert!(!hits_a.is_empty());
    for h in &hits_a {
        assert!(
            h.snippet.contains("ALFA-A") || h.snippet.contains("500"),
            "hit en A: {}",
            h.snippet
        );
        assert!(
            !h.snippet.contains("ALFA-B"),
            "filtración notebook B en A: {}",
            h.snippet
        );
    }

    let hits_b = search_sources_inner(
        &state,
        SearchSourcesRequest {
            api_version: 1,
            notebook_id: nb_b.clone(),
            query: "ALFA-B".into(),
            limit: Some(10),
            selected_source_ids: None,
        },
    )
    .data
    .unwrap()
    .hits;
    assert!(!hits_b.is_empty());
    for h in &hits_b {
        assert!(
            h.snippet.contains("ALFA-B") || h.snippet.contains("120"),
            "hit en B: {}",
            h.snippet
        );
        assert!(
            !h.snippet.contains("ALFA-A"),
            "filtración notebook A en B: {}",
            h.snippet
        );
    }
}

struct PanicOnSearch;

impl ChunkSearch for PanicOnSearch {
    fn search(&self, query: SearchQuery) -> Vec<ChunkHit> {
        if query.notebook_id.is_some() {
            panic!("FTS no debe ejecutarse en chat general");
        }
        Vec::new()
    }
}

/// R-PROD-007 (parcial, coherente con 006): chat general sin recuperación FTS.
#[test]
fn e2e_task0037_general_chat_skips_notebook_fts() {
    let conn = storage::open_memory_connection().expect("db");
    let arc = Arc::new(Mutex::new(conn));
    let staging = std::env::temp_dir().join("sanctumlm-e2e-general");
    let _ = std::fs::create_dir_all(&staging);
    let state = AppState::from_connection_with_router(
        Arc::clone(&arc),
        staging,
        None,
        ModelRouter::local_only(FakeModelAdapter::ollama_loopback_stub()),
        Arc::new(PanicOnSearch),
    );

    let resp = chat_send_inner_state(
        &state,
        ChatSendRequest {
            api_version: 1,
            chat_id: None,
            mode: "general".into(),
            notebook_id: None,
            model_id: "llama3.2:latest".into(),
            messages: vec![ChatTurnDto {
                role: "user".into(),
                content: "hola".into(),
            }],
            selected_source_ids: None,
        },
    );
    assert!(resp.ok);
    let data = resp.data.unwrap();
    assert!(data.citations.is_empty());
    assert!(data.content.contains("fake-echo"));
}
