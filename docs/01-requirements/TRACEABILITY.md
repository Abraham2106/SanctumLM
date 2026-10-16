# Matriz de trazabilidad — baseline 0.1

La trazabilidad no es un checklist decorativo. Un requisito aprobado debe tener trabajo responsable, prueba/inspección y evidencia enlazada. `TC-*` se define en [plan de pruebas](../04-quality/TEST_PLAN.md). Los PRs se incorporarán cuando existan.

| Requisito(s) | Paquete WBS | Verificación inicial | Estado |
| --- | --- | --- | --- |
| R-ENG-001, R-ENG-002 | WBS-1.3, WBS-2.1 | TC-ENG-01 | Propuesto |
| R-ENG-003, R-NFR-003 | WBS-2.3 | TC-ENG-02 | Propuesto |
| R-ENG-004 | WBS-2.2 | TC-ENG-03 | Propuesto |
| R-ENG-005, R-ENG-009 | WBS-1.2, WBS-4.3 | TC-ENG-04 | Propuesto |
| R-ENG-006, R-ENG-011, R-ENG-012 | WBS-2.4, WBS-4.4 | TC-ENG-05 | Propuesto |
| R-ENG-007, R-NFR-006 | WBS-4.1, WBS-4.2 | TC-ENG-06 | Propuesto |
| R-ENG-008, R-NFR-005 | WBS-1.4, WBS-4.5 | TC-ENG-07 | Propuesto |
| R-ENG-010 | WBS-2.1, WBS-2.3 | TC-ENG-08 | Propuesto |
| R-NFR-001, R-NFR-007 | WBS-3.3, WBS-4.5 | TC-SEC-01 | Propuesto |
| R-NFR-002, R-NFR-004 | WBS-2.5, WBS-4.3 | TC-ENG-09 | Propuesto |
| R-PROD-* | Pendiente de G0 | Pendiente de G0 | No definido |

Cada PR incorpora un bloque `Traceability` con IDs de requisito, WBS, TC, ADR y evidencia. El planner revisa enlaces rotos, requisitos sin pruebas, pruebas sin dueño y elementos implementados sin requisito. Actualizar tabla en el mismo PR que modifica cualquiera de sus enlaces.
