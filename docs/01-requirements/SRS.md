# SRS — Software Requirements Specification

**Proyecto:** SanctumLM. **Versión:** 1.0. **Estado:** línea base de producto aprobada en G0. **Línea base:** 2026-10-16.

## 1. Objeto y límites

Este SRS contiene los requisitos del **proceso de desarrollo agéntico** (secciones 3–4) y los **requisitos iniciales del producto** aprobados en G0 (sección 5), derivados del [CHARTER](../00-project/CHARTER.md). El stack, las interfaces y el modelo de datos del producto se deciden en G1 mediante ADR.

Clasificación: `R-ENG` (flujo de ingeniería), `R-NFR` (calidad/seguridad del flujo), y `R-PROD` (funciones del producto, aún sin aprobar). Prioridad `M` = obligatoria para el flujo; `P` = propuesta sujeta a confirmación. Cada requisito deberá vincular tarea y prueba.

## 2. Actores del flujo

Patrocinador humano, planner, architect, implementer, reviewer, QA y security. Un mismo modelo puede asumir roles secuencialmente, pero no revisar independientemente su propia implementación.

## 3. Requisitos funcionales del desarrollo

| ID | Requisito normativo | Prioridad | Aceptación comprobable |
| --- | --- | --- | --- |
| R-ENG-001 | Cada trabajo tiene ficha con `task-id`, objetivo, alcance, dueño y aceptación. | M | No existe PR sin ficha enlazada. |
| R-ENG-002 | El planner centraliza WBS, backlog, prioridades y dependencias. | M | Estado canónico mantenido por un solo responsable lógico. |
| R-ENG-003 | Cada agregación usa rama y `git worktree` exclusivos. | M | Dos ejecutores no comparten directorio editable. |
| R-ENG-004 | Toda decisión arquitectónica relevante queda en ADR Markdown con estado y alternativas. | M | PR afectado enlaza ADR evaluado. |
| R-ENG-005 | Todo requisito aprobado tiene ID estable, fuente y criterio observable. | M | La matriz detecta huérfanos. |
| R-ENG-006 | La comunicación deja huella durable de bloqueos, handoffs y acuerdos. | M | Un tercero puede reconstruir qué y por qué cambió. |
| R-ENG-007 | Cada entrega pasa validación automática pertinente y revisión independiente. | M | PR muestra comandos, resultados y revisión. |
| R-ENG-008 | Solo el humano aprueba G0, decisiones de alto impacto y acciones externas sensibles. | M | Existe constancia de aprobación antes del cambio. |
| R-ENG-009 | Cada cambio conserva trazabilidad requisito → WBS → prueba → PR. | M | Se puede recorrer la cadena sin adivinar. |
| R-ENG-010 | El planner detecta conflictos y asigna propiedad temporal de archivos. | M | No se fusionan ediciones concurrentes incompatibles. |
| R-ENG-011 | Al cerrar la tarea se registra handoff, revisión, integración y limpieza. | M | Estado final y evidencia disponibles. |
| R-ENG-012 | El proceso admite pausa, cancelación, recuperación y reasignación. | P | Tarea suspendida deja checkpoint retomable. |

## 4. Requisitos no funcionales del flujo

| ID | Requisito | Prioridad | Método |
| --- | --- | --- | --- |
| R-NFR-001 | No se exponen secretos ni datos sensibles en Git, logs o mensajes. | M | Revisión de diff y secretos, evidencias sanitizadas. |
| R-NFR-002 | El proceso es reproducible con Git y herramientas documentadas. | M | Un tercero recrea worktree y validación desde instrucciones. |
| R-NFR-003 | El fracaso de un agente no corrompe `main` ni otro worktree. | M | Prueba de aislamiento/rollback de tarea fallida. |
| R-NFR-004 | Las decisiones y artefactos tienen historial auditable. | M | Git y PR enlazan autor, motivación y revisión. |
| R-NFR-005 | Ninguna acción irreversible externa se ejecuta sin autorización. | M | Simulación de puerta bloquea operación no autorizada. |
| R-NFR-006 | Las pruebas informan resultado real, no estimado ni supuesto. | M | Reporte incluye comando, entorno y evidencia. |
| R-NFR-007 | El flujo limita autonomía y permisos por tarea. | M | Alcance y permisos constan en ficha de delegación. |

## 5. Requisitos del producto — línea base G0

Origen común: [CHARTER](../00-project/CHARTER.md) v1.0, redactado por IA y aprobado por el patrocinador (ver su sección de procedencia). Prioridad `M` = obligatorio para el MVP; `P` = deseable en el MVP. Los casos de prueba `TC-PROD-*` se definen en G1, cuando exista arquitectura.

| ID | El sistema deberá… | Prioridad | Aceptación comprobable |
| --- | --- | --- | --- |
| R-PROD-001 | Permitir crear, renombrar y eliminar colecciones locales de documentos. | M | Una colección creada persiste tras reiniciar la aplicación; al eliminarla desaparece de la lista. |
| R-PROD-002 | Importar archivos PDF con texto, DOCX, TXT y MD, uno a uno o por carpeta, informando cada archivo rechazado y el motivo. | M | Una carpeta mixta con un archivo corrupto se importa con el resto y muestra un error específico para ese archivo. |
| R-PROD-003 | Indexar localmente los documentos importados (extracción, fragmentación e incrustaciones) mostrando el progreso, y reanudar tras una interrupción. | M | Cerrar la aplicación a mitad del indexado y reabrirla continúa sin reprocesar lo ya indexado. |
| R-PROD-004 | Responder preguntas en lenguaje natural sobre una colección con un modelo local, citando el documento y la página o sección de cada afirmación. Cada cita abre el fragmento de origen. | M | En el conjunto de evaluación, cada respuesta incluye ≥ 1 cita y cada cita apunta a un fragmento existente (KPI-1). |
| R-PROD-005 | Indicar explícitamente cuando los documentos no contienen evidencia suficiente, en lugar de inventar una respuesta. | M | Las preguntas sin respuesta del conjunto de evaluación devuelven la indicación de «sin evidencia», sin afirmaciones citadas falsas. |
| R-PROD-006 | Ofrecer búsqueda por palabra clave y semántica sobre una colección, con vista previa del fragmento. | M | Una búsqueda por un término exacto presente en un documento lo devuelve entre los cinco primeros resultados. |
| R-PROD-007 | Generar el resumen de un documento con citas a sus secciones. | P | El resumen de un documento de prueba contiene citas válidas a ese documento. |
| R-PROD-008 | Funcionar sin conexión tras la instalación y no enviar contenido documental a la red en el modo predeterminado. | M | La prueba de tráfico registra 0 bytes de contenido documental saliente durante importación, indexado y consulta (KPI-2). |
| R-PROD-009 | Detectar el entorno de ejecución del modelo local, guiar al usuario si falta y permitir elegir entre modelos instalados. | M | Sin entorno de modelo instalado, la aplicación muestra pasos de configuración en lugar de un error genérico. |
| R-PROD-010 | Al eliminar una colección, borrar sus copias de documentos, índices y conversaciones; permitir exportar conversaciones a Markdown. | M | Tras eliminar una colección no quedan archivos asociados en el directorio de datos; la exportación produce un `.md` legible con citas. |
| R-PROD-011 | Conservar localmente el historial de conversaciones de cada colección. | P | Una conversación sigue disponible tras reiniciar la aplicación. |
| R-PROD-012 | Ofrecer la interfaz en español, operable por completo con teclado y con contraste conforme a WCAG 2.2 AA. | M | Una auditoría de teclado y contraste de los flujos principales no encuentra bloqueos. |
| R-PROD-013 | Completar la primera respuesta citada sobre 100 PDFs en ≤ 15 minutos desde la instalación, con una latencia mediana de respuesta ≤ 8 s en el equipo de referencia. | P | Medición documentada de KPI-3 y KPI-4. |

Para cada requisito nuevo crear:
- ID permanente `R-PROD-###`, enunciado «El sistema deberá...», usuario/origen y justificación.
- Precondiciones, entradas/salidas, prioridad, límites, errores, criterio medible y prueba `TC-PROD-###`.
- Dependencias y exclusiones explícitas; decisión de aprobación y versión de línea base.

No convertir propuestas del agente en requisitos «aprobados». Fuera del MVP (ver CHARTER): nube, multiusuario en red, OCR, móvil, agentes con acciones externas, edición de documentos e integraciones de terceros.

## 6. Interfaz y datos

`OPEN-005` sistemas externos/MCP/APIs; `OPEN-006` datos, clasificación y retención; `OPEN-007` experiencia de usuario, accesibilidad e idioma; `OPEN-008` plataforma, rendimiento y disponibilidad. Todas sujetas a G0/G1. Su estado y responsable se siguen en el [registro OPEN](../00-project/OPEN_QUESTIONS.md).

## 7. Validación, control y aceptación

Cambios al SRS requieren propuesta `RFC` o issue, análisis de impacto en WBS/ADR/pruebas, revisión del planner, aprobación del PO para requisitos de producto y PR. No se borran IDs aprobados; se marcan `deprecated` o `superseded`. La línea base de producto empieza en `v1.0` solo al completar G0.
