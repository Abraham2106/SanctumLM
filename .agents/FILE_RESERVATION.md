# Reserva de archivos

**Dueño del mapa:** planner. **Bitácora canónica:** rama **`main`** (squash-merge); histórico `integration/mvp` obsoleto post PR #12.

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

## OLA 3 — datos e ingesta (cimientos)

**Vigente desde:** 2026-10-16 (cierre OPEN-013). **Base integración:** `integration/mvp` @ `1b3c875` (post TASK-0021).

| Tarea | Superficie exclusiva | Prohibido |
| --- | --- | --- |
| TASK-0022 | `crates/storage/**` (migraciones SQL, `rusqlite`, pruebas de esquema/FTS en crate storage) | `crates/retrieval/**`, `workers/documents/**`, `apps/desktop/src/App.tsx`, `.github/**`, SQLCipher, `load_extension` usuario |
| TASK-0023 | `crates/retrieval/**` (consultas FTS, trait `VectorIndex`, tests aislamiento léxico) | `crates/storage/migrations/**` (salvo usar API pública de storage), `workers/documents/**`, `App.tsx`, `.github/**` |
| TASK-0024 | `workers/documents/**` (binario/proceso sidecar, contrato IPC, stubs adaptadores, tests contrato) | `crates/storage/migrations/**`, lógica FTS en retrieval, parsers Docling/MarkItDown pesados, `App.tsx`, `.github/**` |

**Compartidos (planner al integrar):** `docs/02-planning/WBS.md`, `docs/00-project/OPEN_QUESTIONS.md`, `.agents/TASKS.md`, `.agents/STATUS.md`, `.agents/GOAL.md`, este archivo.

**Cross-task:** ningún agente en vuelo edita la superficie exclusiva de otra fila OLA 3. Cambios en `packages/contracts` o `crates/core-domain` requieren tarea dedicada (no incluidos en 0022..0024).

**OLA 3 — liberación post-merge:** TASK-0022..0024 **liberadas** en integración @ `119743a`; cambios futuros vía ficha OLA 4+.

## OLA 4 — vertical slice tramo 1 (import → FTS → UI)

**Vigente desde:** 2026-10-17 (cierre OPEN-014). **Base integración:** `integration/mvp` @ `119743a` (OLA 3 cerrada: TASK-0022..0024).

| Tarea | Superficie exclusiva | Prohibido |
| --- | --- | --- |
| TASK-0025 planner | `.agents/*`, `docs/02-planning/WBS.md`, `docs/00-project/OPEN_QUESTIONS.md` (OPEN-014), consolidación | código producto |
| TASK-0026 | `workers/documents/**` | `crates/storage/**`, `apps/desktop/**`, `.github/**`, parsers pesados |
| TASK-0027 | `apps/desktop/src-tauri/src/**`, `crates/storage/src/**` excepto `migrations/**` | `apps/desktop/src/**` (UI), `packages/contracts/**`, `workers/documents/**` (salvo spawn bin), `.github/**`, chat/RAG |
| TASK-0028 | `packages/contracts/**`, `apps/desktop/src/**` excepto `src-tauri/**` | `apps/desktop/src-tauri/**`, `crates/**`, `workers/**`, `.github/**` |

**Compartidos (planner al integrar):** `.agents/TASKS.md`, `.agents/STATUS.md`, `.agents/GOAL.md`, `.agents/FILE_RESERVATION.md`, `docs/02-planning/WBS.md`.

**Cross-task:** 0027 implementa comandos; 0028 define tipos y UI — acordar nombres de comandos vía contratos (0028) antes de merge final E2E.

**Paralelo seguro:** 0026 ∥ 0023 ∥ 0028 (solo contratos/UI mock) mientras 0027 no esté en la misma superficie.

**OLA 4 tramo 1 — liberación post-merge:** TASK-0026..0028 **liberadas** en integración @ `15eba56`; cambios futuros vía ficha tramo 2+.

## OLA 4 — vertical slice tramo 2 (chat / RAG / Ollama)

**Vigente desde:** 2026-10-17 (cierre OPEN-015). **Base integración:** `integration/mvp` @ `15eba56` (OLA 4 tramo 1 cerrado: TASK-0026..0028).

| Tarea | Superficie exclusiva | Prohibido |
| --- | --- | --- |
| TASK-0029 planner | `.agents/*`, `docs/02-planning/WBS.md`, `docs/00-project/OPEN_QUESTIONS.md` (OPEN-015), consolidación | código producto |
| TASK-0030 | `crates/ai-router/**` | `apps/desktop/**`, `crates/retrieval/**`, `crates/storage/**`, `packages/contracts/**`, `.github/**`, remoto real |
| TASK-0031 | `crates/retrieval/**` | `crates/ai-router/**`, `apps/desktop/**`, `packages/contracts/**`, `storage/migrations/**`, `.github/**` |
| TASK-0032 | `apps/desktop/src-tauri/**`, `crates/storage/**` (incl. migración **0002** chats), `Cargo.lock` | `apps/desktop/src/**` (UI), `packages/contracts/**`, `crates/ai-router/**`, `crates/retrieval/**` (solo deps), `.github/**` |
| TASK-0033 | `packages/contracts/**`, `apps/desktop/src/**` excepto `src-tauri/**` | `apps/desktop/src-tauri/**`, `crates/**`, `workers/**`, `.github/**` |

**Compartidos (planner al integrar):** `.agents/TASKS.md`, `.agents/STATUS.md`, `.agents/GOAL.md`, `.agents/FILE_RESERVATION.md`, `docs/02-planning/WBS.md`.

**Cross-task:** 0032 implementa comandos; 0033 define tipos y UI — acordar nombres/payloads `chat_*` y `list_models` vía contratos (0033) antes de merge E2E final.

**Paralelo seguro:** 0030 ∥ 0031 ∥ 0033 (contratos/UI mock) mientras 0032 no esté en la misma superficie. **0032** tras 0030+0031 listos para integración.

**OLA 4 tramo 2 — liberación post-merge:** TASK-0032 **liberada** en `main` @ `78bd8ba` (squash PR #14); TASK-0033 **liberada** en squash PR #12 @ `f477628`; cambios futuros vía ficha tramo 3.

## OLA 4 — tramo 3 (E2E UI + jobs durables)

**Vigente desde:** 2026-10-16 post plan tramo 3 @ `e238ade`. **Base:** `origin/main`. PRs abiertos: #16 (0034), jobs SQLite (0035).

| Tarea | Superficie exclusiva | Prohibido |
| --- | --- | --- |
| TASK-0034 | `apps/desktop/src/**` excepto `src-tauri/**`, `packages/contracts/**` (mínimo), tests UI | `src-tauri/**`, `crates/**`, `workers/**`, `.github/**` |
| TASK-0035 | `crates/storage/**`, `apps/desktop/src-tauri/src/**` (jobs/import), `Cargo.lock` | `apps/desktop/src/**` (React), `packages/contracts/**`, `workers/**`, `crates/retrieval/**`, `crates/ai-router/**` |

**Compartidos (planner al integrar):** `.agents/TASKS.md`, `.agents/STATUS.md`, `.agents/GOAL.md`, este archivo.

**Paralelo seguro:** **0034** ∥ **0035** (superficies disjointas). Integrar 0035 antes o después de 0034; conflicto posible solo en `Cargo.lock` → planner secuencia merge si ambos tocan lock.

**Cross-task:** ningún agente en vuelo edita la superficie exclusiva de la otra fila tramo 3.

**OLA 4 tramo 3 — liberación post-merge:** TASK-0034 **liberada** en `main` @ `c2a7dd9` (PR #16); TASK-0035 **liberada** @ `b75d7ff` (PR #20); bitácora @ `6a600ed`.

## OLA 4 — tramo 4 (cierre slice MVP)

**Vigente desde:** 2026-10-16 post TASK-0035 @ `6a600ed`. **Base:** `origin/main`.

| Tarea | Superficie exclusiva | Prohibido |
| --- | --- | --- |
| TASK-0036 | `apps/desktop/src/**` excepto `src-tauri/**` | `src-tauri/**`, `crates/**`, `workers/**`, `.github/**` |
| TASK-0037 | `docs/04-quality/evidence/**`, `docs/01-requirements/TRACEABILITY.md`, `scripts/smoke-desktop.*` (opcional), `benchmarks/README.md` (mínimo) | código Rust/UI; `.github/**` |
| TASK-0038 | `.github/workflows/**`, `apps/desktop/package.json` (+ lock), `packages/contracts/package.json` (+ lock) si CI lo exige | `apps/desktop/src/**`, `src-tauri/**`, `crates/**`, build Tauri |

**Compartidos (planner al integrar):** `.agents/TASKS.md`, `.agents/STATUS.md`, `.agents/GOAL.md`, este archivo, `docs/02-planning/WBS.md`.

**Paralelo seguro:** **0036** ∥ **0037** ∥ **0038**. Conflicto posible en `package-lock.json` si 0038 toca deps — secuenciar merge con planner si ambos PRs abiertos.

**Cross-task:** ningún agente en vuelo edita la superficie exclusiva de otra fila tramo 4.
