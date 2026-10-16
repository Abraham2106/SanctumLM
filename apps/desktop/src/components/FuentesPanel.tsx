import { useCallback, useEffect, useId, useState } from "react";
import type {
  ImportInlineMime,
  ImportSourcesRequest,
  JobRecord,
  NotebookSummary,
  SearchHit,
  SourceRecord,
} from "../ipc-types.ts";
import {
  ensureMockSeed,
  getIpcTransport,
  importSources,
  jobStatus,
  listSources,
  notebookCreate,
  notebookList,
  searchSources,
} from "../lib/ipc-client.ts";
import { formatProductError } from "../lib/product-errors.ts";

const ACCEPTED_EXT = [".txt", ".md"];

function mimeFromName(name: string): ImportInlineMime {
  return name.toLowerCase().endsWith(".md") ? "text/markdown" : "text/plain";
}

function isAcceptedFile(name: string): boolean {
  const lower = name.toLowerCase();
  return ACCEPTED_EXT.some((ext) => lower.endsWith(ext));
}

async function readFileAsText(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => resolve(String(reader.result ?? ""));
    reader.onerror = () => reject(reader.error);
    reader.readAsText(file);
  });
}

export default function FuentesPanel() {
  const notebookSelectId = useId();
  const searchId = useId();
  const fileInputId = useId();

  const [notebooks, setNotebooks] = useState<NotebookSummary[]>([]);
  const [activeNotebookId, setActiveNotebookId] = useState<string>("");
  const [sources, setSources] = useState<SourceRecord[]>([]);
  const [searchQuery, setSearchQuery] = useState("");
  const [searchHits, setSearchHits] = useState<SearchHit[]>([]);
  const [statusMessage, setStatusMessage] = useState<string>("");
  const [errorMessage, setErrorMessage] = useState<string>("");
  const [busy, setBusy] = useState(false);
  const [activeJob, setActiveJob] = useState<JobRecord | null>(null);
  const [transportLabel, setTransportLabel] = useState("");

  const refreshSources = useCallback(async (notebookId: string) => {
    if (!notebookId) {
      setSources([]);
      return;
    }
    const result = await listSources(notebookId);
    if (!result.ok) {
      setErrorMessage(formatProductError(result.error));
      return;
    }
    setSources(result.data.sources);
  }, []);

  const loadNotebooks = useCallback(async () => {
    setErrorMessage("");
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
    setTransportLabel(getIpcTransport() === "mock" ? "simulado" : "núcleo Tauri");
  }, []);

  useEffect(() => {
    void loadNotebooks();
  }, [loadNotebooks]);

  useEffect(() => {
    void refreshSources(activeNotebookId);
  }, [activeNotebookId, refreshSources]);

  useEffect(() => {
    if (!activeJob || activeJob.state === "completed" || activeJob.state === "failed" || activeJob.state === "cancelled") {
      return;
    }
    const timer = window.setInterval(() => {
      void (async () => {
        const result = await jobStatus(activeJob.job_id);
        if (!result.ok) {
          setErrorMessage(formatProductError(result.error));
          return;
        }
        setActiveJob(result.data);
        if (
          result.data.state === "completed" ||
          result.data.state === "failed" ||
          result.data.state === "cancelled"
        ) {
          await refreshSources(activeNotebookId);
          setStatusMessage(
            result.data.state === "completed"
              ? "Importación completada."
              : "Importación finalizada con incidencias.",
          );
        }
      })();
    }, 600);
    return () => window.clearInterval(timer);
  }, [activeJob, activeNotebookId, refreshSources]);

  const handleCreateNotebook = async () => {
    setBusy(true);
    setErrorMessage("");
    const title = window.prompt("Nombre del cuaderno", "Nuevo cuaderno");
    if (!title) {
      setBusy(false);
      return;
    }
    const result = await notebookCreate(title);
    setBusy(false);
    if (!result.ok) {
      setErrorMessage(formatProductError(result.error));
      return;
    }
    setNotebooks((prev) => [...prev, result.data.notebook]);
    setActiveNotebookId(result.data.notebook.notebook_id);
    setStatusMessage("Cuaderno creado.");
  };

  const handleImportFiles = async (files: FileList | null) => {
    if (!files || files.length === 0 || !activeNotebookId) {
      return;
    }
    setBusy(true);
    setErrorMessage("");
    setStatusMessage("Preparando importación…");

    const items: ImportSourcesRequest["items"] = [];
    for (const file of files) {
      if (!isAcceptedFile(file.name)) {
        setErrorMessage(`Tipo no admitido en este tramo: ${file.name}. Usa .txt o .md.`);
        setBusy(false);
        return;
      }
      const text = await readFileAsText(file);
      items.push({
        kind: "inline_text",
        display_name: file.name,
        mime: mimeFromName(file.name),
        text,
      });
    }

    const result = await importSources(activeNotebookId, items);
    setBusy(false);
    if (!result.ok) {
      setErrorMessage(formatProductError(result.error));
      return;
    }

    const itemErrors = result.data.item_results.filter((r) => r.error);
    if (itemErrors.length > 0) {
      setErrorMessage(
        itemErrors.map((r) => formatProductError(r.error!)).join(" "),
      );
    }

    setStatusMessage(`Trabajo de importación ${result.data.job_id} en cola.`);
    const job = await jobStatus(result.data.job_id);
    if (job.ok) {
      setActiveJob(job.data);
    }
    await refreshSources(activeNotebookId);
  };

  const handleSearch = async () => {
    if (!activeNotebookId) {
      return;
    }
    setErrorMessage("");
    setStatusMessage("");
    const result = await searchSources(activeNotebookId, searchQuery);
    if (!result.ok) {
      setErrorMessage(formatProductError(result.error));
      setSearchHits([]);
      return;
    }
    setSearchHits(result.data.hits);
    setStatusMessage(
      result.data.hits.length === 0
        ? "Sin coincidencias en el cuaderno activo."
        : `${result.data.hits.length} fragmento(s) encontrado(s).`,
    );
  };

  const activeNotebook = notebooks.find((n) => n.notebook_id === activeNotebookId);

  return (
    <div className="fuentes-panel">
      <p className="fuentes-lead">
        Importa TXT/MD, revisa fuentes del cuaderno activo y busca texto indexado (FTS).
        Canal IPC: <strong>{transportLabel || "…"}</strong>.
      </p>

      <div className="fuentes-toolbar">
        <label className="field" htmlFor={notebookSelectId}>
          Cuaderno activo
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

        <button type="button" className="ipc-button" onClick={() => void handleCreateNotebook()} disabled={busy}>
          Crear cuaderno
        </button>

        <label className="ipc-button file-label" htmlFor={fileInputId}>
          Importar archivos
          <input
            id={fileInputId}
            type="file"
            accept=".txt,.md,text/plain,text/markdown"
            multiple
            disabled={busy || !activeNotebookId}
            onChange={(e) => void handleImportFiles(e.target.files)}
          />
        </label>
      </div>

      {activeJob && activeJob.state !== "completed" && activeJob.state !== "failed" && (
        <p className="job-status" aria-live="polite">
          Importación: {activeJob.progress}% ({activeJob.state})
        </p>
      )}

      {activeNotebook && (
        <p className="meta-line">
          Clasificación: {activeNotebook.classification === "public" ? "pública" : "privada"}
        </p>
      )}

      <section aria-labelledby="fuentes-list-heading">
        <h3 id="fuentes-list-heading">Fuentes del cuaderno</h3>
        {sources.length === 0 ? (
          <p className="placeholder-note">Aún no hay fuentes. Importa un archivo de texto.</p>
        ) : (
          <ul className="source-list">
            {sources.map((source) => (
              <li key={source.source_id}>
                <span className="source-name">{source.display_name}</span>
                <span className="source-meta">
                  {source.status}
                  {source.sha256_prefix ? ` · ${source.sha256_prefix}` : ""}
                </span>
              </li>
            ))}
          </ul>
        )}
      </section>

      <section className="search-block" aria-labelledby="search-heading">
        <h3 id="search-heading">Búsqueda en fuentes</h3>
        <div className="search-row">
          <label className="field field-grow" htmlFor={searchId}>
            Texto a buscar
            <input
              id={searchId}
              type="search"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Ej.: indexa, metodología…"
              disabled={busy || !activeNotebookId}
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  void handleSearch();
                }
              }}
            />
          </label>
          <button type="button" className="ipc-button" onClick={() => void handleSearch()} disabled={busy || !activeNotebookId}>
            Buscar
          </button>
        </div>

        {searchHits.length > 0 && (
          <ol className="hit-list">
            {searchHits.map((hit) => (
              <li key={hit.chunk_id}>
                <p className="hit-source">{hit.source_display_name}</p>
                <p className="hit-snippet">{hit.snippet}</p>
                <p className="hit-offset">
                  Caracteres {hit.char_start}–{hit.char_end}
                  {hit.page != null ? ` · pág. ${hit.page}` : ""}
                </p>
              </li>
            ))}
          </ol>
        )}
      </section>

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
