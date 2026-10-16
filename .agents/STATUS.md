# STATUS — Checkpoint canónico

- **Timestamp de referencia:** 2026-10-16T08:00Z
- **Fase:** **G1 aprobado** — **OLA 4 tramo 1 cerrada** (TASK-0026..0028); **OLA 4 tramo 2** en ejecución (TASK-0029 plan @ `919b2fc`; núcleo 0030/0031 integrados)
- **Implementación de producto:** import → chunks → FTS → UI búsqueda @ `15eba56`; núcleo **ModelRouter/Ollama** (`4c9a775`) y **RAG FTS + citas** (`7826254`) en crates; chat desktop **sin cablear**
- **Gate actual:** G1 cerrado 2026-10-16; **no** declarar G2/G3/G4
- **G0:** aprobado 2026-10-16
- **`origin/main`:** `828415a` — G1 ADRs; **no modificar** en olas de producto
- **Integración:** `integration/mvp` @ **`7826254`** (merge TASK-0031 `--no-ff` ← `2222bf4`; bitácora post-merge 0031)
- **OLA 1–3:** cerradas (TASK-0008..0024)
- **OLA 4 tramo 1:** TASK-0025..0028 **done**; OPEN-014 resuelta
- **OLA 4 tramo 2:** TASK-0029 **done**; TASK-0030..0031 **done**; TASK-0032..0033 **ready**; OPEN-015 resuelta
- **TASK-0005:** paraguas `WBS-4.1`; tramo 1 entregado; tramo 2 = 0030..0033; G2 pendiente humano
- **Avance R-PROD (código+pruebas+revisión):** parcial 001/002/105 tramo 1; **104** parcial vía núcleo RAG/citations (0031); 006/007/008 pendientes cableado desktop/chat
- **Validador:** `python scripts/validate_docs.py` — obligatorio tras consolidación planner
- **Spikes:** sqlite-vec Windows **pass**; Ollama **not-run** (RSK-12)
- **Riesgos:** RSK-13, RSK-12, RSK-14, RSK-11
- **Bloqueo:** OPEN-005..012 + OPEN-010 abiertas; no bloquean tramo 2 documentado
- **Siguiente:** TASK-0032 (Tauri chat) tras 0030+0031; TASK-0033 en paralelo donde aplique; **no** declarar G2

Reserva: `.agents/FILE_RESERVATION.md`.
