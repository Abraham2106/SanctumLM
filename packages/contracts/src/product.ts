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
