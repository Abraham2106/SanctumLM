# ADR-0002 — Un task, una rama y un worktree

- **Estado:** accepted
- **Fecha:** 2026-10-15
- **Alcance:** proceso de desarrollo
- **Origen:** requerimiento explícito de worktree propio por agregación
- **Vínculos:** R-ENG-003, R-NFR-003, R-ENG-010; WBS-2.3
- **Aprobación:** merge de [PR #1](https://github.com/Abraham2106/SanctumLM/pull/1) por @Abraham2106 el 2026-10-16 UTC; sin revisión independiente registrada en el PR.

## Contexto

Agentes paralelos en un mismo checkout pueden sobrescribir cambios, contaminar estados, cambiar de rama ajena o alterar artefactos intermedios.

## Decisión

Cada tarea `TASK-####` usa su rama `agent/TASK-####-slug` y un `git worktree` independiente. La integración se efectúa vía PR con revisión y limpieza tras merge. El planner asigna propiedad de archivos y evita dependencias contradictorias.

## Alternativas

Compartir un checkout: barato pero inseguro al paralelizar. Un repositorio clonado completo por agente: mayor aislamiento, sincronización y costo adicionales sin justificación inicial.

## Consecuencias

Más carpetas y mantenimiento de worktrees; mayor seguridad de cambios locales. Los archivos del worktree NO son un canal compartido en tiempo real.
