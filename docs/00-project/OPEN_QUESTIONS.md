# Registro de cuestiones abiertas — OPEN

**Responsable del registro:** planner. **Responsable de resolver:** quien figure en cada fila. **Línea base:** 2026-10-16.

Una cuestión `OPEN-###` es información necesaria que **nadie ha confirmado**. Mientras esté abierta, el trabajo que depende de ella queda `blocked`; el resto puede avanzar. Este archivo es la única fuente de IDs `OPEN-*`: los demás documentos los citan, no los definen.

| ID | Pregunta | Origen | Resuelve | Bloquea | Estado |
| --- | --- | --- | --- | --- | --- |
| OPEN-001 | ¿Qué problema resuelve SanctumLM, para quién, en qué entorno y con qué beneficio? | [CHARTER](CHARTER.md) | Patrocinador humano | G0, TASK-0001 | resuelta 2026-10-16 — patrocinador, a partir de su [investigación](research/INVESTIGACION_NOTEBOOK_IA_LOCAL_HIBRIDO.md); respuesta en [CHARTER](CHARTER.md) |
| OPEN-002 | ¿Quiénes son el patrocinador/PO, los usuarios primarios, los operadores y las partes afectadas? | [CHARTER](CHARTER.md) | Patrocinador humano | G0, TASK-0001 | resuelta 2026-10-16 — ídem |
| OPEN-003 | ¿Qué indicadores (máximo cinco) miden el éxito, con línea base, objetivo, ventana y método? | [CHARTER](CHARTER.md) | Patrocinador humano | G0, TASK-0001 | resuelta 2026-10-16 — ídem; umbrales de KPI-3/KPI-4 se fijan en G2 |
| OPEN-004 | ¿Qué restricciones aplican: stack, presupuesto, dispositivos, licencias, plataformas, datos, privacidad, conectividad, accesibilidad, integración? | [CHARTER](CHARTER.md) | Patrocinador humano | G0, TASK-0001 | resuelta 2026-10-16 — ídem; stack a ratificar en G1 |
| OPEN-005 | ¿Qué APIs, proveedores remotos, buscador web y conectores se integran, con qué formatos y versiones? | [SRS §6](../01-requirements/SRS.md) | Humano + architect | G1, TASK-0003 | abierta (propuesta en investigación §6, §10) |
| OPEN-006 | ¿Qué clasificación de datos (`public/private` o los cuatro niveles), retención y borrado se aplican? | [SRS §6](../01-requirements/SRS.md) | Humano + security | G1, TASK-0003 | abierta (propuesta en investigación §9.2) |
| OPEN-007 | ¿Qué detalle de UX (disposición fuentes / chat / estudio), nivel de accesibilidad y segundo idioma se exigen? | [SRS §6](../01-requirements/SRS.md) | Humano/PO | G1, TASK-0003 | abierta (propuesta en investigación §1.3) |
| OPEN-008 | ¿Qué modelo local de referencia y qué perfiles mínimos de hardware se soportan oficialmente? | [SRS §6](../01-requirements/SRS.md), investigación §16 | Humano + architect | G1, TASK-0003 | abierta |
| OPEN-009 | ¿Windows primero o Windows, macOS y Linux a la vez? | Investigación §16 | Patrocinador humano | G1 (empaquetado), R-PROD-107 | abierta |
| OPEN-010 | ¿Qué nivel de cifrado en reposo es obligatorio en el MVP (base de datos, originales, cachés, temporales) y cómo se gestionan las claves? | Investigación §4.1, §16 | Humano + security | G1 (ADR de almacenamiento) | abierta |
| OPEN-011 | ¿Qué conector de nube es el piloto del MVP (Drive, OneDrive, Dropbox…)? | Investigación §6.3, §16 | Patrocinador humano | R-PROD-012 | abierta |
| OPEN-012 | ¿Las infografías de v1 son solo SVG editable o incluyen también ilustraciones generativas? | Investigación §7.3, §16 | Patrocinador humano | R-PROD-010 | abierta |

## Cómo abrir una cuestión

1. Tomar el siguiente número libre; nunca reutilizar uno cerrado.
2. Redactar una **pregunta** concreta (no una propuesta de respuesta), indicar origen, quién puede resolverla y qué bloquea.
3. Marcar `blocked` solo las tareas dependientes y citar el ID en ellas.

## Cómo cerrar una cuestión

Cambiar el estado a `resuelta` con fecha, persona/rol que decidió y enlace a la evidencia (issue, PR o acta). La respuesta se incorpora al documento canónico que corresponda (CHARTER, SRS o ADR); esta tabla solo conserva el vínculo. Una inferencia de un agente no cierra una cuestión: como mucho se anota como `hipótesis` en el documento de origen.
