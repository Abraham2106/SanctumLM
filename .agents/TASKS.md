# TASKS — Backlog canónico

**Última actualización:** 2026-10-16T05:30Z. **Consolidador:** planner (TASK-0008).

| ID | Título | WBS | Estado | Owner | Dependencia / criterio |
| --- | --- | --- | --- | --- | --- |
| TASK-0001 | Validar misión, usuarios y MVP | WBS-1.1 | done | Humano + planner | G0 aprobado |
| TASK-0002 | Completar SRS funcional | WBS-1.2 | done | Planner | Línea base R-PROD en SRS §5; TC-PROD en G1 (A7) |
| TASK-0003 | Aprobar arquitectura de producto | WBS-3.2 | in_progress | Architect + humano | OLA 1 A1–A7; G1 humano |
| TASK-0004 | Habilitar flujo CI y reglas de revisión | WBS-2.5 | draft | Planner | Tras G1; B1 CI |
| TASK-0005 | Primer vertical slice | WBS-4.1 | blocked | Por asignar | G1 aprobado |
| TASK-0006 | Coherencia documental | WBS-2.2, WBS-2.5 | done | Implementer | PR #2 merge a main |
| TASK-0007 | Definición de producto y cierre de G0 | WBS-1.1, WBS-1.2 | done | Implementer | PR #3 merge a main |
| TASK-0008 | Planner: OLA 1 plan, reserva, consolidación | WBS-2.1, WBS-3.2 | in_progress | Planner | Checkpoint; no implementa producto |
| TASK-0009 | A1 ADR-0006 stack Tauri 2 + §12 | WBS-3.2 | done | Architect | Merge local mvp `eb08f64`; reviewer r2+security approved; ADR proposed (G1 pendiente) |
| TASK-0010 | A2 ADR-0007 storage + spike sqlite-vec | WBS-3.2 | in_progress | Architect | Spike %TEMP%; OPEN-010 no se cierra |
| TASK-0011 | A3 ADR-0008 ModelAdapter + spike Ollama | WBS-3.2 | done | Architect | Merge local mvp `568b367`; reviewer+security approved; ADR proposed (G1 pendiente) |
| TASK-0012 | A4 ADR-0009 ingesta aislada | WBS-3.2, WBS-3.3 | done | Architect | Merge local mvp `1ae9c32`; reviewer r2+security approved; ADR proposed (G1 pendiente) |
| TASK-0013 | A5 ADR-0010 ArtifactSpec | WBS-3.2 | done | Architect | Merge local mvp `cf7cbd6`; reviewer+security approved; ADR proposed (G1 pendiente) |
| TASK-0014 | A6 ADR-0011 amenazas/egress/§15 | WBS-3.3 | in_review | Security | Fix r1 en `3a3fc80`; ronda 2 de reviewer |
| TASK-0015 | A7 C4, contratos, TC-PROD, OPEN-005..008 | WBS-3.2 | blocked | Architect | Tras merge A1–A6 |

Estados: `ready` implica ficha en `.agents/briefs/` y worktree creado desde `integration/mvp`.
