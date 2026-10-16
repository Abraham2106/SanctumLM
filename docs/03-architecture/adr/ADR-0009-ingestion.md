# ADR-0009 — Ingesta aislada, parsers por tipo y citas localizables

- **Estado:** proposed
- **Fecha:** 2026-10-16
- **Alcance:** producto — módulo de ingesta de fuentes (extracción y segmentación; no recuperación ni chat)
- **Tarea:** TASK-0012
- **Vínculos:** R-PROD-002, R-PROD-103, R-PROD-104, R-PROD-106, R-PROD-111; RSK-14; WBS-3.2, WBS-3.3
- **Aprobación:** pendiente

## Contexto

SanctumLM debe importar PDF, TXT, MD, DOCX, HTML y texto pegado, conservando el original intacto, registrando hash SHA-256, procedencia y versiones inmutables, y produciendo segmentos (`chunks`) con ubicación verificable (página y offsets de carácter) para citas auditables (R-PROD-002, R-PROD-103, R-PROD-104). La investigación §5.1 describe un pipeline de siete pasos desde validación MIME hasta marcar la fuente como lista; §9.1 exige tratar fuentes como datos no confiables (prompt injection, RAG poisoning, HTML/PDF malicioso), parsers aislados y prohibición de ejecutar JavaScript en HTML importado. RSK-14 advierte que un parser deficiente puede producir citas engañosas aunque el modelo responda bien.

Hoy no existe integración de Docling, MarkItDown ni un worker de ingesta en el repositorio. El núcleo de la aplicación (Tauri/Rust) no debe cargar dependencias pesadas de OCR ni ejecutar lógica de conversión no confiable en el mismo proceso que gestiona permisos y secretos. La recuperación híbrida (FTS, vectores, reranking) y la validación de citas en respuestas pertenecen a otros módulos; este ADR delimita únicamente **ingesta → extracción → chunking → persistencia del original y metadatos → indexación léxica/vectorial inicial** como contrato hacia esos módulos.

Fuerzas en tensión: calidad estructural (tablas, encabezados, páginas) frente a peso de dependencias y superficie de ataque; resiliencia por archivo en lotes mixtos (R-PROD-002); continuidad ante interrupciones (R-PROD-106); y necesidad de marcar incertidumbre en PDF escaneados (escenario §15 (h)).

## Decisión propuesta

Implementar un **servicio de ingesta aislado** (sidecar/proceso hijo dedicado, sin acceso a red ni al keychain del SO) invocado por el núcleo mediante IPC tipado. El sidecar recibe rutas a archivos ya validados en una zona de staging, ejecuta la extracción según una **tabla de enrutamiento por tipo**, devuelve texto normalizado, metadatos de ubicación, estado de extracción y advertencias; el núcleo persiste el original, crea `source_versions` inmutables y escribe `chunks` e índices. **Docling no está integrado** en el producto en la fecha de este ADR; se reserva como adaptador opcional futuro dentro del sidecar, no como dependencia del núcleo.

### Aislamiento y límites de confianza

| Control | Obligación |
| --- | --- |
| Proceso | Sidecar separado del núcleo y del renderer; reinicio aislado ante fallo del parser |
| Red | Sin sockets salientes en el sidecar durante extracción (modo offline estricto para parsers) |
| Secretos | Sin lectura del keychain ni variables de entorno con credenciales |
| Entrada | Solo archivos en staging con ruta acordada; tamaño máximo y firma MIME ya validados por el núcleo |
| Salida | JSON/esquema acordado hacia el núcleo; sin ejecutar código del documento importado |
| HTML | Sanitizar (eliminar `<script>`, handlers `on*`, `javascript:` URLs); **nunca** interpretar ni ejecutar JS del HTML importado (R-PROD-111) |
| Sustituibilidad | Adaptadores de parser intercambiables por tipo sin cambiar contratos de `chunks` |

### Pipeline (alcance de ingesta)

Secuencia lógica por ítem de lote; la recuperación y el chat consumen después el resultado persistido:

1. **Identificar tipo** — MIME real, extensión y magic bytes; rechazar tipos fuera del catálogo MVP con código de error específico.
2. **Registrar original** — Calcular SHA-256 del bytes originales; almacenar copia inmutable en almacenamiento local versionado; crear o actualizar `sources` y una nueva fila `source_versions` (ver reimportación).
3. **Extraer** — Invocar el adaptador elegido en el sidecar; conservar estructura útil (página, orden de bloques, tablas como texto/markdown estructurado cuando el adaptador lo permita).
4. **Normalizar y segmentar** — Normalizar espacios sin destruir números, fórmulas ni citas literales; dividir en `chunks` con `page`, `char_start`, `char_end` relativos al texto extraído de **esa** `source_version_id`, y `ordinal` estable dentro de la versión.
5. **Original inmutable** — El blob en `storage_path` de cada versión no se reescribe; transformaciones quedan en metadatos de job y en chunks nuevos ligados a la nueva versión.
6. **Indexar** — En la misma transacción de commit de chunks: actualizar FTS5 y, si la política lo permite, encolar embeddings locales (modelo y dimensión registrados). Fallos de embedding no borran chunks ya confirmados (R-PROD-106).
7. **Estado** — Marcar `sources.status` / job (`ready`, `failed`, `partial`) y exponer errores por archivo al usuario.

El orquestador RAG, la fusión híbrida de candidatos y la generación de respuestas **no** forman parte de este pipeline; solo dependen de que existan `chunks` autorizados y localizables.

### Enrutamiento de extracción por tipo (adaptadores)

| Tipo / señal | Adaptador propuesto | Rol |
| --- | --- | --- |
| TXT, MD, texto pegado | **Nativo** (lectura UTF-8/ normalización) | Mínima superficie; offsets triviales |
| HTML | **Nativo + sanitización** | DOM/texto seguro; sin motor JS embebido |
| DOCX | **MarkItDown** (ligero) o nativo OOXML limitado | Conversión a markdown/texto; tablas simplificadas |
| PDF (capa de texto, layout moderado) | **MarkItDown** como línea base MVP | Menor peso que Docling |
| PDF (tablas complejas, layout denso) | **Docling** (adaptador opcional, sidecar) | Mejor estructura; requiere sandbox y dependencias OCR separadas |
| PDF escaneado / sin capa de texto | **OCR local opcional** (dentro del sidecar, p. ej. vía Docling u otro motor aprobado) | Siempre con banderas de incertidumbre (§15 (h)) |
| Imágenes sueltas (si se admiten) | OCR opcional + metadatos | Fuera del MVP salvo decisión explícita |

**Regla:** un solo adaptador primario por job; si falla con error recuperable, se puede registrar fallback a MarkItDown/nativo **solo** si la política de calidad lo permite y queda auditado en `extraction_metadata` (nunca silencioso para PDFs que requerían estructura).

### Metadatos de extracción e incertidumbre (PDF escaneado)

Cada `source_version` incluye un registro de transformación (R-PROD-103) con, como mínimo:

- `adapter_id`, `adapter_version`, duración, código de salida del sidecar
- `ocr_used` (boolean), `ocr_confidence` agregado y/o por página
- `uncertain_pages` o rangos marcados cuando la confianza OCR está por debajo del umbral configurable
- `warnings[]` legibles para UI («Revisar página N: posible omisión de cifra por OCR»), alineado con §15 (h)

Los chunks en páginas inciertas pueden llevar flag `location_uncertain=true` para que la UI y el módulo de citas muestren limitación explícita.

### Lotes, errores y resiliencia

- **Archivo corrupto o ilegible:** error específico (`EXTRACTION_CORRUPT`, `EXTRACTION_UNSUPPORTED`, etc.); el job de ese ítem termina en `failed`; **el resto del lote continúa** (R-PROD-002).
- **Interrupción:** estados de job persistidos; reintento idempotente usa el mismo SHA-256 y no duplica `source_versions` si ya se confirmó el commit de una versión completa (R-PROD-106).
- **Sidecar caído:** el núcleo reporta fallo de infraestructura sin marcar chunks parciales como `ready`.

### Reimportación y versiones inmutables

- Cada importación exitosa que produce nuevo contenido indexable crea una **`source_version` nueva** (`revision` monótono por `source_id`).
- El SHA-256 del original se almacena por versión; un cambio de bytes implica nueva versión aunque la URI lógica sea la misma.
- **`chunks` y citas históricas** referencian `source_version_id` fijo; no se reasignan al reimportar. Las citas antiguas siguen apuntando a la revisión anterior; las nuevas consultas usan la versión activa por defecto configurable (escenario §15 (f)).
- No se actualizan in-place textos de chunks de versiones anteriores.

### Invariante de contrato: validación de `chunk_id` (módulo backend futuro)

El LLM **no** es fuente de verdad para identificadores de evidencia. Invariante obligatorio en el backend de chat/RAG:

1. Toda cita o paquete de evidencia que el modelo proponga incluye uno o más `chunk_id` candidatos.
2. El backend **revalida** cada `chunk_id` contra la base de datos: existe, pertenece a un `source_version_id` autorizado para el notebook y conjunto de fuentes seleccionados en la consulta, y el texto citado (o subcadena en offsets) coincide con tolerancia definida.
3. IDs inexistentes, fuera de alcance o con desajuste de texto se **descartan o marcan** `verification_status=failed`; no se presentan al usuario como citas verificadas (R-PROD-104).
4. Este ADR no implementa el orquestador; fija el contrato para que ingesta garantice chunks estables y localizables que hagan posible dicha validación.

## Alternativas evaluadas

| Alternativa | Ventaja | Desventaja | Veredicto |
| --- | --- | --- | --- |
| **Sidecar + enrutamiento híbrido (propuesta)** | Aislamiento, parsers reemplazables, alineado con §3.1 investigación | Operacionalizar IPC, empaquetar sidecar por SO | **Propuesta** |
| **Docling único para todo** | Máxima calidad en PDF/tables | Dependencias pesadas, OCR, mayor RSK-14 si no hay sandbox | Rechazada como único camino MVP |
| **MarkItDown único** | Ligero, fácil de empaquetar | Pérdida de estructura en PDF difíciles; citas menos precisas (RSK-14) | Aceptada como **baseline**, no exclusiva |
| **Extracción nativa únicamente** | Superficie mínima | Insuficiente para DOCX/PDF reales del corpus de evaluación | Solo para tipos triviales |
| **Parsing en el núcleo Rust (sin sidecar)** | Menos procesos | Mezcla datos no confiables con privilegios; difícil aislar Python/OCR | Rechazada (R-PROD-111) |
| **No hacer ingesta estructurada** | Cero coste | Incumple R-PROD-002/103/104 y KPI de citas | Rechazada |

## Consecuencias

**Beneficios:** frontera clara entre datos no confiables y núcleo; citas ancladas a versión y offsets; parsers sustituibles sin reescribir esquema; alineación con batería §15 (a), (f), (h).

**Costos:** empaquetado y actualización del sidecar; pruebas por adaptador y por SO; latencia IPC y arranque del proceso; necesidad de conjunto de evaluación con PDF escaneado y HTML malicioso sintético.

**Riesgos residuales:** RSK-14 persiste hasta medir `citation_precision` en corpus real; Docling/OCR ampliará superficie cuando se habilite — exige revisión de seguridad y fijación de versiones antes de `accepted` funcional.

**Pruebas previstas (implementación futura):** lote mixto con un corrupto (R-PROD-002); reimportación con citas históricas (§15 (f)); PDF escaneado sintético con página de baja confianza (§15 (h)); HTML con `<script>` (contenido ignorado, sin ejecución); interrupción mid-job (R-PROD-106); validación backend de `chunk_id` inválido (R-PROD-104).

**Reversibilidad:** cambiar adaptador por tipo no exige migrar `source_versions` pasadas; nuevas extracciones usan el adaptador nuevo en revisiones posteriores.

## Evidencia y aprobación

- Investigación: §5.1 (pipeline), §9.1 (amenazas), §15 (h) (OCR e incertidumbre), §4.2 (modelo `chunks` / `source_versions`).
- Requisitos: SRS R-PROD-002, R-PROD-103, R-PROD-104, R-PROD-106, R-PROD-111.
- Riesgo: RSK-14 en `docs/02-planning/RISK_REGISTER.md`.
- Implementación de sidecar, Docling y workers: **fuera de alcance** de TASK-0012; pendiente de tareas de implementación tras aprobación G1/G2.

**Aprobación humana:** pendiente (gate de arquitectura producto).

## Sustitución

Ninguna decisión previa superseded. Complementa la arquitectura de componentes (servicio de ingestión aislado en investigación §3.1). Decisiones de recuperación híbrida, embeddings y política de modelos quedarán en ADR distintos.
