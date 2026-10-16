# Reserva de archivos — OLA 1 (G1)

**Vigente desde:** 2026-10-16. **Dueño del mapa:** planner (TASK-0008).
**Base:** `integration/mvp` @ `4ff44a7` (main = merge de TASK-0007).

Los archivos de dueño único **no** los edita ningún agente de esta ola. Cada tarea escribe solo su superficie.

| Tarea | Superficie exclusiva | Prohibido |
| --- | --- | --- |
| TASK-0008 planner | `.agents/*` (excepto briefs ya publicados), `docs/03-architecture/adr/README.md`, `docs/01-requirements/TRACEABILITY.md`, `docs/02-planning/RISK_REGISTER.md` | código de producto |
| TASK-0009 A1 | `docs/03-architecture/adr/ADR-0006-desktop-stack.md` | índice ADR, `.agents/*`, crear `apps/` |
| TASK-0010 A2 | `docs/03-architecture/adr/ADR-0007-storage.md`, `docs/04-quality/evidence/spike-sqlite-vec-windows.md` | `Cargo.toml`, migraciones reales, índice ADR |
| TASK-0011 A3 | `docs/03-architecture/adr/ADR-0008-model-router.md`, `docs/04-quality/evidence/spike-ollama-local-models.md` | APIs de pago, modelos >4 GB, índice ADR |
| TASK-0012 A4 | `docs/03-architecture/adr/ADR-0009-ingestion.md` | workers reales, índice ADR |
| TASK-0013 A5 | `docs/03-architecture/adr/ADR-0010-artifact-engine.md` | implementar PptxGenJS/SVG, §8.3, índice ADR |
| TASK-0014 A6 | `docs/03-architecture/adr/ADR-0011-threat-model-egress.md`, `docs/03-architecture/THREAT_MODEL.md` (nuevo) | `docs/04-quality/SECURITY.md` (proceso), índice ADR, `.agents/*` |

**Compartidos (planner, secuencial al integrar):** `adr/README.md`, `.agents/DECISIONS.md`, `.agents/STATUS.md`, `.agents/TASKS.md`, `ARCHITECTURE.md`, `DATA_AND_INTERFACES.md`, `TEST_PLAN.md`, `OPEN_QUESTIONS.md`.

**A7 (TASK-0015)** no se lanza hasta integrar A1–A6.
