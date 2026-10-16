import type {
  ArtifactSpec,
  DataClassification,
  ImportSourceItemResult,
  JobRecord,
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

export interface ListModelsResponse {
  models: Array<{
    model_id: string;
    adapter_id: string;
    capabilities: {
      chat: boolean;
      embedding: boolean;
      offline_available: boolean;
      context_window: number | null;
    };
  }>;
}

export interface ChatSendRequest extends ApiVersioned {
  mode: "general" | "notebook";
  notebook_id?: string;
  model_id: string;
  messages: Array<{ role: "system" | "user" | "assistant"; content: string }>;
  selected_source_ids?: string[];
}

export interface ChatSendResponse {
  content: string;
  citations: Array<{
    chunk_id: string;
    source_version_id: string;
    quote: string;
    verification: "backed" | "inference" | "discrepancy" | "unverified";
  }>;
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

/** Comandos acordados con TASK-0027 (núcleo Tauri). */
export type IpcCommandName =
  | "ping"
  | "notebook_create"
  | "notebook_list"
  | "list_sources"
  | "import_sources"
  | "job_status"
  | "job_cancel"
  | "search_sources";

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
