# Reserva de archivos

**Dueño del mapa:** planner. **Bitácora canónica:** rama `integration/mvp` (worktree `SanctumLM-wt-integration`).

## OLA 1 (G1) — cerrada

Reservas TASK-0009..0015 **liberadas** tras G1 y merges documentales. Referencia histórica: base `4ff44a7`.

## OLA 2 — B1 cimientos

**Vigente desde:** 2026-10-16 post-merge TASK-0020. **Base integración:** `integration/mvp` @ **`5546344`** (+ bitácora planner).

| Tarea | Superficie exclusiva | Estado |
| --- | --- | --- |
| TASK-0008 planner | `.agents/*` (excepto briefs publicados), consolidación post-merge | activa (solo bitácora) |
| TASK-0017 | `benchmarks/**` (corpus B3) | **liberada** — merged `1cc29e3` |
| TASK-0018 | raíz `Cargo.toml`, `Cargo.lock`, `crates/**`, `workers/documents/**`, `packages/artifacts/**`, `scripts/verify.ps1`, `security/README.md`, entradas `.gitignore` de workspace | **liberada** — merged `77eb850`; cambios futuros vía nueva tarea |
| TASK-0019 | `apps/desktop/**`, config Tauri (`src-tauri/**` bajo desktop), `package.json` / lockfile UI del desktop, miembros workspace que añada solo el stub desktop | **activa** — en curso |
| TASK-0020 | `.github/workflows/**` (CI mínimo docs + Rust; sin desktop) | **liberada** — merged `5546344`; cambios futuros vía nueva tarea |
| TASK-0021 | contratos B2 en `docs/03-architecture/DATA_AND_INTERFACES.md` y anexos acordados en ficha | **bloqueada** — espera TASK-0019 |

**Compartidos (planner, secuencial al integrar):** `.agents/DECISIONS.md`, `.agents/STATUS.md`, `.agents/TASKS.md`, `docs/03-architecture/adr/README.md` cuando aplique.

**Prohibido cross-task:** ningún agente en vuelo edita la superficie activa de otra fila. TASK-0019 **no** debe modificar lógica de crates ya merged salvo el wiring mínimo autorizado en su ficha (p. ej. member en `Cargo.toml`).
