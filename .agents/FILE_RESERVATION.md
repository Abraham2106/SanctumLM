# Reserva de archivos

**Dueño del mapa:** planner. **Bitácora canónica:** rama `integration/mvp` (worktree `SanctumLM-wt-integration`).

## OLA 1 (G1) — cerrada

Reservas TASK-0009..0015 **liberadas** tras G1 y merges documentales. Referencia histórica: base `4ff44a7`.

## OLA 2 — B1 cimientos + B2 contratos

**Vigente desde:** 2026-10-16 post-merge TASK-0021. **Base integración:** `integration/mvp` (HEAD post-merge TASK-0021).

| Tarea | Superficie exclusiva | Estado |
| --- | --- | --- |
| TASK-0008 planner | `.agents/*` (excepto briefs publicados), consolidación post-merge | activa (solo bitácora) |
| TASK-0017 | `benchmarks/**` (corpus B3) | **liberada** — merged `1cc29e3` |
| TASK-0018 | raíz `Cargo.toml`, `Cargo.lock`, `crates/**`, `workers/documents/**`, `packages/artifacts/**`, `scripts/verify.ps1`, `security/README.md`, entradas `.gitignore` de workspace | **liberada** — merged `77eb850`; cambios futuros vía nueva tarea |
| TASK-0019 | `apps/desktop/**`, config Tauri (`src-tauri/**` bajo desktop), `package.json` / lockfile UI del desktop, miembros workspace del stub desktop | **liberada** — merged `2180b6a`; cambios futuros vía nueva tarea |
| TASK-0020 | `.github/workflows/**` (CI mínimo docs + Rust; job desktop residual `if: false`) | **liberada** — merged `5546344`; cambios futuros vía nueva tarea |
| TASK-0021 B2 | `crates/core-domain/**`, `crates/storage/**`, `crates/retrieval/**`, `crates/ai-router/**`, `crates/connectors/**`, `workers/documents/**`, `packages/contracts/**`, `apps/desktop/src/ipc-types.ts` | **liberada** — merged `d9a8b20`; prohibido futuro sin nueva ficha: `App.tsx`, `.github/**`, RAG/SQLite/Ollama real |

**Compartidos (planner, secuencial al integrar):** `.agents/DECISIONS.md`, `.agents/STATUS.md`, `.agents/TASKS.md`, `docs/03-architecture/adr/README.md` cuando aplique.

**Prohibido cross-task:** ningún agente en vuelo edita la superficie activa de otra fila.

**A7 (TASK-0015)** exclusiva histórica: `ARCHITECTURE.md`, `DATA_AND_INTERFACES.md`, `TEST_PLAN.md`. Cerrada G1; cambios futuros vía nueva tarea.
