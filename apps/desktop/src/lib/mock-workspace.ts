import type {
  ChatCitation,
  ChatCreateResponse,
  ChatListResponse,
  ChatMessageRecord,
  ChatMode,
  ChatSendRequest,
  ChatSendResponse,
  ChatSummary,
  ImportSourceInlineItem,
  ImportSourcesResponse,
  JobRecord,
  JobStatusResponse,
  ListModelsResponse,
  ListSourcesResponse,
  NotebookCreateResponse,
  NotebookListResponse,
  NotebookSummary,
  SearchHit,
  SearchSourcesResponse,
  SourceRecord,
} from "../ipc-types.ts";
interface MockChunk {
  chunk_id: string;
  source_id: string;
  text: string;
  char_start: number;
  char_end: number;
}

interface MockJob extends JobRecord {
  notebook_id: string;
  pending_sources: string[];
}

const notebooks: NotebookSummary[] = [];
const sources: SourceRecord[] = [];
const chunks: MockChunk[] = [];
const jobs = new Map<string, MockJob>();
const chats = new Map<string, ChatSummary>();
const messagesByChat = new Map<string, ChatMessageRecord[]>();

const MOCK_MODELS: ListModelsResponse["models"] = [
  {
    model_id: "llama3.2:3b-instruct",
    adapter_id: "ollama-loopback",
    capabilities: {
      chat: true,
      embedding: false,
      offline_available: true,
      context_window: 8192,
    },
  },
  {
    model_id: "nomic-embed-text",
    adapter_id: "ollama-loopback",
    capabilities: {
      chat: false,
      embedding: true,
      offline_available: true,
      context_window: null,
    },
  },
];

let idCounter = 0;

function nextId(prefix: string): string {
  idCounter += 1;
  return `${prefix}-${idCounter.toString(16).padStart(4, "0")}`;
}

function hashPreview(text: string): string {
  let hash = 0;
  for (let i = 0; i < text.length; i += 1) {
    hash = (hash * 31 + text.charCodeAt(i)) >>> 0;
  }
  return hash.toString(16).padStart(8, "0").slice(0, 8);
}

function isoNow(): string {
  return new Date().toISOString();
}

function indexText(sourceId: string, displayName: string, text: string): void {
  const parts = text.split(/\n\s*\n/).map((p) => p.trim()).filter(Boolean);
  if (parts.length === 0) {
    parts.push(text.trim() || "(vacío)");
  }
  let offset = 0;
  for (const part of parts) {
    const start = text.indexOf(part, offset);
    const charStart = start >= 0 ? start : offset;
    const charEnd = charStart + part.length;
    offset = charEnd;
    chunks.push({
      chunk_id: nextId("chunk"),
      source_id: sourceId,
      text: part,
      char_start: charStart,
      char_end: charEnd,
    });
    void displayName;
  }
}

function advanceJob(jobId: string): JobRecord {
  const job = jobs.get(jobId);
  if (!job) {
    return {
      job_id: jobId,
      job_type: "import_batch",
      state: "failed",
      progress: 0,
      error_code: "POLICY_DENIED",
    };
  }

  if (job.state === "completed" || job.state === "failed" || job.state === "cancelled") {
    return job;
  }

  job.progress = Math.min(100, job.progress + 40);
  if (job.progress >= 100) {
    job.state = "completed";
    for (const sourceId of job.pending_sources) {
      const source = sources.find((s) => s.source_id === sourceId);
      if (source) {
        source.status = "ready";
        source.updated_at = isoNow();
      }
    }
    job.pending_sources = [];
  } else {
    job.state = "running";
  }

  return job;
}

export const mockWorkspace = {
  notebookCreate(title: string): NotebookCreateResponse {
    const notebook: NotebookSummary = {
      notebook_id: nextId("nb"),
      title: title.trim() || "Cuaderno sin título",
      classification: "private",
    };
    notebooks.push(notebook);
    return { notebook };
  },

  notebookList(): NotebookListResponse {
    return { notebooks: [...notebooks] };
  },

  listSources(notebookId: string): ListSourcesResponse {
    return {
      sources: sources.filter((s) => s.notebook_id === notebookId),
    };
  },

  importSources(notebookId: string, items: ImportSourceInlineItem[]): ImportSourcesResponse {
    const jobId = nextId("job");
    const pending: string[] = [];
    const item_results: ImportSourcesResponse["item_results"] = [];

    items.forEach((item, item_index) => {
      if (!item.text.trim()) {
        item_results.push({
          item_index,
          error: {
            code: "VALIDATION_FAILED",
            message: "El archivo está vacío.",
          },
        });
        return;
      }

      const sourceId = nextId("src");
      const record: SourceRecord = {
        source_id: sourceId,
        notebook_id: notebookId,
        display_name: item.display_name,
        kind: "file",
        mime: item.mime,
        status: "processing",
        classification: item.classification ?? "private",
        sha256_prefix: hashPreview(item.text),
        updated_at: isoNow(),
      };
      sources.push(record);
      indexText(sourceId, item.display_name, item.text);
      pending.push(sourceId);
      item_results.push({ item_index, source_id: sourceId });
    });

    const job: MockJob = {
      job_id: jobId,
      job_type: "import_batch",
      state: pending.length > 0 ? "queued" : "completed",
      progress: pending.length > 0 ? 5 : 100,
      notebook_id: notebookId,
      pending_sources: pending,
    };
    jobs.set(jobId, job);

    return { job_id: jobId, item_results };
  },

  jobStatus(jobId: string): JobStatusResponse {
    return advanceJob(jobId);
  },

  jobCancel(jobId: string): JobRecord {
    const job = jobs.get(jobId);
    if (!job) {
      return {
        job_id: jobId,
        job_type: "import_batch",
        state: "failed",
        progress: 0,
      };
    }
    job.state = "cancelled";
    job.error_code = "CANCELLED";
    for (const sourceId of job.pending_sources) {
      const source = sources.find((s) => s.source_id === sourceId);
      if (source) {
        source.status = "failed";
        source.updated_at = isoNow();
      }
    }
    job.pending_sources = [];
    return job;
  },

  searchSources(
    notebookId: string,
    query: string,
    limit = 20,
    selectedSourceIds?: string[],
  ): SearchSourcesResponse {
    const q = query.trim().toLowerCase();
    if (!q) {
      return { query, hits: [] };
    }

    const allowed = new Set(
      sources
        .filter((s) => s.notebook_id === notebookId && s.status === "ready")
        .map((s) => s.source_id),
    );

    if (selectedSourceIds && selectedSourceIds.length > 0) {
      for (const id of allowed) {
        if (!selectedSourceIds.includes(id)) {
          allowed.delete(id);
        }
      }
    }

    const hits: SearchHit[] = [];
    for (const chunk of chunks) {
      if (!allowed.has(chunk.source_id)) {
        continue;
      }
      if (!chunk.text.toLowerCase().includes(q)) {
        continue;
      }
      const source = sources.find((s) => s.source_id === chunk.source_id);
      hits.push({
        chunk_id: chunk.chunk_id,
        source_id: chunk.source_id,
        source_display_name: source?.display_name ?? chunk.source_id,
        snippet: chunk.text.length > 240 ? `${chunk.text.slice(0, 237)}…` : chunk.text,
        page: null,
        char_start: chunk.char_start,
        char_end: chunk.char_end,
        rank: hits.length + 1,
      });
      if (hits.length >= limit) {
        break;
      }
    }

    return { query, hits };
  },

  listModels(): ListModelsResponse {
    return {
      models: MOCK_MODELS.filter((m) => m.capabilities.chat),
    };
  },

  chatCreate(mode: ChatMode, notebookId?: string, title?: string): ChatCreateResponse {
    if (mode === "notebook" && !notebookId) {
      throw new Error("notebook_id requerido en modo cuaderno");
    }
    const chatId = nextId("chat");
    const now = isoNow();
    const chat: ChatSummary = {
      chat_id: chatId,
      mode,
      notebook_id: mode === "notebook" ? (notebookId ?? null) : null,
      title:
        title?.trim() ||
        (mode === "general" ? "Chat general" : "Conversación del cuaderno"),
      created_at: now,
      updated_at: now,
    };
    chats.set(chatId, chat);
    messagesByChat.set(chatId, []);
    return { chat };
  },

  chatList(mode?: ChatMode, notebookId?: string): ChatListResponse {
    const filtered = [...chats.values()].filter((c) => {
      if (mode && c.mode !== mode) {
        return false;
      }
      if (notebookId && c.notebook_id !== notebookId) {
        return false;
      }
      return true;
    });
    const messages_by_chat: ChatListResponse["messages_by_chat"] = {};
    for (const chat of filtered) {
      messages_by_chat[chat.chat_id] = [...(messagesByChat.get(chat.chat_id) ?? [])];
    }
    return { chats: filtered, messages_by_chat };
  },

  chatSend(req: ChatSendRequest): ChatSendResponse {
    let chat = req.chat_id ? chats.get(req.chat_id) : undefined;
    if (chat && chat.mode !== req.mode) {
      throw new Error("El chat indicado no coincide con el modo solicitado.");
    }
    if (!chat) {
      const created = mockWorkspace.chatCreate(req.mode, req.notebook_id);
      chat = created.chat;
    }

    if (req.mode === "notebook" && !req.notebook_id) {
      throw new Error("notebook_id requerido");
    }

    const lastUser = [...req.messages].reverse().find((m) => m.role === "user");
    const userText = lastUser?.content.trim() ?? "";
    const now = isoNow();

    const userMessage: ChatMessageRecord = {
      message_id: nextId("msg"),
      chat_id: chat.chat_id,
      role: "user",
      content: userText,
      model_id: req.model_id,
      citations: [],
      created_at: now,
    };

    const thread = messagesByChat.get(chat.chat_id) ?? [];
    thread.push(userMessage);

    let citations: ChatCitation[] = [];
    let assistantBody: string;

    if (req.mode === "general") {
      assistantBody =
        `[Simulado · ${req.model_id}] Respuesta sin contexto de cuaderno. ` +
        `Ninguna fuente local se incluyó (R-PROD-007).`;
    } else {
      const queryTokens = userText.split(/\s+/).filter((t) => t.length > 3);
      const query = queryTokens[0] ?? userText.slice(0, 40);
      const search = mockWorkspace.searchSources(
        req.notebook_id!,
        query,
        3,
        req.selected_source_ids,
      );
      citations = search.hits.map((hit) => ({
        chunk_id: hit.chunk_id,
        source_version_id: hit.source_id,
        quote: hit.snippet,
        verification: "backed" as const,
      }));
      if (citations.length === 0) {
        assistantBody =
          `[Simulado · ${req.model_id}] No encontré fragmentos en las fuentes del cuaderno ` +
          `para «${userText.slice(0, 80)}». Reformula o importa más texto.`;
      } else {
        assistantBody =
          `[Simulado · ${req.model_id}] Según las fuentes indexadas: ` +
          citations.map((c) => c.quote).join(" … ");
      }
    }

    const assistantMessage: ChatMessageRecord = {
      message_id: nextId("msg"),
      chat_id: chat.chat_id,
      role: "assistant",
      content: assistantBody,
      model_id: req.model_id,
      citations,
      created_at: isoNow(),
    };
    thread.push(assistantMessage);
    messagesByChat.set(chat.chat_id, thread);

    chat.updated_at = assistantMessage.created_at;
    chats.set(chat.chat_id, chat);

    return {
      chat_id: chat.chat_id,
      message_id: assistantMessage.message_id,
      content: assistantBody,
      citations,
    };
  },

  seedDemoNotebook(): NotebookSummary {
    if (notebooks.length > 0) {
      return notebooks[0];
    }
    const { notebook } = mockWorkspace.notebookCreate("Cuaderno de demostración");
    mockWorkspace.importSources(notebook.notebook_id, [
      {
        kind: "inline_text",
        display_name: "introduccion.md",
        mime: "text/markdown",
        text: "# Bienvenido\n\nSanctumLM indexa este texto para búsqueda local.\n\nPrueba buscar «indexa» o «búsqueda».",
      },
    ]);
    const job = [...jobs.values()].at(-1);
    if (job) {
      job.progress = 100;
      job.state = "completed";
      for (const sourceId of job.pending_sources) {
        const source = sources.find((s) => s.source_id === sourceId);
        if (source) {
          source.status = "ready";
        }
      }
      job.pending_sources = [];
    }
    return notebook;
  },
};
