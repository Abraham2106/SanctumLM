type SearchHitsEmptyStateProps = {
  query: string;
  hasIndexedSources: boolean;
};

export default function SearchHitsEmptyState({
  query,
  hasIndexedSources,
}: SearchHitsEmptyStateProps) {
  const trimmed = query.trim();

  if (!hasIndexedSources) {
    return (
      <div className="empty-state empty-state-search" aria-labelledby="search-empty-no-sources">
        <h4 id="search-empty-no-sources">Sin fuentes indexadas</h4>
        <p>
          Importa archivos TXT o MD en este cuaderno antes de buscar fragmentos con FTS
          (R-PROD-105).
        </p>
      </div>
    );
  }

  return (
    <div className="empty-state empty-state-search" aria-labelledby="search-empty-no-hits">
      <h4 id="search-empty-no-hits">Sin coincidencias</h4>
      <p>
        {trimmed
          ? `No hay fragmentos que contengan «${trimmed}» en las fuentes listas del cuaderno activo.`
          : "Escribe un término y pulsa Buscar (o Enter) para buscar en el índice FTS."}
      </p>
      <p className="placeholder-note">
        Prueba sinónimos, raíces de palabras o revisa que la fuente esté en estado listo.
      </p>
    </div>
  );
}
