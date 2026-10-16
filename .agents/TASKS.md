# TASKS — Backlog canónico

**Última actualización:** 2026-10-17T07:25Z. **Consolidador:** planner (integración TASK-0023 en `integration/mvp`).

| ID | Título | WBS | Estado | Owner | Dependencia / criterio |
| --- | --- | --- | --- | --- | --- |
| TASK-0001 | Validar misión, usuarios y MVP | WBS-1.1 | done | Humano + planner | G0 aprobado |
| TASK-0002 | Completar SRS funcional | WBS-1.2 | done | Planner | Línea base R-PROD en SRS §5; TC-PROD en G1 (A7) |
| TASK-0003 | Aprobar arquitectura de producto | WBS-3.2 | done | Architect + humano | G1 aprobado 2026-10-16; ADR-0006–0011 accepted |
| TASK-0004 | Habilitar flujo CI y reglas de revisión | WBS-2.5 | draft | Planner | Tras TASK-0019; ver TASK-0020 |
| TASK-0005 | Primer vertical slice | WBS-4.1 | ready | Por asignar | G1 aprobado; pendiente ficha y asignación |
| TASK-0006 | Coherencia documental | WBS-2.2, WBS-2.5 | done | Implementer | PR #2 merge a main |
| TASK-0007 | Definición de producto y cierre de G0 | WBS-1.1, WBS-1.2 | done | Implementer | PR #3 merge a main |
| TASK-0008 | Planner: OLA 1 plan, reserva, consolidación | WBS-2.1, WBS-3.2 | done | Planner | PARADA 1 + G1 humano |
| TASK-0009 | A1 ADR-0006 stack Tauri 2 + §12 | WBS-3.2 | done | Architect | ADR-0006 accepted G1 2026-10-16 |
| TASK-0010 | A2 ADR-0007 storage + spike sqlite-vec | WBS-3.2 | done | Architect | ADR-0007 accepted; OPEN-010 abierta |
| TASK-0011 | A3 ADR-0008 ModelAdapter + spike Ollama | WBS-3.2 | done | Architect | ADR-0008 accepted; spike Ollama not-run (RSK-12) |
| TASK-0012 | A4 ADR-0009 ingesta aislada | WBS-3.2, WBS-3.3 | done | Architect | ADR-0009 accepted G1 2026-10-16 |
| TASK-0013 | A5 ADR-0010 ArtifactSpec | WBS-3.2 | done | Architect | ADR-0010 accepted G1 2026-10-16 |
| TASK-0014 | A6 ADR-0011 amenazas/egress/§15 | WBS-3.3 | done | Security | ADR-0011 accepted G1 2026-10-16 |
| TASK-0015 | A7 C4, contratos, TC-PROD, OPEN-005..008 | WBS-3.2 | done | Architect | PARADA 1; soporte G1 |
| TASK-0016 | Consolidación G1: accept ADR-0006–0011 y merge main | WBS-3.2 | done | Implementer | Evidencia humana 2026-10-16; `origin/main` @ `828415a` |
| TASK-0017 | Corpus de evaluación B3 | WBS-3.4 | done | Implementer | Merge `integration/mvp` @ `1cc29e3` ← `047e3c4` |
| TASK-0018 | Esqueleto Cargo workspace y crates §12 | WBS-3.4 | done | Implementer | Merge `77eb850` ← `dee152d` APPROVED; 6 tests workspace |
| TASK-0019 | Stub desktop Tauri 2 + React/TS (B1) | WBS-3.4 | done | Implementer | Merge `2180b6a` ← `87dff19` APPROVED; `apps/desktop/**` |
| TASK-0020 | CI workspace Rust + reglas revisión (B1) | WBS-2.5, WBS-3.4 | done | Implementer | Merge `5546344` ← `4ce8c9d` APPROVED; bitácora `8554070` |
| TASK-0021 | Contratos B2 Rust/TS + fakes in-memory | WBS-3.4 | done | Implementer | Merge `integration/mvp` ← `d9a8b20` APPROVED reviewer + security |
| TASK-0022 | SQLite migraciones + esquema FTS5 (OLA 3 C1) | WBS-3.4.1 | done | Implementer | Merge `integration/mvp` ← `325d4c6` @ `fccd17d`; APPROVE reviewer r2 + security; OPEN-010: sin SQLCipher |
| TASK-0023 | Recuperación FTS + VectorIndex stub | WBS-3.4.2 | done | Implementer | Merge `integration/mvp` ← `1aec186` @ `119743a`; APPROVE reviewer + security; OPEN-010 abierta |
| TASK-0024 | Sidecar ingesta skeleton | WBS-3.4.3 | done | Implementer | Merge `integration/mvp` ← `ca24b48` @ `f002f47`; APPROVE reviewer r2 + security r2; ADR-0009 IPC/staging |

Estados: `ready` implica ficha en `.agents/briefs/` y worktree creado desde `integration/mvp`.
