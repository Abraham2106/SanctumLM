import { useCallback, useEffect, useId, useRef, useState } from "react";
import type {
  ChatCitation,
  ChatMessageRecord,
  ChatMode,
  ChatSummary,
  ModelDescriptor,
  NotebookSummary,
} from "../ipc-types.ts";
import {
  chatCreate,
  chatList,
  chatSend,
  ensureMockSeed,
  getIpcTransport,
  listModels,
  notebookList,
} from "../lib/ipc-client.ts";
import { formatProductError } from "../lib/product-errors.ts";

const VERIFICATION_ES: Record<ChatCitation["verification"], string> = {
  backed: "con respaldo",
  inference: "inferencia",
  discrepancy: "discrepancia",
  unverified: "sin verificar",
};

function citationLabel(citation: ChatCitation): string {
  return `${citation.quote.slice(0, 72)}${citation.quote.length > 72 ? "…" : ""} (${VERIFICATION_ES[citation.verification]})`;
}

export default function ChatPanel() {
  const modeGeneralId = useId();
  const modeNotebookId = useId();
  const modelSelectId = useId();
  const notebookSelectId = useId();
  const composerId = useId();
  const threadRef = useRef<HTMLDivElement>(null);

  const [chatMode, setChatMode] = useState<ChatMode>("notebook");
  const [notebooks, setNotebooks] = useState<NotebookSummary[]>([]);
  const [activeNotebookId, setActiveNotebookId] = useState("");
  const [models, setModels] = useState<ModelDescriptor[]>([]);
  const [modelId, setModelId] = useState("");
  const [activeChat, setActiveChat] = useState<ChatSummary | null>(null);
  const [messages, setMessages] = useState<ChatMessageRecord[]>([]);
  const [draft, setDraft] = useState("");
  const [busy, setBusy] = useState(false);
  const [statusMessage, setStatusMessage] = useState("");
  const [errorMessage, setErrorMessage] = useState("");
  const [transportLabel, setTransportLabel] = useState("");
  const [citationNotice, setCitationNotice] = useState("");

  const loadModels = useCallback(async () => {
    const result = await listModels();
    if (!result.ok) {
      setErrorMessage(formatProductError(result.error));
      return;
    }
    setModels(result.data.models);
    setModelId((current) => {
      if (current && result.data.models.some((m) => m.model_id === current)) {
        return current;
      }
      return result.data.models[0]?.model_id ?? "";
    });
  }, []);

  const loadNotebooks = useCallback(async () => {
    let result = await notebookList();
    if (!result.ok) {
      setErrorMessage(formatProductError(result.error));
      return;
    }
    if (result.data.notebooks.length === 0) {
      ensureMockSeed();
      result = await notebookList();
      if (!result.ok) {
        setErrorMessage(formatProductError(result.error));
        return;
      }
    }
    setNotebooks(result.data.notebooks);
    setActiveNotebookId((current) => {
      if (current && result.ok && result.data.notebooks.some((n) => n.notebook_id === current)) {
        return current;
      }
      return result.ok && result.data.notebooks[0] ? result.data.notebooks[0].notebook_id : "";
    });
  }, []);

  const syncThread = useCallback(
    async (mode: ChatMode, notebookId: string) => {
      setErrorMessage("");
      const listResult = await chatList(
        mode,
        mode === "notebook" ? notebookId : undefined,
      );
      if (!listResult.ok) {
        setErrorMessage(formatProductError(listResult.error));
        return;
      }

      let chat = listResult.data.chats.sort(
        (a, b) => b.updated_at.localeCompare(a.updated_at),
      )[0];

      if (!chat) {
        const created = await chatCreate(
          mode,
          mode === "notebook" ? notebookId : undefined,
        );
        if (!created.ok) {
          setErrorMessage(formatProductError(created.error));
          return;
        }
        setActiveChat(created.data.chat);
        setMessages([]);
        return;
      }

      setActiveChat(chat);
      setMessages(listResult.data.messages_by_chat[chat.chat_id] ?? []);
    },
    [],
  );

  useEffect(() => {
    void loadModels();
    void loadNotebooks();
    setTransportLabel(getIpcTransport() === "mock" ? "simulado" : "núcleo Tauri");
  }, [loadModels, loadNotebooks]);

  useEffect(() => {
    if (chatMode === "notebook" && !activeNotebookId) {
      return;
    }
    void syncThread(chatMode, activeNotebookId);
  }, [chatMode, activeNotebookId, syncThread]);

  useEffect(() => {
    const el = threadRef.current;
    if (el) {
      el.scrollTop = el.scrollHeight;
    }
  }, [messages]);

  const handleModeChange = (mode: ChatMode) => {
    setChatMode(mode);
    setCitationNotice("");
    setStatusMessage(
      mode === "general"
        ? "Modo general: sin recuperación de fuentes del cuaderno."
        : "Modo cuaderno: respuestas simuladas con citas FTS del cuaderno activo.",
    );
  };

  const handleSend = async () => {
    const text = draft.trim();
    if (!text || !modelId || busy) {
      return;
    }
    if (chatMode === "notebook" && !activeNotebookId) {
      setErrorMessage("Elige un cuaderno antes de chatear.");
      return;
    }

    setBusy(true);
    setErrorMessage("");
    setStatusMessage("Enviando…");

    const history = messages.map((m) => ({
      role: m.role,
      content: m.content,
    }));
    history.push({ role: "user" as const, content: text });

    const result = await chatSend({
      chat_id: activeChat?.chat_id,
      mode: chatMode,
      notebook_id: chatMode === "notebook" ? activeNotebookId : undefined,
      model_id: modelId,
      messages: history,
    });

    setBusy(false);
    setDraft("");

    if (!result.ok) {
      setErrorMessage(formatProductError(result.error));
      setStatusMessage("");
      return;
    }

    const refreshed = await chatList(
      chatMode,
      chatMode === "notebook" ? activeNotebookId : undefined,
    );
    if (refreshed.ok && activeChat) {
      setMessages(refreshed.data.messages_by_chat[result.data.chat_id] ?? []);
    } else {
      setMessages((prev) => [
        ...prev,
        {
          message_id: `local-${Date.now()}`,
          chat_id: result.data.chat_id,
          role: "user",
          content: text,
          model_id: modelId,
          citations: [],
          created_at: new Date().toISOString(),
        },
        {
          message_id: result.data.message_id,
          chat_id: result.data.chat_id,
          role: "assistant",
          content: result.data.content,
          model_id: modelId,
          citations: result.data.citations,
          created_at: new Date().toISOString(),
        },
      ]);
    }

    if (!activeChat || activeChat.chat_id !== result.data.chat_id) {
      setActiveChat((current) =>
        current ??
        ({
          chat_id: result.data.chat_id,
          mode: chatMode,
          notebook_id: chatMode === "notebook" ? activeNotebookId : null,
          title: chatMode === "general" ? "Chat general" : "Conversación del cuaderno",
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        } satisfies ChatSummary),
      );
    }

    setStatusMessage("Respuesta recibida (simulada hasta TASK-0032).");
    setTransportLabel(getIpcTransport() === "mock" ? "simulado" : "núcleo Tauri");
  };

  const handleCitationClick = (citation: ChatCitation) => {
    setCitationNotice(
      `Visor de fragmentos pendiente. chunk_id=${citation.chunk_id}, fuente=${citation.source_version_id}.`,
    );
  };

  const showNotebookChrome = chatMode === "notebook";

  return (
    <div className="chat-panel">
      <p className="chat-lead">
        Conversación vía IPC (<strong>{transportLabel || "…"}</strong>). El núcleo enruta el
        modelo local; la UI no contacta Ollama directamente.
      </p>

      <fieldset className="chat-mode-fieldset">
        <legend>Modo de chat</legend>
        <label className="mode-option">
          <input
            id={modeNotebookId}
            type="radio"
            name="chat-mode"
            checked={chatMode === "notebook"}
            onChange={() => handleModeChange("notebook")}
          />
          Cuaderno (fuentes indexadas)
        </label>
        <label className="mode-option">
          <input
            id={modeGeneralId}
            type="radio"
            name="chat-mode"
            checked={chatMode === "general"}
            onChange={() => handleModeChange("general")}
          />
          General (sin contexto de fuentes)
        </label>
      </fieldset>

      <div className="chat-toolbar">
        <label className="field" htmlFor={modelSelectId}>
          Modelo
          <select
            id={modelSelectId}
            value={modelId}
            onChange={(e) => setModelId(e.target.value)}
            disabled={busy || models.length === 0}
          >
            {models.length === 0 ? (
              <option value="">Sin modelos</option>
            ) : (
              models.map((m) => (
                <option key={m.model_id} value={m.model_id}>
                  {m.model_id} ({m.adapter_id})
                </option>
              ))
            )}
          </select>
        </label>

        {showNotebookChrome && (
          <label className="field" htmlFor={notebookSelectId}>
            Cuaderno para contexto
            <select
              id={notebookSelectId}
              value={activeNotebookId}
              onChange={(e) => setActiveNotebookId(e.target.value)}
              disabled={busy || notebooks.length === 0}
            >
              {notebooks.length === 0 ? (
                <option value="">Sin cuadernos</option>
              ) : (
                notebooks.map((nb) => (
                  <option key={nb.notebook_id} value={nb.notebook_id}>
                    {nb.title}
                  </option>
                ))
              )}
            </select>
          </label>
        )}
      </div>

      {!showNotebookChrome && (
        <p className="general-mode-note" role="note">
          En chat general no se muestran fuentes ni panel de importación: el núcleo no debe
          recuperar fragmentos de cuadernos (R-PROD-007).
        </p>
      )}

      <div className="chat-layout">
        {showNotebookChrome && (
          <aside className="chat-context-aside" aria-label="Contexto del cuaderno">
            <h3>Fuentes del cuaderno</h3>
            <p className="placeholder-note">
              Resumen contextual: las citas en respuestas enlazarán fragmentos indexados del
              cuaderno seleccionado. Importación y búsqueda detallada están en el panel Fuentes.
            </p>
          </aside>
        )}

        <div className="chat-thread-wrap">
          <div
            ref={threadRef}
            className="chat-thread"
            role="log"
            aria-live="polite"
            aria-relevant="additions"
          >
            {messages.length === 0 ? (
              <p className="placeholder-note">Escribe un mensaje para iniciar la conversación.</p>
            ) : (
              messages.map((msg) => (
                <article
                  key={msg.message_id}
                  className={`chat-bubble chat-bubble-${msg.role}`}
                >
                  <header>{msg.role === "user" ? "Tú" : "Asistente"}</header>
                  <p>{msg.content}</p>
                  {msg.citations.length > 0 && (
                    <ul className="citation-list">
                      {msg.citations.map((citation) => (
                        <li key={`${msg.message_id}-${citation.chunk_id}`}>
                          <button
                            type="button"
                            className="citation-link"
                            onClick={() => handleCitationClick(citation)}
                          >
                            {citationLabel(citation)}
                          </button>
                        </li>
                      ))}
                    </ul>
                  )}
                </article>
              ))
            )}
          </div>

          <div className="chat-composer">
            <label className="field field-grow" htmlFor={composerId}>
              Mensaje
              <textarea
                id={composerId}
                rows={3}
                value={draft}
                disabled={busy || !modelId}
                placeholder={
                  chatMode === "general"
                    ? "Pregunta general al modelo local…"
                    : "Pregunta sobre las fuentes del cuaderno…"
                }
                onChange={(e) => setDraft(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === "Enter" && !e.shiftKey) {
                    e.preventDefault();
                    void handleSend();
                  }
                }}
              />
            </label>
            <button
              type="button"
              className="ipc-button send-button"
              disabled={busy || !draft.trim() || !modelId}
              onClick={() => void handleSend()}
            >
              Enviar
            </button>
          </div>
        </div>
      </div>

      {citationNotice && (
        <p className="status-ok" role="status">
          {citationNotice}
        </p>
      )}
      {statusMessage && (
        <p className="status-ok" role="status">
          {statusMessage}
        </p>
      )}
      {errorMessage && (
        <p className="status-error" role="alert">
          {errorMessage}
        </p>
      )}
    </div>
  );
}
