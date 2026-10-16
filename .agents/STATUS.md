# STATUS — Checkpoint canónico

- **Timestamp de referencia:** 2026-10-17T14:00Z
- **Fase:** **G1 aprobado** — **OLA 4 tramo 1 cerrada** (TASK-0026..0028 @ `15eba56`); **OLA 4 tramo 2** plan en vuelo (TASK-0029)
- **Implementación de producto:** import → chunks → FTS → UI búsqueda cableados en `integration/mvp` @ **`15eba56`**; chat/RAG/Ollama **sin cablear**
- **Gate actual:** G1 cerrado 2026-10-16; **no** declarar G2/G3/G4
- **G0:** aprobado 2026-10-16
- **`origin/main`:** `828415a` — G1 ADRs; **no modificar** en olas de producto
- **Integración:** `integration/mvp` @ **`15eba56`** (bitácora post TASK-0027; merges 0026..0028)
- **OLA 1–3:** cerradas (TASK-0008..0024)
- **OLA 4 tramo 1:** TASK-0025..0028 **done**; OPEN-014 resuelta
- **OLA 4 tramo 2:** TASK-0029 **in_progress** (plan docs); TASK-0030..0033 **ready**; OPEN-015 resuelta en rama planner
- **TASK-0005:** paraguas `WBS-4.1`; tramo 1 entregado; tramo 2 = 0030..0033; G2 pendiente humano
- **Avance R-PROD (código+pruebas+revisión):** parcial en 001/002/105 vía tramo 1; 006/007/008/104 pendientes tramo 2
- **Validador:** `python scripts/validate_docs.py` — obligatorio tras consolidación planner
- **Spikes:** sqlite-vec Windows **pass**; Ollama **not-run** (RSK-12)
- **Riesgos:** RSK-13, RSK-12, RSK-14, RSK-11
- **Bloqueo:** OPEN-005..012 + OPEN-010 abiertas; no bloquean tramo 2 documentado
- **Siguiente:** merge PR TASK-0029; asignar TASK-0030/0031/0033 en paralelo; TASK-0032 tras crates listos; **no** declarar G2

Reserva: `.agents/FILE_RESERVATION.md`.
