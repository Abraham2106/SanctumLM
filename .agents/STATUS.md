# STATUS — Checkpoint canónico

- **Timestamp de referencia:** 2026-10-17T07:05Z
- **Fase:** **G1 aprobado** — OLA 2 cerrada; **OLA 3 en curso** (TASK-0022 + TASK-0024 integrados; TASK-0023 in_progress)
- **Implementación de producto:** workspace Cargo + crates §12 (TASK-0018); CI mínimo docs+Rust (TASK-0020); stub desktop Tauri 2 + React/TS (TASK-0019); contratos B2 Rust/TS + fakes in-memory (TASK-0021); **SQLite migración 0001 + FTS5 + notebook repo** (TASK-0022); **sidecar documents-worker IPC/staging skeleton** (TASK-0024, ADR-0009 — no cableado desktop)
- **Gate actual:** G1 cerrado 2026-10-16; **no** declarar G2/G3/G4
- **G0:** aprobado 2026-10-16
- **`origin/main`:** `828415a` — G1 ADRs; **no modificar** en esta ola
- **Integración:** `integration/mvp` @ **`f002f47`** (merge TASK-0024 `--no-ff` ← `ca24b48`); previo `0a3b5ae` (bitácora TASK-0022), `fccd17d` (merge TASK-0022); TASK-0021 en historial `d9a8b20`
- **OLA 1:** TASK-0008..0016 cerradas (G1)
- **OLA 2:** TASK-0017..0021 **done** — cimientos B1+B2 integrados
- **OLA 3:** TASK-0022 **done**; TASK-0024 **done** (integrado); TASK-0023 **in_progress**; OPEN-013 resuelta
- **ADRs producto G1 accepted:** 0006–0011 (ratificados en main G1)
- **Avance R-PROD (código+pruebas+revisión):** **0/26** — cimientos y contratos; sin features de producto cableadas
- **Evidencia reciente (TASK-0024):** `cargo test --workspace --locked` **33** unit + doc-tests OK (documents-worker +13 IPC/staging; storage +11); `Cargo.lock` ort sin conflictos manuales (rusqlite/storage + serde_json/documents-worker)
- **Evidencia previa (TASK-0022):** `cargo test --workspace` **22** tests OK (storage FTS/WAL/FK)
- **Evidencia previa (TASK-0021):** `npm test` (`tsc --noEmit`) en `packages/contracts` OK
- **Validador:** `python scripts/validate_docs.py` — ejecutar tras cada consolidación documental en `.agents/`
- **Spikes:** sqlite-vec 0.1.9 Windows **pass**; Ollama modelos **not-run** (RSK-12)
- **Riesgos:** RSK-13, RSK-12, RSK-14, RSK-11
- **Security residuales TASK-0021 (no blockers):** api_key en contratos; nested forbidden keys; R-PROD-105 al cablear IPC — ver nota en DECISIONS
- **Bloqueo:** OPEN-009..012 + OPEN-005/006/008 abiertas; no bloquean cimientos integrados
- **Security residuales TASK-0024 (no blockers):** validación staging path-only; sin egress en tests; R-PROD-105 y spawn desktop al cablear — ver nota en DECISIONS
- **Siguiente:** implementar TASK-0023; TASK-0005 vertical slice sigue sin ficha G2; **no** declarar G2

Reserva: `.agents/FILE_RESERVATION.md`.
