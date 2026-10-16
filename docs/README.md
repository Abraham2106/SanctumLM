# Índice y control documental

Estado inicial: `propuesto`. Fecha de línea base: 2026-10-15. Responsable de coherencia: agente `planner`; responsable último de alcance: humano patrocinador.

## Por dónde empezar

| Si eres… | Lee primero | Después |
| --- | --- | --- |
| Patrocinador / PO | [CHARTER](00-project/CHARTER.md), [OPEN_QUESTIONS](00-project/OPEN_QUESTIONS.md) | [ROADMAP](02-planning/ROADMAP.md), [ACCEPTANCE_CHECKLIST](04-quality/ACCEPTANCE_CHECKLIST.md) |
| Planner | [AGENTS.md](../AGENTS.md), [PLANNER_RUNBOOK](05-agents/PLANNER_RUNBOOK.md) | [STATUS](../.agents/STATUS.md), [TASKS](../.agents/TASKS.md), [TRACEABILITY](01-requirements/TRACEABILITY.md) |
| Implementer | [AGENTS.md](../AGENTS.md), [WORKTREE_PROTOCOL](05-agents/WORKTREE_PROTOCOL.md) | Ficha de su tarea, [DEFINITION_OF_DONE](02-planning/DEFINITION_OF_DONE.md) |
| Reviewer / QA / Security | [roles](05-agents/roles/REVIEWER_QA.md), [TEST_PLAN](04-quality/TEST_PLAN.md) | [SECURITY](04-quality/SECURITY.md), [EVALUATION](04-quality/EVALUATION.md) |

Los términos propios (gate, handoff, `OPEN-*`, `not-run`…) están en el [glosario](00-project/GLOSSARY.md).

## Mapa de documentos

| Área | Fuente canónica | Propósito |
| --- | --- | --- |
| Gobierno | [CHARTER](00-project/CHARTER.md) | Problema, límites, aprobación |
| Gobierno | [Investigación de producto](00-project/research/INVESTIGACION_NOTEBOOK_IA_LOCAL_HIBRIDO.md) | Fuente de G0: referente, requisitos RF, arquitectura recomendada y plan por fases |
| Gobierno | [OPEN_QUESTIONS](00-project/OPEN_QUESTIONS.md) | Cuestiones sin respuesta que bloquean trabajo |
| Gobierno | [GLOSSARY](00-project/GLOSSARY.md) | Vocabulario común |
| Requisitos | [SRS](01-requirements/SRS.md) | Requisitos y aceptación |
| Requisitos | [USE_CASES](01-requirements/USE_CASES.md) | Flujos y escenarios |
| Requisitos | [TRACEABILITY](01-requirements/TRACEABILITY.md) | Requisito → trabajo → prueba |
| Planificación | [WBS](02-planning/WBS.md) | Paquetes de trabajo |
| Planificación | [ROADMAP](02-planning/ROADMAP.md) | Fases, dependencias, puertas |
| Planificación | [RISK_REGISTER](02-planning/RISK_REGISTER.md) | Riesgos y respuestas |
| Planificación | [DEFINITION_OF_DONE](02-planning/DEFINITION_OF_DONE.md) | Calidad de entrega |
| Arquitectura | [ARCHITECTURE](03-architecture/ARCHITECTURE.md) | Criterios y vistas |
| Arquitectura | [DATA_AND_INTERFACES](03-architecture/DATA_AND_INTERFACES.md) | IDs, estados, eventos y contratos |
| Arquitectura | [ADR](03-architecture/adr/README.md) | Decisiones versionadas |
| Calidad | [TEST_PLAN](04-quality/TEST_PLAN.md) | Estrategia de pruebas |
| Calidad | [ACCEPTANCE_CHECKLIST](04-quality/ACCEPTANCE_CHECKLIST.md) | Evidencia y estado por gate |
| Calidad | [EVALUATION](04-quality/EVALUATION.md) | Evaluación de agentes y del proceso |
| Calidad | [SECURITY](04-quality/SECURITY.md) | Amenazas, permisos, datos |
| Calidad | [OPERATIONS](04-quality/OPERATIONS.md) | Observabilidad, incidentes |
| Calidad | [RELEASE](04-quality/RELEASE.md) | Entrega y reversión |
| Agentes | [OPERATING_MODEL](05-agents/OPERATING_MODEL.md) | Agentes y responsabilidades |
| Agentes | [WORKTREE_PROTOCOL](05-agents/WORKTREE_PROTOCOL.md) | Aislamiento Git |
| Agentes | [PLANNER_RUNBOOK](05-agents/PLANNER_RUNBOOK.md) | Rutina del coordinador |
| Agentes | [COMMUNICATION](05-agents/COMMUNICATION.md) | Contratos de mensajes |
| Agentes | Roles: [planner](05-agents/roles/PLANNER.md), [architect](05-agents/roles/ARCHITECT.md), [implementer](05-agents/roles/IMPLEMENTER.md), [reviewer/QA](05-agents/roles/REVIEWER_QA.md), [security](05-agents/roles/SECURITY.md) | Límites de cada rol |
| Plantillas | [tarea](templates/TASK_BRIEF_TEMPLATE.md), [handoff](templates/AGENT_REPORT_TEMPLATE.md), [ADR](templates/ADR_TEMPLATE.md), [RFC](templates/RFC_TEMPLATE.md), [caso de prueba](templates/TEST_CASE_TEMPLATE.md), [incidente](templates/INCIDENT_TEMPLATE.md) | Formatos reutilizables |
| Coordinación | [Registro](../.agents/README.md) | Estado y decisiones |

## Jerarquía de fuentes

1. Aprobaciones humanas, límites de seguridad y políticas del repositorio.
2. `CHARTER` aprobado y `SRS` vigente.
3. ADRs `accepted` dentro de su ámbito.
4. WBS, plan de pruebas y tareas autorizadas.
5. Notas de agentes, PRs y propuestas (no son autorización autónoma).

Un conflicto se eleva al `planner`, que documenta alternativas. No se resuelve por orden de llegada, reputación del modelo ni mayoría de agentes. Los documentos cambian en PR con revisión, historial Git, estado, fecha y vínculo de tarea.

Los ejemplos y plantillas no son especificaciones aprobadas. `TBD` y `OPEN-*` son deuda de definición visible, no información confirmada.

## Validación

`scripts/validate_docs.py` comprueba enlaces locales, archivos obligatorios y la coherencia de IDs entre documentos (detalle en [TEST_PLAN](04-quality/TEST_PLAN.md#capas-de-verificación)). Se ejecuta en CI en cada PR y debe pasar localmente antes de abrirlo.
