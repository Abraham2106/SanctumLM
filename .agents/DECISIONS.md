# DECISIONS — Índice consolidado

El contenido normativo y completo de cada decisión está en [ADR](../docs/03-architecture/adr/README.md). Este archivo solo facilita estado del planner.

| ID | Tema | Estado | Nota |
| --- | --- | --- | --- |
| ADR-0001 | Docs-as-code | accepted | Flujo de desarrollo |
| ADR-0002 | Worktree por task | accepted | Flujo de desarrollo |
| ADR-0003 | Planner coordinador | accepted | Flujo de desarrollo |
| ADR-0004 | Canal durable/efímero | proposed | Requiere decidir runtime |
| ADR-0005 | Puertas y autoridad | proposed | Requiere ratificar criterios |
| ADR-0006 | Stack Tauri 2 + React/TS | proposed | TASK-0009 merge `eb08f64`; reviewer r2+security approved; G1 pendiente |
| ADR-0008 | Router ModelAdapter / Ollama | proposed | TASK-0011 merge `568b367`; spike Ollama not-run (RSK-12); G1 pendiente |
| ADR-0009 | Ingesta aislada | proposed | TASK-0012 merge `1ae9c32`; reviewer r2+security approved; G1 pendiente |
| ADR-0010 | Motor ArtifactSpec | proposed | TASK-0013 merge `cf7cbd6`; reviewer+security approved; G1 pendiente |
| ADR-0011 | Amenazas y egress | proposed | TASK-0014 merge `b4581bd`; reviewer r2 contenido OK; G1 pendiente |

**Decisiones de producto:** ADR-0006, 0008–0011 proposed en `integration/mvp` (no accepted). Falta ADR-0007 (storage, en revisión). No convertir propuesta en aceptación sin G1 y campo Aprobación.
