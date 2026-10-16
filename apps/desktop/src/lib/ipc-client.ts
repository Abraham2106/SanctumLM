import { invoke } from "@tauri-apps/api/core";
import type {
  ApiVersioned,
  ChatCreateRequest,
  ChatCreateResponse,
  ChatListRequest,
  ChatListResponse,
  ChatSendRequest,
  ChatSendResponse,
  IpcCommandName,
  IpcResult,
  ImportSourcesRequest,
  ImportSourcesResponse,
  JobCancelRequest,
  JobCancelResponse,
  JobStatusRequest,
  JobStatusResponse,
  ListModelsRequest,
  ListModelsResponse,
  ListSourcesRequest,
  ListSourcesResponse,
  NotebookCreateRequest,
  NotebookCreateResponse,
  NotebookListRequest,
  NotebookListResponse,
  SearchSourcesRequest,
  SearchSourcesResponse,
} from "../ipc-types.ts";
import { IPC_API_VERSION } from "../../../../packages/contracts/src/ipc.ts";
import { mockWorkspace } from "./mock-workspace.ts";
import { shouldUseMockIpc } from "./ipc-runtime.ts";

export type IpcTransport = "tauri" | "mock";

let transport: IpcTransport = shouldUseMockIpc() ? "mock" : "tauri";

export function getIpcTransport(): IpcTransport {
  return transport;
}

function withVersion<T extends ApiVersioned>(payload: Omit<T, "api_version">): T {
  return { ...payload, api_version: IPC_API_VERSION } as T;
}

function invokeTransportError(err: unknown): IpcResult<never> {
  const detail =
    err instanceof Error && err.message.trim().length > 0
      ? err.message.trim()
      : "error desconocido";
  return {
    ok: false,
    error: {
      code: "LOCAL_UNAVAILABLE",
      message: `No se pudo invocar el núcleo local (${detail}). Comprueba que la aplicación de escritorio esté actualizada.`,
    },
  };
}

async function tryInvoke<TData>(
  command: IpcCommandName,
  request: ApiVersioned,
): Promise<IpcResult<TData>> {
  try {
    const result = await invoke<IpcResult<TData>>(command, { request });
    if (result && typeof result === "object" && "ok" in result) {
      return result;
    }
    return { ok: true, data: result as TData };
  } catch (err) {
    return invokeTransportError(err) as IpcResult<TData>;
  }
}

function mockDispatch<TData>(
  command: IpcCommandName,
  request: ApiVersioned,
): IpcResult<TData> {
  switch (command) {
    case "notebook_create": {
      const req = request as NotebookCreateRequest;
      return { ok: true, data: mockWorkspace.notebookCreate(req.title) as TData };
    }
    case "notebook_list": {
      return { ok: true, data: mockWorkspace.notebookList() as TData };
    }
    case "list_sources": {
      const req = request as ListSourcesRequest;
      return {
        ok: true,
        data: mockWorkspace.listSources(req.notebook_id) as TData,
      };
    }
    case "import_sources": {
      const req = request as ImportSourcesRequest;
      return {
        ok: true,
        data: mockWorkspace.importSources(req.notebook_id, req.items) as TData,
      };
    }
    case "job_status": {
      const req = request as JobStatusRequest;
      return { ok: true, data: mockWorkspace.jobStatus(req.job_id) as TData };
    }
    case "job_cancel": {
      const req = request as JobCancelRequest;
      return { ok: true, data: mockWorkspace.jobCancel(req.job_id) as TData };
    }
    case "search_sources": {
      const req = request as SearchSourcesRequest;
      return {
        ok: true,
        data: mockWorkspace.searchSources(
          req.notebook_id,
          req.query,
          req.limit,
          req.selected_source_ids,
        ) as TData,
      };
    }
    case "list_models": {
      return { ok: true, data: mockWorkspace.listModels() as TData };
    }
    case "chat_create": {
      const req = request as ChatCreateRequest;
      if (req.mode === "notebook" && !req.notebook_id) {
        return {
          ok: false,
          error: {
            code: "VALIDATION_FAILED",
            message: "Selecciona un cuaderno para el chat contextual.",
          },
        };
      }
      return {
        ok: true,
        data: mockWorkspace.chatCreate(req.mode, req.notebook_id, req.title) as TData,
      };
    }
    case "chat_list": {
      const req = request as ChatListRequest;
      return {
        ok: true,
        data: mockWorkspace.chatList(req.mode, req.notebook_id) as TData,
      };
    }
    case "chat_send": {
      const req = request as ChatSendRequest;
      if (req.mode === "notebook" && !req.notebook_id) {
        return {
          ok: false,
          error: {
            code: "VALIDATION_FAILED",
            message: "Falta el cuaderno activo para enviar el mensaje.",
          },
        };
      }
      try {
        return { ok: true, data: mockWorkspace.chatSend(req) as TData };
      } catch (err) {
        return {
          ok: false,
          error: {
            code: "VALIDATION_FAILED",
            message: err instanceof Error ? err.message : "No se pudo enviar el mensaje.",
          },
        };
      }
    }
    default:
      return {
        ok: false,
        error: {
          code: "LOCAL_UNAVAILABLE",
          message: "Comando no disponible en modo simulado.",
        },
      };
  }
}

async function callIpc<TData>(
  command: IpcCommandName,
  request: ApiVersioned,
): Promise<IpcResult<TData>> {
  if (transport === "mock") {
    return mockDispatch(command, request);
  }

  return tryInvoke<TData>(command, request);
}

export async function notebookCreate(
  title: string,
): Promise<IpcResult<NotebookCreateResponse>> {
  return callIpc("notebook_create", withVersion<NotebookCreateRequest>({ title }));
}

export async function notebookList(): Promise<IpcResult<NotebookListResponse>> {
  return callIpc("notebook_list", withVersion<NotebookListRequest>({}));
}

export async function listSources(
  notebookId: string,
): Promise<IpcResult<ListSourcesResponse>> {
  return callIpc(
    "list_sources",
    withVersion<ListSourcesRequest>({ notebook_id: notebookId }),
  );
}

export async function importSources(
  notebookId: string,
  items: ImportSourcesRequest["items"],
): Promise<IpcResult<ImportSourcesResponse>> {
  return callIpc(
    "import_sources",
    withVersion<ImportSourcesRequest>({ notebook_id: notebookId, items }),
  );
}

export async function jobStatus(jobId: string): Promise<IpcResult<JobStatusResponse>> {
  return callIpc("job_status", withVersion<JobStatusRequest>({ job_id: jobId }));
}

export async function jobCancel(jobId: string): Promise<IpcResult<JobCancelResponse>> {
  return callIpc("job_cancel", withVersion<JobCancelRequest>({ job_id: jobId }));
}

export async function searchSources(
  notebookId: string,
  query: string,
  selectedSourceIds?: string[],
): Promise<IpcResult<SearchSourcesResponse>> {
  return callIpc(
    "search_sources",
    withVersion<SearchSourcesRequest>({
      notebook_id: notebookId,
      query,
      limit: 25,
      selected_source_ids: selectedSourceIds,
    }),
  );
}

export function ensureMockSeed(): void {
  if (transport !== "mock") {
    return;
  }
  mockWorkspace.seedDemoNotebook();
}

export async function listModels(): Promise<IpcResult<ListModelsResponse>> {
  return callIpc("list_models", withVersion<ListModelsRequest>({}));
}

export async function chatCreate(
  mode: ChatCreateRequest["mode"],
  notebookId?: string,
  title?: string,
): Promise<IpcResult<ChatCreateResponse>> {
  return callIpc(
    "chat_create",
    withVersion<ChatCreateRequest>({
      mode,
      notebook_id: notebookId,
      title,
    }),
  );
}

export async function chatList(
  mode?: ChatListRequest["mode"],
  notebookId?: string,
): Promise<IpcResult<ChatListResponse>> {
  return callIpc(
    "chat_list",
    withVersion<ChatListRequest>({ mode, notebook_id: notebookId }),
  );
}

export async function chatSend(
  payload: Omit<ChatSendRequest, "api_version">,
): Promise<IpcResult<ChatSendResponse>> {
  return callIpc("chat_send", withVersion<ChatSendRequest>(payload));
}
