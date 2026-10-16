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
| [0008](ADR-0008-model-router.md) | Router ModelAdapter y Ollama loopback | proposed |
| [0010](ADR-0010-artifact-engine.md) | Motor ArtifactSpec SVG/PPTX | proposed |

Las decisiones `accepted` de 0001-0003 derivan de las instrucciones explícitas de organización del desarrollo. No implican aceptación de arquitectura funcional de SanctumLM. Las propuestas de producto (p. ej. ADR-0008) solo son candidatas hasta G1 humano.

**Proceso:** abrir propuesta vinculada a tarea/RFC → enumerar alternativas → revisar riesgos/impacto → obtener aprobación adecuada → marcar estado en PR → implementar → actualizar trazabilidad. No borrar ADR antiguos; si cambia la decisión crear otro y enlazar `superseded by`.
