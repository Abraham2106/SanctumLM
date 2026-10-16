# Handoff — TASK-0021 B2 contratos y fakes

## Git

- **Rama:** `agent/TASK-0021-b2-contracts`
- **Base:** `agent/TASK-0019-b1-desktop` @ `87dff19`
- **Worktree:** `C:\Users\solan\Documents\Personal\Startup\SanctumLM-wt-TASK-0021`

## Alcance

Contratos Rust (traits/tipos) + fakes in-memory + paquete TS `packages/contracts` + reexport mínimo `apps/desktop/src/ipc-types.ts`. Sin RAG real, sin IPC wired, sin tocar `App.tsx`.

## Archivos principales

- `crates/core-domain/` — `ProductError`, `IpcResult`, jobs, `ArtifactSpec` + validación prohibiciones ADR-0010
- `crates/ai-router/` — `ModelAdapter` + `FakeModelAdapter`
- `crates/storage/` — `NotebookRepository` + `InMemoryNotebookRepository`
- `crates/retrieval/` — `ChunkSearch` + `FakeChunkSearch`
- `crates/connectors/` — web/URL stubs + fakes (SSRF loopback bloqueado en fake)
- `workers/documents/` — `DocumentExtractor` + fake
- `packages/contracts/` — tipos IPC/producto TS
- `apps/desktop/src/ipc-types.ts` — puente renderer
- `.agents/FILE_RESERVATION.md` — fila TASK-0021

## Pruebas

```text
cargo test   # workspace completo
cd packages/contracts && npm install && npm test
cd apps/desktop && npm test
```

## Gaps / OPEN (no bloqueantes)

- **OPEN-005:** marcas concretas de buscador/OAuth siguen abiertas; fakes simulan resultados.
- **OPEN-006:** reserva `internal|restricted` no modelada; solo `public|private` MVP.
- **OPEN-008:** modelos Ollama reales no probados; fake usa `llama3.2:latest` como id de ejemplo.
- **Dual source of truth:** tipos TS y Rust se mantienen alineados manualmente hasta generación de esquema (futuro ADR).

## Riesgos (security / IPC)

- Tipos IPC incluyen `selected_source_ids` y URLs: el núcleo debe fijar alcance (R-PROD-105) — no confiar en listas del renderer cuando se cablee IPC.
- `ArtifactSpec` valida claves prohibidas en stub; lista no exhaustiva frente a ADR-0010 (tokens anidados en JSON profundo — revisar en implementación real).
- Payloads IPC aún no validados en Tauri; exponer solo tras validación Rust + `api_version`.
