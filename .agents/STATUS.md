# STATUS — Checkpoint canónico

- **Timestamp de referencia:** 2026-10-16T09:00Z
- **Política git (humano 2026-10-16):** rama única de integración **`main`**; merges vía **squash**; **`integration/mvp`** histórico @ `c56e7a5`; worktrees desde `main`
- **Fase:** **G1 aprobado** — OLA 4 tramos 1–3 **done** (0026..0035); **OLA 4 tramo 4** planificado (TASK-0036..0038 **ready**)
- **Implementación de producto:** `main` @ **`6a600ed`** (bitácora post TASK-0035); jobs SQLite @ `b75d7ff` (PR #20); UI IPC @ `c2a7dd9` (PR #16); chat @ `78bd8ba` (#14)
- **Gate actual:** G1 cerrado 2026-10-16; **no** declarar G2/G3/G4
- **G0:** aprobado 2026-10-16
- **`origin/main`:** **`6a600ed`** — bitácora TASK-0035; squash #20 sobre #16; sin force-push
- **OLA 1–3:** cerradas (TASK-0008..0024)
- **OLA 4 tramo 1–2:** TASK-0025..0033 **done**
- **OLA 4 tramo 3:** TASK-0034 **done** (PR #16); TASK-0035 **done** (PR #20)
- **OLA 4 tramo 4:** TASK-0036 (UI cancel import), TASK-0037 (evidencia E2E), TASK-0038 (CI tsc) — **ready**, paralelo
- **TASK-0005:** paraguas `WBS-4.1`; G2 pendiente humano; TC-PROD formal tras criterio patrocinador
- **Avance R-PROD:** parcial en código; evidencia E2E pendiente TASK-0037; **no** declarar 26/26 ni G2
- **Validador:** `python scripts/validate_docs.py` — obligatorio tras cambios en `.agents/` o TRACEABILITY
- **Spikes:** sqlite-vec Windows **pass**; Ollama **not-run** (RSK-12)
- **Riesgos:** RSK-13, RSK-12, RSK-14, RSK-11
- **Bloqueo producto (humanos):** OPEN-009..012 (empaquetado, conectores piloto, artefactos visuales, etc.); OPEN-005..008, OPEN-010 abiertas — **no** bloquean TASK-0036..0038
- **Siguiente:** implementers en `cursor/task-0036-import-cancel-ui-65d0`, `cursor/task-0037-e2e-evidence-65d0`, `cursor/task-0038-ci-desktop-tsc-65d0`; smoke manual `tauri dev` (RSK-12); **no** declarar G2

Reserva: `.agents/FILE_RESERVATION.md`.
