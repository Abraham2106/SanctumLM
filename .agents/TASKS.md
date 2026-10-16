# TASKS — Snapshot inicial del backlog

**Última actualización:** 2026-10-16. **Consolidador:** planner (rol). No representa tareas ya implementadas.

| ID | Título | WBS | Estado | Owner | Dependencia / criterio |
| --- | --- | --- | --- | --- | --- |
| TASK-0001 | Validar misión, usuarios y MVP | WBS-1.1 | done | Humano + planner | G0 aprobado 2026-10-16 (TASK-0007) |
| TASK-0002 | Completar SRS funcional | WBS-1.2 | ready | Planner | Línea base R-PROD-001..013 creada; detallar TC-PROD en G1 |
| TASK-0003 | Aprobar arquitectura de producto | WBS-3.2 | ready | Architect + humano | G0 aprobado; resolver OPEN-005..008 / G1 |
| TASK-0004 | Habilitar flujo CI y reglas de revisión | WBS-2.5 | draft | Planner | PR #1 fusionado; el workflow `docs-governance` existe. Falta verificar protección de rama y revisión obligatoria |
| TASK-0005 | Primer vertical slice | WBS-4.1 | blocked | Por asignar | G1 y tarea detallada |
| TASK-0006 | Coherencia documental: registro OPEN, glosario, índice, trazabilidad y validador | WBS-2.2, WBS-2.5 | in_progress | Implementer (IA) a petición del humano | Revisión independiente y merge por el humano |
| TASK-0007 | Definición de producto y cierre de G0 | WBS-1.1, WBS-1.2 | in_progress | Implementer (IA) a petición del humano | Apilada sobre TASK-0006; revisión y merge por el humano |

Los IDs de esta tabla son de planificación **no asignada a worktree aún**. No crear ramas automáticamente. Para pasar a `ready` se debe completar ficha según plantilla, abrir issue y registrar aprobación relevante.
