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

## Anexo — OLA 3 (cimientos de datos e ingesta)

**Alcance:** plan de implementación post-OLA 2 (TASK-0017..0021 integradas en `integration/mvp`); **no** es cambio de producto ni gate G2. Desglosa el paquete padre **WBS-3.4** hacia persistencia local, recuperación léxica/vectorial inicial y sidecar de ingesta, según ADR-0006 (layout §12), ADR-0007, ADR-0009 y requisitos MVP ya aprobados en G0/G1.

**Prerrequisito:** OLA 2 cerrada (workspace §12, contratos B2 TASK-0021).

| Sub-paquete | Entregable | Requisitos (diseño) | ADR | Verificación objetivo | Tarea |
| --- | --- | --- | --- | --- | --- |
| **WBS-3.4.1** Persistencia SQLite | Migraciones versionadas (`schema_migrations`), `PRAGMA` WAL + FK, esquema migración **0001** (notebooks, sources, versiones, chunks, FTS5 `chunks_fts`, jobs/audit mínimo según ADR-0007); API de conexión en `crates/storage` sin **SQLCipher** hasta cierre `OPEN-010` | R-PROD-001, R-PROD-103, R-PROD-106; soporte esquema R-PROD-105 | ADR-0007 | Tests unitarios migración idempotente; triggers FTS; datos de prueba solo `public` si aplica gate OPEN-010 | **TASK-0022** |
| **WBS-3.4.2** Recuperación (FTS + vector) | Consultas FTS5 con filtros notebook/fuentes (`§11.3` ADR-0007); trait `VectorIndex` + stub Plan B; **sin** chat general accediendo a índices (R-PROD-007) | R-PROD-006 (parcial), R-PROD-105, R-PROD-007 | ADR-0007, ADR-0008 (embeddings posteriores) | TC-PROD-105 (preparación); escenario (e) léxico en tests de crate | **TASK-0023** |
| **WBS-3.4.3** Sidecar ingesta (esqueleto) | Proceso/worker aislado en `workers/documents`: contrato IPC JSON hacia núcleo, límites sin red/keychain, stubs de adaptadores; **sin** parsers pesados ni Docling en esta tarea | R-PROD-002 (parcial), R-PROD-106, R-PROD-111 | ADR-0009 | Tests de contrato sidecar; job estados recuperables | **TASK-0024** |

### Orden y dependencias (OLA 3)

```text
TASK-0021 (done)
    ├── TASK-0022 (WBS-3.4.1) ──► TASK-0023 (WBS-3.4.2)
    └── TASK-0024 (WBS-3.4.3)   [paralelo a 0022; integración funcional con storage vía contratos/núcleo en tareas posteriores]
```

- **0022 antes de 0023:** la recuperación consume esquema y conexión SQLite/FTS materializados en storage.
- **0024 en paralelo con 0022:** no modifica migraciones ni `crates/retrieval`; cableado end-to-end importación → chunks → FTS queda para OLA 4 / vertical slice (TASK-0005), no inventado aquí.
- **Cuestiones abiertas:** `OPEN-010` bloquea SQLCipher y persistencia `private`/`restricted` en builds de usuario; `OPEN-005`..`012` no bloquean estas tres tareas (hipótesis en fichas).

**Fuera de OLA 3 (referencia WBS existente):** UI producto (`apps/desktop` salvo IPC tipado ya reservado), CI desktop, RAG/chat cableado, ModelAdapter real, conectores — WBS-4.1+ / tareas futuras.
