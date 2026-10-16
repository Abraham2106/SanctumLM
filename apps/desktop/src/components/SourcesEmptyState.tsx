type SourcesEmptyStateProps = {
  hasActiveNotebook: boolean;
};

export default function SourcesEmptyState({ hasActiveNotebook }: SourcesEmptyStateProps) {
  if (!hasActiveNotebook) {
    return (
      <div className="empty-state empty-state-notebooks" aria-labelledby="sources-empty-no-nb">
        <h4 id="sources-empty-no-nb">Sin cuaderno activo</h4>
        <p>
          Crea un cuaderno en la barra superior o elige uno de la lista para importar fuentes TXT/MD
          (R-PROD-001, R-PROD-002).
        </p>
      </div>
    );
  }

  return (
    <div className="empty-state empty-state-sources" aria-labelledby="sources-empty-title">
      <h4 id="sources-empty-title">Aún no hay fuentes en este cuaderno</h4>
      <p>
        Usa <strong>Importar archivos</strong> para añadir texto plano o Markdown. Los fragmentos se
        indexan para búsqueda FTS y para el chat contextual del cuaderno.
      </p>
      <p className="placeholder-note">Formatos admitidos en este tramo: .txt y .md.</p>
    </div>
  );
}
