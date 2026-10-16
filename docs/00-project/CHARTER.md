# Project charter — SanctumLM

**Versión:** 1.0. **Estado:** aprobado (G0). **Fecha:** 2026-10-16.

## Procedencia de esta versión

Las respuestas a OPEN-001..004 se extraen de la [investigación del patrocinador](research/INVESTIGACION_NOTEBOOK_IA_LOCAL_HIBRIDO.md) («Notebook IA local e híbrido», 15 oct. 2026), aportada por @Abraham2106, que aprobó G0 por instrucción directa el 2026-10-16 UTC. Cuando la investigación da una **recomendación** se adopta como decisión de G0; cuando deja algo **abierto** (su §16) se registra como `OPEN-*` y bloquea solo la parte afectada. La investigación es documental: no incluye entrevistas con usuarios (RSK-11).

## Contexto confirmado

Existe el repositorio `Abraham2106/SanctumLM`. Su desarrollo es agéntico, con planificación central, decisiones registradas como Markdown/ADR y ejecución de cada tarea en un worktree separado.

## Propósito, producto y éxito

Las preguntas siguientes se siguen en el [registro OPEN](OPEN_QUESTIONS.md); al resolverse, la respuesta se escribe aquí y el registro guarda el vínculo.

### Misión (OPEN-001, resuelta)

**SanctumLM es una aplicación de escritorio local-first de investigación y estudio asistidos por IA**, inspirada en NotebookLM / Gemini Notebook. Organiza notebooks independientes con fuentes locales y web, notas, chat basado en fuentes con citas verificables, chat general, búsqueda web y, más adelante, informes, guías de estudio, infografías y presentaciones.

Su rasgo distintivo no es solo ejecutar un modelo sin internet, sino establecer **fronteras verificables** entre almacenamiento local, consulta externa, inferencia local y envío autorizado a proveedores de IA. Enviar fragmentos de documentos a una API externa es excepcional, visible, opt-in y auditable. El producto promete **trazabilidad y verificación**, nunca veracidad infalible.

### Usuarios y actores (OPEN-002, resuelta)

| Actor | Necesidad principal |
| --- | --- |
| Estudiante | Reunir PDFs, apuntes y artículos; explicaciones progresivas; guías y autoevaluación; estudiar sin internet. |
| Docente | Estructurar un tema; esquemas, informes y presentaciones revisables con citas. |
| Investigador | Comparar versiones, detectar contradicciones, exportar informes con trazabilidad. |
| Usuario general | Notebooks temáticos y chat de IA convencional al margen de las fuentes. |
| Patrocinador / PO | @Abraham2106. |

Una instalación = un usuario. Colaboración en tiempo real, sincronización entre dispositivos y publicación pública quedan fuera del MVP.

### Resultados medibles (OPEN-003, resuelta)

La investigación prohíbe inventar umbrales de calidad: se fijan con el conjunto de evaluación y el hardware objetivo. Por eso hay dos **gates binarios** desde ya y tres indicadores cuyo umbral se fija tras la primera medición (G2).

| ID | Indicador | Objetivo | Método de evidencia |
| --- | --- | --- | --- |
| KPI-1 | Gate de privacidad: envíos a API externa sin decisión de autorización | 0, siempre | Pruebas de egress y captura de tráfico; escenarios adversariales (c) y (d) de la investigación §15 |
| KPI-2 | Gate de fuentes: citas a un ID inexistente o a una fuente no autorizada / de otro notebook | 0, siempre | Validación de citas en backend y escenario (e) |
| KPI-3 | `citation_precision`, `citation_recall` y abstención justificada | Umbral tras la línea base de G2 | Conjunto de evaluación de 10–20 documentos con preguntas imposibles y conflictos, más revisión humana |
| KPI-4 | Latencia de respuesta y de indexado por perfil de hardware | Umbral tras benchmark en hardware real | `benchmarks/` en equipos de 16 GB y 32 GB |
| KPI-5 | Uso offline real: abrir notebook, buscar, conversar con modelo local y reabrir la sesión sin red | Cumplido / no cumplido | Criterio de salida de la fase 1 con red deshabilitada |

### Restricciones (OPEN-004, resuelta)

- **Plataformas:** Windows, macOS y Linux. La prioridad entre ellas sigue abierta (OPEN-009).
- **Hardware:** 16 GB de RAM como mínimo práctico recomendado; con 8 GB funciona en modo degradado y sin promesas de rendimiento.
- **Privacidad por defecto:** sin APIs externas ni telemetría; `cloud_fallback=false` aplicado en el backend; si el modelo local falla, nunca hay cambio automático a un proveedor remoto.
- **Offline real:** todo lo esencial funciona sin red, siempre que los modelos y las fuentes se hayan descargado antes.
- **Seguridad:** las fuentes son datos no confiables; la política de salida es código determinista, nunca una decisión del LLM; secretos en el keychain del SO.
- **Licencias:** código, modelos y datasets con licencia compatible y revisada (la licencia de pesos puede diferir de la de uso comercial).
- **Idioma:** español primero, como recomienda la investigación.
- **Costes:** la inferencia local no paga tokens; búsqueda web y APIs remotas pueden tener tarifas y solo se usan con configuración explícita.
- **Stack:** la investigación recomienda Tauri 2 + React/TypeScript + núcleo Rust, SQLite con FTS5 y sqlite-vec, SQLCipher, Ollama con adaptadores (llama.cpp, LM Studio, remotos), Docling/MarkItDown en un sidecar aislado y PptxGenJS. **Es la recomendación de partida; se ratifica o cambia por ADR en G1.**

## Alcance

Se sigue la ruta de la investigación (§13): primero un notebook basado en fuentes que sea fiable, offline y local; después web y conectores con capa de permisos; por último, artefactos sobre el mismo motor de evidencia.

| Tramo | Fases de la investigación | Incluye |
| --- | --- | --- |
| **MVP** | 1. Núcleo local, 2. RAG robusto, 3. Híbrido seguro | Notebooks, importación, extracción, búsqueda híbrida, chat con citas, chat general, modelo local, offline, colas; web, URLs, proveedor remoto opt-in y un conector piloto |
| **v1** | 4. Estudio y documentos, 5. Estudio visual, 6. Endurecimiento | Notas como fuentes, resúmenes, informes, guías, exportación de notebooks, infografías SVG, presentaciones PPTX, QA multiplataforma y piloto |
| **Fuera** | — | Podcasts, narración y vídeo; multiusuario en tiempo real; sincronización; publicación pública |

## Gobernanza

- Patrocinador humano: aprueba visión, límites, costos/despliegues, criterios de éxito y decisiones irreversibles.
- Product owner humano: @Abraham2106; prioriza necesidades y acepta el comportamiento del producto.
- Agente planner: prepara y mantiene SRS/WBS/backlog, dependencias, asignaciones y reportes.
- Agentes especializados: proponen e implementan dentro del marco delegado; sus entregas requieren verificación y revisión.

## Puertas

`G0 — descubrimiento`: el humano aprueba problema, perfiles de usuario, MVP, métricas, restricciones y requisitos iniciales. **Aprobado el 2026-10-16** (ver procedencia arriba).

`G1 — solución`: arquitectura objetivo y ADRs de alto impacto aprobados, amenazas revisadas, aceptación por característica definida.

`G2 — vertical slice`: primer recorrido de extremo a extremo reproducible, con pruebas, documentación y revisión.

`G3 — calidad`: cumplimiento de requisitos, controles, cobertura de riesgos y evidencias de aceptación.

`G4 — entrega`: paquete versionado, notas, rollback y aprobación correspondiente.

## Procedimiento de confirmación de G0

El planner abre una propuesta/issue con respuestas a `OPEN-001..004`, actualiza CHARTER/SRS/USE_CASES, obtiene decisión humana registrada y recién entonces cambia el estado a `approved`. Toda inferencia no confirmada se rotula `hipótesis`.
