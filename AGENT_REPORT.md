# Handoff — TASK-0027 núcleo Tauri import → storage/jobs

## Git

- **PR:** [#8](https://github.com/Abraham2106/SanctumLM/pull/8)
- **Rama:** `agent/TASK-0027-tauri-import-jobs`
- **Worktree:** `C:\Users\solan\Documents\Personal\Startup\SanctumLM-wt-TASK-0027`
- **Base:** `4df4de5` (TASK-0028 contratos IPC)

## Jobs in-memory (sin migración SQL)

Estado de jobs (`import_batch`, progreso, cancelación) en `storage::InMemoryJobStore` + hilo en
`import_service`. **No se añadió** tabla `jobs` a migraciones (fuera de alcance); persistencia SQL
según ADR-0007 §0001 queda planificada por planner. Documentación operativa:
`apps/desktop/src-tauri/README.md` y comentarios en `job_memory.rs` / `import_service.rs`.

## Cancelación cooperativa

`job_cancel` → `CancelFlag`; el worker revisa **entre ítems** del lote. Ítems ya confirmados como
`ready` no se revierten. Test: `batch_cancel_while_running_reports_cancelled`.

## Pruebas

```powershell
cd C:\Users\solan\Documents\Personal\Startup\SanctumLM-wt-TASK-0027
cargo test -p storage -p sanctumlm-desktop
```

## Comandos IPC (`api_version: 1`, `invoke({ request })`, `IpcResult`)

`notebook_create`, `notebook_list`, `list_sources`, `import_sources`, `job_status`, `job_cancel`, `search_sources`
