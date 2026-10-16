# WBS — Work Breakdown Structure de SanctumLM

**Versión:** 0.1. **Naturaleza:** orientada a entregables, sin estimaciones inventadas. **Estado:** propuesta hasta aprobación de G0. El trabajo sobre funcionalidades reales se refina tras descubrimiento.

## 1.0 Definición y gobierno

- **WBS-1.1 Charter y stakeholders:** problema, usuarios, objetivos, restricciones, exclusiones, aprobación G0. **Entrega:** CHARTER aprobado.
- **WBS-1.2 Ingeniería de requisitos:** SRS del producto, casos de uso, NFR, fuentes, criterios y trazabilidad. **Entrega:** SRS versionado.
- **WBS-1.3 Backlog y desglose:** épicas, dependencias, responsables, fichas de tarea y estimaciones informadas. **Entrega:** WBS y backlog operativo.
- **WBS-1.4 Gobierno y puertas:** facultades humanas, política de decisiones, aprobaciones y control de cambios. **Entrega:** gates documentados.

## 2.0 Plataforma de colaboración agéntica

- **WBS-2.1 Planner/orquestación:** cola y asignaciones, conflictos, estados, checkpoints y priorización. **Entrega:** runbook y registro canónico.
- **WBS-2.2 ADR/RFC:** decisiones y alternativas auditables, ciclo de aprobación. **Entrega:** registro y plantillas.
- **WBS-2.3 Worktrees/ramas:** aislamiento, naming, sincronización, integraciones y limpieza. **Entrega:** procedimiento repetible.
- **WBS-2.4 Mensajería/handoffs:** eventos, bloqueos, contratos y reporte final. **Entrega:** comunicación comprobable.
- **WBS-2.5 Reglas automatizadas:** validación documental/CI básica y protección por revisión. **Entrega:** validadores y política.

## 3.0 Descubrimiento y arquitectura de producto — dependiente de G0

- **WBS-3.1 Modelado funcional:** módulos, casos de uso y límites acordados. **Entrega:** vistas funcionales verificadas.
- **WBS-3.2 Diseño de solución:** alternativas, ADRs, datos, interfaces, seguridad y despliegue. **Entrega:** arquitectura aprobada en G1.
- **WBS-3.3 Modelo de amenazas:** activos, fronteras, permisos, dependencias y tratamiento. **Entrega:** controles trazados.
- **WBS-3.4 Preparación técnica:** estructura de código, entornos, estilo, herramientas y build **solo tras ADR de stack**.

## 4.0 Construcción y calidad — dependiente de G1

- **WBS-4.1 Vertical slice:** capacidad mínima end-to-end aprobada con pruebas. **Entrega:** demostración G2.
- **WBS-4.2 Pruebas por historia:** unitarias, integración, contrato y negativas; evidencia. **Entrega:** TC ejecutados.
- **WBS-4.3 Auditoría de trazabilidad:** requisitos, ADRs, tareas, pruebas y PRs. **Entrega:** matriz coherente.
- **WBS-4.4 Revisión cruzada y robustez:** fallos, observabilidad, desempeño medido y correcciones. **Entrega:** reporte G3.
- **WBS-4.5 Seguridad y aprobaciones:** controles de secretos, acciones externas y aceptación humana. **Entrega:** checklist firmado.

## 5.0 Entrega y evolución — dependiente de G3

- **WBS-5.1 Empaquetado y versión:** instrucciones, artefactos, licencias y notas de release.
- **WBS-5.2 Operación y reversión:** observabilidad, respaldo si aplica, incidentes, rollback.
- **WBS-5.3 Aceptación y transición:** demostración a usuarios, aprobación G4, deuda y mantenimiento.

## Diccionario y reglas de desglose

Cada paquete se transforma en tareas `TASK-####` con *resultado observable*, límite de archivos, esfuerzo estimado por el agente responsable, dependencias, requisito `R-*`, pruebas `TC-*`, ADR relevante y criterio de Done. Ningún paquete significa una funcionalidad concreta hasta G0. Evitar asignar simultáneamente dos tareas sobre el mismo contrato o archivo de coordinación. El planner actualiza el porcentaje solo con evidencias, no con apreciaciones del modelo.
