# STATUS — Checkpoint canónico

- **Timestamp de referencia:** 2026-10-17T07:30Z
- **Fase:** **G1 aprobado** — **OLA 3 cerrada**; **OLA 4 tramo 1** en curso (TASK-0026..0028 integrados en `integration/mvp`; E2E import→storage pendiente 0027)
- **Implementación de producto:** `integration/mvp` — sidecar `native-text` TXT/MD (TASK-0026); contratos/UI Fuentes mock (TASK-0028 vía PR #7); **sin cableado desktop E2E** import→storage (0027 pendiente)
- **Gate actual:** G1 cerrado 2026-10-16; **no** declarar G2/G3/G4
- **G0:** aprobado 2026-10-16
- **`origin/main`:** `828415a` — G1 ADRs; **no modificar** en olas de producto
- **Integración:** `integration/mvp` @ **`0d67101`** (bitácora TASK-0026); merges producto: `e9c1d0a` (PR #7 TASK-0028 ← `11dea11`); `0e92dd6` (TASK-0026)
- **OLA 1:** TASK-0008..0016 cerradas (G1)
- **OLA 2:** TASK-0017..0021 **done**
- **OLA 3:** TASK-0022..0024 **done**; OPEN-013 resuelta; OPEN-010 abierta
- **OLA 4:** TASK-0025 **done**; TASK-0026 **done**; TASK-0027 **ready**; TASK-0028 **done** (PR #7); OPEN-014 resuelta
- **TASK-0005:** paraguas `WBS-4.1`; tramo 1 = 0026..0028; chat/RAG/citas = tramo posterior
- **Avance R-PROD (código+pruebas+revisión):** cimientos + extractores TXT/MD en sidecar; UI mock import/búsqueda; sin E2E storage
- **Evidencia reciente (TASK-0026):** `cargo test -p documents-worker --locked` **24** unit OK
- **Validador:** `python scripts/validate_docs.py` — obligatorio tras consolidación planner
- **Spikes:** sqlite-vec Windows **pass**; Ollama **not-run** (RSK-12)
- **Riesgos:** RSK-13, RSK-12, RSK-14, RSK-11
- **Bloqueo:** OPEN-005..012 + OPEN-010 abiertas
- **Security residuales (no blockers):** R-PROD-105 al cablear IPC/spawn — ver DECISIONS (0021, 0023, 0024, 0026, 0028)
- **Siguiente:** TASK-0027 desde `integration/mvp`; **no** declarar G2

Reserva: `.agents/FILE_RESERVATION.md`.
