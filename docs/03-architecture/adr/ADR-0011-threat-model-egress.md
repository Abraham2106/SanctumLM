# ADR-0011 — Modelo de amenazas, egress determinista y batería §15

- **Estado:** proposed
- **Fecha:** 2026-10-16
- **Alcance:** producto SanctumLM (MVP)
- **Tarea:** TASK-0014
- **Vínculos:** R-PROD-003, R-PROD-101, R-PROD-102, R-PROD-104, R-PROD-105, R-PROD-110, R-PROD-111; WBS-3.3, WBS-4.5
- **Aprobación:** pendiente

## Contexto

SanctumLM combina almacenamiento local de fuentes (SQLite), inferencia preferentemente en loopback (`127.0.0.1`), importación de URLs públicas y conectores opcionales con egress controlado. El renderer (Tauri/WebView) y el contenido importado (HTML, PDF, web) son **datos no confiables**; el LLM puede proponer acciones o citas, pero no debe decidir política de red, clasificación ni autorización ([investigación §9.1](../../00-project/research/INVESTIGACION_NOTEBOOK_IA_LOCAL_HIBRIDO.md), [R-PROD-111](../../01-requirements/SRS.md)).

Los requisitos G0 exigen: privacidad por defecto sin APIs externas ni telemetría ([R-PROD-101](../../01-requirements/SRS.md)); control remoto al 100 % en backend ([R-PROD-102](../../01-requirements/SRS.md)); citas y aislamiento validados en base de datos ([R-PROD-104](../../01-requirements/SRS.md), [R-PROD-105](../../01-requirements/SRS.md)); auditoría local sin volcar secretos ni prompts sensibles completos ([R-PROD-110](../../01-requirements/SRS.md)); importación URL con copia local y rechazo SSRF ([R-PROD-003](../../01-requirements/SRS.md)). La clasificación de datos, retención y borrado siguen en [`OPEN-006`](../../00-project/OPEN_QUESTIONS.md) (sin cerrar).

El detalle operativo del modelo de amenazas vive en [THREAT_MODEL.md](../THREAT_MODEL.md); este ADR fija decisiones de arquitectura de seguridad para G1.

## Decisión

### 1. Fronteras, activos y actores

Adoptar las fronteras de confianza de la investigación §3.1, documentadas en [THREAT_MODEL.md §1](../THREAT_MODEL.md#1-fronteras-activos-y-actores):

| Frontera | Lado de confianza | Lado no confiable / restringido |
| --- | --- | --- |
| Almacenamiento local | Núcleo Rust + SQLite (datos del usuario) | Renderer, sidecars de conversión, contenido importado |
| Inferencia loopback | Adaptadores a motores en `127.0.0.1` (política explícita) | Cualquier otro destino de red |
| Egress autorizado | Broker Rust tras política determinista | Internet, APIs remotas, buscadores, conectores en nube |
| Fuentes no confiables | Tratadas como datos; nunca como instrucciones de sistema | HTML/PDF/web con prompt injection, RAG poisoning, parsers |

**Activos críticos:** notebooks y fuentes; `chunks` y `source_versions`; embeddings; notas y chats; credenciales en keychain del SO; registro `audit_events`; copias locales de URLs importadas.

**Actores:** usuario legítimo; contenido malicioso en fuentes; proceso local no autorizado que intente usar endpoints expuestos; proveedor remoto (solo tras consentimiento); sidecar de ingestión comprometido.

### 2. Egress: broker Rust determinista (nunca el LLM)

Implementar un **egress broker** en el núcleo Rust que sea la única vía de salida de red del producto (salvo el flujo OAuth del sistema operativo para conectores, fuera del renderer). Reglas:

1. **Ningún componente de UI ni sidecar de parsing realiza `fetch` a destinos externos**; solo el núcleo, tras `authorize_egress(...)`.
2. La función de autorización es **código determinista**, testeable unitariamente, alineada al pseudocódigo de investigación §11.4. El LLM y otros modelos **solo emiten intenciones** (`task`, fragmentos candidatos, consultas públicas propuestas); el núcleo resuelve destino, clasificación, minimización y decisión `ALLOW` / `DENY` / `REQUIRE_APPROVAL`.
3. **`cloud_fallback=false` por defecto y aplicado en backend:** si un modelo local falla (timeout, caída de Ollama, error HTTP en loopback), **no** se reintenta ni se sustituye automáticamente por un proveedor remoto. La UI puede informar el fallo; cualquier cambio a remoto exige nueva solicitud evaluada por política ([investigación §6.2](../../00-project/research/INVESTIGACION_NOTEBOOK_IA_LOCAL_HIBRIDO.md)).
4. **Secretos:** tokens OAuth, API keys y material criptográfico **solo** en keychain del SO (o equivalente por plataforma). SQLite, `connectors.scopes_json`, logs y `audit_events` **no** almacenan secretos en claro ([R-PROD-111](../../01-requirements/SRS.md)).

### 3. Cuatro modos de egress (MVP)

Los modos siguen investigación §6.2 y §9.2; el broker los combina con clasificación de fuentes y política del notebook:

| Modo | Destinos típicos | Regla resumida |
| --- | --- | --- |
| **Local privado** | `127.0.0.1` (LLM, embeddings) | Sin egress a Internet; datos del notebook permitidos según clasificación local |
| **Híbrido consultivo** | Gateway web / buscador | Solo consultas o descargas **públicas** explícitas; **ningún** fragmento `private`/`restricted` en la consulta |
| **Híbrido con proveedor** | API de modelo remoto | Minimización a `chunks` aprobados; bloqueo de `restricted`; confirmación explícita no persistente con vista previa de payload |
| **Offline estricto** | Ninguno | Denegar todo tráfico de red del producto (incl. telemetría, actualizaciones, búsqueda de modelos, remotos) |

Matriz de ejemplo (investigación §9.2; valores finales sujetos a ratificación de `OPEN-006`):

| Clasificación | Local LLM | Web pública | Remoto |
| --- | --- | --- | --- |
| `public` | Permitir | Permitir con controles SSRF | Permitir con aprobación por solicitud |
| `private` | Permitir | Bloquear contenido privado en consulta | Bloquear salvo aprobación explícita de esa solicitud |
| `internal` / `restricted` (diseño) | Permitir con política notebook | Bloquear | `restricted`: denegar siempre según configuración |

### 4. Citas e aislamiento (backend como fuente de verdad)

Antes de incluir una cita en una respuesta o artefacto:

1. Validar `chunk_id` contra la base de datos con el notebook activo y la lista **autorizada** de `source_id` / `selected_source_ids` (equivalente a investigación §11.3).
2. Comprobar `source_version_id` y que el texto/offset citado coincide con el segmento almacenado.
3. **Ignorar** identificadores de chunk o fuente generados por el LLM que no superen la consulta de autorización ([R-PROD-104](../../01-requirements/SRS.md), [R-PROD-105](../../01-requirements/SRS.md)).

### 5. Importador web y SSRF ([R-PROD-003](../../01-requirements/SRS.md))

El gateway web del núcleo aplica controles de investigación §9.3 y OWASP SSRF [S25]:

- Esquemas permitidos: `https` (y excepciones explícitas documentadas en implementación, p. ej. ninguna en MVP salvo OAuth del SO).
- Tras resolución DNS y **en cada conexión/redirección** (límite de profundidad): denegar destinos loopback (`127.0.0.0/8`, `::1`), link-local, RFC1918, CGNAT `100.64.0.0/10`, metadata cloud (`169.254.169.254`, endpoints equivalentes), direcciones no global unicast sospechosas e IPs codificadas (decimal/octal/hex/IPv6 alternativas).
- Tamaño máximo, timeout, profundidad de redirecciones y tipo MIME aceptado acotados.
- Tras éxito: persistir **copia textual local** (`source_version`) para uso offline; la lectura posterior no requiere red.
- El documento remoto no puede provocar lectura `file://`, keychain ni IPC privilegiado.

### 6. Parsers aislados y motores locales

- Conversión HTML/PDF/office en **sidecar o worker aislado** sin acceso directo a red ni secretos; comunicación acotada con el núcleo.
- **No ejecutar JavaScript** de HTML importado; sanitizar y limitar recursos embebidos.
- Adaptadores de inferencia local **solo** hacia `127.0.0.1` (no `0.0.0.0` ni LAN por defecto) ([R-PROD-111](../../01-requirements/SRS.md)).

### 7. Auditoría ([R-PROD-110](../../01-requirements/SRS.md))

Registrar en `audit_events` (diseño): `action`, `destination`, `policy_decision`, `task_id`, clasificaciones agregadas (conteos por nivel), `model_id`, `source_version_id` cuando aplique, timestamp. **Prohibido:** volcar secretos, tokens, cuerpos completos de prompts privados o contenido íntegro de fuentes; usar hashes/truncados/redacción.

### 8. Batería adversarial §15 (diseño de aceptación)

La batería a–j de investigación §15 se adopta como **criterios de diseño y prueba futura** (implementación y `TC-*` fuera de este ADR). Escenario, control, evidencia esperada y trazabilidad R-PROD: ver [THREAT_MODEL.md §5](../THREAT_MODEL.md#5-batería-adversarial-investigación-15).

### 9. Propuesta para `OPEN-006` (no cierra la cuestión)

**Propuesta de security para decisión humana:**

| Tema | Propuesta MVP | Evolución |
| --- | --- | --- |
| Clasificación | Campo `sources.classification` con valores **`public` \| `private`** obligatorios en MVP; esquema y UI preparados para **`internal` \| `restricted`** sin usarlos como default | Ratificar cuatro niveles en G1 o mantener binario hasta post-MVP |
| Default | Fuentes importadas por archivo/URL usuario: `private` salvo marcado explícito `public` | Política por notebook en `settings_json` |
| Retención | Conservar `source_versions` inmutables mientras exista cita o artefacto dependiente; borrado en cascada **deliberado** documentado en ADR de datos | Plazos legales/PO pendientes |
| Borrado | Acción usuario «eliminar fuente/notebook» borra chunks, vectores, copias en disco y entradas de auditoría referenciadas, sin recuperación desde la app | Wipe seguro y backups: `OPEN-010` |
| Cifrado en reposo | Fuera del alcance de este ADR; depende de `OPEN-010` | SQLCipher o equivalente |

Esta propuesta **no** cambia el estado de [`OPEN-006`](../../00-project/OPEN_QUESTIONS.md); requiere aprobación explícita del patrocinador + security.

## Alternativas evaluadas

| Opción | Descripción | Motivo de descarte / diferimiento |
| --- | --- | --- |
| **A (propuesta)** | Broker Rust central, cuatro modos, SSRF estricto, LLM sin egress | Alineada a KPI-1, CHARTER e investigación |
| **B** | Política en TypeScript del renderer + guardas en backend | Duplicación, bypass vía WebView, incumple R-PROD-102 |
| **C** | LLM clasifica sensibilidad y autoriza envío | No determinista, vulnerable a prompt injection |
| **D** | Sin gateway URL (solo archivos locales) | Incumple R-PROD-003 y modo híbrido consultivo |
| **No hacer nada** | Confiar en capacidades Tauri por defecto | Insuficiente para SSRF, citas y egress |

## Consecuencias

**Beneficios:** superficie de red única auditable; gates binarios de privacidad y citas comprobables; alineación con batería §15 y trazabilidad WBS-3.3.

**Costos:** complejidad del broker y pruebas adversariales; latencia en validación de citas y resolución DNS; sidecars y sandbox de parsers.

**Riesgos residuales:** DNS rebinding sofisticado (mitigar revalidación por conexión); calidad de parsers (citas engañosas); dependencia de keychain por SO; conectores en nube ampliando superficie (fase posterior).

**Pruebas (diseño):** unitarias del broker; SSRF sin red real contra Internet en CI (IPs/resoluciones simuladas); captura de tráfico en escenarios (c) y (d); revisión de `audit_events` tras §15. Implementación en tareas posteriores.

**Reversibilidad:** modularizar broker detrás de trait `EgressPolicy`; modos activables por feature flags hasta G2.

## Evidencia y aprobación

- Documento de producto: [THREAT_MODEL.md](../THREAT_MODEL.md).
- Insumo: [investigación §9.1–§9.4, §11.4, §15](../../00-project/research/INVESTIGACION_NOTEBOOK_IA_LOCAL_HIBRIDO.md).
- **Aprobación:** pendiente de revisión humana en G1 y PR de TASK-0014.

## Sustitución

Ninguna. Complementa ADRs de stack y datos cuando existan; no modifica ADR-0001–0005 (proceso agéntico).
