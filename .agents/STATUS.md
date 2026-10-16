# STATUS — Checkpoint canónico

- **Timestamp de referencia:** 2026-10-16T20:00Z
- **Fase:** **G1 aprobado** — OLA 2 (WBS-3.4) cimientos en curso en `integration/mvp`
- **Implementación de producto:** workspace Cargo + crates §12, worker stub (TASK-0018); CI mínimo docs+Rust (TASK-0020); stub desktop Tauri 2 + React/TS (TASK-0019 merged)
- **Gate actual:** G1 cerrado 2026-10-16; **no** declarar G2/G3/G4
- **G0:** aprobado 2026-10-16
- **`origin/main`:** `828415a` — G1 ADRs; **no modificar** en esta ola
- **Integración:** `integration/mvp` @ **`2180b6a`** (merge TASK-0019 `--no-ff` desde `87dff19`, APPROVED); incluye `8554070` bitácora TASK-0020, `5546344` (TASK-0020), `77eb850` (TASK-0018), `1cc29e3` (TASK-0017)
- **OLA 1:** TASK-0008..0016 cerradas (G1)
- **OLA 2 (activa):** TASK-0017 **done**; TASK-0018 **done**; TASK-0019 **done**; TASK-0020 **done**; TASK-0021 **in_progress** (contratos B2)
- **ADRs producto G1 accepted:** 0006–0011 (ratificados en main G1)
- **Avance R-PROD (código+pruebas+revisión):** **0/26** — solo cimientos (stub); diseño G1: 26/26 con TC-PROD y ADR
- **Evidencia reciente (TASK-0019):** `cargo test --workspace` 7 tests OK (incl. `sanctumlm_desktop` ping_stub); `npm test` (`tsc --noEmit`) en `apps/desktop`; job CI `desktop` sigue `if: false` (residual TASK-0020)
- **Validador:** `python scripts/validate_docs.py` — ejecutar tras cada consolidación documental en `.agents/`
- **Spikes:** sqlite-vec 0.1.9 Windows **pass**; Ollama modelos **not-run** (RSK-12)
- **Riesgos:** RSK-13, RSK-12, RSK-14, RSK-11
- **Bloqueo:** OPEN-009..012 abiertas; no bloquean cimientos ya integrados
- **Siguiente:** TASK-0021 (contratos B2 en docs); habilitación CI desktop fuera de alcance hasta tarea dedicada

Reserva: `.agents/FILE_RESERVATION.md`.
