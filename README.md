# SanctumLM

**SanctumLM** es un asistente de IA privado y local para trabajar con documentos confidenciales: preguntas, búsqueda y resúmenes con citas a la fuente, sin que el contenido salga del equipo. Ver [CHARTER](docs/00-project/CHARTER.md).

**Estado:** G0 aprobado (2026-10-16); arquitectura (G1) pendiente. Sin implementación ni stack aprobados todavía.

Este repositorio establece una base **docs-as-code** para desarrollo agéntico: un agente planificador gobierna el backlog y la trazabilidad; agentes especializados implementan tareas acotadas en ramas y `git worktree` independientes; las decisiones se registran en ADR y el trabajo se integra mediante pull requests con revisión.

## Documentos fuente

- [Índice documental](docs/README.md)
- [Acta del proyecto](docs/00-project/CHARTER.md)
- [SRS — especificación de requisitos](docs/01-requirements/SRS.md)
- [WBS — estructura de desglose](docs/02-planning/WBS.md)
- [Hoja de ruta y puertas de control](docs/02-planning/ROADMAP.md)
- [Arquitectura — estado inicial](docs/03-architecture/ARCHITECTURE.md)
- [Modelo de operación de agentes](docs/05-agents/OPERATING_MODEL.md)
- [Protocolo de worktrees](docs/05-agents/WORKTREE_PROTOCOL.md)
- [Registro de coordinación](.agents/README.md)
- [Normas obligatorias de agentes](AGENTS.md)

## Primer ciclo de trabajo

1. El responsable humano confirma misión, usuarios, alcance, restricciones y criterios de éxito (G0).
2. El agente `planner` actualiza SRS, WBS, riesgos y trazabilidad, crea tareas con criterios comprobables y propone ADRs.
3. Por cada tarea aprobada se abre un worktree y rama únicos. Un ejecutor entrega código, pruebas y documentación.
4. Un revisor independiente valida y el planificador coordina la integración por PR.
5. Ningún agente se autoaprueba decisiones críticas ni incorpora cambios directamente a `main`.

### Estado y principios

Los requisitos `R-ENG-*` se refieren al **proceso de desarrollo**, no a capacidades funcionales del producto. Los `R-PROD-*` solo se incorporarán cuando G0 tenga decisiones verificadas. Los ADRs registran decisiones; el SRS registra necesidades verificables; la WBS registra trabajo.

Se permite crear documentación antes de G0, pero **no** declarar listo el producto ni elegir un stack definitivo.
