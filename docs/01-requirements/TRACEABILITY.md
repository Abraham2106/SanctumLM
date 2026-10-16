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
| R-PROD-* | Pendiente de G0 | Pendiente de G0 | — | No definido |

Cada PR incorpora un bloque `Traceability` con IDs de requisito, WBS, TC, ADR y evidencia. El planner revisa enlaces rotos, requisitos sin pruebas, pruebas sin dueño y elementos implementados sin requisito. Actualizar tabla en el mismo PR que modifica cualquiera de sus enlaces.

`scripts/validate_docs.py` rechaza la matriz si cita un `R-*`, `WBS-*` o `TC-*` que no esté definido en su documento fuente, o si un requisito del SRS no aparece aquí.
