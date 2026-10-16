# ADR-0001 — Docs-as-code para requisitos y decisiones

- **Estado:** accepted
- **Fecha:** 2026-10-15
- **Alcance:** proceso de desarrollo
- **Origen:** instrucción explícita de mantener requerimientos y decisiones mediante `.md`
- **Vínculos:** R-ENG-004, R-ENG-005, R-ENG-009; WBS-1.2, WBS-2.2

## Contexto

Varios agentes requieren un estado de conocimiento auditable y transferible; decisiones implícitas en conversaciones provocan divergencia.

## Decisión

Guardar SRS, WBS, ADRs, trazabilidad y protocolos en Markdown versionado con Git. El SRS es normativo para requisitos; los ADRs registran decisiones y sus argumentos; las fichas de tareas ejecutan el trabajo. Cambios mediante PR enlazado.

## Alternativas

Solo chat/prompts: difícil auditar y reconstruir. Wiki externa como fuente única: introduce dependencia y riesgo de divergencia sin necesidad demostrada.

## Consecuencias

Mayor revisión documental y disciplina de enlaces; a cambio, histórico, trazabilidad, diffs y posibilidad de validación automática. No equivale a haber validado requisitos del producto.
