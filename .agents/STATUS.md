# STATUS — Checkpoint canónico

- **Timestamp de referencia:** 2026-10-16T14:25Z
- **Política git (humano 2026-10-16):** rama única de integración **`main`**; merges vía **squash**; **`integration/mvp`** deja de ser destino de integración (histórico @ `c56e7a5`); worktrees futuros desde `main`
- **Fase:** **G1 aprobado** — OLA 4 tramo 1 **done** (0026..0028); tramo 2 **0030..0033** en `main`; **TASK-0032 done** (núcleo Tauri chat)
- **Implementación de producto:** `main` @ **`84f2ca0`** (bitácora post TASK-0032); código chat @ **`78bd8ba`** (squash PR #14); CI @ **`e99cd64`** (#13); squash cimientos #12 @ `f477628`
- **Gate actual:** G1 cerrado 2026-10-16; **no** declarar G2/G3/G4
- **G0:** aprobado 2026-10-16
- **`origin/main`:** **`84f2ca0`** — bitácora TASK-0032 done; squash PR #14 @ `78bd8ba`; sin force-push
- **Integración (legado):** `integration/mvp` @ `c56e7a5` (incl. TASK-0033 merge local → push previo a squash)
- **OLA 1–3:** cerradas (TASK-0008..0024)
- **OLA 4 tramo 1:** TASK-0025..0028 **done**
- **OLA 4 tramo 2:** TASK-0029 **done**; TASK-0030..0033 **done**; UI aún degrada a mock si `invoke` falla o en Vite sin Tauri
- **OLA 4 tramo 3:** TASK-0034 **ready** (E2E UI↔IPC, prioridad chat); TASK-0035 **ready** (jobs SQLite) ∥ 0034
- **TASK-0005:** paraguas `WBS-4.1`; G2 pendiente humano
- **Avance R-PROD (código+pruebas+revisión):** parcial; **no** declarar 26/26 ni G2
- **Validador:** `python scripts/validate_docs.py` — obligatorio tras cambios en `.agents/`
- **Spikes:** sqlite-vec Windows **pass**; Ollama **not-run** (RSK-12)
- **Riesgos:** RSK-13, RSK-12, RSK-14, RSK-11
- **Bloqueo:** OPEN-005..012 + OPEN-010 abiertas
- **Siguiente:** TASK-0034 implementer (E2E); TASK-0035 jobs DB; criterio humano TASK-0005 / G2; **no** declarar G2 ni 26/26 R-PROD

Reserva: `.agents/FILE_RESERVATION.md`.
