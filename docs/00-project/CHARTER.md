# Project charter — SanctumLM

**Versión:** 1.0. **Estado:** aprobado (G0). **Fecha:** 2026-10-16.

## Procedencia de esta versión

Las respuestas a OPEN-001..004 fueron **redactadas por un agente de IA a petición expresa del patrocinador** (@Abraham2106), que las aprobó por instrucción directa el 2026-10-16 UTC ("respóndelo y marca G0 como cerrado"). No provienen de entrevistas con usuarios reales. Por eso las métricas son objetivos iniciales y el riesgo RSK-11 sigue abierto hasta validarlas con pilotos.

## Contexto confirmado

Existe el repositorio `Abraham2106/SanctumLM`. Su desarrollo es agéntico, con planificación central, decisiones registradas como Markdown/ADR y ejecución de cada tarea en un worktree separado.

## Propósito, producto y éxito

Las preguntas siguientes se siguen en el [registro OPEN](OPEN_QUESTIONS.md); al resolverse, la respuesta se escribe aquí y el registro guarda el vínculo.

### Misión (OPEN-001, resuelta)

**SanctumLM es un asistente de IA privado y local para trabajar con documentos confidenciales.** Permite importar documentos propios, hacerles preguntas en lenguaje natural, buscarlos y resumirlos. Cada respuesta cita la página o sección de origen, y **ningún contenido sale del equipo del usuario**.

Problema que resuelve: los profesionales que manejan documentos confidenciales (contratos, expedientes, estados financieros, informes de clientes) no pueden usar asistentes en la nube por obligaciones de confidencialidad. Hoy buscan a mano en decenas de PDFs. SanctumLM les da el beneficio de un asistente de IA sin ceder sus datos a terceros.

### Usuarios y actores (OPEN-002, resuelta)

| Actor | Descripción |
| --- | --- |
| Usuario primario | Profesional independiente o miembro de un despacho pequeño (2–20 personas) de habla hispana: abogados, contadores, consultores. Usuario de Windows, sin perfil técnico. |
| Usuario secundario | Responsable informático del despacho (a menudo el propio socio), que instala la aplicación y elige el modelo local. |
| Patrocinador / PO | @Abraham2106. |
| Partes afectadas | Clientes del profesional cuyos datos aparecen en los documentos; no usan la aplicación, pero su privacidad es el motivo del producto. |

### Resultados medibles (OPEN-003, resuelta)

Producto nuevo, así que no hay línea base: se mide por primera vez en G2. Los objetivos son iniciales y se revisan con datos reales en G3.

| ID | Indicador | Objetivo inicial | Método de evidencia |
| --- | --- | --- | --- |
| KPI-1 | Respuestas correctas y con cita verificable | ≥ 85 % | Conjunto de evaluación propio de ≥ 100 preguntas sobre documentos de prueba públicos o sintéticos |
| KPI-2 | Contenido documental enviado a la red en modo predeterminado | 0 bytes | Prueba automatizada de tráfico de red durante importación, indexado y consulta |
| KPI-3 | Tiempo desde la instalación hasta la primera respuesta citada (100 PDFs) | ≤ 15 min | Medición cronometrada en el equipo de referencia |
| KPI-4 | Latencia mediana de respuesta | ≤ 8 s | Benchmark sobre el conjunto de evaluación en el equipo de referencia |
| KPI-5 | Despachos piloto con uso semanal durante 4 semanas | ≥ 5 | Registro manual del piloto (sin telemetría) |

### Restricciones (OPEN-004, resuelta)

- **Plataforma:** Windows 10/11 x64 primero; macOS después del MVP.
- **Equipo de referencia:** 16 GB de RAM, CPU de los últimos cinco años, GPU opcional.
- **Conectividad:** totalmente funcional sin conexión una vez instalado el modelo. Ninguna petición de red con contenido documental. Sin telemetría por defecto.
- **Presupuesto de infraestructura del MVP:** cero; sin servidores propios ni servicios de pago.
- **Licencias:** dependencias con licencias permisivas (MIT, Apache-2.0, BSD o equivalentes); modelos cuya licencia permita uso comercial.
- **Formatos del MVP:** PDF con texto extraíble, DOCX, TXT y Markdown. El OCR de escaneados queda fuera del MVP.
- **Idioma y accesibilidad:** interfaz en español (inglés después). Objetivo WCAG 2.2 AA: uso completo con teclado y contraste suficiente.
- **Stack:** no se decide en G0. La preferencia del patrocinador es una aplicación de escritorio instalable; el stack concreto se decide por ADR en G1.

## Alcance del MVP

**Incluye:** colecciones locales, importación de documentos, indexado local, preguntas con respuestas citadas, búsqueda, resumen de documento, historial de conversaciones, exportación y borrado de datos, y gestión del modelo local.

**Excluye:** sincronización en la nube, uso multiusuario en red, OCR, aplicación móvil, agentes que ejecuten acciones externas, edición de documentos e integración con servicios de terceros.

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
