/**
 * Tipos IPC mínimos para el renderer (B2).
 * Fuente canónica: `packages/contracts` — no duplicar aquí.
 */
export { IPC_API_VERSION } from "../../../packages/contracts/src/ipc.ts";

export type {
  ApiVersioned,
  ChatSendRequest,
  ChatSendResponse,
  GenerateArtifactRequest,
  GenerateArtifactResponse,
  ImportInlineMime,
  ImportSourceInlineItem,
  ImportSourcesRequest,
  ImportSourcesResponse,
  IpcCommandName,
  IpcResult,
  JobCancelRequest,
  JobCancelResponse,
  JobStatusRequest,
  JobStatusResponse,
  ListModelsResponse,
  ListSourcesRequest,
  ListSourcesResponse,
  NotebookCreateRequest,
  NotebookCreateResponse,
  NotebookListRequest,
  NotebookListResponse,
  PingResponse,
  SearchSourcesRequest,
  SearchSourcesResponse,
} from "../../../packages/contracts/src/index.ts";

export type {
  ArtifactSpec,
  DataClassification,
  ImportSourceItemResult,
  JobRecord,
  JobState,
  NotebookSummary,
  ProductError,
  ProductErrorCode,
  SearchHit,
  SourceKind,
  SourceRecord,
  SourceStatus,
} from "../../../packages/contracts/src/product.ts";
