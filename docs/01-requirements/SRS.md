# SRS — Software Requirements Specification

**Proyecto:** SanctumLM. **Versión:** 1.0. **Estado:** línea base de producto aprobada en G0. **Línea base:** 2026-10-16.

## 1. Objeto y límites

Este SRS contiene los requisitos del **proceso de desarrollo agéntico** (secciones 3–4) y los **requisitos iniciales del producto** aprobados en G0 (sección 5), derivados del [CHARTER](../00-project/CHARTER.md). El stack, las interfaces y el modelo de datos del producto se deciden en G1 mediante ADR.

Clasificación: `R-ENG` (flujo de ingeniería), `R-NFR` (calidad/seguridad del flujo), y `R-PROD` (producto: `001–099` funcionales, `101–199` no funcionales). En los requisitos del flujo, prioridad `M` = obligatoria y `P` = propuesta sujeta a confirmación; en los de producto manda el tramo (`MVP` / `v1`). Cada requisito deberá vincular tarea y prueba.

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

Origen: [CHARTER](../00-project/CHARTER.md) v1.0 y la [investigación del patrocinador](../00-project/research/INVESTIGACION_NOTEBOOK_IA_LOCAL_HIBRIDO.md) (columna «Origen»: `RF-xx` de su §2.2, `§n` de otras secciones). **Tramo** indica cuándo se entrega según el CHARTER: `MVP` (fases 1–3) o `v1` (fases 4–6). Los casos `TC-PROD-*` se definen en G1.

### 5.1 Funcionales

| ID | El sistema deberá… | Origen | Tramo | Aceptación comprobable |
| --- | --- | --- | --- | --- |
| R-PROD-001 | Crear, renombrar, archivar, etiquetar, buscar, duplicar y eliminar notebooks. | RF-01 | MVP | Cada operación persiste tras reiniciar; eliminar borra sus fuentes, índices y chats. |
| R-PROD-002 | Importar PDF, TXT, MD, DOCX, HTML y texto pegado, registrando hash SHA-256, origen, versión y estado de extracción. | RF-02, §5.1 | MVP | Un lote mixto con un archivo corrupto importa el resto y muestra un error específico; cada fuente guarda hash y estado. |
| R-PROD-003 | Agregar URLs públicas tras revisar dominio, formato y vista previa, guardando una copia textual local para uso offline. | RF-03, §9.3 | MVP | Una URL importada se consulta sin red; URLs a loopback, red privada o redirecciones hacia ellas se rechazan. |
| R-PROD-004 | Buscar en internet con proveedor configurable, mostrando fuente, dominio, fecha y fragmento, e importar solo lo seleccionado. | RF-04 | MVP | Nada se importa sin selección; la consulta nunca incluye contenido de fuentes privadas. |
| R-PROD-005 | Crear notas editables y convertir respuestas o notas en fuentes seleccionables sin romper referencias. | RF-05 | v1 | Editar una nota-fuente crea una versión nueva; las citas anteriores siguen apuntando a la versión original. |
| R-PROD-006 | Chat basado en fuentes con citas a fragmentos y acceso al original, distinguiendo respaldado / inferencia / discrepancia / no comprobado, y respondiendo «no hay evidencia suficiente» cuando proceda. | RF-06, §5.2 | MVP | Cada cita abre exactamente el fragmento citado; las preguntas imposibles del conjunto de evaluación producen abstención. |
| R-PROD-007 | Chat general independiente del notebook, con elección explícita de modelo y sin acceso automático a documentos. | RF-07 | MVP | Una conversación general no recupera ningún fragmento de ningún notebook. |
| R-PROD-008 | Motor de modelos con al menos un backend local y un adaptador remoto opt-in, mostrando destino, datos a enviar y coste estimado cuando haya información. | RF-08, §6.2 | MVP | Con el modelo local caído no hay tráfico a APIs remotas; un envío remoto muestra antes el payload exacto. |
| R-PROD-009 | Generar resúmenes, fichas de lectura, guías e informes con plantilla, alcance, extensión y bibliografía. | RF-09, §8.2 | v1 | Cada sección generada tiene citas válidas y la sección «Limitaciones y dudas» cuando hay incertidumbre. |
| R-PROD-010 | Generar infografías por secciones editables, con la fuente de cada afirmación y exportación SVG/PNG. | RF-10, §7 | v1 | Editar un paso no regenera el resto; ninguna afirmación sustantiva carece de evidencia en modo «basado en fuentes». |
| R-PROD-011 | Generar presentaciones con guion, comprobación de hechos, vista previa y exportación PPTX/PDF con texto editable. | RF-11, §8.1 | v1 | El PPTX abre con texto editable; los tests detectan desbordes. |
| R-PROD-012 | Importar archivos de proveedores de nube mediante conectores autorizados, copiando solo lo seleccionado al almacenamiento local. | RF-12, §6.3 | MVP (1 conector piloto) | El acceso es revocable y borra tokens y copias; nada se sube de vuelta. |
| R-PROD-013 | Funcionar offline: abrir notebook, buscar en fuentes importadas, conversar con modelo local instalado y crear artefactos básicos. | RF-13 | MVP | KPI-5 cumplido con la red deshabilitada. |
| R-PROD-014 | Administrar colas de importación y generación, con progreso, cancelación y reintento sin duplicar archivos. | RF-14, §3.3 | MVP | Cancelar y reintentar no duplica fuentes; interrumpir no pierde información confirmada. |
| R-PROD-015 | Exportar e importar notebooks en un paquete versionado, con advertencias de secretos, permisos y derechos de autor. | RF-15 | v1 | Exportar e importar un notebook reproduce fuentes, notas y citas. |

### 5.2 No funcionales

| ID | Requisito | Origen | Tramo | Verificación |
| --- | --- | --- | --- | --- |
| R-PROD-101 | Privacidad: sin llamadas a APIs externas ni telemetría por defecto. | §2.3 | MVP | Captura de tráfico en modo offline (KPI-1). |
| R-PROD-102 | Control remoto: el 100 % de los envíos de contenido a modelos remotos requiere una decisión autorizada por política, aplicada en backend y no solo en la UI. | §2.3, §9.2, §11.4 | MVP | Tests unitarios y adversariales de la política de egress. |
| R-PROD-103 | Integridad: hash SHA-256 del original, registro de transformaciones e historial de versiones. | §2.3, §4.2 | MVP | Una fuente actualizada conserva sus versiones previas y sus citas históricas. |
| R-PROD-104 | Citas: cada cita apunta a un `source_id` y a un segmento existente y autorizado; el backend nunca confía en IDs generados por el LLM. | §2.3, §7.2, §11.3 | MVP | KPI-2. |
| R-PROD-105 | Aislamiento: cada consulta queda restringida al notebook y a las fuentes seleccionadas, validado en base de datos. | §2.3, §11.3 | MVP | Escenario (e) de §15. |
| R-PROD-106 | Resiliencia: no perder información confirmada si se interrumpe una extracción o generación. | §2.3 | MVP | Prueba de interrupción forzada. |
| R-PROD-107 | Portabilidad: instaladores probados en Windows, macOS y Linux, con divergencias documentadas. | §2.3 | v1 | Matriz de pruebas por SO (prioridad según OPEN-009). |
| R-PROD-108 | Accesibilidad: navegación por teclado, contraste adecuado, contenido semántico y alternativas textuales. | §2.3 | MVP | Auditoría de teclado y contraste de los flujos principales. |
| R-PROD-109 | Rendimiento: metas definidas mediante benchmark en hardware real, sin prometer tokens/segundo universales. | §2.3, §14 | MVP | KPI-4. |
| R-PROD-110 | Auditabilidad: registro local de conectores, autorizaciones, versión de fuente y modelo usado, sin volcar datos sensibles. | §2.3 | MVP | Revisión de `audit_events` tras los escenarios de §15. |
| R-PROD-111 | Seguridad: fuentes tratadas como datos no confiables (prompt injection, RAG poisoning, HTML/PDF malicioso), parsers aislados, sin JS de HTML importado, motores locales solo en `127.0.0.1`, secretos en keychain. | §9.1 | MVP | Batería adversarial de §15. |

Para cada requisito nuevo crear:
- ID permanente `R-PROD-###`, enunciado «El sistema deberá...», usuario/origen y justificación.
- Precondiciones, entradas/salidas, prioridad, límites, errores, criterio medible y prueba `TC-PROD-###`.
- Dependencias y exclusiones explícitas; decisión de aprobación y versión de línea base.

No convertir propuestas del agente en requisitos «aprobados». Fuera de alcance (ver CHARTER): podcasts, narración y vídeo; multiusuario en tiempo real; sincronización entre dispositivos; publicación pública.

## 6. Interfaz y datos

La investigación propone un modelo de datos (§4), contratos IPC (§3.3, §12) y una política de egress (§9.2, §11.4) como punto de partida para G1. Siguen abiertas `OPEN-005` (APIs y conectores), `OPEN-006` (clasificación, cifrado y retención), `OPEN-007` (detalle de UX y accesibilidad) y `OPEN-008` (modelo de referencia y perfiles de hardware), además de OPEN-009..012. Su estado se sigue en el [registro OPEN](../00-project/OPEN_QUESTIONS.md).

## 7. Validación, control y aceptación

Cambios al SRS requieren propuesta `RFC` o issue, análisis de impacto en WBS/ADR/pruebas, revisión del planner, aprobación del PO para requisitos de producto y PR. No se borran IDs aprobados; se marcan `deprecated` o `superseded`. La línea base de producto empieza en `v1.0` solo al completar G0.
