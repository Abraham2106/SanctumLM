# Project charter — SanctumLM

**Versión:** 0.1 (borrador). **Estado:** pendiente de validación de G0. **Fecha:** 2026-10-15.

## Contexto confirmado

Existe el repositorio `Abraham2106/SanctumLM` y se ha definido que **su desarrollo** será agéntico, con planificación central, decisiones registradas como Markdown/ADR y ejecución de cada agregación en un worktree separado. El repositorio no contiene implementación ni especificación funcional validada a esta fecha.

## Propósito, producto y éxito

Las preguntas siguientes se siguen en el [registro OPEN](OPEN_QUESTIONS.md); al resolverse, la respuesta se escribe aquí y el registro guarda el vínculo.

**Misión funcional del producto:** `OPEN-001` — describir problema que resuelve SanctumLM, usuarios, entorno y beneficio esperado; no asumir que el nombre determina su función.

**Usuarios y actores:** `OPEN-002` — patrocinador/PO, usuarios primarios, operadores y partes afectadas.

**Resultados medibles:** `OPEN-003` — definir hasta cinco indicadores con línea base, objetivo, ventana de medición y método de evidencia. No asignar cifras arbitrarias.

**Restricciones:** `OPEN-004` — stack permitido, presupuesto, dispositivos, licencias, plataformas, datos, privacidad, conectividad, accesibilidad e integración.

## Alcance de la línea base documental

Incluye gobierno del desarrollo, proceso multiagente, gestión de cambios, requisitos de ingeniería, arquitectura a decidir, trazabilidad, seguridad, pruebas, entregas y operación. No incluye, hasta resolver G0, promesas de funcionalidades, modelos, APIs, almacenamiento, interfaz, infraestructura o despliegue específicos.

## Gobernanza

- Patrocinador humano: aprueba visión, límites, costos/despliegues, criterios de éxito y decisiones irreversibles.
- Product owner humano (si se designa): prioriza necesidades y acepta comportamiento del producto.
- Agente planner: prepara y mantiene SRS/WBS/backlog, dependencias, asignaciones y reportes.
- Agentes especializados: proponen e implementan dentro del marco delegado; sus entregas requieren verificación y revisión.

## Puertas

`G0 — descubrimiento`: el humano aprueba problema, perfiles de usuario, MVP, métricas, restricciones y requisitos iniciales. Sin G0 no se inicia código funcional basado en supuestos.

`G1 — solución`: arquitectura objetivo y ADRs de alto impacto aprobados, amenazas revisadas, aceptación por característica definida.

`G2 — vertical slice`: primer recorrido de extremo a extremo reproducible, con pruebas, documentación y revisión.

`G3 — calidad`: cumplimiento de requisitos, controles, cobertura de riesgos y evidencias de aceptación.

`G4 — entrega`: paquete versionado, notas, rollback y aprobación correspondiente.

## Procedimiento de confirmación de G0

El planner abre una propuesta/issue con respuestas a `OPEN-001..004`, actualiza CHARTER/SRS/USE_CASES, obtiene decisión humana registrada y recién entonces cambia el estado a `approved`. Toda inferencia no confirmada se rotula `hipótesis`.
