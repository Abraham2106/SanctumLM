import type {
  ArtifactSpec,
  ChatCitation,
  ChatMessageRecord,
  ChatMessageRole,
  ChatMode,
  ChatSummary,
  DataClassification,
  ImportSourceItemResult,
  JobRecord,
  ModelDescriptor,
  NotebookSummary,
  ProductError,
  SearchHit,
  SourceRecord,
} from "./product.js";

/** Envelope IPC estándar (renderer ↔ núcleo). */
export type IpcResult<T> =
  | { ok: true; data: T }
  | { ok: false; error: ProductError };

export const IPC_API_VERSION = 1 as const;

export interface ApiVersioned {
  api_version: typeof IPC_API_VERSION;
}

export interface PingResponse {
  message: "pong";
}

export interface ListModelsRequest extends ApiVersioned {}

export interface ListModelsResponse {
  models: ModelDescriptor[];
}

export interface ChatSendRequest extends ApiVersioned {
  chat_id?: string;
  mode: ChatMode;
  notebook_id?: string;
  model_id: string;
  messages: Array<{ role: ChatMessageRole; content: string }>;
  selected_source_ids?: string[];
}

export interface ChatSendResponse {
  chat_id: string;
  message_id: string;
  content: string;
  citations: ChatCitation[];
}

export interface ChatCreateRequest extends ApiVersioned {
  mode: ChatMode;
  notebook_id?: string;
  title?: string;
}

export interface ChatCreateResponse {
  chat: ChatSummary;
}

export interface ChatListRequest extends ApiVersioned {
  mode?: ChatMode;
  notebook_id?: string;
}

export interface ChatListResponse {
  chats: ChatSummary[];
  messages_by_chat: Record<string, ChatMessageRecord[]>;
}

export interface GenerateArtifactRequest extends ApiVersioned {
  notebook_id: string;
  spec: Partial<ArtifactSpec> & Pick<ArtifactSpec, "type" | "title" | "language">;
}

export interface GenerateArtifactResponse {
  job_id: string;
  validated_spec: ArtifactSpec;
}

export interface JobStatusRequest extends ApiVersioned {
  job_id: string;
}

export type JobStatusResponse = JobRecord;

/**
 * Comandos IPC acordados (0027 import/búsqueda; 0032 chat — implementación Rust pendiente).
 * El renderer usa mock si `invoke` falla (TASK-0033).
 */
export type IpcCommandName =
  | "ping"
  | "notebook_create"
  | "notebook_list"
  | "list_sources"
  | "import_sources"
  | "job_status"
  | "job_cancel"
  | "search_sources"
  | "list_models"
  | "chat_create"
  | "chat_list"
  | "chat_send";

export interface NotebookCreateRequest extends ApiVersioned {
  title: string;
  classification?: DataClassification;
}

export interface NotebookCreateResponse {
  notebook: NotebookSummary;
}

export interface NotebookListRequest extends ApiVersioned {}

export interface NotebookListResponse {
  notebooks: NotebookSummary[];
}

export interface ListSourcesRequest extends ApiVersioned {
  notebook_id: string;
}

export interface ListSourcesResponse {
  sources: SourceRecord[];
}

export type ImportInlineMime = "text/plain" | "text/markdown";

export interface ImportSourceInlineItem {
  kind: "inline_text";
  display_name: string;
  mime: ImportInlineMime;
  text: string;
  classification?: DataClassification;
}

export interface ImportSourcesRequest extends ApiVersioned {
  notebook_id: string;
  items: ImportSourceInlineItem[];
}

export interface ImportSourcesResponse {
  job_id: string;
  item_results: ImportSourceItemResult[];
}

export interface JobCancelRequest extends ApiVersioned {
  job_id: string;
}

export type JobCancelResponse = JobRecord;

export interface SearchSourcesRequest extends ApiVersioned {
  notebook_id: string;
  query: string;
  limit?: number;
  selected_source_ids?: string[];
}

export interface SearchSourcesResponse {
  query: string;
  hits: SearchHit[];
}
