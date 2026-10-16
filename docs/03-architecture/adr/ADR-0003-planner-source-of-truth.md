# ADR-0003 — Planner como coordinador único

- **Estado:** accepted
- **Fecha:** 2026-10-15
- **Alcance:** proceso de desarrollo
- **Origen:** indicación explícita de un agente planificador que centralice agentes, requisitos y decisiones
- **Vínculos:** R-ENG-001, R-ENG-002, R-ENG-006, R-ENG-010; WBS-2.1

## Contexto

Agentes ejecutores autónomos pueden duplicar tareas, cambiar prioridades y producir decisiones incompatibles sin dueño claro.

## Decisión

Un único rol lógico `planner` administra backlog, dependencias, asignaciones y consolidación documental. Otros agentes pueden proponer cambios mediante PR/mensajes, pero no reemplazar unilateralmente las fuentes canónicas. La aprobación humana permanece para visión y operaciones críticas.

## Alternativas

Coordinación totalmente distribuida: sin árbitro de conflictos. Un único agente que implementa y revisa todo: reduce independencia de validación.

## Consecuencias

Un coordinador simplifica coherencia, pero es un posible cuello de botella. Se exigen checkpoints y recuperación de su estado.
