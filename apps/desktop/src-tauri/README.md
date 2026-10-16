# Núcleo Tauri (SanctumLM desktop)

## Jobs de importación (MVP / TASK-0027)

Los estados de `import_batch` (`queued`, `running`, `completed`, `cancelled`, …) viven en
[`InMemoryJobStore`](../../../crates/storage/src/job_memory.rs) dentro del proceso Tauri.
**No hay tabla `jobs` en SQLite** en la migración 0001 actual; la fila `jobs` descrita en
[ADR-0007](../../../docs/03-architecture/adr/ADR-0007-storage.md) queda para una tarea planner
posterior (sin schema ad hoc en este crate).

Implicaciones MVP:

- Tras reiniciar la app, `job_status` no recupera jobs antiguos; fuentes/chunks ya `ready` sí persisten.
- `job_cancel` es **cooperativo**: la señal se observa **entre ítems** del lote, no a mitad de un ítem
  (ver comentario en `src/import_service.rs`).

## Sidecar opcional

Variable `SANCTUMLM_DOCUMENTS_WORKER`: ruta al binario `documents-worker` para tipos no TXT/MD.
