import { useCallback, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ChatPanel from "./components/ChatPanel.tsx";
import FuentesPanel from "./components/FuentesPanel.tsx";

type PanelId = "fuentes" | "chat" | "estudio";

const PANELS: { id: PanelId; label: string; hint: string }[] = [
  {
    id: "fuentes",
    label: "Fuentes",
    hint: "Importación, listado y búsqueda FTS del cuaderno activo.",
  },
  {
    id: "chat",
    label: "Chat",
    hint: "Chat de cuaderno o general; modelos vía IPC (mock hasta TASK-0032).",
  },
  {
    id: "estudio",
    label: "Estudio",
    hint: "Notas, resúmenes y artefactos del cuaderno (placeholder).",
  },
];

export default function App() {
  const [active, setActive] = useState<PanelId>("fuentes");
  const [ipcStatus, setIpcStatus] = useState<string>("sin probar");

  const probeIpc = useCallback(async () => {
    try {
      const reply = await invoke<string>("ping");
      setIpcStatus(reply);
    } catch {
      setIpcStatus("IPC no disponible (modo solo navegador)");
    }
  }, []);

  const current = PANELS.find((p) => p.id === active) ?? PANELS[0];

  return (
    <div className="app-shell">
      <header className="app-header">
        <h1>SanctumLM</h1>
        <p className="tagline">Cuaderno local — fuentes, búsqueda y chat (OLA 4 tramo 2)</p>
        <button type="button" className="ipc-button" onClick={() => void probeIpc()}>
          Probar IPC stub
        </button>
        <span className="ipc-status" aria-live="polite">
          Estado IPC: {ipcStatus}
        </span>
      </header>

      <nav className="panel-tabs" aria-label="Paneles principales">
        {PANELS.map((panel) => (
          <button
            key={panel.id}
            type="button"
            className={panel.id === active ? "tab active" : "tab"}
            aria-current={panel.id === active ? "page" : undefined}
            onClick={() => setActive(panel.id)}
          >
            {panel.label}
          </button>
        ))}
      </nav>

      <main className="panel-grid" id="main-content">
        {PANELS.map((panel) => (
          <section
            key={panel.id}
            className={`panel ${panel.id === active ? "panel-visible" : "panel-hidden"}`}
            aria-labelledby={`heading-${panel.id}`}
            hidden={panel.id !== active}
          >
            <h2 id={`heading-${panel.id}`}>{panel.label}</h2>
            {panel.id === "fuentes" ? (
              <FuentesPanel />
            ) : panel.id === "chat" ? (
              <ChatPanel />
            ) : (
              <>
                <p>{panel.hint}</p>
                <p className="placeholder-note">
                  Sin datos reales ni llamadas de red externa desde la UI.
                </p>
              </>
            )}
          </section>
        ))}
      </main>

      <footer className="app-footer">
        <span>Panel activo: {current.label}</span>
      </footer>
    </div>
  );
}
