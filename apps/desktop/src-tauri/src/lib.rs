//! Núcleo Tauri: IPC import → storage/jobs (TASK-0027). Sin plugins de red HTTP.

pub mod chat_service;
pub mod chunk_search;
pub mod commands;
mod import_service;
pub mod ipc_types;
mod sidecar;
pub mod state;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use storage::open_connection;
use tauri::Manager;

use crate::sidecar::worker_binary_from_env;
use crate::state::AppState;

/// Respuesta mínima para comprobar el canal IPC desde la UI.
#[tauri::command]
fn ping() -> &'static str {
    "pong"
}

fn default_db_path(app: &tauri::App) -> PathBuf {
    app.path()
        .app_data_dir()
        .unwrap_or_else(|_| std::env::temp_dir())
        .join("sanctumlm.db")
}

fn default_staging_root(app: &tauri::App) -> PathBuf {
    app.path()
        .app_data_dir()
        .unwrap_or_else(|_| std::env::temp_dir())
        .join("staging")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let db_path = default_db_path(app);
            if let Some(parent) = db_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let conn = open_connection(&db_path).expect("abrir sqlite");
            let arc = Arc::new(Mutex::new(conn));
            let staging = default_staging_root(app);
            let worker = worker_binary_from_env();
            app.manage(AppState::from_connection(arc, staging, worker));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ping,
            commands::notebook_create,
            commands::notebook_list,
            commands::list_sources,
            commands::import_sources,
            commands::job_status,
            commands::job_cancel,
            commands::search_sources,
            commands::list_models,
            commands::chat_create,
            commands::chat_list,
            commands::chat_send,
        ])
        .run(tauri::generate_context!())
        .expect("error al ejecutar la aplicación Tauri");
}

#[cfg(test)]
mod integration_tests {
    use core_domain::{DataClassification, IpcResult};

    use std::sync::{Arc, Mutex};

    use ai_router::adapter::ModelAdapter;
    use ai_router::types::{ChatRequest, ChatResponse, ChatRole};
    use ai_router::{FakeModelAdapter, ModelRouter};
    use retrieval::search::{ChunkHit, ChunkSearch, SearchQuery};
    use storage::ChatMode;

    use crate::chat_service::list_models_inner;
    use crate::commands::{
        chat_send_inner_state, import_sources_inner, job_cancel_inner, job_status_inner,
        list_sources_inner, notebook_create_inner, search_sources_inner,
    };
    use crate::ipc_types::{
        ChatSendRequest, ChatTurnDto, ImportInlineMime, ImportSourceInlineItem,
        ImportSourcesRequest, JobCancelRequest, JobStatusRequest, ListModelsRequest,
        ListSourcesRequest, NotebookCreateRequest, SearchSourcesRequest,
    };
    use crate::state::AppState;

    struct PanicOnSearch;

    impl ChunkSearch for PanicOnSearch {
        fn search(&self, query: SearchQuery) -> Vec<ChunkHit> {
            if query.notebook_id.is_some() {
                panic!("FTS no debe ejecutarse en chat general");
            }
            Vec::new()
        }
    }

    struct CitationEchoAdapter;

    impl ModelAdapter for CitationEchoAdapter {
        fn adapter_id(&self) -> &str {
            "citation-echo"
        }

        fn list_models(&self) -> Result<Vec<ai_router::types::ModelDescriptor>, core_domain::AdapterError> {
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
                    "Respuesta con cita {{\"chunk_id\":\"{chunk_id}\",\"quote\":\"Evaporacion del ciclo\"}}"
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

    fn wait_job(state: &AppState, job_id: &str) {
        for _ in 0..50 {
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
                if matches!(
                    job.state.as_str(),
                    "completed" | "failed" | "cancelled"
                ) {
                    return;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    }

    #[test]
    fn import_inline_text_is_fts_searchable() {
        let state = AppState::in_memory();
        let nb = notebook_create_inner(
            &state,
            NotebookCreateRequest {
                api_version: 1,
                title: "FTS".into(),
                classification: Some(DataClassification::Public),
            },
        );
        let nb_id = nb.data.unwrap().notebook.notebook_id;

        let imported = import_sources_inner(
            &state,
            ImportSourcesRequest {
                api_version: 1,
                notebook_id: nb_id.clone(),
                items: vec![ImportSourceInlineItem {
                    kind: "inline_text".into(),
                    display_name: "nota.md".into(),
                    mime: ImportInlineMime::TextMarkdown,
                    text: "# Agua\n\nEvaporacion del ciclo hidrologico.".into(),
                    classification: Some(DataClassification::Public),
                }],
            },
        );
        assert!(imported.ok);
        let job_id = imported.data.unwrap().job_id;
        wait_job(&state, &job_id);

        let listed = list_sources_inner(
            &state,
            ListSourcesRequest {
                api_version: 1,
                notebook_id: nb_id.clone(),
            },
        );
        let sources = &listed.data.as_ref().unwrap().sources;
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].status, "ready");
        assert_eq!(sources[0].classification, DataClassification::Public);

        let search = search_sources_inner(
            &state,
            SearchSourcesRequest {
                api_version: 1,
                notebook_id: nb_id,
                query: "Evaporacion".into(),
                limit: Some(5),
                selected_source_ids: None,
            },
        );
        let hits = search.data.unwrap().hits;
        assert!(!hits.is_empty());
        assert!(hits[0].snippet.to_lowercase().contains("evaporacion"));
    }

    #[test]
    fn dedupe_ready_source_by_sha256() {
        let state = AppState::in_memory();
        let nb = notebook_create_inner(
            &state,
            NotebookCreateRequest {
                api_version: 1,
                title: "Dedupe".into(),
                classification: Some(DataClassification::Public),
            },
        );
        let nb_id = nb.data.unwrap().notebook.notebook_id;
        let text = "contenido estable para hash";

        for _ in 0..2 {
            let imported = import_sources_inner(
                &state,
                ImportSourcesRequest {
                    api_version: 1,
                    notebook_id: nb_id.clone(),
                    items: vec![ImportSourceInlineItem {
                        kind: "inline_text".into(),
                        display_name: "a.txt".into(),
                        mime: ImportInlineMime::TextPlain,
                        text: text.into(),
                        classification: Some(DataClassification::Public),
                    }],
                },
            );
            wait_job(&state, &imported.data.unwrap().job_id);
        }

        let listed = list_sources_inner(
            &state,
            ListSourcesRequest {
                api_version: 1,
                notebook_id: nb_id.clone(),
            },
        )
        .data
        .unwrap();
        let ready = listed
            .sources
            .iter()
            .filter(|s| s.status == "ready")
            .count();
        assert_eq!(ready, 1);
    }

    fn batch_inline_items(count: usize) -> Vec<ImportSourceInlineItem> {
        (0..count)
            .map(|i| ImportSourceInlineItem {
                kind: "inline_text".into(),
                display_name: format!("lote-{i}.txt"),
                mime: ImportInlineMime::TextPlain,
                text: format!(
                    "Item {i} del lote cancelacion.\n\nParrafo adicional {i} con lexico unico."
                ),
                classification: Some(DataClassification::Public),
            })
            .collect()
    }

    /// Encola lote, cancela en `running`; verifica terminal `cancelled` + `CANCELLED`.
    /// Cancelación cooperativa: efectiva entre ítems (ver `import_service`).
    #[test]
    fn batch_cancel_while_running_reports_cancelled() {
        let state = AppState::in_memory();
        let nb = notebook_create_inner(
            &state,
            NotebookCreateRequest {
                api_version: 1,
                title: "Cancel".into(),
                classification: Some(DataClassification::Public),
            },
        );
        let nb_id = nb.data.unwrap().notebook.notebook_id;

        let imported = import_sources_inner(
            &state,
            ImportSourcesRequest {
                api_version: 1,
                notebook_id: nb_id,
                items: batch_inline_items(12),
            },
        );
        assert!(imported.ok);
        let job_id = imported.data.unwrap().job_id;

        let mut saw_running = false;
        for _ in 0..300 {
            let status = job_status_inner(
                &state,
                JobStatusRequest {
                    api_version: 1,
                    job_id: job_id.clone(),
                },
            );
            if let IpcResult {
                ok: true,
                data: Some(job),
                ..
            } = status
            {
                if job.state == "running" {
                    saw_running = true;
                    break;
                }
                if job.state == "completed" {
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(2));
        }
        assert!(saw_running, "el job debe alcanzar running antes de cancelar");

        let cancelled = job_cancel_inner(
            &state,
            JobCancelRequest {
                api_version: 1,
                job_id: job_id.clone(),
            },
        );
        assert!(cancelled.ok);
        let record = cancelled.data.unwrap();
        assert_eq!(record.state, "cancelled");
        assert_eq!(record.error_code.as_deref(), Some("CANCELLED"));

        for _ in 0..50 {
            let status = job_status_inner(
                &state,
                JobStatusRequest {
                    api_version: 1,
                    job_id: job_id.clone(),
                },
            );
            if let IpcResult {
                ok: true,
                data: Some(job),
                ..
            } = status
            {
                assert_eq!(job.state, "cancelled");
                assert_eq!(job.error_code.as_deref(), Some("CANCELLED"));
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        panic!("job_status no reflejó cancelación terminal");
    }

    #[test]
    fn ping_stub() {
        assert_eq!(super::ping(), "pong");
    }

    #[test]
    fn list_models_uses_router_fake() {
        let state = AppState::in_memory();
        let resp = list_models_inner(
            &state.chat,
            ListModelsRequest { api_version: 1 },
        );
        assert!(resp.ok);
        assert!(!resp.data.unwrap().models.is_empty());
    }

    #[test]
    fn general_chat_does_not_invoke_fts() {
        let conn = storage::open_memory_connection().expect("db");
        let arc = Arc::new(Mutex::new(conn));
        let staging = std::env::temp_dir().join("sanctumlm-chat-general-test");
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
        assert!(data.content.contains("fake-echo"));
        assert!(data.citations.is_empty());
    }

    #[test]
    fn notebook_chat_persists_validated_citations() {
        let conn = storage::open_memory_connection().expect("db");
        let arc = Arc::new(Mutex::new(conn));
        let staging = std::env::temp_dir().join("sanctumlm-chat-nb-test");
        let _ = std::fs::create_dir_all(&staging);
        let state = AppState::from_connection_with_router(
            Arc::clone(&arc),
            staging,
            None,
            ModelRouter::local_only(CitationEchoAdapter),
            Arc::new(crate::chunk_search::StorageChunkSearch::new(arc)),
        );

        let nb = notebook_create_inner(
            &state,
            NotebookCreateRequest {
                api_version: 1,
                title: "Chat NB".into(),
                classification: Some(DataClassification::Public),
            },
        );
        let nb_id = nb.data.unwrap().notebook.notebook_id;

        let imported = import_sources_inner(
            &state,
            ImportSourcesRequest {
                api_version: 1,
                notebook_id: nb_id.clone(),
                items: vec![ImportSourceInlineItem {
                    kind: "inline_text".into(),
                    display_name: "nota.md".into(),
                    mime: ImportInlineMime::TextMarkdown,
                    text: "Evaporacion del ciclo hidrologico.".into(),
                    classification: Some(DataClassification::Public),
                }],
            },
        );
        wait_job(&state, &imported.data.unwrap().job_id);

        let search = search_sources_inner(
            &state,
            SearchSourcesRequest {
                api_version: 1,
                notebook_id: nb_id.clone(),
                query: "Evaporacion".into(),
                limit: Some(1),
                selected_source_ids: None,
            },
        );
        let _chunk_id = search.data.unwrap().hits[0].chunk_id.clone();

        let resp = chat_send_inner_state(
            &state,
            ChatSendRequest {
                api_version: 1,
                chat_id: None,
                mode: "notebook".into(),
                notebook_id: Some(nb_id.clone()),
                model_id: "llama3.2:latest".into(),
                messages: vec![ChatTurnDto {
                    role: "user".into(),
                    content: "Evaporacion".into(),
                }],
                selected_source_ids: None,
            },
        );
        assert!(resp.ok, "{:?}", resp.error);
        let data = resp.data.unwrap();
        assert!(!data.citations.is_empty());

        let listed = crate::chat_service::chat_list_inner(
            &state.chat,
            crate::ipc_types::ChatListRequest {
                api_version: 1,
                mode: Some("notebook".into()),
                notebook_id: Some(nb_id),
            },
        );
        let list = listed.data.unwrap();
        let msgs = list.messages_by_chat.get(&data.chat_id).expect("messages");
        assert_eq!(msgs.len(), 2);
        assert!(!msgs[1].citations.is_empty());
    }

    #[test]
    fn chat_send_rejects_mode_mismatch() {
        let state = AppState::in_memory();
        let nb = notebook_create_inner(
            &state,
            NotebookCreateRequest {
                api_version: 1,
                title: "NB".into(),
                classification: Some(DataClassification::Public),
            },
        );
        let nb_id = nb.data.unwrap().notebook.notebook_id;
        let created = state
            .chat
            .chats
            .create_chat(ChatMode::General, None, Some("g"))
            .expect("chat");
        let resp = chat_send_inner_state(
            &state,
            ChatSendRequest {
                api_version: 1,
                chat_id: Some(created.chat_id),
                mode: "notebook".into(),
                notebook_id: Some(nb_id),
                model_id: "llama3.2:latest".into(),
                messages: vec![ChatTurnDto {
                    role: "user".into(),
                    content: "hola".into(),
                }],
                selected_source_ids: None,
            },
        );
        assert!(!resp.ok);
        assert_eq!(
            resp.error.unwrap().code,
            core_domain::ProductErrorCode::ValidationFailed
        );
    }
}
