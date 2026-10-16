# Datos, interfaces y contratos

## Proceso de desarrollo (definido)

**IDs:** no reciclar identificadores. Cada familia tiene un único documento que la define; los demás solo la citan.

| Familia | Formato | Define |
| --- | --- | --- |
| Requisitos | `R-ENG-###`, `R-NFR-###`, `R-PROD-###` | [SRS](../01-requirements/SRS.md) |
| Paquetes de trabajo | `WBS-n.n` | [WBS](../02-planning/WBS.md) |
| Tareas | `TASK-####` | [TASKS](../../.agents/TASKS.md) |
| Decisiones | `ADR-####` | [ADR](adr/README.md) |
| Pruebas | `TC-ENG-##`, `TC-SEC-##`, `TC-PROD-###` | [TEST_PLAN](../04-quality/TEST_PLAN.md) |
| Cuestiones abiertas | `OPEN-###` | [OPEN_QUESTIONS](../00-project/OPEN_QUESTIONS.md) |
| Riesgos | `RSK-##` | [RISK_REGISTER](../02-planning/RISK_REGISTER.md) |

**Objeto mínimo de tarea:** id, título, estado, dueño/rol, requisito, paquete WBS, criterios de aceptación, dependencias, archivos permitidos, rama, worktree, plazo o límites si procede, revisión y enlace de PR.

**Estados válidos:** `draft → ready → in_progress → in_review → done`; transiciones a `blocked` o `cancelled` requieren motivo, responsable y checkpoint. `done` solo después de merge/aprobaciones.

**Eventos de comunicación:** `ASSIGNED`, `STARTED`, `BLOCKED`, `ADR_PROPOSED`, `READY_FOR_REVIEW`, `CHANGES_REQUESTED`, `APPROVED`, `MERGED`, `CANCELLED`, con timestamp UTC, actor, tarea, enlaces y síntesis. No usar mensajes como única fuente de verdad del SRS.

**Consistencia:** planner único escritor del backlog canónico; contribuciones y propuestas via PR/issue. Una edición de ADR/SRS en paralelo debe resolverse antes de integrar.

## Producto (pendiente de definición)

`OPEN-005` identifica APIs, MCPs, formatos, versión de esquemas y compatibilidad. `OPEN-006` identifica entidades, datos personales, cifrado y retención. `OPEN-007` accesibilidad/idioma. `OPEN-008` límites operativos. Sin contratos del producto aprobados, no publicar esquema ficticio.

Para cada interfaz futura registrar origen/destino, autorización, validación de entrada, errores, versiones, rate limits, logging seguro, idempotencia y pruebas de contrato.
