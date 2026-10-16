# Núcleo Tauri (SanctumLM desktop)

## Jobs de importación (MVP / TASK-0027)

Los estados de `import_batch` (`queued`, `running`, `completed`, `cancelled`, …) persisten en SQLite
vía [`SqliteJobStore`](../../../crates/storage/src/sqlite_jobs.rs) (migración **0003**, TASK-0035).
[`InMemoryJobStore`](../../../crates/storage/src/job_memory.rs) queda solo para referencia/tests.

Implicaciones:

- Tras reiniciar la app, `job_status` recupera jobs no terminales; fuentes/chunks ya `ready` siguen en SQLite.
- `job_cancel` es **cooperativo**: la señal se observa **entre ítems** del lote, no a mitad de un ítem
  (ver comentario en `src/import_service.rs`).

## Sidecar opcional

Variable `SANCTUMLM_DOCUMENTS_WORKER`: ruta al binario `documents-worker` para tipos no TXT/MD.
