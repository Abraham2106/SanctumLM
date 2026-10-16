import type { ArtifactSpec, JobRecord, ProductError } from "./product.js";

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
