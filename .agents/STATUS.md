# STATUS — Checkpoint canónico

- **Timestamp de referencia:** 2026-10-16T19:30Z
- **Fase:** **G1 aprobado** — OLA 2 (WBS-3.4) cimientos en curso en `integration/mvp`
- **Implementación de producto:** workspace Cargo + crates §12 y worker stub (TASK-0018 merged); CI mínimo docs+Rust (TASK-0020 merged); **sin** app desktop Tauri/React hasta TASK-0019
- **Gate actual:** G1 cerrado 2026-10-16; **no** declarar G2/G3/G4
- **G0:** aprobado 2026-10-16
- **`origin/main`:** `828415a` — G1 ADRs; **no modificar** en esta ola
- **Integración:** `integration/mvp` @ **`5546344`** (merge TASK-0020 `--no-ff` desde `4ce8c9d`, APPROVED); incluye `77eb850` (TASK-0018) y `1cc29e3` (TASK-0017)
- **OLA 1:** TASK-0008..0016 cerradas (G1)
- **OLA 2 (activa):** TASK-0017 **done**; TASK-0018 **done**; TASK-0020 **done**; TASK-0019 **in_progress**; TASK-0021 **draft** (espera cierre 0019)
- **ADRs producto G1 accepted:** 0006–0011 (ratificados en main G1)
- **Avance R-PROD (código+pruebas+revisión):** **0/26** — solo cimientos; diseño G1: 26/26 con TC-PROD y ADR
- **Evidencia reciente (TASK-0020):** `cargo test --workspace` 6 tests OK; `python scripts/validate_docs.py` OK (82 md); workflow `.github/workflows/ci.yml`
- **Validador:** `python scripts/validate_docs.py` — ejecutar tras cada consolidación documental en `.agents/`
- **Spikes:** sqlite-vec 0.1.9 Windows **pass**; Ollama modelos **not-run** (RSK-12)
- **Riesgos:** RSK-13, RSK-12, RSK-14, RSK-11
- **Bloqueo:** OPEN-009..012 abiertas; no bloquean merge de cimientos ya integrados
- **Siguiente:** completar TASK-0019 (stub desktop); luego planner puede lanzar TASK-0021 (contratos B2)

Reserva: `.agents/FILE_RESERVATION.md`.
