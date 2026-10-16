# STATUS — Checkpoint canónico

- **Timestamp de referencia:** 2026-10-17T07:45Z
- **Fase:** **G1 aprobado** — **OLA 3 cerrada**; **OLA 4 tramo 1** en curso (TASK-0026..0028 integrados en `integration/mvp`; núcleo import→storage TASK-0027 integrado)
- **Implementación de producto:** `integration/mvp` — sidecar `native-text` TXT/MD (TASK-0026); contratos/UI Fuentes (TASK-0028 PR #7); núcleo Tauri import→SQLite/FTS + jobs in-memory (TASK-0027 PR #8 @ `8cc33d3`)
- **Gate actual:** G1 cerrado 2026-10-16; **no** declarar G2/G3/G4
- **G0:** aprobado 2026-10-16
- **`origin/main`:** `828415a` — G1 ADRs; **no modificar** en olas de producto
- **Integración:** `integration/mvp` @ **`4ef6dba`** (bitácora TASK-0027); merge follow-up `c8ceca9` ← `8cc33d3`; PR #8 @ **`16acd90`** ← **`a5a8f82`**; PR #7 @ **`e9c1d0a`** ← **`11dea11`**
- **OLA 1:** TASK-0008..0016 cerradas (G1)
- **OLA 2:** TASK-0017..0021 **done**
- **OLA 3:** TASK-0022..0024 **done**; OPEN-013 resuelta; OPEN-010 abierta
- **OLA 4:** TASK-0025 **done**; TASK-0026 **done**; TASK-0027 **done** (PR #8 + follow-up); TASK-0028 **done** (PR #7); OPEN-014 resuelta
- **TASK-0005:** paraguas `WBS-4.1`; tramo 1 = 0026..0028; chat/RAG/citas = tramo posterior
- **Avance R-PROD (código+pruebas+revisión):** extractores sidecar + UI Fuentes + núcleo Tauri import→storage; demo E2E manual pendiente smoke UI↔invoke
- **Evidencia reciente (TASK-0027):** `cargo test -p storage -p sanctumlm-desktop` **16** + **4** integration OK
- **Validador:** `python scripts/validate_docs.py` — obligatorio tras consolidación planner
- **Spikes:** sqlite-vec Windows **pass**; Ollama **not-run** (RSK-12)
- **Riesgos:** RSK-13, RSK-12, RSK-14, RSK-11
- **Bloqueo:** OPEN-005..012 + OPEN-010 abiertas
- **Security residuales (no blockers):** R-PROD-105 spawn/egress y límites invoke — ver DECISIONS (0021, 0023, 0024, 0026, 0027, 0028)
- **Siguiente:** smoke E2E UI↔Tauri desde `integration/mvp`; **no** declarar G2

Reserva: `.agents/FILE_RESERVATION.md`.
