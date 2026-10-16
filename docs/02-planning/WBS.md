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

## Anexo — OLA 4 (cableado hacia vertical slice)

**Alcance:** plan de implementación post-OLA 3 (TASK-0022..0024 integradas; TASK-0023 al cierre de merge); **no** declara gate **G2** ni sustituye la ficha paraguas **TASK-0005** (`WBS-4.1`). Desglosa el **primer tramo** del vertical slice acordado en G0/G1: cableado **importación → chunks → FTS → UI/búsqueda**, dejando chat/RAG+citas y demostración G2 para tramo posterior bajo la misma `WBS-4.1` (sin nuevos requisitos `R-PROD`).

**Prerrequisito:** OLA 3 cerrada (0022, 0024 integrados; 0023 mergeado o en integración).

| Sub-paquete | Entregable | Requisitos (diseño) | ADR | Verificación objetivo | Tarea |
| --- | --- | --- | --- | --- | --- |
| **WBS-4.1.1** Extractores sidecar ligeros | Adaptadores **TXT/MD** (y pegado vía mismo pipeline) en `workers/documents`; límites de tamaño; **sin** Docling/MarkItDown ni red | R-PROD-002 (parcial), R-PROD-111 | ADR-0009 | Tests contrato por MIME; lote mixto con archivo inválido no aborta el resto (preparación TC-PROD-002) | **TASK-0026** |
| **WBS-4.1.2** Nucleo import → persistencia | Comandos Tauri + servicios: staging, spawn sidecar, persistir fuente/chunks/jobs vía API pública `storage`; SHA-256 y estados; **sin** UI React | R-PROD-002 (parcial), R-PROD-014 (parcial), R-PROD-106 (parcial), R-PROD-103 | ADR-0007, ADR-0009 | Tests integración Rust (DB temp); job cancel/retry idempotente a nivel núcleo (preparación TC-PROD-014) | **TASK-0027** |
| **WBS-4.1.3** Contratos IPC + UI tramo import/búsqueda | Tipos en `packages/contracts`; panel fuentes + acción importar + búsqueda FTS en UI (español); invoca comandos de 0027; **sin** chat notebook ni ModelAdapter real | R-PROD-001 (parcial), R-PROD-002 (parcial), R-PROD-105 (parcial vía UI+API), R-PROD-108 (hipótesis OPEN-007) | ADR-0006, ADR-0007 | `tsc --noEmit`; smoke manual import+search; TC-PROD-001/002/105 **not-run** E2E hasta tramo chat | **TASK-0028** |

### Orden y dependencias (OLA 4 — tramo 1)

```text
OLA 3 (0022, 0023, 0024)
    ├── TASK-0026 (WBS-4.1.1)  [paralelo a 0023 si superficies distintas]
    ├── TASK-0027 (WBS-4.1.2)  [depende 0022; sidecar 0024; opcional 0026 para TXT/MD real]
    └── TASK-0028 (WBS-4.1.3)  [depende contratos acordados; cableado real tras 0027]
```

- **Paralelizable ya:** **0026** (worker) vs **0023** (retrieval) vs **0028** (solo contratos/types + UI mock) mientras **0027** no toque los mismos archivos.
- **0027 antes de E2E UI:** la UI de 0028 asume comandos `import_*`, `job_*`, `notebook_*` mínimos implementados en núcleo.
- **Chat / RAG / citas / TC-PROD-006–008:** fuera de este tramo; siguen bajo **TASK-0005** (`WBS-4.1`) — ver **OPEN-014** (planificación, no requisito nuevo).
- **Cuestiones abiertas:** `OPEN-010` (sin SQLCipher); `OPEN-008`/`RSK-12` (Ollama not-run); `OPEN-005` (fakes en tests).

**Fuera de OLA 4 tramo 1:** PDF/DOCX/HTML parsers pesados, conectores nube, artefactos, egress remoto, CI desktop habilitado, declaración G2 — cubierto en tramo 2 o `WBS-4.2+`.

### OLA 4 — tramo 2 (chat / RAG léxico / Ollama loopback)

**Alcance:** plan post-tramo 1 (`integration/mvp` @ cierre TASK-0026..0028); **no** declara gate **G2** ni sustituye **TASK-0005**. Desglosa el **segundo tramo** de `WBS-4.1`: cableado **recuperación FTS → prompt → inferencia local → citas validadas → UI chat** (notebook y general), sin nuevos requisitos `R-PROD`. Embeddings vectoriales reales y reranking quedan fuera (stub `VectorIndex` de OLA 3); RAG del tramo usa **FTS** según ADR-0007/0008.

**Prerrequisito:** OLA 4 tramo 1 cerrado (TASK-0026..0028 integrados en `integration/mvp`).

| Sub-paquete | Entregable | Requisitos (diseño) | ADR | Verificación objetivo | Tarea |
| --- | --- | --- | --- | --- | --- |
| **WBS-4.1.4** ModelRouter + adaptador Ollama | `ModelAdapter` + `ModelRouter` en `crates/ai-router`: loopback `127.0.0.1:11434`, `cloud_fallback=false`, errores tipados; fake HTTP en tests; **sin** remoto ni UI | R-PROD-008 (parcial), R-PROD-101, R-PROD-102 (parcial) | ADR-0008, ADR-0011 | Tests unitarios: local caído → `LocalUnavailable`, cero egress remoto; contrato chat/embed contra fake | **TASK-0030** |
| **WBS-4.1.5** RAG léxico + validación citas | Orquestación en `crates/retrieval`: consulta FTS acotada (`R-PROD-105`), ensamblado de contexto, parser de propuestas de cita, validación BD (`R-PROD-104`), estados backed/inference/abstención; **sin** llamadas LLM ni Tauri | R-PROD-006 (parcial), R-PROD-104, R-PROD-105, R-PROD-007 (aislamiento en API) | ADR-0007, ADR-0009 | Tests crate: escenario (e) aislamiento; cita con `chunk_id` inválido descartada; imposibles → abstención sin contexto | **TASK-0031** |
| **WBS-4.1.6** Núcleo Tauri chat + persistencia | Comandos `chat_send`, `list_models`, `chat_list`/`chat_create` mínimos; persistir mensajes/citas en SQLite vía `storage`; notebook vs general (`notebook_id` nulo); integra retrieval + ai-router; **sin** React | R-PROD-006 (parcial), R-PROD-007, R-PROD-008 (parcial), R-PROD-104 (parcial) | ADR-0006, ADR-0007, ADR-0008 | Tests integración Rust (DB temp + fake adapter); chat general no invoca FTS; TC-PROD-006–008 preparación (Ollama real **not-run** en CI) | **TASK-0032** |
| **WBS-4.1.7** Contratos IPC + UI chat | Tipos chat/citas/modelos en `packages/contracts`; panel chat notebook + entrada chat general; selector modelo vía IPC; español; **sin** HTTP a Ollama desde renderer | R-PROD-006 (parcial), R-PROD-007 (parcial), R-PROD-108 (hipótesis OPEN-007) | ADR-0006, ADR-0008 | `tsc --noEmit`; smoke manual con Ollama local opcional; TC-PROD-006–008 **not-run** E2E formal hasta gate G2 humano | **TASK-0033** |

### Orden y dependencias (OLA 4 — tramo 2)

```text
OLA 4 tramo 1 (0026..0028)
    ├── TASK-0030 (WBS-4.1.4)  [paralelo a 0031, 0033 contratos]
    ├── TASK-0031 (WBS-4.1.5)  [paralelo a 0030; depende API FTS 0023]
    ├── TASK-0033 (WBS-4.1.7)  [contratos/UI mock ∥ 0030/0031]
    └── TASK-0032 (WBS-4.1.6)  [depende 0030 + 0031; E2E UI tras merge 0032+0033]
```

- **Paralelizable:** **0030** (`ai-router`) vs **0031** (`retrieval`) vs **0033** (tipos + UI con mock) mientras **0032** no edite las mismas rutas.
- **0032 antes de demo E2E chat:** la UI de 0033 asume comandos `chat_*` y `list_models` implementados en núcleo.
- **0033 define nombres IPC** acordados con 0032 antes del merge E2E final (mismo patrón 0027/0028).
- **Demostración G2 / TC-PROD-006–008 ejecutados:** fuera de este desglose documental; requieren criterio humano y evidencia (Ollama local `RSK-12` puede permanecer **not-run** en CI).
- **Cuestiones abiertas:** `OPEN-010`; `OPEN-008`/`RSK-12`; `OPEN-005`; `OPEN-007` (layout chat).

**Fuera de OLA 4 tramo 2:** adaptador remoto real, `import_url`/`web_search`, artefactos (`R-PROD-009+`), embeddings sqlite-vec productivos, CI desktop habilitado, PDF/DOCX pesados — `WBS-4.2+` o fases v1.
