# Datos, interfaces y contratos

## Proceso de desarrollo (definido)

**IDs:** no reciclar identificadores. Cada familia tiene un único documento que la define; los demás solo la citan.

| Familia | Formato | Define |
| --- | --- | --- |
| Requisitos | `R-ENG-###`, `R-NFR-###`, `R-PROD-###` | [SRS](../01-requirements/SRS.md) |
| Paquetes de trabajo | `WBS-n.n` | [WBS](../02-planning/WBS.md) |
| Tareas | `TASK-####` | [TASKS](../../.agents/TASKS.md) |
| Decisiones | `ADR-####` | [ADR](adr/README.md) |
| Pruebas | `TC-ENG-##`, `TC-SEC-##`, `TC-PROD-###` | [TEST_PLAN](../04-quality/TEST_PLAN.md) |
| Cuestiones abiertas | `OPEN-###` | [OPEN_QUESTIONS](../00-project/OPEN_QUESTIONS.md) |
| Riesgos | `RSK-##` | [RISK_REGISTER](../02-planning/RISK_REGISTER.md) |

**Objeto mínimo de tarea:** id, título, estado, dueño/rol, requisito, paquete WBS, criterios de aceptación, dependencias, archivos permitidos, rama, worktree, plazo o límites si procede, revisión y enlace de PR.

**Estados válidos:** `draft → ready → in_progress → in_review → done`; transiciones a `blocked` o `cancelled` requieren motivo, responsable y checkpoint. `done` solo después de merge/aprobaciones.

**Eventos de comunicación:** `ASSIGNED`, `STARTED`, `BLOCKED`, `ADR_PROPOSED`, `READY_FOR_REVIEW`, `CHANGES_REQUESTED`, `APPROVED`, `MERGED`, `CANCELLED`, con timestamp UTC, actor, tarea, enlaces y síntesis. No usar mensajes como única fuente de verdad del SRS.

**Consistencia:** planner único escritor del backlog canónico; contribuciones y propuestas via PR/issue. Una edición de ADR/SRS en paralelo debe resolverse antes de integrar.

## Producto (contratos G1 — propuesta TASK-0015)

**Estado:** propuesto para PARADA 1; coherente con [ARCHITECTURE.md](ARCHITECTURE.md) Plano B y ADR-0006–0011 (`proposed`). La implementación en `apps/` / `packages/contracts` queda post-aprobación G1.

### Hipótesis ligadas a cuestiones abiertas (no las cierran)

| ID | Hipótesis de diseño |
| --- | --- |
| OPEN-005 | Tests de contrato usan **fakes locales** (HTTP loopback, buscador stub, conector simulado) en lugar de proveedores reales en CI/MVP. |
| OPEN-006 | Clasificación MVP **`public` \| `private`**; esquema reserva **`internal` \| `restricted`**. Cifrado según [ADR-0007](adr/ADR-0007-storage.md) **sujeto a** [`OPEN-010`](../00-project/OPEN_QUESTIONS.md). |
| OPEN-007 | UI **fuentes \| chat \| estudio** en español; WCAG teclado/contraste en flujos MVP (`R-PROD-108`); segundo idioma no exigido en MVP. |
| OPEN-008 | Primer `ModelAdapter`: **Ollama** en `127.0.0.1:11434`; hardware mínimo práctico **16 GB RAM**; benchmark de modelos **not-run** (`RSK-12`). |

### IPC Tauri (renderer ↔ núcleo)

- **Forma:** comandos invocables desde el renderer con payloads JSON validados en Rust; respuestas `{ ok: true, data }` o `{ ok: false, error: ProductError }`.
- **Versionado:** campo `api_version` en solicitudes de operaciones que evolucionen; rechazar versiones desconocidas con `UnsupportedApiVersion`.
- **Autorización:** cada comando comprueba sesión local (usuario único por instalación) y, cuando aplique, `notebook_id` + permisos de política del notebook.
- **Prohibido en payloads:** rutas absolutas arbitrarias del SO, URLs de red privada/loopback para importación sin pasar por gateway, secretos, banderas `skip_security_checks`.

Comandos conceptuales MVP (nombres orientativos; tipos finales en `packages/contracts`):

| Comando | Entrada resumida | Salida | Reqs. |
| --- | --- | --- | --- |
| `notebook_*` | CRUD, búsqueda, duplicar | IDs y metadatos | R-PROD-001 |
| `import_sources` | `notebook_id`, lote de archivos/pegado | `job_id`, errores por ítem | R-PROD-002, 014 |
| `import_url` | URL pública tras vista previa | `source_id` o error SSRF | R-PROD-003 |
| `web_search` | consulta pública | resultados seleccionables | R-PROD-004 |
| `chat_send` | `mode`, `notebook_id?`, mensajes, `model_id`, selección fuentes | mensaje + citas validadas | R-PROD-006, 007 |
| `list_models` | — | modelos locales/remotos permitidos | R-PROD-008 |
| `generate_artifact` | `ArtifactSpec` parcial | `job_id`, spec validado | R-PROD-009–011 |
| `export_notebook` / `import_notebook` | paquete versionado | manifest + advertencias | R-PROD-015 |
| `job_status` / `job_cancel` | `job_id` | progreso, estado terminal | R-PROD-014, 106 |

### ModelAdapter (núcleo ↔ motores)

Contrato lógico [ADR-0008](adr/ADR-0008-model-router.md); el renderer **nunca** implementa este contrato.

| Operación | Entrada | Salida / errores |
| --- | --- | --- |
| `list_models()` | — | Lista `{ model_id, adapter_id, capabilities }` |
| `describe_capabilities(model_id)` | — | Flags `chat`, `embedding`, `context_window`, `offline_available`, … |
| `chat(request)` | mensajes, `model_id`, timeout, cancelación | texto asistente + metadatos |
| `embed(request)` | textos[], `model_id`, límites de lote | vectores dimensión uniforme |

**Errores tipados (mapeo a IPC):** `LocalUnavailable`, `ModelNotFound`, `CapabilityUnsupported`, `Timeout`, `Cancelled`, `PolicyDenied`, `RemoteUnauthorized`, `InvalidResponse`, `RateLimited`.

**Primera implementación:** adaptador `ollama-loopback` → `http://127.0.0.1:11434` (`/api/chat`, `/api/embed`, `/api/tags`). Remoto: registro explícito + fake HTTP en tests (`OPEN-005`).

### Jobs (colas de importación y generación)

| Campo | Tipo | Uso |
| --- | --- | --- |
| `job_id` | UUID | Identificador estable |
| `type` | enum | `import_batch`, `extract`, `embed_index`, `generate_artifact`, … |
| `state` | enum | `queued`, `running`, `completed`, `failed`, `cancelled` |
| `progress` | 0–100 o etapas | UI de progreso |
| `error_code` | enum tipado | p. ej. `EXTRACTION_CORRUPT`, `POLICY_DENIED` |
| `dedupe_key` | hash SHA-256 + operación | Reintento sin duplicar fuentes confirmadas |

Cancelación: señal cooperativa al worker/sidecar; persistir estado terminal antes de liberar recursos. Interrupción de proceso: al reiniciar, jobs `running` → `failed` o reanudación según idempotencia del tipo.

### ArtifactSpec (generación estructurada)

Fuente de verdad JSON en `artifacts.spec_json` [ADR-0010](adr/ADR-0010-artifact-engine.md).

- **Tipos MVP/v1:** `infographic`, `slides`, `report`.
- **Campos comunes:** `schema_version`, `type`, `title`, `language`, `notebook_id`, `source_scope[]`, `evidence_mode`, `sections[]`, citas a `chunk_id` verificadas, `limitations_and_doubts` cuando proceda.
- **Prohibiciones (rechazo en validación):** `skip_security_checks`, `bypass_policy`, credenciales, tokens, HTML/JS crudo de fuentes importadas en el spec.
- **Secretos:** la autorización de modelos remotos vive en política/sesión del núcleo, **no** en el spec.

Ejemplo mínimo (infografía, sin credenciales):

```json
{
  "schema_version": 1,
  "type": "infographic",
  "title": "Tema",
  "language": "es",
  "evidence_mode": "source_backed",
  "source_scope": ["src-1"],
  "sections": [{ "id": "s1", "heading": "Paso 1", "body": "…" }]
}
```

### Evidencia, citas y estados de verificación

| Concepto | Regla |
| --- | --- |
| Cita en mensaje/artefacto | `chunk_id` + `source_version_id` + `quote` validados contra BD (patrón investigación §11.3) |
| Estados de afirmación | `backed`, `inference`, `discrepancy`, `unverified`, abstención explícita |
| LLM | Puede proponer citas; el núcleo **descarta** IDs no autorizados (`R-PROD-104`) |
| Selección de contexto | Lista `selected_source_ids` fijada por el núcleo, no confiar en listas del modelo (`R-PROD-105`) |

### Errores de producto (`ProductError`)

Estructura común en respuestas IPC fallidas:

```json
{
  "code": "POLICY_DENIED",
  "message": "Texto localizable para UI",
  "details": { "field": "optional" }
}
```

Familias: validación de entrada (`VALIDATION_*`), política/egress (`POLICY_*`, `SSRF_*`), infraestructura (`LOCAL_UNAVAILABLE`, `SIDECAR_CRASHED`), dominio (`NOT_FOUND`, `CONFLICT`, `JOB_CANCELLED`). Logs: registrar `code` y metadatos; **no** volcar contenido `private` ni secretos.

### Sidecar de ingesta (núcleo ↔ worker)

- **Entrada:** ruta en staging, `mime`, límites de tamaño, `adapter_hint`.
- **Salida:** texto normalizado, `chunks[]` con `page`, `char_start`, `char_end`, `warnings[]`, `extraction_metadata`.
- **Errores:** códigos `EXTRACTION_*` por archivo; lote mixto continúa (R-PROD-002).
- **Red:** ninguna en el sidecar durante extracción.

Para cada interfaz futura adicional registrar origen/destino, autorización, validación de entrada, versiones, rate limits, logging seguro, idempotencia y prueba `TC-PROD-*` asociada.
