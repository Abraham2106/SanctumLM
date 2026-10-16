import { useCallback, useEffect, useId, useRef, useState } from "react";
import type {
  ImportInlineMime,
  ImportSourcesRequest,
  JobRecord,
  JobState,
  SearchHit,
  SourceRecord,
  SourceStatus,
} from "../ipc-types.ts";
import { useNotebookContext } from "../context/NotebookContext.tsx";
import {
  getIpcTransport,
  importSources,
  jobCancel,
  jobStatus,
  listSources,
  searchSources,
} from "../lib/ipc-client.ts";
import { formatProductError } from "../lib/product-errors.ts";
import ProductMessage from "./ProductMessage.tsx";
import SearchHitsEmptyState from "./SearchHitsEmptyState.tsx";
import SourcesEmptyState from "./SourcesEmptyState.tsx";

const ACCEPTED_EXT = [".txt", ".md"];

const SOURCE_STATUS_ES: Record<SourceStatus, string> = {
  pending: "pendiente",
  processing: "procesando",
  ready: "listo",
  failed: "fallido",
  partial: "parcial",
};

const JOB_STATE_ES: Record<JobState, string> = {
  queued: "en cola",
  running: "en curso",
  completed: "completado",
  failed: "fallido",
  cancelled: "cancelado",
};

const JOB_ERROR_ES: Record<NonNullable<JobRecord["error_code"]>, string> = {
  EXTRACTION_CORRUPT: "Uno o más archivos no se pudieron extraer correctamente.",
  POLICY_DENIED: "La política del cuaderno impidió completar la importación.",
  CANCELLED: "La importación fue cancelada antes de terminar.",
};

function formatSourceStatus(status: SourceStatus): string {
  return SOURCE_STATUS_ES[status] ?? status;
}

function formatJobState(state: JobState): string {
  return JOB_STATE_ES[state] ?? state;
}

function formatJobError(job: JobRecord): string | null {
  if (!job.error_code) {
    return null;
  }
  return JOB_ERROR_ES[job.error_code] ?? job.error_code;
}

function isActiveImportJob(job: JobRecord): boolean {
  return job.state === "queued" || job.state === "running";
}

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
  const searchId = useId();
  const searchResultsId = useId();
  const fileInputId = useId();
  const fileInputRef = useRef<HTMLInputElement>(null);

  const { activeNotebookId, activeNotebook, createNotebook } = useNotebookContext();
  const [sources, setSources] = useState<SourceRecord[]>([]);
  const [searchQuery, setSearchQuery] = useState("");
  const [searchHits, setSearchHits] = useState<SearchHit[]>([]);
  const [searchCompleted, setSearchCompleted] = useState(false);
  const [statusMessage, setStatusMessage] = useState<string>("");
  const [errorMessage, setErrorMessage] = useState<string>("");
  const [busy, setBusy] = useState(false);
  const [activeJob, setActiveJob] = useState<JobRecord | null>(null);
  const [cancellingImport, setCancellingImport] = useState(false);
  const [transportLabel, setTransportLabel] = useState("");

  const importInFlight =
    activeJob != null && isActiveImportJob(activeJob);
  const panelBusy = busy || cancellingImport || importInFlight;

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

  useEffect(() => {
    setTransportLabel(getIpcTransport() === "mock" ? "simulado" : "núcleo Tauri");
  }, []);

  useEffect(() => {
    void refreshSources(activeNotebookId);
  }, [activeNotebookId, refreshSources]);

  useEffect(() => {
    setSearchHits([]);
    setSearchCompleted(false);
    setSearchQuery("");
  }, [activeNotebookId]);

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
          const jobError = formatJobError(result.data);
          if (result.data.state === "completed") {
            setStatusMessage("Importación completada.");
            if (jobError) {
              setErrorMessage(jobError);
            }
          } else if (result.data.state === "cancelled") {
            setStatusMessage("Importación cancelada.");
            if (jobError) {
              setErrorMessage(jobError);
            }
          } else {
            setStatusMessage("Importación finalizada con incidencias.");
            setErrorMessage(jobError ?? "El trabajo de importación falló.");
          }
          setActiveJob(null);
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
    const ok = await createNotebook(title);
    setBusy(false);
    if (ok) {
      setStatusMessage("Cuaderno creado.");
    } else {
      setErrorMessage("No se pudo crear el cuaderno. Revisa el mensaje en la barra de cuadernos.");
    }
  };

  const handleImportFiles = async (
    files: FileList | null,
    inputEl?: HTMLInputElement | null,
  ) => {
    if (!files || files.length === 0 || !activeNotebookId) {
      return;
    }
    setBusy(true);
    setErrorMessage("");
    setStatusMessage("Preparando importación…");

    try {
      const items: ImportSourcesRequest["items"] = [];
      for (const file of files) {
        if (!isAcceptedFile(file.name)) {
          setErrorMessage(
            `Tipo no admitido en este tramo: ${file.name}. Usa .txt o .md.`,
          );
          return;
        }
        let text: string;
        try {
          text = await readFileAsText(file);
        } catch {
          setErrorMessage(
            `No se pudo leer el archivo «${file.name}». Comprueba que no esté dañado o en uso.`,
          );
          return;
        }
        items.push({
          kind: "inline_text",
          display_name: file.name,
          mime: mimeFromName(file.name),
          text,
        });
      }

      const result = await importSources(activeNotebookId, items);
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
    } catch {
      setErrorMessage("Ocurrió un error inesperado al importar archivos.");
    } finally {
      setBusy(false);
      const input = inputEl ?? fileInputRef.current;
      if (input) {
        input.value = "";
      }
    }
  };

  const handleCancelImport = async () => {
    if (!activeJob || !isActiveImportJob(activeJob)) {
      return;
    }
    setCancellingImport(true);
    setErrorMessage("");
    const result = await jobCancel(activeJob.job_id);
    setCancellingImport(false);
    if (!result.ok) {
      setErrorMessage(formatProductError(result.error));
      return;
    }
    setActiveJob(result.data);
    if (result.data.state === "cancelled") {
      setStatusMessage("Importación cancelada.");
      setActiveJob(null);
      await refreshSources(activeNotebookId);
    } else {
      setStatusMessage("Cancelación solicitada; el trabajo terminará en breve.");
    }
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
    setSearchCompleted(true);
    setSearchHits(result.data.hits);
    setStatusMessage(
      result.data.hits.length === 0
        ? ""
        : `${result.data.hits.length} fragmento(s) encontrado(s).`,
    );
  };

  const hasReadySources = sources.some((s) => s.status === "ready");

  return (
    <div className="fuentes-panel" aria-busy={panelBusy}>
      <p className="fuentes-lead">
        Importa TXT/MD, revisa fuentes del cuaderno activo y busca texto indexado (FTS).
        Canal IPC: <strong>{transportLabel || "…"}</strong>.
      </p>

      <div className="fuentes-toolbar">
        <p className="meta-line fuentes-active-notebook" aria-live="polite">
          Cuaderno activo:{" "}
          <strong>{activeNotebook?.title ?? "ninguno (elige uno arriba)"}</strong>
        </p>

        <button
          type="button"
          className="ipc-button"
          onClick={() => void handleCreateNotebook()}
          disabled={panelBusy}
        >
          Crear cuaderno
        </button>

        <label className="ipc-button file-label" htmlFor={fileInputId}>
          Importar archivos
          <input
            ref={fileInputRef}
            id={fileInputId}
            type="file"
            accept=".txt,.md,text/plain,text/markdown"
            multiple
            aria-label="Seleccionar archivos TXT o Markdown para importar"
            disabled={panelBusy || !activeNotebookId}
            onChange={(e) => void handleImportFiles(e.target.files, e.currentTarget)}
          />
        </label>
      </div>

      {activeJob && isActiveImportJob(activeJob) && (
        <div className="job-status-row import-job-active" aria-live="polite">
          <p className="job-status">
            Importación: {activeJob.progress}% ({formatJobState(activeJob.state)})
          </p>
          <button
            type="button"
            className="ipc-button ipc-button-danger import-cancel-button"
            onClick={() => void handleCancelImport()}
            disabled={busy || cancellingImport}
          >
            {cancellingImport ? "Cancelando…" : "Cancelar importación"}
          </button>
        </div>
      )}

      {activeNotebook && (
        <p className="meta-line">
          Clasificación: {activeNotebook.classification === "public" ? "pública" : "privada"}
        </p>
      )}

      <section aria-labelledby="fuentes-list-heading">
        <h3 id="fuentes-list-heading">Fuentes del cuaderno</h3>
        {sources.length === 0 ? (
          <SourcesEmptyState hasActiveNotebook={Boolean(activeNotebookId)} />
        ) : (
          <ul className="source-list">
            {sources.map((source) => (
              <li key={source.source_id}>
                <span className="source-name">{source.display_name}</span>
                <span className="source-meta">
                  {formatSourceStatus(source.status)}
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
              disabled={panelBusy || !activeNotebookId}
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  void handleSearch();
                }
              }}
            />
          </label>
          <button
            type="button"
            className="ipc-button"
            onClick={() => void handleSearch()}
            disabled={panelBusy || !activeNotebookId}
          >
            Buscar
          </button>
        </div>

        <div
          id={searchResultsId}
          className="search-results"
          role="region"
          aria-live="polite"
          aria-label="Resultados de búsqueda en fuentes"
          aria-busy={panelBusy}
        >
          {searchCompleted && searchHits.length === 0 ? (
            <SearchHitsEmptyState query={searchQuery} hasIndexedSources={hasReadySources} />
          ) : searchHits.length > 0 ? (
            <ol className="hit-list" aria-label="Fragmentos encontrados">
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
          ) : null}
        </div>
      </section>

      <ProductMessage variant="status" message={statusMessage} />
      <ProductMessage variant="error" message={errorMessage} />
    </div>
  );
}
