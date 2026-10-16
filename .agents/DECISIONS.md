# DECISIONS — Índice consolidado

El contenido normativo y completo de cada decisión está en [ADR](../docs/03-architecture/adr/README.md). Este archivo solo facilita estado del planner.

| ID | Tema | Estado | Nota |
| --- | --- | --- | --- |
| ADR-0001 | Docs-as-code | accepted | Flujo de desarrollo |
| ADR-0002 | Worktree por task | accepted | Flujo de desarrollo |
| ADR-0003 | Planner coordinador | accepted | Flujo de desarrollo |
| ADR-0004 | Canal durable/efímero | proposed | Requiere decidir runtime |
| ADR-0005 | Puertas y autoridad | proposed | Requiere ratificar criterios |
| ADR-0006 | Stack Tauri 2 + React/TS | accepted | G1 2026-10-16; TASK-0009 |
| ADR-0007 | SQLite FTS5 sqlite-vec | accepted | G1 2026-10-16; TASK-0010; OPEN-010 abierta |
| ADR-0008 | Router ModelAdapter / Ollama | accepted | G1 2026-10-16; TASK-0011; spike Ollama not-run (RSK-12) |
| ADR-0009 | Ingesta aislada | accepted | G1 2026-10-16; TASK-0012 |
| ADR-0010 | Motor ArtifactSpec | accepted | G1 2026-10-16; TASK-0013 |
| ADR-0011 | Amenazas y egress | accepted | G1 2026-10-16; TASK-0014 |

**Decisiones de producto:** ADR-0006–0011 `accepted` tras G1 humano (2026-10-16). OPEN-005..012 permanecen abiertas; no implican rechazo de los ADR.

**Integración TASK-0021 (2026-10-16):** security APPROVE con residuales documentados — tratamiento `api_key`, validación nested forbidden keys, y R-PROD-105 al cablear IPC desktop; no bloquean merge de contratos/fakes.

**Integración TASK-0024 (2026-10-17):** reviewer r2 + security r2 APPROVE con residuales — staging restringe paths bajo root acordado; sidecar sin red en pruebas actuales; spawn/capabilities desktop y política egress (R-PROD-105) al integrar con Tauri; no bloquean merge del skeleton.

**Integración TASK-0023 (2026-10-17):** reviewer + security APPROVE con residuales — `selected_source_ids` vacío interpreta todas las fuentes **ready** del notebook (contrato núcleo futuro); pruebas cubren no-fuga entre notebooks; R-PROD-105/007 al cablear chat e IPC; no bloquean merge del crate retrieval.

**Integración TASK-0025 (2026-10-17):** planner — anexo OLA 4 en WBS, fichas TASK-0026..0028, reserva de archivos tramo 1; OPEN-014 resuelta; sin código Rust de producto; `validate_docs.py` OK.

**Integración TASK-0026 (2026-10-17):** reviewer r2 + security r2 APPROVE con residuales — `secure_io` canonicaliza bajo `staging_root` materializado (r2 `52d1b37`); offsets de chunks incluyen separadores de párrafo (`0e55eb0`); lote IPC no aborta por ítem fallido (R-PROD-002); R-PROD-105 al cablear spawn desktop; no bloquean merge del adaptador `native-text`.

**Integración TASK-0028 (2026-10-17):** reviewer r2 + security r2 APPROVE con residuales — `ipc-client` degrada a mock local sin `fetch` externo hasta comandos Tauri (TASK-0027); lectura de archivos import vía File API del usuario; validar límites de tamaño y sanitización al cablear invoke real (R-PROD-105); no bloquean merge PR #7 @ `e9c1d0a` (HEAD rama `11dea11`).

**Integración TASK-0027 (2026-10-17):** reviewer r2 + security r2 APPROVE con residuales — estado de jobs en memoria (`job_memory`) hasta migración/tabla `jobs`; sidecar spawn y paths bajo staging acordados; R-PROD-105 egress/capabilities al ampliar superficie IPC; no bloquean merge PR #8 @ `16acd90` + follow-up `8cc33d3`.
