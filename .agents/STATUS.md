# STATUS — Checkpoint canónico

- **Timestamp de referencia:** 2026-10-16T08:05Z
- **Política git (humano 2026-10-16):** rama única de integración **`main`**; merges vía **squash**; **`integration/mvp`** deja de ser destino de integración (histórico @ `c56e7a5`); worktrees futuros desde `main`
- **Fase:** **G1 aprobado** — OLA 4 tramo 1 **done** (0026..0028); tramo 2 núcleo **0030/0031/0033** en `main`; **0032 no iniciado**
- **Implementación de producto:** consolidada en `main` @ **`f477628`** (squash PR #12: OLA 2–4 + chat UI mock); import/FTS/UI Fuentes + ModelRouter/Ollama + RAG/citas + contratos/UI chat mock
- **Gate actual:** G1 cerrado 2026-10-16; **no** declarar G2/G3/G4
- **G0:** aprobado 2026-10-16
- **`origin/main`:** **`f477628`** — squash #12 sobre base **`520234b`** (PR #5 intacto; sin force-push)
- **Integración (legado):** `integration/mvp` @ `c56e7a5` (incl. TASK-0033 merge local → push previo a squash)
- **OLA 1–3:** cerradas (TASK-0008..0024)
- **OLA 4 tramo 1:** TASK-0025..0028 **done**
- **OLA 4 tramo 2:** TASK-0029 **done**; TASK-0030..0031 **done**; TASK-0033 **done** (UI/contratos mock); TASK-0032 **ready** (Tauri chat núcleo — **no asignar aún**)
- **TASK-0005:** paraguas `WBS-4.1`; G2 pendiente humano
- **Avance R-PROD (código+pruebas+revisión):** parcial; **no** declarar 26/26 ni G2
- **Validador:** `python scripts/validate_docs.py` — obligatorio tras cambios en `.agents/`
- **Spikes:** sqlite-vec Windows **pass**; Ollama **not-run** (RSK-12)
- **Riesgos:** RSK-13, RSK-12, RSK-14, RSK-11
- **Bloqueo:** OPEN-005..012 + OPEN-010 abiertas
- **Siguiente:** planificar TASK-0032 desde **`main`** cuando el humano lo autorice; **no** declarar G2

Reserva: `.agents/FILE_RESERVATION.md`.
