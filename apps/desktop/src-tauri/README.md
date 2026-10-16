# Núcleo Tauri (SanctumLM desktop)

## Jobs de importación

Los estados de `import_batch` persisten en SQLite (migración **0003**, tabla `jobs`, ADR-0007) vía
[`SqliteJobRepository`](../../../crates/storage/src/sqlite_job.rs). Tras reiniciar el proceso,
`job_status` puede consultar jobs no terminales que quedaron en disco.

`job_cancel` es **cooperativo**: [`CancelFlag`](../../../crates/storage/src/job_memory.rs) vive en
memoria del proceso; la señal se observa **entre ítems** del lote (ver `src/import_service.rs`).
Fuentes ya persistidas como `ready` en ítems anteriores no se revierten (R-PROD-014); reimportar
el mismo SHA-256 reutiliza la fuente `ready` existente.

## Sidecar opcional

Variable `SANCTUMLM_DOCUMENTS_WORKER`: ruta al binario `documents-worker` para tipos no TXT/MD.
