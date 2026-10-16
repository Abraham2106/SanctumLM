# STATUS — Checkpoint canónico

- **Timestamp de referencia:** 2026-10-16T08:52Z
- **Política git (humano 2026-10-16):** rama única de integración **`main`**; merges vía **squash**; **`integration/mvp`** deja de ser destino de integración (histórico @ `c56e7a5`); worktrees futuros desde `main`
- **Fase:** **G1 aprobado** — OLA 4 tramo 1 **done** (0026..0028); tramo 2 **0030..0033** en `main`; **TASK-0032 done** (núcleo Tauri chat)
- **Implementación de producto:** `main` @ **`b75d7ff`** (squash PR #20 TASK-0035 jobs SQLite); UI IPC @ **`c2a7dd9`** (squash PR #16 TASK-0034); CI exclude desktop @ **`e99cd64`** (#13); chat @ **`78bd8ba`** (#14)
- **Gate actual:** G1 cerrado 2026-10-16; **no** declarar G2/G3/G4
- **G0:** aprobado 2026-10-16
- **`origin/main`:** **`b75d7ff`** — squash PR #20 (TASK-0035) sobre `c2a7dd9` (PR #16); APPROVE quality + security r2; sin force-push
- **Integración (legado):** `integration/mvp` @ `c56e7a5` (incl. TASK-0033 merge local → push previo a squash)
- **OLA 1–3:** cerradas (TASK-0008..0024)
- **OLA 4 tramo 1:** TASK-0025..0028 **done**
- **OLA 4 tramo 2:** TASK-0029 **done**; TASK-0030..0033 **done**
- **OLA 4 tramo 3:** TASK-0034 **done** (PR #16); TASK-0035 **done** (PR #20); jobs durables SQLite 0003
- **TASK-0005:** paraguas `WBS-4.1`; G2 pendiente humano
- **Avance R-PROD (código+pruebas+revisión):** parcial; **no** declarar 26/26 ni G2
- **Validador:** `python scripts/validate_docs.py` — obligatorio tras cambios en `.agents/`
- **Spikes:** sqlite-vec Windows **pass**; Ollama **not-run** (RSK-12)
- **Riesgos:** RSK-13, RSK-12, RSK-14, RSK-11
- **Bloqueo:** OPEN-005..012 + OPEN-010 abiertas
- **Siguiente:** smoke manual `tauri dev` (RSK-12); criterio humano TASK-0005 / G2; **no** declarar G2 ni 26/26 R-PROD

Reserva: `.agents/FILE_RESERVATION.md`.
