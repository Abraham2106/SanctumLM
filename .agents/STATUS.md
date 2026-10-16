# STATUS — Checkpoint canónico

- **Timestamp de referencia:** 2026-10-16T22:00Z
- **Fase:** **G1 aprobado** — OLA 2 (WBS-3.4) **cimientos cerrados** en `integration/mvp`
- **Implementación de producto:** workspace Cargo + crates §12 (TASK-0018); CI mínimo docs+Rust (TASK-0020); stub desktop Tauri 2 + React/TS (TASK-0019); contratos B2 Rust/TS + fakes in-memory (TASK-0021)
- **Gate actual:** G1 cerrado 2026-10-16; **no** declarar G2/G3/G4
- **G0:** aprobado 2026-10-16
- **`origin/main`:** `828415a` — G1 ADRs; **no modificar** en esta ola
- **Integración:** `integration/mvp` @ **`70e411c`** (merge TASK-0021 `a88f28f` `--no-ff` desde `d9a8b20`, APPROVED reviewer + security); base previa `6bf5eaa` (0019+0020)
- **OLA 1:** TASK-0008..0016 cerradas (G1)
- **OLA 2:** TASK-0017..0021 **done** — cimientos B1+B2 integrados
- **ADRs producto G1 accepted:** 0006–0011 (ratificados en main G1)
- **Avance R-PROD (código+pruebas+revisión):** **0/26** — cimientos y contratos; sin features de producto cableadas
- **Evidencia reciente (TASK-0021):** `cargo test --workspace` **13** tests OK; `npm test` (`tsc --noEmit`) en `packages/contracts` OK; `python scripts/validate_docs.py` OK (84 MD)
- **Validador:** `python scripts/validate_docs.py` — ejecutar tras cada consolidación documental en `.agents/`
- **Spikes:** sqlite-vec 0.1.9 Windows **pass**; Ollama modelos **not-run** (RSK-12)
- **Riesgos:** RSK-13, RSK-12, RSK-14, RSK-11
- **Security residuales TASK-0021 (no blockers):** api_key en contratos; nested forbidden keys; R-PROD-105 al cablear IPC — ver nota en DECISIONS
- **Bloqueo:** OPEN-009..012 + OPEN-005/006/008 abiertas; no bloquean cimientos integrados
- **Siguiente:** planner asigna OLA 3 / vertical slice (TASK-0005) cuando corresponda; habilitación CI desktop fuera de alcance hasta tarea dedicada

Reserva: `.agents/FILE_RESERVATION.md`.
