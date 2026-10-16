use std::collections::HashMap;
use std::sync::Arc;

use core_domain::{IpcResult, NotebookId, ProductError, ProductErrorCode};
use storage::{parse_job_id, search_sources_fts, source_meta, NotebookRepository, SourceMeta};
use tauri::State;

use crate::chat_service::{
    chat_create_inner, chat_list_inner, chat_send_inner, list_models_inner,
};
use crate::ipc_types::{
    check_api_version, ChatCreateRequest, ChatCreateResponse, ChatListRequest, ChatListResponse,
    ChatSendRequest, ChatSendResponse, ImportSourcesRequest, ImportSourcesResponse,
    JobCancelRequest, JobRecordDto, JobStatusRequest, ListModelsRequest, ListModelsResponse,
    ListSourcesRequest, ListSourcesResponse, NotebookCreateRequest, NotebookCreateResponse,
    NotebookListRequest, NotebookListResponse, NotebookSummaryDto, SearchHitDto,
    SearchSourcesRequest, SearchSourcesResponse, SourceRecordDto,
};
use crate::state::AppState;

fn ensure_notebook_exists(state: &AppState, notebook_id: &str) -> Result<(), ProductError> {
    state
        .notebooks
        .lock()
        .expect("notebook lock")
        .get(&NotebookId(notebook_id.to_string()))
        .map(|_| ())
}

fn source_to_dto(meta: SourceMeta) -> SourceRecordDto {
    SourceRecordDto {
        source_id: meta.source_id.0,
        notebook_id: meta.notebook_id.0,
        display_name: meta.display_name,
        kind: match meta.kind {
            source_meta::SourceKind::File => "file".into(),
            source_meta::SourceKind::Paste => "paste".into(),
            source_meta::SourceKind::Url => "url".into(),
        },
        mime: meta.mime,
        status: match meta.status {
            source_meta::SourceStatus::Pending => "pending".into(),
            source_meta::SourceStatus::Processing => "processing".into(),
            source_meta::SourceStatus::Ready => "ready".into(),
            source_meta::SourceStatus::Failed => "failed".into(),
            source_meta::SourceStatus::Partial => "partial".into(),
        },
        classification: meta.classification,
        sha256_prefix: Some(meta.sha256_prefix),
        updated_at: meta.updated_at,
    }
}

pub fn notebook_create_inner(
    state: &AppState,
    request: NotebookCreateRequest,
) -> IpcResult<NotebookCreateResponse> {
    if let Err(err) = check_api_version(request.api_version) {
        return IpcResult::err(err);
    }
    let classification = request
        .classification
        .unwrap_or(core_domain::DataClassification::Private);
    let mut repo = state.notebooks.lock().expect("notebook lock");
    let created = repo.create(request.title, classification);
    IpcResult::ok(NotebookCreateResponse {
        notebook: NotebookSummaryDto {
            notebook_id: created.id.0,
            title: created.title,
            classification: created.classification,
        },
    })
}

#[tauri::command]
pub fn notebook_create(
    state: State<'_, AppState>,
    request: NotebookCreateRequest,
) -> IpcResult<NotebookCreateResponse> {
    notebook_create_inner(&state, request)
}

#[tauri::command]
pub fn notebook_list(
    state: State<'_, AppState>,
    request: NotebookListRequest,
) -> IpcResult<NotebookListResponse> {
    if let Err(err) = check_api_version(request.api_version) {
        return IpcResult::err(err);
    }
    let repo = state.notebooks.lock().expect("notebook lock");
    let notebooks = repo
        .list()
        .into_iter()
        .map(|n| NotebookSummaryDto {
            notebook_id: n.id.0,
            title: n.title,
            classification: n.classification,
        })
        .collect();
    IpcResult::ok(NotebookListResponse { notebooks })
}

pub fn list_sources_inner(
    state: &AppState,
    request: ListSourcesRequest,
) -> IpcResult<ListSourcesResponse> {
    if let Err(err) = check_api_version(request.api_version) {
        return IpcResult::err(err);
    }
    let notebook_id = NotebookId(request.notebook_id);
    match state.sources.list_by_notebook(&notebook_id) {
        Ok(rows) => IpcResult::ok(ListSourcesResponse {
            sources: rows.into_iter().map(source_to_dto).collect(),
        }),
        Err(err) => IpcResult::err(err),
    }
}

#[tauri::command]
pub fn list_sources(
    state: State<'_, AppState>,
    request: ListSourcesRequest,
) -> IpcResult<ListSourcesResponse> {
    list_sources_inner(&state, request)
}

pub fn import_sources_inner(
    state: &AppState,
    request: ImportSourcesRequest,
) -> IpcResult<ImportSourcesResponse> {
    if let Err(err) = check_api_version(request.api_version) {
        return IpcResult::err(err);
    }
    if state
        .notebooks
        .lock()
        .expect("notebook lock")
        .get(&NotebookId(request.notebook_id.clone()))
        .is_err()
    {
        return IpcResult::err(ProductError::not_found("notebook"));
    }
    let response = state.import_runtime.enqueue_batch(
        Arc::clone(&state.jobs),
        NotebookId(request.notebook_id),
        request.items,
    );
    IpcResult::ok(response)
}

#[tauri::command]
pub fn import_sources(
    state: State<'_, AppState>,
    request: ImportSourcesRequest,
) -> IpcResult<ImportSourcesResponse> {
    import_sources_inner(&state, request)
}

pub fn job_status_inner(state: &AppState, request: JobStatusRequest) -> IpcResult<JobRecordDto> {
    if let Err(err) = check_api_version(request.api_version) {
        return IpcResult::err(err);
    }
    let job_id = match parse_job_id(&request.job_id) {
        Ok(id) => id,
        Err(err) => return IpcResult::err(err),
    };
    match state.jobs.get(&job_id) {
        Some(job) => IpcResult::ok(JobRecordDto::from(job)),
        None => IpcResult::err(ProductError::not_found("job")),
    }
}

#[tauri::command]
pub fn job_status(
    state: State<'_, AppState>,
    request: JobStatusRequest,
) -> IpcResult<JobRecordDto> {
    job_status_inner(&state, request)
}

pub fn job_cancel_inner(state: &AppState, request: JobCancelRequest) -> IpcResult<JobRecordDto> {
    if let Err(err) = check_api_version(request.api_version) {
        return IpcResult::err(err);
    }
    let job_id = match parse_job_id(&request.job_id) {
        Ok(id) => id,
        Err(err) => return IpcResult::err(err),
    };
    state.import_runtime.cancel_job(job_id);
    match state.jobs.mark_cancelled(&job_id) {
        Ok(job) => IpcResult::ok(JobRecordDto::from(job)),
        Err(err) => IpcResult::err(err),
    }
}

#[tauri::command]
pub fn job_cancel(
    state: State<'_, AppState>,
    request: JobCancelRequest,
) -> IpcResult<JobRecordDto> {
    job_cancel_inner(&state, request)
}

pub fn search_sources_inner(
    state: &AppState,
    request: SearchSourcesRequest,
) -> IpcResult<SearchSourcesResponse> {
    if let Err(err) = check_api_version(request.api_version) {
        return IpcResult::err(err);
    }
    let notebook_id = NotebookId(request.notebook_id.clone());
    let limit = request.limit.unwrap_or(25).max(1) as usize;
    let selected = request
        .selected_source_ids
        .unwrap_or_default()
        .into_iter()
        .map(core_domain::SourceId)
        .collect::<Vec<_>>();

    let display_names: HashMap<String, String> = state
        .sources
        .list_by_notebook(&notebook_id)
        .unwrap_or_default()
        .into_iter()
        .map(|s| (s.source_id.0, s.display_name))
        .collect();

    let hits_raw = search_sources_fts(
        &state.conn,
        &notebook_id,
        &request.query,
        &selected,
        limit,
    );

    let hits = hits_raw
        .into_iter()
        .enumerate()
        .map(|(idx, hit)| {
            let snippet = if hit.text.len() > 240 {
                format!("{}…", &hit.text[..237])
            } else {
                hit.text.clone()
            };
            SearchHitDto {
                chunk_id: hit.chunk_id.0,
                source_id: hit.source_id.0.clone(),
                source_display_name: display_names
                    .get(&hit.source_id.0)
                    .cloned()
                    .unwrap_or_else(|| hit.source_id.0.clone()),
                snippet,
                page: hit.page,
                char_start: hit.char_start.unwrap_or(0),
                char_end: hit.char_end.unwrap_or(hit.text.len() as i32),
                rank: hit.rank.max(1.0) + idx as f32 * 0.001,
            }
        })
        .collect();

    IpcResult::ok(SearchSourcesResponse {
        query: request.query,
        hits,
    })
}

#[tauri::command]
pub fn search_sources(
    state: State<'_, AppState>,
    request: SearchSourcesRequest,
) -> IpcResult<SearchSourcesResponse> {
    search_sources_inner(&state, request)
}

#[tauri::command]
pub fn list_models(
    state: State<'_, AppState>,
    request: ListModelsRequest,
) -> IpcResult<ListModelsResponse> {
    if let Err(err) = check_api_version(request.api_version) {
        return IpcResult::err(err);
    }
    list_models_inner(&state.chat, request)
}

pub fn chat_create_inner_state(
    state: &AppState,
    request: ChatCreateRequest,
) -> IpcResult<ChatCreateResponse> {
    if let Err(err) = check_api_version(request.api_version) {
        return IpcResult::err(err);
    }
    if request.mode == "notebook" {
        let Some(nb) = request.notebook_id.as_deref() else {
            return IpcResult::err(ProductError::new(
                ProductErrorCode::ValidationFailed,
                "notebook_id es obligatorio en modo notebook",
            ));
        };
        if let Err(err) = ensure_notebook_exists(state, nb) {
            return IpcResult::err(err);
        }
    }
    chat_create_inner(&state.chat, request)
}

#[tauri::command]
pub fn chat_create(
    state: State<'_, AppState>,
    request: ChatCreateRequest,
) -> IpcResult<ChatCreateResponse> {
    chat_create_inner_state(&state, request)
}

#[tauri::command]
pub fn chat_list(
    state: State<'_, AppState>,
    request: ChatListRequest,
) -> IpcResult<ChatListResponse> {
    if let Err(err) = check_api_version(request.api_version) {
        return IpcResult::err(err);
    }
    chat_list_inner(&state.chat, request)
}

pub fn chat_send_inner_state(
    state: &AppState,
    request: ChatSendRequest,
) -> IpcResult<ChatSendResponse> {
    if let Err(err) = check_api_version(request.api_version) {
        return IpcResult::err(err);
    }
    if request.mode == "notebook" {
        let Some(nb) = request.notebook_id.as_deref() else {
            return IpcResult::err(ProductError::new(
                ProductErrorCode::ValidationFailed,
                "notebook_id es obligatorio en modo notebook",
            ));
        };
        if let Err(err) = ensure_notebook_exists(state, nb) {
            return IpcResult::err(err);
        }
    }
    chat_send_inner(&state.chat, request)
}

#[tauri::command]
pub fn chat_send(
    state: State<'_, AppState>,
    request: ChatSendRequest,
) -> IpcResult<ChatSendResponse> {
    chat_send_inner_state(&state, request)
}
