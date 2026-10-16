# ADR-0008 — Router de modelos y contrato `ModelAdapter` (Ollama primero)

- **Estado:** accepted
- **Fecha:** 2026-10-16
- **Alcance:** producto — capa de inferencia local/híbrida (`crates/ai-router` conceptual)
- **Tarea:** TASK-0011
- **Vínculos:** R-PROD-007, R-PROD-008, R-PROD-101, R-PROD-102, R-PROD-109; WBS-3.2, WBS-3.3, WBS-4.5; RSK-12; OPEN-008 (recomendación, sin cierre)
- **Aprobación:** Patrocinador humano, 2026-10-16, chat Cursor: instrucción explícita «g1 aprobado push a main».

## Contexto

SanctumLM debe ejecutar chat e embeddings **preferentemente en el equipo**, sin telemetría ni APIs externas por defecto (`R-PROD-101`), y con inferencia remota **opt-in** autorizada en backend, no en la UI (`R-PROD-102`). El motor de modelos (`R-PROD-008`) exige al menos un backend local y un adaptador remoto futuro; si el local falla, **no** debe haber tráfico automático a proveedores remotos.

La investigación (§6.1–§6.2, §11.1–§11.4, §14.1) propone:

- Ollama como primer runtime local (chat y embeddings en `127.0.0.1`).
- Una interfaz `ModelAdapter` desacoplada de marcas comerciales, con enrutamiento por **política y capacidad**, nunca por decisión del LLM.
- `cloud_fallback=false` como predeterminado, **aplicado en el núcleo** (Rust/Tauri), no solo ocultando opciones en el frontend.
- El chat general (`R-PROD-007`) es independiente del notebook: **no** recupera fragmentos ni embeddings de documentos.
- La UI no debe llamar directamente a Ollama ni a APIs de terceros; todo pasa por comandos IPC al backend.
- Metas de rendimiento (`R-PROD-109`) y calidad en español dependen de benchmark en hardware real (`RSK-12`); sin mediciones, no se fijan KPI numéricos.

`OPEN-008` (modelo de referencia y perfiles mínimos oficiales) sigue abierta; este ADR propone candidatos y deja la ratificación humana para G1.

## Decisión

### 1. Componentes y frontera de confianza

| Componente | Responsabilidad | Ubicación |
| --- | --- | --- |
| **ModelRouter** | Resolver modelo/capacidad según política de workspace, tarea y configuración; invocar el adaptador correcto; aplicar egress antes de cualquier destino de red. | Núcleo (`crates/ai-router`) |
| **ModelAdapter** | Traducir operaciones del dominio a un backend concreto; exponer capacidades; respetar timeout y cancelación. | Implementaciones registradas en el núcleo |
| **EgressPolicy** | Autorización determinista de salida (local, web pública, remoto); pseudocódigo alineado con investigación §11.4. | Núcleo, **antes** del adaptador remoto |
| **UI (renderer)** | Mostrar modelos disponibles, destino efectivo y vista previa de envío remoto; **no** realizar `fetch` a proveedores. | `apps/desktop` vía IPC |

### 2. Configuración predeterminada (backend)

```text
inference.local_host = "127.0.0.1"
inference.local_port = 11434          # Ollama por convención documentada [S18]
inference.cloud_fallback = false      # obligatorio en núcleo; no sobreescribible por prompt del LLM
inference.default_mode = "local_only" # coherente con modo «Local privado» (§6.2)
```

Si el adaptador local no responde, el router devuelve error tipado (`LocalUnavailable` / `ModelNotFound`) y **termina**. No reintenta en un adaptador remoto salvo que una política explícita y aprobada por el usuario (registrada en sesión, no inferida) lo autorice en una solicitud concreta.

Ollama debe asumirse escuchando solo en loopback. **Prohibido** configurar el producto para enlazar o asumir Ollama en `0.0.0.0` (amenaza §9.1).

### 3. Contrato `ModelAdapter` (conceptual)

Cada adaptador implementa el mismo contrato lógico (tipos concretos en `packages/contracts` tras ADR de stack; aquí se fija semántica):

**Identidad y descubrimiento**

- `adapter_id`: cadena estable interna (p. ej. `ollama-loopback`), no ID comercial del proveedor.
- `list_models()`: modelos instalados/disponibles en ese backend.
- `describe_capabilities(model_id)`: objeto con flags y límites:

| Campo | Tipo | Uso |
| --- | --- | --- |
| `chat` | bool | Completaciones conversacionales |
| `embedding` | bool | Vectores para RAG |
| `json_schema` | bool | Salida estructurada (fase posterior) |
| `vision` | bool | Multimodal (fase posterior) |
| `image_generation` | bool | Ilustraciones (fase posterior) |
| `context_window` | int \| null | Tokens máximos declarados por el backend |
| `offline_available` | bool | true si no requiere red para inferir |
| `estimated_cost` | money \| null | Solo remoto, cuando el proveedor lo permita |

**Operaciones**

- `chat(request) → ChatResponse`  
  - Entrada: `model_id`, mensajes `{ role, content }`, opciones (`temperature`, `max_tokens`, …), `deadline`/`timeout`, `AbortSignal` equivalente en Rust (`CancellationToken`).  
  - Salida: texto del asistente, metadatos (`tokens`, `finish_reason`, `adapter_id`, `model_id`).

- `embed(request) → EmbedResponse`  
  - Entrada: `model_id`, lista de textos, límites de lote/tamaño, timeout, cancelación.  
  - Salida: vectores con dimensión uniforme; error si cuenta o dimensión no coincide.

**Errores tipados** (mapeo a IPC; no cadenas libres al UI):

| Código | Cuándo |
| --- | --- |
| `LocalUnavailable` | Backend local caído o puerto cerrado |
| `ModelNotFound` | `model_id` no instalado |
| `CapabilityUnsupported` | p. ej. embeddings en modelo solo chat |
| `Timeout` | Superado `timeout` |
| `Cancelled` | Cancelación solicitada |
| `PolicyDenied` | EgressPolicy bloqueó destino o datos |
| `RemoteUnauthorized` | Remoto sin aprobación de usuario/política |
| `InvalidResponse` | JSON/contrato del backend incoherente |
| `RateLimited` | Límite del proveedor (remoto) |

**Requisitos transversales:** idempotencia donde el backend lo permita; no registrar texto íntegro de prompts `private` en logs; validar tamaño de entrada antes de enviar.

### 4. Adaptador Ollama (primera implementación)

- **Base URL:** `http://127.0.0.1:11434` (sin HTTPS local en MVP; tráfico no sale de loopback).
- **Chat:** `POST /api/chat` con `stream: false` en el vertical slice inicial; streaming como extensión posterior con la misma política de cancelación.
- **Embeddings:** `POST /api/embed` con validación de conteo y dimensión (patrón §11.2).
- **Descubrimiento:** `GET /api/tags` (o equivalente documentado) para alimentar selector de modelos en UI vía IPC.
- **Autenticación:** ninguna clave en MVP local; si Ollama exige token en el futuro, leer de keychain, nunca del renderer.

Referencia de forma, no código de producción en frontend (§11.1): el ejemplo TypeScript de la investigación ilustra el adaptador en backend, no en el renderer.

### 5. Adaptador remoto (fase posterior, opt-in)

- Registro explícito en configuración (`allow_remote_models=true` a nivel workspace).
- Validación de endpoint al **configurar** la conexión (§6.2), no en cada chat.
- Toda invocación pasa por `EgressPolicy`: vista previa de payload, minimización a chunks aprobados, denegación de `restricted` salvo política explícita.
- **Pruebas de contrato:** usar un **fake HTTP** (servidor local de test) que simule OpenAI-compatible o el contrato acordado; **prohibido** en CI usar APIs de pago ni claves reales.

### 6. Comportamiento del router por tipo de tarea

| Tarea / canal | Recuperación documental | Modelo |
| --- | --- | --- |
| Chat general (`R-PROD-007`) | **Prohibida** — el router no recibe `notebook_id` ni `source_ids`; el pipeline RAG no se invoca. | Chat local por defecto |
| Chat en notebook | Permitida solo vía módulo retrieval autorizado; el router recibe contexto **ya filtrado** por política, no el notebook completo. | Chat local por defecto |
| Indexación / consulta RAG | Embeddings locales; mismo `model_id` en indexación y consulta [S18]. | Embedding |

El router **no** interpreta instrucciones del usuario del tipo «usa GPT-4» como autorización de egress; solo actualiza preferencia de modelo **dentro** del conjunto permitido por política.

### 7. Recomendación para `OPEN-008` (no cierra la cuestión)

Pendiente spike/benchmark en hardware del patrocinador (`RSK-12`). Propuesta de **línea base de trabajo** para el primer benchmark G1:

| Rol | Candidato (tag Ollama orientativo) | Tamaño orientativo | Perfil hardware |
| --- | --- | --- | --- |
| Chat instruct ES | Familia **Qwen2.5** o **Llama 3.2** instruct, cuantización **Q4_K_M**, orden de magnitud **3–8B** parámetros | Comprobar con `ollama show` antes de descargar; **no** bajar si &gt; 4 GB sin aprobación | Mínimo práctico investigación §14.1: **16 GB RAM** |
| Embeddings RAG | **nomic-embed-text** o **embeddinggemma** (priorizar el menor que pase calidad en español) | Objetivo &lt; ~1 GB; verificar en registro Ollama | Mismo host; reservar RAM para índice SQLite |

La ratificación oficial (modelo único de referencia, instalador pre-descargado, mínimos de GPU) queda para decisión humana + evidencia en [spike Ollama](../../04-quality/evidence/spike-ollama-local-models.md) cuando exista medición real.

## Alternativas evaluadas

| Opción | Ventajas | Desventajas | Veredicto |
| --- | --- | --- | --- |
| **A. Ollama loopback + `ModelAdapter` (propuesta)** | Alineado con investigación; embeddings y chat unificados; ecosistema GGUF | Dependencia externa; calidad ES variable (`RSK-12`) | **Elegida para MVP** |
| **B. llama.cpp embebido / sidecar sin Ollama** | Menos proceso externo | Mayor esfuerzo de empaquetado y actualización de modelos | Reserva para ADR futuro si Ollama falla en empaquetado |
| **C. LM Studio como único local** | API OpenAI-compatible [S20] | Menos control de instalación headless; segundo runtime | Adaptador adicional post-MVP, no primero |
| **D. UI llama directamente a Ollama** | Implementación rápida | Rompe `R-PROD-101/102`, bypass de egress, superficie XSS→red | **Rechazada** |
| **E. Fallback automático a nube si local cae** | Mejor disponibilidad percibida | Viola `R-PROD-008` y regla §6.2 | **Rechazada** |

## Consecuencias

**Positivas**

- Un solo punto (`ModelRouter` + `EgressPolicy`) para tests adversariales (escenario §15.c: local caído → sin tráfico externo).
- Sustituir Ollama por otro adaptador sin cambiar UI ni contratos IPC de producto.
- Trazabilidad clara de chat general vs notebook (`R-PROD-007`).

**Costes y riesgos**

- Implementación inicial del crate `ai-router` y fake remoto en tests (tareas posteriores a aprobación de este ADR).
- `RSK-12`: sin benchmark, no hay promesa de latencia/calidad en español; el spike documenta `not-run` si falta Ollama.
- Index ADR seguirá fallando hasta que el planner indexe `ADR-0008` (esperado en esta tarea).

**Pruebas previstas (post-implementación)**

- Unitarias: `cloud_fallback=false` con local caído → `LocalUnavailable`, cero llamadas al fake remoto.
- Contrato: chat + embed contra Ollama de test o fake loopback.
- Seguridad: renderer sin permisos de red hacia `:11434` (capabilities Tauri).
- Producto: chat general sin `notebook_id` → respuesta sin citas ni chunks.

## Evidencia y aprobación

| Artefacto | Enlace |
| --- | --- |
| Spike latencia/RAM Ollama (TASK-0011) | [spike-ollama-local-models.md](../../04-quality/evidence/spike-ollama-local-models.md) |
| Investigación §6.1–§6.2, §11.1–§11.4, §14.1 | [INVESTIGACION_NOTEBOOK_IA_LOCAL_HIBRIDO.md](../../00-project/research/INVESTIGACION_NOTEBOOK_IA_LOCAL_HIBRIDO.md) |

**Aprobación:** pendiente — gate G1, revisión cruzada (rol distinto al autor) y humano patrocinador.

## Sustitución

Ninguna. Complementa decisiones de stack pendientes en G1; no modifica ADR-0001–0005.
