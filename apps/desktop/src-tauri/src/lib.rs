//! Núcleo Tauri: IPC import → storage/jobs (TASK-0027). Sin plugins de red HTTP.

mod commands;
mod import_service;
mod ipc_types;
mod sidecar;
mod state;

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
        ])
        .run(tauri::generate_context!())
        .expect("error al ejecutar la aplicación Tauri");
}

#[cfg(test)]
mod integration_tests {
    use core_domain::{DataClassification, IpcResult};

    use crate::commands::{
        import_sources_inner, job_status_inner, list_sources_inner, notebook_create_inner,
        search_sources_inner,
    };
    use crate::ipc_types::{
        ImportInlineMime, ImportSourceInlineItem, ImportSourcesRequest, JobStatusRequest,
        ListSourcesRequest, NotebookCreateRequest, SearchSourcesRequest,
    };
    use crate::state::AppState;

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

    #[test]
    fn ping_stub() {
        assert_eq!(super::ping(), "pong");
    }
}
