/** Tipos compartidos alineados a `DATA_AND_INTERFACES` (G1). */

export type ProductErrorCode =
  | "VALIDATION_FAILED"
  | "UNSUPPORTED_API_VERSION"
  | "POLICY_DENIED"
  | "NOT_FOUND"
  | "CONFLICT"
  | "JOB_CANCELLED"
  | "LOCAL_UNAVAILABLE"
  | "SSR_BLOCKED";

export interface ProductError {
  code: ProductErrorCode;
  message: string;
  details?: Record<string, unknown>;
}

export type DataClassification = "public" | "private";

export type JobType =
  | "import_batch"
  | "extract"
  | "embed_index"
  | "generate_artifact";

export type JobState =
  | "queued"
  | "running"
  | "completed"
  | "failed"
  | "cancelled";

export interface JobRecord {
  job_id: string;
  job_type: JobType;
  state: JobState;
  progress: number;
  error_code?: "EXTRACTION_CORRUPT" | "POLICY_DENIED" | "CANCELLED";
  dedupe_key?: string;
}

export type SourceKind = "file" | "paste" | "url";

export type SourceStatus =
  | "pending"
  | "processing"
  | "ready"
  | "failed"
  | "partial";

export interface NotebookSummary {
  notebook_id: string;
  title: string;
  classification: DataClassification;
}

export interface SourceRecord {
  source_id: string;
  notebook_id: string;
  display_name: string;
  kind: SourceKind;
  mime: string | null;
  status: SourceStatus;
  classification: DataClassification;
  sha256_prefix?: string;
  updated_at: string;
}

export interface ImportSourceItemResult {
  item_index: number;
  source_id?: string;
  error?: ProductError;
}

export interface SearchHit {
  chunk_id: string;
  source_id: string;
  source_display_name: string;
  snippet: string;
  page: number | null;
  char_start: number;
  char_end: number;
  rank: number;
}

export type ChatMode = "general" | "notebook";

export type ChatMessageRole = "system" | "user" | "assistant";

export type CitationVerification =
  | "backed"
  | "inference"
  | "discrepancy"
  | "unverified";

export interface ChatCitation {
  chunk_id: string;
  source_version_id: string;
  quote: string;
  verification: CitationVerification;
}

export interface ChatSummary {
  chat_id: string;
  mode: ChatMode;
  notebook_id: string | null;
  title: string;
  created_at: string;
  updated_at: string;
}

export interface ChatMessageRecord {
  message_id: string;
  chat_id: string;
  role: ChatMessageRole;
  content: string;
  model_id: string | null;
  citations: ChatCitation[];
  created_at: string;
}

export interface ModelDescriptor {
  model_id: string;
  adapter_id: string;
  capabilities: {
    chat: boolean;
    embedding: boolean;
    offline_available: boolean;
    context_window: number | null;
  };
}

export type ArtifactType = "infographic" | "slides" | "report";

export type EvidenceMode = "source_backed" | "exploratory";

export interface ArtifactSection {
  id: string;
  heading: string;
  body?: string;
}

export interface ArtifactSpec {
  schema_version: number;
  type: ArtifactType;
  title: string;
  language: string;
  evidence_mode: EvidenceMode;
  source_scope?: string[];
  sections: ArtifactSection[];
}
