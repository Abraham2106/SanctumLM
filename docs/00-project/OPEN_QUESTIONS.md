# Registro de cuestiones abiertas — OPEN

**Responsable del registro:** planner. **Responsable de resolver:** quien figure en cada fila. **Línea base:** 2026-10-15.

Una cuestión `OPEN-###` es información necesaria que **nadie ha confirmado**. Mientras esté abierta, el trabajo que depende de ella queda `blocked`; el resto puede avanzar. Este archivo es la única fuente de IDs `OPEN-*`: los demás documentos los citan, no los definen.

| ID | Pregunta | Origen | Resuelve | Bloquea | Estado |
| --- | --- | --- | --- | --- | --- |
| OPEN-001 | ¿Qué problema resuelve SanctumLM, para quién, en qué entorno y con qué beneficio? | [CHARTER](CHARTER.md) | Patrocinador humano | G0, TASK-0001 | abierta |
| OPEN-002 | ¿Quiénes son el patrocinador/PO, los usuarios primarios, los operadores y las partes afectadas? | [CHARTER](CHARTER.md) | Patrocinador humano | G0, TASK-0001 | abierta |
| OPEN-003 | ¿Qué indicadores (máximo cinco) miden el éxito, con línea base, objetivo, ventana y método? | [CHARTER](CHARTER.md) | Patrocinador humano | G0, TASK-0001 | abierta |
| OPEN-004 | ¿Qué restricciones aplican: stack, presupuesto, dispositivos, licencias, plataformas, datos, privacidad, conectividad, accesibilidad, integración? | [CHARTER](CHARTER.md) | Patrocinador humano | G0, TASK-0001 | abierta |
| OPEN-005 | ¿Con qué sistemas externos, MCP o APIs se integra el producto, y con qué formatos y versiones? | [SRS §6](../01-requirements/SRS.md) | Humano + architect | G1, TASK-0003 | abierta |
| OPEN-006 | ¿Qué datos maneja, cómo se clasifican, cifran y retienen, y hay datos personales? | [SRS §6](../01-requirements/SRS.md) | Humano + security | G1, TASK-0003 | abierta |
| OPEN-007 | ¿Qué experiencia de usuario, nivel de accesibilidad e idiomas se exigen? | [SRS §6](../01-requirements/SRS.md) | Humano/PO | G1, TASK-0003 | abierta |
| OPEN-008 | ¿En qué plataforma corre y con qué límites de rendimiento y disponibilidad? | [SRS §6](../01-requirements/SRS.md) | Humano + architect | G1, TASK-0003 | abierta |

## Cómo abrir una cuestión

1. Tomar el siguiente número libre; nunca reutilizar uno cerrado.
2. Redactar una **pregunta** concreta (no una propuesta de respuesta), indicar origen, quién puede resolverla y qué bloquea.
3. Marcar `blocked` solo las tareas dependientes y citar el ID en ellas.

## Cómo cerrar una cuestión

Cambiar el estado a `resuelta` con fecha, persona/rol que decidió y enlace a la evidencia (issue, PR o acta). La respuesta se incorpora al documento canónico que corresponda (CHARTER, SRS o ADR); esta tabla solo conserva el vínculo. Una inferencia de un agente no cierra una cuestión: como mucho se anota como `hipótesis` en el documento de origen.
