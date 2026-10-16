# ADR — Architecture Decision Records

Un ADR explica **por qué se escogió** una solución; no sustituye al SRS (qué se necesita) ni a la WBS (qué se hará). También registra decisiones importantes del proceso de ingeniería.

Formato: `ADR-####-slug.md` con título, estado (`proposed` / `accepted` / `rejected` / `superseded`), fecha, contexto, decisión, alternativas, consecuencias, vínculos `R-*`/`WBS-*` y campo **Aprobación** con evidencia enlazada. Un ADR `accepted` sin evidencia de aprobación es un defecto. El estado de cada ADR debe coincidir en su archivo, en esta tabla y en [DECISIONS](../../../.agents/DECISIONS.md); el validador lo comprueba. Numeración secuencial, IDs inmutables, historia en Git. Se crea con [plantilla](../../templates/ADR_TEMPLATE.md).

| ADR | Tema | Estado |
| --- | --- | --- |
| [0001](ADR-0001-docs-as-code.md) | Requisitos y decisiones como Markdown | accepted |
| [0002](ADR-0002-one-task-one-worktree.md) | Aislamiento por task/worktree | accepted |
| [0003](ADR-0003-planner-source-of-truth.md) | Gobierno central del planner | accepted |
| [0004](ADR-0004-agent-runtime-boundaries.md) | Estado efímero vs. durable | proposed |
| [0005](ADR-0005-approval-gates.md) | Puertas de aprobación y controles | proposed |
| [0006](ADR-0006-desktop-stack.md) | Stack Tauri 2 + React/TS + layout §12 | accepted |
| [0007](ADR-0007-storage.md) | SQLite FTS5 sqlite-vec y cifrado | accepted |
| [0008](ADR-0008-model-router.md) | Router ModelAdapter y Ollama loopback | accepted |
| [0009](ADR-0009-ingestion.md) | Ingesta aislada y citas localizables | accepted |
| [0010](ADR-0010-artifact-engine.md) | Motor ArtifactSpec SVG/PPTX | accepted |
| [0011](ADR-0011-threat-model-egress.md) | Amenazas, egress y batería §15 | accepted |

Las decisiones `accepted` de 0001-0003 derivan de las instrucciones explícitas de organización del desarrollo. ADR-0006–0011 fueron aceptados en G1 (2026-10-16) con evidencia en cada archivo; OPEN-005..012 siguen abiertas como hipótesis.

**Proceso:** abrir propuesta vinculada a tarea/RFC → enumerar alternativas → revisar riesgos/impacto → obtener aprobación adecuada → marcar estado en PR → implementar → actualizar trazabilidad. No borrar ADR antiguos; si cambia la decisión crear otro y enlazar `superseded by`.
