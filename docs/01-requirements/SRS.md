# SRS — Software Requirements Specification

**Proyecto:** SanctumLM. **Versión:** 0.1. **Estado:** borrador parcial. **Línea base:** 2026-10-15.

## 1. Objeto y límites

Este SRS contiene requisitos verificables **del proceso/entorno de desarrollo agéntico** confirmados por el encargo. **No constituye aún una especificación completa del producto SanctumLM**: no hay visión funcional, usuarios, stack ni restricciones verificadas. La sección de producto queda deliberadamente pendiente de G0, sin requisitos ficticios.

Clasificación: `R-ENG` (flujo de ingeniería), `R-NFR` (calidad/seguridad del flujo), y `R-PROD` (funciones del producto, aún sin aprobar). Prioridad `M` = obligatoria para el flujo; `P` = propuesta sujeta a confirmación. Cada requisito deberá vincular tarea y prueba.

## 2. Actores del flujo

Patrocinador humano, planner, architect, implementer, reviewer, QA y security. Un mismo modelo puede asumir roles secuencialmente, pero no revisar independientemente su propia implementación.

## 3. Requisitos funcionales del desarrollo

| ID | Requisito normativo | Prioridad | Aceptación comprobable |
| --- | --- | --- | --- |
| R-ENG-001 | Cada trabajo tiene ficha con `task-id`, objetivo, alcance, dueño y aceptación. | M | No existe PR sin ficha enlazada. |
| R-ENG-002 | El planner centraliza WBS, backlog, prioridades y dependencias. | M | Estado canónico mantenido por un solo responsable lógico. |
| R-ENG-003 | Cada agregación usa rama y `git worktree` exclusivos. | M | Dos ejecutores no comparten directorio editable. |
| R-ENG-004 | Toda decisión arquitectónica relevante queda en ADR Markdown con estado y alternativas. | M | PR afectado enlaza ADR evaluado. |
| R-ENG-005 | Todo requisito aprobado tiene ID estable, fuente y criterio observable. | M | La matriz detecta huérfanos. |
| R-ENG-006 | La comunicación deja huella durable de bloqueos, handoffs y acuerdos. | M | Un tercero puede reconstruir qué y por qué cambió. |
| R-ENG-007 | Cada entrega pasa validación automática pertinente y revisión independiente. | M | PR muestra comandos, resultados y revisión. |
| R-ENG-008 | Solo el humano aprueba G0, decisiones de alto impacto y acciones externas sensibles. | M | Existe constancia de aprobación antes del cambio. |
| R-ENG-009 | Cada cambio conserva trazabilidad requisito → WBS → prueba → PR. | M | Se puede recorrer la cadena sin adivinar. |
| R-ENG-010 | El planner detecta conflictos y asigna propiedad temporal de archivos. | M | No se fusionan ediciones concurrentes incompatibles. |
| R-ENG-011 | Al cerrar la tarea se registra handoff, revisión, integración y limpieza. | M | Estado final y evidencia disponibles. |
| R-ENG-012 | El proceso admite pausa, cancelación, recuperación y reasignación. | P | Tarea suspendida deja checkpoint retomable. |

## 4. Requisitos no funcionales del flujo

| ID | Requisito | Prioridad | Método |
| --- | --- | --- | --- |
| R-NFR-001 | No se exponen secretos ni datos sensibles en Git, logs o mensajes. | M | Revisión de diff y secretos, evidencias sanitizadas. |
| R-NFR-002 | El proceso es reproducible con Git y herramientas documentadas. | M | Un tercero recrea worktree y validación desde instrucciones. |
| R-NFR-003 | El fracaso de un agente no corrompe `main` ni otro worktree. | M | Prueba de aislamiento/rollback de tarea fallida. |
| R-NFR-004 | Las decisiones y artefactos tienen historial auditable. | M | Git y PR enlazan autor, motivación y revisión. |
| R-NFR-005 | Ninguna acción irreversible externa se ejecuta sin autorización. | M | Simulación de puerta bloquea operación no autorizada. |
| R-NFR-006 | Las pruebas informan resultado real, no estimado ni supuesto. | M | Reporte incluye comando, entorno y evidencia. |
| R-NFR-007 | El flujo limita autonomía y permisos por tarea. | M | Alcance y permisos constan en ficha de delegación. |

## 5. Requisitos del producto — abiertos

`R-PROD-*` **no puede enumerarse aún con validez**. El planner debe entrevistar/confirmar `OPEN-001..004` de CHARTER, identificar journeys, casos normales/errores, interfaces externas, datos, SLAs/SLOs, accesibilidad, privacidad, plataforma y criterios de aceptación. Para cada requisito nuevo crear:
- ID permanente `R-PROD-###`, enunciado «El sistema deberá...», usuario/origen y justificación.
- Precondiciones, entradas/salidas, prioridad, límites, errores, criterio medible y prueba `TC-PROD-###`.
- Dependencias y exclusiones explícitas; decisión de aprobación y versión de línea base.

No convertir propuestas del agente en requisitos «aprobados».

## 6. Interfaz y datos

`OPEN-005` sistemas externos/MCP/APIs; `OPEN-006` datos, clasificación y retención; `OPEN-007` experiencia de usuario, accesibilidad e idioma; `OPEN-008` plataforma, rendimiento y disponibilidad. Todas sujetas a G0/G1. Su estado y responsable se siguen en el [registro OPEN](../00-project/OPEN_QUESTIONS.md).

## 7. Validación, control y aceptación

Cambios al SRS requieren propuesta `RFC` o issue, análisis de impacto en WBS/ADR/pruebas, revisión del planner, aprobación del PO para requisitos de producto y PR. No se borran IDs aprobados; se marcan `deprecated` o `superseded`. La línea base de producto empieza en `v1.0` solo al completar G0.
