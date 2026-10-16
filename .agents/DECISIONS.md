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
