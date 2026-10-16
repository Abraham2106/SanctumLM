import { useEffect, useState, type CSSProperties } from "react";
import ChatPanel from "./components/ChatPanel.tsx";
import FuentesPanel from "./components/FuentesPanel.tsx";
import NotebookListBar from "./components/NotebookListBar.tsx";
import { NotebookProvider } from "./context/NotebookContext.tsx";
import { getIpcTransport, notebookList } from "./lib/ipc-client.ts";
import { shouldUseMockIpc } from "./lib/ipc-runtime.ts";

const ESTUDIO_EMPTY_TITLE = "Sin artefactos todavía";
const ESTUDIO_EMPTY_BODY =
  "Aquí irán notas, resúmenes y exportaciones del cuaderno activo. " +
  "Mientras tanto puedes importar fuentes y conversar en el chat contextual.";
const ESTUDIO_EMPTY_FOOTER =
  "La UI no abre red ni proveedores de IA por su cuenta; todo pasa por IPC al núcleo.";

function transportLabelFromMode(): string {
  return getIpcTransport() === "mock" ? "simulado" : "núcleo Tauri";
}

export default function App() {
  const [transportLabel, setTransportLabel] = useState(() => transportLabelFromMode());
  const [wideLayout, setWideLayout] = useState(() =>
    typeof window !== "undefined" && window.matchMedia("(min-width: 960px)").matches,
  );

  useEffect(() => {
    const mq = window.matchMedia("(min-width: 960px)");
    const onChange = () => setWideLayout(mq.matches);
    mq.addEventListener("change", onChange);
    return () => mq.removeEventListener("change", onChange);
  }, []);

  useEffect(() => {
    let cancelled = false;
    void notebookList().finally(() => {
      if (!cancelled) {
        setTransportLabel(transportLabelFromMode());
      }
    });
    return () => {
      cancelled = true;
    };
  }, []);

  const workspaceStyle: CSSProperties = {
    flex: 1,
    display: "grid",
    gap: "1rem",
    gridTemplateColumns: wideLayout
      ? "minmax(0, 1fr) minmax(0, 1.15fr) minmax(0, 0.85fr)"
      : "1fr",
    alignItems: "start",
  };

  const mockDevNote = shouldUseMockIpc()
    ? " Mock IPC activo (navegador sin Tauri o VITE_IPC_MOCK=1)."
    : "";

  return (
    <NotebookProvider>
    <div className="app-shell">
      <style>{`
        .skip-link {
          position: absolute;
          left: -9999px;
          top: 0.5rem;
          z-index: 100;
          padding: 0.35rem 0.75rem;
          background: var(--panel-bg);
          border: 1px solid var(--border);
          border-radius: 6px;
          color: inherit;
          text-decoration: none;
        }
        .skip-link:focus {
          left: 1rem;
        }
        .transport-indicator {
          margin: 0.5rem 0 0;
          font-size: 0.9rem;
          opacity: 0.92;
        }
        .estudio-empty h3 {
          margin: 0 0 0.5rem;
          font-size: 1rem;
        }
        .estudio-empty p {
          margin: 0 0 0.65rem;
        }
      `}</style>
      <a className="skip-link" href="#main-content">
        Ir al contenido principal
      </a>

      <header className="app-header">
        <h1>SanctumLM</h1>
        <p className="tagline">
          Cuaderno local en español — columnas fuentes, chat y estudio (hipótesis OPEN-007).
        </p>
        <p className="transport-indicator" role="status" aria-live="polite">
          Transporte IPC: <strong>{transportLabel}</strong>.
          {transportLabel === "núcleo Tauri"
            ? " Los paneles usan comandos Tauri reales."
            : " Datos de demostración en memoria para desarrollo UI."}
          {mockDevNote}
        </p>
      </header>

      <NotebookListBar />

      <main className="panel-grid workspace-layout" id="main-content" style={workspaceStyle}>
        <section className="panel panel-visible" aria-labelledby="heading-fuentes">
          <h2 id="heading-fuentes">Fuentes</h2>
          <FuentesPanel />
        </section>

        <section className="panel panel-visible" aria-labelledby="heading-chat">
          <h2 id="heading-chat">Chat</h2>
          <ChatPanel />
        </section>

        <section className="panel panel-visible" aria-labelledby="heading-estudio">
          <h2 id="heading-estudio">Estudio</h2>
          <div className="estudio-empty" aria-labelledby="estudio-empty-title">
            <h3 id="estudio-empty-title">{ESTUDIO_EMPTY_TITLE}</h3>
            <p>{ESTUDIO_EMPTY_BODY}</p>
            <p className="placeholder-note">{ESTUDIO_EMPTY_FOOTER}</p>
          </div>
        </section>
      </main>

      <footer className="app-footer">
        <span>
          Diseño: fuentes | chat | estudio. Transporte: {transportLabel}.
        </span>
      </footer>
    </div>
    </NotebookProvider>
  );
}
