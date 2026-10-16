# STATUS — Checkpoint canónico

- **Timestamp de referencia:** 2026-10-16T06:00Z
- **Fase:** **PARADA 1 (G1)** — paquete listo; espera «G1 aprobado»
- **Implementación de producto:** ninguna (sin Cargo.toml / package.json)
- **Gate actual:** G1 — ADRs 0006–0011 `proposed`; no accepted hasta evidencia humana
- **G0:** aprobado 2026-10-16; `main` @ `4ff44a7` (PR #2 y #3)
- **Integración:** `integration/mvp` @ merge TASK-0015; nada en `main`
- **OLA 1:** TASK-0008..0015 cerradas localmente (reviewer + security donde aplicaba)
- **ADRs accepted (proceso):** 0001–0003
- **ADRs proposed (proceso):** 0004, 0005
- **ADRs producto G1 proposed:** 0006 stack, 0007 storage, 0008 router, 0009 ingesta, 0010 artefactos, 0011 amenazas
- **Avance R-PROD (código+pruebas+revisión):** **0/26**. Diseño G1: 26/26 R-PROD tienen TC-PROD y ADR
- **Validador:** `python scripts/validate_docs.py` en `integration/mvp` (tras consolidación)
- **Spikes:** sqlite-vec 0.1.9 Windows **pass**; Ollama modelos **not-run** (RSK-12, no en PATH)
- **Riesgos:** RSK-13, RSK-12 (benchmark no medido), RSK-14, RSK-11
- **Bloqueo:** sin «G1 aprobado»; OPEN-005..012 abiertas; sin push/PR autorizado
- **Siguiente:** humano aprueba o corrige G1. **No OLA 2** hasta entonces.

Reserva: `.agents/FILE_RESERVATION.md`.
