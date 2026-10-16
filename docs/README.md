# Índice y control documental

Estado inicial: `propuesto`. Fecha de línea base: 2026-10-15. Responsable de coherencia: agente `planner`; responsable último de alcance: humano patrocinador.

| Área | Fuente canónica | Propósito |
| --- | --- | --- |
| Gobierno | [CHARTER](00-project/CHARTER.md) | Problema, límites, aprobación |
| Requisitos | [SRS](01-requirements/SRS.md) | Requisitos y aceptación |
| Requisitos | [USE_CASES](01-requirements/USE_CASES.md) | Flujos y escenarios |
| Requisitos | [TRACEABILITY](01-requirements/TRACEABILITY.md) | Requisito → trabajo → prueba |
| Planificación | [WBS](02-planning/WBS.md) | Paquetes de trabajo |
| Planificación | [ROADMAP](02-planning/ROADMAP.md) | Fases, dependencias, puertas |
| Planificación | [RISK_REGISTER](02-planning/RISK_REGISTER.md) | Riesgos y respuestas |
| Planificación | [DEFINITION_OF_DONE](02-planning/DEFINITION_OF_DONE.md) | Calidad de entrega |
| Arquitectura | [ARCHITECTURE](03-architecture/ARCHITECTURE.md) | Criterios y vistas |
| Arquitectura | [ADR](03-architecture/adr/README.md) | Decisiones versionadas |
| Calidad | [TEST_PLAN](04-quality/TEST_PLAN.md) | Estrategia de pruebas |
| Calidad | [SECURITY](04-quality/SECURITY.md) | Amenazas, permisos, datos |
| Calidad | [OPERATIONS](04-quality/OPERATIONS.md) | Observabilidad, incidentes |
| Calidad | [RELEASE](04-quality/RELEASE.md) | Entrega y reversión |
| Agentes | [OPERATING_MODEL](05-agents/OPERATING_MODEL.md) | Agentes y responsabilidades |
| Agentes | [WORKTREE_PROTOCOL](05-agents/WORKTREE_PROTOCOL.md) | Aislamiento Git |
| Agentes | [PLANNER_RUNBOOK](05-agents/PLANNER_RUNBOOK.md) | Rutina del coordinador |
| Agentes | [COMMUNICATION](05-agents/COMMUNICATION.md) | Contratos de mensajes |
| Coordinación | [Registro](../.agents/README.md) | Estado y decisiones |

## Jerarquía de fuentes

1. Aprobaciones humanas, límites de seguridad y políticas del repositorio.
2. `CHARTER` aprobado y `SRS` vigente.
3. ADRs `accepted` dentro de su ámbito.
4. WBS, plan de pruebas y tareas autorizadas.
5. Notas de agentes, PRs y propuestas (no son autorización autónoma).

Un conflicto se eleva al `planner`, que documenta alternativas. No se resuelve por orden de llegada, reputación del modelo ni mayoría de agentes. Los documentos cambian en PR con revisión, historial Git, estado, fecha y vínculo de tarea.

Los ejemplos y plantillas no son especificaciones aprobadas. `TBD` y `OPEN-*` son deuda de definición visible, no información confirmada.
