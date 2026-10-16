# ADR-0006 — Stack de escritorio Tauri 2 y layout del repositorio (§12)

- **Estado:** proposed
- **Fecha:** 2026-10-16
- **Alcance:** producto — capa de escritorio y contrato de layout futuro
- **Tarea:** TASK-0009
- **Vínculos:** R-PROD-013, R-PROD-101, R-PROD-107, R-PROD-108; WBS-3.2, WBS-3.4
- **Aprobación:** pendiente

## Contexto

SanctumLM es una aplicación **local-first** de notebook con IA: debe operar offline con modelos locales (R-PROD-013), sin llamadas externas ni telemetría por defecto (R-PROD-101), con instaladores multiplataforma documentados (R-PROD-107, tramo v1) y una UI accesible por teclado y contraste (R-PROD-108). El SRS reserva el stack y las interfaces del producto para decisión en G1 mediante ADR.

La investigación [`INVESTIGACION_NOTEBOOK_IA_LOCAL_HIBRIDO.md`](../../00-project/research/INVESTIGACION_NOTEBOOK_IA_LOCAL_HIBRIDO.md) compara Tauri 2 y Electron (§3.2), define procesos e IPC (§3.3), propone una estructura de repositorio y contratos (§12) y enumera decisiones abiertas incluido el orden de plataformas (§16). El diagrama de §3.1 sitúa la UI en Tauri/React/TypeScript, un núcleo Rust, SQLite y adaptadores a motores en localhost.

Fuerzas en tensión: (a) núcleo privilegiado auditable y defensible frente a datos no confiables (R-PROD-111); (b) equipo con fuerte componente web en la UI; (c) empaquetado y WebView heterogéneos por SO (RSK-14, OPEN-009 sin cerrar); (d) no materializar código ni dependencias hasta aprobación humana de G1.

## Decisión

1. **Stack de escritorio ratificado:** **Tauri 2** como contenedor de aplicación, **React + TypeScript** en el renderer (`apps/desktop/src/`) y **Rust** en el núcleo de comandos, políticas y acceso a datos (`apps/desktop/src-tauri/` y crates de dominio). Esta elección alinea con la recomendación de §3.2 para una app local-first con núcleo de seguridad en Rust.

2. **Vigencia:** la decisión entra en vigor para implementación **tras aceptación de este ADR y puerta G1**. Hasta entonces no se crean directorios `apps/` ni `crates/` ni se añaden entradas a manifiestos de build.

3. **Layout §12 como contrato de OLA 2:** la estructura listada en la investigación §12 se adopta como **objetivo normativo de layout** para la **OLA 2** (preparación técnica WBS-3.4, posterior a G1). **No describe el estado actual del repositorio**; es el mapa acordado hacia el que migrará el código cuando una tarea autorizada materialice el esqueleto.

4. **Plataformas:** R-PROD-107 exige Windows, macOS y Linux en v1. **OPEN-009 permanece abierta.** Se **recomienda** desarrollar y empaquetar **Windows primero** para reducir variabilidad de WebView y acelerar el vertical slice (§16, §14.3), sin renunciar al objetivo multiplataforma de v1.

### Invariantes de arquitectura (obligatorios en implementación)

Estos invariantes derivan de §3.1, §3.3, §9, CHARTER y R-PROD-101/111; toda implementación del stack debe respetarlos:

| Invariante | Regla |
| --- | --- |
| Egress desde UI | El renderer **no** ejecuta `fetch()` ni equivalentes hacia red externa ni hacia proveedores de IA; toda salida pasa por comandos IPC al núcleo Rust. |
| IPC | Comandos **tipados**, versionados de forma compatible; errores tipados en respuestas (§12). |
| Capabilities Tauri | Permisos **explícitos** por ventana/capability; sin acceso libre del renderer a archivos, red, secretos o shell (§3.1). |
| Motores locales | Adaptadores HTTP a Ollama/llama.cpp/LM Studio escuchan **solo en `127.0.0.1`**; **nunca** en `0.0.0.0` ni exposición LAN por defecto (§3.3, R-PROD-111). |
| Fallback remoto | **`cloud_fallback=false`** por defecto y **aplicado en el backend**; un fallo del modelo local **no** dispara envío automático a API remota (investigación §6.2, CHARTER). |
| Colas y sidecars | Trabajos largos en cola asíncrona con progreso y cancelación; conversor de documentos como sidecar restringido (§3.3). |

### Contrato de layout futuro (investigación §12)

Referencia verbatim de la estructura objetivo; **no existe aún en el repo**:

```text
apps/desktop/              # Tauri 2 y frontend React
apps/desktop/src/          # UI: notebooks, sources, chat, studio, settings
apps/desktop/src-tauri/    # Núcleo Rust, comandos y capabilities
crates/core-domain/        # Entidades, clasificación, policies, errores
crates/storage/            # SQLite, cifrado, migraciones, archivos
crates/retrieval/          # FTS, vectores, ranking, citas
crates/ai-router/          # Adaptadores y autorización de inferencia
crates/connectors/         # Web, OAuth y proveedores de archivos
workers/documents/         # Ingesta aislada (Docling/MarkItDown)
packages/artifacts/        # Esquemas JSON, infografías SVG, PPTX
packages/contracts/        # Esquemas JSON y tipos compartidos
security/                  # Threat model, tests SSRF y egress
benchmarks/                # Corpus, evaluaciones y hardware
```

Los comandos IPC no aceptan rutas de sistema arbitrarias sin verificación y confinamiento; los adaptadores soportan timeout, cancelación y límites de contexto (§12). Decisiones no cubiertas aquí (cifrado obligatorio OPEN-010, conector piloto OPEN-011, modelos de referencia, infografías OPEN-012) **requieren ADR o cierre de OPEN-* aparte** — no forman parte de este ADR.

## Alternativas evaluadas

| Opción | Criterios principales | Resultado |
| --- | --- | --- |
| **Tauri 2 + React/TS + Rust** (propuesta) | Núcleo Rust para políticas y datos; WebView del SO → menor peso relativo; capabilities e IPC alineados con §3.1; curva Rust asumida en el plan (§3.2, fase 1 §13). | **Seleccionada** |
| **Electron + React/TS + Node** | Ecosistema JS maduro; Chromium embebido → distribución más uniforme y mayor peso; seguridad depende de contextIsolation, sandbox, CSP y preload mínimo mantenidos manualmente (§3.2 [S11, S12]). | Viable si el equipo priorizara Node sobre Rust; descartada para SanctumLM por preferencia explícita de núcleo seguro en Rust y offline-first. |
| **No decidir stack aún** | Evita compromiso prematuro. | Rechazada: G1 y WBS-3.4 bloquean la preparación técnica y el vertical slice sin una línea base; la investigación ya documenta el trade-off (§3.2). |

## Consecuencias

**Beneficios:** coherencia con el diagrama de §3.1; un solo lenguaje de sistemas (Rust) para egress, aislamiento de notebook y persistencia; UI web estándar para accesibilidad (R-PROD-108); alineación con crates/workers de §12 para separar parsers no confiables.

**Costos:** curva de aprendizaje Rust/Tauri; pruebas en tres WebViews (OPEN-009); sidecars y colas añaden complejidad operativa en desarrollo.

**Riesgos:** RSK-14 (multiplataforma real); RSK-12 (modelos en hardware modesto) independiente del framework; comandos Rust mal validados equivalen a escalada de privilegios (§3.1 [S11]).

**Migración / reversibilidad:** mientras no exista código de producto, revertir es documental. Tras materializar OLA 2, migrar a Electron implicaría reescribir el núcleo privilegiado y revisar todo el modelo de seguridad — coste alto; por eso la aceptación debe ocurrir antes del esqueleto.

**Pruebas que falsan la decisión:** captura de tráfico con red deshabilitada (KPI-1, R-PROD-101); escenarios §15 (c) modelo local caído sin tráfico externo; matriz de instaladores por SO cuando OPEN-009 se cierre; auditoría de teclado/contraste en flujos React.

## Evidencia y aprobación

- **Evidencia documental:** investigación §3.2, §3.3, §12, §16; SRS R-PROD-013, R-PROD-101, R-PROD-107, R-PROD-108; CHARTER (`cloud_fallback=false`).
- **Pruebas de producto:** no aplican en esta tarea (solo decisión); suite de producto **not-run**.
- **Aprobación:** pendiente de revisión independiente, patrocinador humano y puerta **G1** (ADR-0005). No marcar `accepted` sin PR/acta.

## Sustitución

Ningún ADR previo fija el stack de producto. Complementa ADR-0001 (registro de decisiones) y condiciona WBS-3.4. Decisiones posteriores esperadas: almacenamiento/cifrado (OPEN-010), conectores (OPEN-011), amenazas detalladas (WBS-3.3).
