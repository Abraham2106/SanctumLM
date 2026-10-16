# Matriz de trazabilidad — baseline 0.1

La trazabilidad no es un checklist decorativo. Un requisito aprobado debe tener trabajo responsable, prueba/inspección y evidencia enlazada. `TC-*` se define en [plan de pruebas](../04-quality/TEST_PLAN.md). Los PRs se incorporarán cuando existan.

Los requisitos `R-ENG`/`R-NFR` rigen el proceso desde hoy, así que su paquete principal debe poder ejecutarse **antes de G0** (WBS-1.x o WBS-2.x). La última columna indica dónde se amplía ese control cuando exista producto; no es condición para cumplir el requisito del proceso.

| Requisito(s) | Paquete WBS (proceso) | Verificación inicial | Continuación en producto | Estado |
| --- | --- | --- | --- | --- |
| R-ENG-001, R-ENG-002 | WBS-1.3, WBS-2.1 | TC-ENG-01 | — | Propuesto |
| R-ENG-003, R-NFR-003 | WBS-2.3 | TC-ENG-02 | — | Propuesto |
| R-ENG-004 | WBS-2.2 | TC-ENG-03 | WBS-3.2 | Propuesto |
| R-ENG-005, R-ENG-009 | WBS-1.2, WBS-2.5 | TC-ENG-04 | WBS-4.3 | Propuesto |
| R-ENG-006, R-ENG-011, R-ENG-012 | WBS-2.1, WBS-2.4 | TC-ENG-05 | WBS-4.4 | Propuesto |
| R-ENG-007, R-NFR-006 | WBS-2.5 | TC-ENG-06 | WBS-4.1, WBS-4.2 | Propuesto |
| R-ENG-008, R-NFR-005 | WBS-1.4 | TC-ENG-07 | WBS-4.5 | Propuesto |
| R-ENG-010 | WBS-2.1, WBS-2.3 | TC-ENG-08 | — | Propuesto |
| R-NFR-001, R-NFR-007 | WBS-2.1, WBS-2.5 | TC-SEC-01 | WBS-3.3, WBS-4.5 | Propuesto |
| R-NFR-002, R-NFR-004 | WBS-2.3, WBS-2.5 | TC-ENG-09 | WBS-4.3 | Propuesto |


## Producto (línea base G0; TC asignados en G1)

Los casos `TC-PROD-*` están definidos en [TEST_PLAN](../04-quality/TEST_PLAN.md) (TASK-0015). **Estado de ejecución:** not-run hasta OLA 2+. ADR-0006–0011 siguen `proposed` hasta «G1 aprobado».

La columna «Fase» usa la numeración de la [investigación §13](../00-project/research/INVESTIGACION_NOTEBOOK_IA_LOCAL_HIBRIDO.md). ADR de diseño: 0006 stack, 0007 storage, 0008 router, 0009 ingesta, 0010 artefactos, 0011 amenazas.

| Requisito(s) | Fase | Paquete WBS | Verificación | Estado |
| --- | --- | --- | --- | --- |
| R-PROD-001, R-PROD-002, R-PROD-007, R-PROD-013, R-PROD-014 | 1 Núcleo local | WBS-3.1, WBS-3.2, WBS-4.1 | TC-PROD-001, 002, 007, 013, 014; KPI-5 | Aprobado G0 (MVP); diseño G1 proposed |
| R-PROD-006, R-PROD-103, R-PROD-104, R-PROD-105, R-PROD-106 | 2 RAG robusto | WBS-4.1, WBS-4.2 | TC-PROD-006, 103, 104, 105, 106; KPI-2, KPI-3 | Aprobado G0 (MVP); diseño G1 proposed |
| R-PROD-003, R-PROD-004, R-PROD-008, R-PROD-012, R-PROD-102, R-PROD-110 | 3 Híbrido seguro | WBS-3.3, WBS-4.5 | TC-PROD-003, 004, 008, 012, 102, 110; KPI-1 | Aprobado G0 (MVP); diseño G1 proposed |
| R-PROD-101, R-PROD-111 | 1–3 transversal | WBS-3.3, WBS-4.5 | TC-PROD-101, 111; batería §15 | Aprobado G0 (MVP); diseño G1 proposed |
| R-PROD-108, R-PROD-109 | 1–3 transversal | WBS-4.4 | TC-PROD-108, 109; KPI-4 | Aprobado G0 (MVP); diseño G1 proposed |
| R-PROD-005, R-PROD-009, R-PROD-015 | 4 Estudio y documentos | WBS-4.1, WBS-4.2 | TC-PROD-005, 009, 015 | Aprobado G0 (v1); diseño G1 proposed |
| R-PROD-010, R-PROD-011 | 5 Estudio visual | WBS-4.1, WBS-4.2 | TC-PROD-010, 011 | Aprobado G0 (v1); diseño G1 proposed |
| R-PROD-107 | 6 Endurecimiento | WBS-4.4, WBS-5.1 | TC-PROD-107 | Aprobado G0 (v1); diseño G1 proposed |

Cada PR incorpora un bloque `Traceability` con IDs de requisito, WBS, TC, ADR y evidencia. El planner revisa enlaces rotos, requisitos sin pruebas, pruebas sin dueño y elementos implementados sin requisito. Actualizar tabla en el mismo PR que modifica cualquiera de sus enlaces.

`scripts/validate_docs.py` rechaza la matriz si cita un `R-*`, `WBS-*` o `TC-*` que no esté definido en su documento fuente, o si un requisito del SRS no aparece aquí.
