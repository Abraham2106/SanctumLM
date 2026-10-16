import { useId, useState } from "react";
import { useNotebookContext } from "../context/NotebookContext.tsx";
import ProductMessage from "./ProductMessage.tsx";

export default function NotebookListBar() {
  const filterId = useId();
  const listId = useId();
  const {
    filteredNotebooks,
    activeNotebookId,
    setActiveNotebookId,
    filter,
    setFilter,
    loading,
    error,
    createNotebook,
  } = useNotebookContext();
  const [creating, setCreating] = useState(false);

  const handleCreate = async () => {
    const title = window.prompt("Nombre del cuaderno", "Nuevo cuaderno");
    if (!title) {
      return;
    }
    setCreating(true);
    await createNotebook(title);
    setCreating(false);
  };

  return (
    <section className="notebook-list-bar" aria-labelledby="notebook-bar-heading">
      <div className="notebook-list-bar-header">
        <h2 id="notebook-bar-heading">Cuadernos</h2>
        <button
          type="button"
          className="ipc-button"
          onClick={() => void handleCreate()}
          disabled={loading || creating}
        >
          {creating ? "Creando…" : "Crear cuaderno"}
        </button>
      </div>

      <label className="field notebook-filter" htmlFor={filterId}>
        Buscar cuaderno
        <input
          id={filterId}
          type="search"
          value={filter}
          onChange={(e) => setFilter(e.target.value)}
          placeholder="Filtrar por título…"
          disabled={loading}
          aria-controls={listId}
        />
      </label>

      {loading ? (
        <p className="placeholder-note" role="status">Cargando cuadernos…</p>
      ) : filteredNotebooks.length === 0 ? (
        <p className="placeholder-note">
          {filter.trim()
            ? "Ningún cuaderno coincide con el filtro."
            : "No hay cuadernos todavía. Crea uno para empezar a importar fuentes."}
        </p>
      ) : (
        <ul id={listId} className="notebook-list" role="listbox" aria-label="Lista de cuadernos">
          {filteredNotebooks.map((nb) => {
            const selected = nb.notebook_id === activeNotebookId;
            return (
              <li key={nb.notebook_id}>
                <button
                  type="button"
                  role="option"
                  aria-selected={selected}
                  className={selected ? "notebook-list-item active" : "notebook-list-item"}
                  onClick={() => setActiveNotebookId(nb.notebook_id)}
                >
                  <span className="notebook-list-title">{nb.title}</span>
                  <span className="notebook-list-meta">
                    {nb.classification === "public" ? "pública" : "privada"}
                  </span>
                </button>
              </li>
            );
          })}
        </ul>
      )}

      <ProductMessage variant="error" message={error} />
    </section>
  );
}
