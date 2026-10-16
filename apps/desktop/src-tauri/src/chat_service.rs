use std::sync::{Arc, Mutex};

use ai_router::types::{ChatMessage as RouterMessage, ChatRequest, ChatRole as RouterRole};
use ai_router::ModelRouter;
use core_domain::{
    AdapterError, IpcResult, NotebookId, ProductError, ProductErrorCode, SourceId,
};
use retrieval::citations::{
    extract_citation_proposals, CitationScope, CitationValidator, CitationVerification,
};
use retrieval::rag::{format_context_prompt, LexicalRag, RagOutcome, RagQuery};
use retrieval::search::ChunkSearch;
use storage::{
    ChatMessageRole, ChatMode, SqliteChatRepository, StoredCitation,
};

use crate::ipc_types::{
    ChatCitationDto, ChatCreateRequest, ChatCreateResponse, ChatListRequest, ChatListResponse,
    ChatMessageRecordDto, ChatSendRequest, ChatSendResponse, ChatSummaryDto, ListModelsRequest,
    ListModelsResponse, ModelCapabilitiesDto, ModelDescriptorDto,
};

pub struct ChatRuntime {
    pub router: Mutex<ModelRouter>,
    pub search: Arc<dyn ChunkSearch + Send + Sync>,
    pub chats: SqliteChatRepository,
    pub citations: CitationValidator,
}

fn adapter_err(err: AdapterError) -> ProductError {
    match err {
        AdapterError::LocalUnavailable => {
            ProductError::new(ProductErrorCode::LocalUnavailable, "modelo local no disponible")
        }
        AdapterError::ModelNotFound => {
            ProductError::new(ProductErrorCode::NotFound, "modelo no encontrado")
        }
        AdapterError::CapabilityUnsupported => ProductError::new(
            ProductErrorCode::ValidationFailed,
            "capacidad no soportada por el modelo",
        ),
        AdapterError::Timeout => {
            ProductError::new(ProductErrorCode::LocalUnavailable, "tiempo de espera agotado")
        }
        AdapterError::Cancelled => {
            ProductError::new(ProductErrorCode::JobCancelled, "inferencia cancelada")
        }
        AdapterError::PolicyDenied => {
            ProductError::new(ProductErrorCode::PolicyDenied, "política de inferencia denegada")
        }
        AdapterError::RemoteUnauthorized | AdapterError::InvalidResponse | AdapterError::RateLimited => {
            ProductError::new(ProductErrorCode::LocalUnavailable, "adaptador remoto inactivo")
        }
    }
}

fn parse_mode(mode: &str) -> Result<ChatMode, ProductError> {
    match mode {
        "general" => Ok(ChatMode::General),
        "notebook" => Ok(ChatMode::Notebook),
        _ => Err(ProductError::new(
            ProductErrorCode::ValidationFailed,
            "mode inválido",
        )),
    }
}

fn parse_ipc_role(role: &str) -> Result<ChatMessageRole, ProductError> {
    match role {
        "system" => Ok(ChatMessageRole::System),
        "user" => Ok(ChatMessageRole::User),
        "assistant" => Ok(ChatMessageRole::Assistant),
        _ => Err(ProductError::new(
            ProductErrorCode::ValidationFailed,
            "role inválido",
        )),
    }
}

fn to_router_role(role: ChatMessageRole) -> RouterRole {
    match role {
        ChatMessageRole::System => RouterRole::System,
        ChatMessageRole::User => RouterRole::User,
        ChatMessageRole::Assistant => RouterRole::Assistant,
    }
}

fn verification_str(v: CitationVerification) -> &'static str {
    match v {
        CitationVerification::Backed => "backed",
        CitationVerification::Inference => "inference",
        CitationVerification::Discrepancy => "discrepancy",
        CitationVerification::Unverified => "unverified",
    }
}

fn to_citation_dto(c: &StoredCitation) -> ChatCitationDto {
    ChatCitationDto {
        chunk_id: c.chunk_id.clone(),
        source_version_id: c.source_version_id.clone(),
        quote: c.quote.clone(),
        verification: c.verification.clone(),
    }
}

fn chat_summary(record: &storage::ChatRecord) -> ChatSummaryDto {
    ChatSummaryDto {
        chat_id: record.chat_id.clone(),
        mode: record.mode.as_str().into(),
        notebook_id: record.notebook_id.clone(),
        title: record.title.clone(),
        created_at: record.created_at.clone(),
        updated_at: record.updated_at.clone(),
    }
}

fn message_dto(record: &storage::MessageRecord) -> ChatMessageRecordDto {
    ChatMessageRecordDto {
        message_id: record.message_id.clone(),
        chat_id: record.chat_id.clone(),
        role: record.role.as_str().into(),
        content: record.content.clone(),
        model_id: record.model_id.clone(),
        citations: record.citations.iter().map(to_citation_dto).collect(),
        created_at: record.created_at.clone(),
    }
}

pub fn list_models_inner(
    runtime: &ChatRuntime,
    _request: ListModelsRequest,
) -> IpcResult<ListModelsResponse> {
    let router = runtime.router.lock().expect("router lock");
    match router.list_models() {
        Ok(models) => IpcResult::ok(ListModelsResponse {
            models: models
                .into_iter()
                .map(|m| ModelDescriptorDto {
                    model_id: m.model_id,
                    adapter_id: m.adapter_id,
                    capabilities: ModelCapabilitiesDto {
                        chat: m.capabilities.chat,
                        embedding: m.capabilities.embedding,
                        json_schema: m.capabilities.json_schema,
                        vision: m.capabilities.vision,
                        image_generation: m.capabilities.image_generation,
                        context_window: m.capabilities.context_window,
                        offline_available: m.capabilities.offline_available,
                    },
                })
                .collect(),
        }),
        Err(err) => IpcResult::err(adapter_err(err)),
    }
}

pub fn chat_create_inner(
    runtime: &ChatRuntime,
    request: ChatCreateRequest,
) -> IpcResult<ChatCreateResponse> {
    let mode = match parse_mode(&request.mode) {
        Ok(m) => m,
        Err(err) => return IpcResult::err(err),
    };
    let notebook_id = request.notebook_id.as_deref();
    match runtime
        .chats
        .create_chat(mode, notebook_id, request.title.as_deref())
    {
        Ok(chat) => IpcResult::ok(ChatCreateResponse {
            chat: chat_summary(&chat),
        }),
        Err(err) => IpcResult::err(err),
    }
}

pub fn chat_list_inner(
    runtime: &ChatRuntime,
    request: ChatListRequest,
) -> IpcResult<ChatListResponse> {
    let mode = match request.mode.as_deref() {
        Some(m) => match parse_mode(m) {
            Ok(parsed) => Some(parsed),
            Err(err) => return IpcResult::err(err),
        },
        None => None,
    };
    let chats = match runtime
        .chats
        .list_chats(mode, request.notebook_id.as_deref())
    {
        Ok(c) => c,
        Err(err) => return IpcResult::err(err),
    };
    let summaries: Vec<ChatSummaryDto> = chats.iter().map(chat_summary).collect();
    let chat_ids: Vec<String> = chats.iter().map(|c| c.chat_id.clone()).collect();
    let messages_by_chat = match runtime.chats.messages_for_chats(&chat_ids) {
        Ok(map) => map,
        Err(err) => return IpcResult::err(err),
    };
    let mut dto_map = std::collections::HashMap::new();
    for (chat_id, msgs) in messages_by_chat {
        dto_map.insert(
            chat_id,
            msgs.iter().map(message_dto).collect::<Vec<_>>(),
        );
    }
    IpcResult::ok(ChatListResponse {
        chats: summaries,
        messages_by_chat: dto_map,
    })
}

fn validate_notebook_scope(
    mode: ChatMode,
    request_notebook: Option<&str>,
    chat_notebook: Option<&str>,
) -> Result<(), ProductError> {
    match mode {
        ChatMode::General => {
            if request_notebook.is_some() {
                return Err(ProductError::new(
                    ProductErrorCode::ValidationFailed,
                    "notebook_id no debe enviarse en modo general",
                ));
            }
            if chat_notebook.is_some() {
                return Err(ProductError::new(
                    ProductErrorCode::ValidationFailed,
                    "chat general no admite notebook_id",
                ));
            }
        }
        ChatMode::Notebook => {
            let req = request_notebook.ok_or_else(|| {
                ProductError::new(
                    ProductErrorCode::ValidationFailed,
                    "notebook_id es obligatorio en modo notebook",
                )
            })?;
            if let Some(chat_nb) = chat_notebook {
                if req != chat_nb {
                    return Err(ProductError::new(
                        ProductErrorCode::ValidationFailed,
                        "notebook_id no coincide con el chat (R-PROD-105)",
                    ));
                }
            }
        }
    }
    Ok(())
}

pub fn chat_send_inner(
    runtime: &ChatRuntime,
    request: ChatSendRequest,
) -> IpcResult<ChatSendResponse> {
    let mode = match parse_mode(&request.mode) {
        Ok(m) => m,
        Err(err) => return IpcResult::err(err),
    };

    let chat = if let Some(chat_id) = request.chat_id.as_deref() {
        let existing = match runtime.chats.get_chat(chat_id) {
            Ok(c) => c,
            Err(err) => return IpcResult::err(err),
        };
        if existing.mode != mode {
            return IpcResult::err(ProductError::new(
                ProductErrorCode::ValidationFailed,
                "mode no coincide con el chat existente",
            ));
        }
        if let Err(err) = validate_notebook_scope(
            mode,
            request.notebook_id.as_deref(),
            existing.notebook_id.as_deref(),
        ) {
            return IpcResult::err(err);
        }
        existing
    } else {
        if let Err(err) = validate_notebook_scope(mode, request.notebook_id.as_deref(), None) {
            return IpcResult::err(err);
        }
        match runtime.chats.create_chat(
            mode,
            request.notebook_id.as_deref(),
            None,
        ) {
            Ok(c) => c,
            Err(err) => return IpcResult::err(err),
        }
    };

    let last_user = request
        .messages
        .iter()
        .rev()
        .find(|m| m.role == "user")
        .ok_or_else(|| {
            ProductError::new(
                ProductErrorCode::ValidationFailed,
                "se requiere al menos un mensaje user",
            )
        });
    let last_user = match last_user {
        Ok(m) => m,
        Err(err) => return IpcResult::err(err),
    };

    if let Err(err) = runtime.chats.insert_message(
        &chat.chat_id,
        ChatMessageRole::User,
        &last_user.content,
        None,
        &[],
    ) {
        return IpcResult::err(err);
    }

    let rag = LexicalRag::new(runtime.search.as_ref());
    let notebook_id = request
        .notebook_id
        .as_ref()
        .map(|id| NotebookId(id.clone()));
    let selected_source_ids = request
        .selected_source_ids
        .clone()
        .unwrap_or_default()
        .into_iter()
        .map(SourceId)
        .collect::<Vec<_>>();

    let outcome = if mode == ChatMode::General {
        RagOutcome::GeneralChatNoContext
    } else {
        rag.retrieve(RagQuery {
            notebook_id: notebook_id.clone(),
            question: last_user.content.clone(),
            selected_source_ids: selected_source_ids.clone(),
            max_chunks: 8,
        })
    };

    let (assistant_content, stored_citations) = match outcome {
        RagOutcome::GeneralChatNoContext => {
            let router_messages = build_router_messages(&request.messages, None);
            let response = match runtime.router.lock().expect("router lock").chat(ChatRequest {
                model_id: request.model_id.clone(),
                messages: router_messages,
                timeout_ms: None,
            }) {
                Ok(r) => r,
                Err(err) => return IpcResult::err(adapter_err(err)),
            };
            (response.content, Vec::new())
        }
        RagOutcome::Abstain { message } => (message, Vec::new()),
        RagOutcome::Context { blocks } => {
            let context = format_context_prompt(&blocks);
            let router_messages =
                build_router_messages(&request.messages, Some(context.as_str()));
            let response = match runtime.router.lock().expect("router lock").chat(ChatRequest {
                model_id: request.model_id.clone(),
                messages: router_messages,
                timeout_ms: None,
            }) {
                Ok(r) => r,
                Err(err) => return IpcResult::err(adapter_err(err)),
            };

            let proposals = extract_citation_proposals(&response.content);
            let scope = CitationScope {
                notebook_id: notebook_id.clone().expect("notebook scope"),
                selected_source_ids: selected_source_ids.clone(),
            };
            let validated = runtime.citations.validate(&scope, &proposals);
            let stored: Vec<StoredCitation> = validated
                .iter()
                .map(|v| StoredCitation {
                    chunk_id: v.chunk_id.0.clone(),
                    source_version_id: v.source_version_id.clone(),
                    quote: v.quote.clone(),
                    verification: verification_str(v.verification).into(),
                })
                .collect();
            (response.content, stored)
        }
    };

    let assistant = match runtime.chats.insert_message(
        &chat.chat_id,
        ChatMessageRole::Assistant,
        &assistant_content,
        Some(request.model_id.as_str()),
        &stored_citations,
    ) {
        Ok(m) => m,
        Err(err) => return IpcResult::err(err),
    };

    IpcResult::ok(ChatSendResponse {
        chat_id: chat.chat_id,
        message_id: assistant.message_id,
        content: assistant_content,
        citations: stored_citations.iter().map(to_citation_dto).collect(),
    })
}

fn build_router_messages(
    messages: &[crate::ipc_types::ChatTurnDto],
    context: Option<&str>,
) -> Vec<RouterMessage> {
    let mut out: Vec<RouterMessage> = Vec::new();
    if let Some(ctx) = context {
        out.push(RouterMessage {
            role: RouterRole::System,
            content: format!("Contexto autorizado del notebook:\n{ctx}"),
        });
    }
    for m in messages {
        if let Ok(role) = parse_ipc_role(&m.role) {
            out.push(RouterMessage {
                role: to_router_role(role),
                content: m.content.clone(),
            });
        }
    }
    out
}
