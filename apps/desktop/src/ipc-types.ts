/**
 * Tipos IPC mínimos para el renderer (B2).
 * Fuente canónica: `packages/contracts` — no duplicar aquí.
 */
export type {
  ApiVersioned,
  ChatSendRequest,
  ChatSendResponse,
  GenerateArtifactRequest,
  GenerateArtifactResponse,
  IpcResult,
  IPC_API_VERSION,
  JobStatusRequest,
  JobStatusResponse,
  ListModelsResponse,
  PingResponse,
} from "../../../packages/contracts/src/index.ts";

export type {
  ArtifactSpec,
  DataClassification,
  JobRecord,
  ProductError,
  ProductErrorCode,
} from "../../../packages/contracts/src/product.ts";
