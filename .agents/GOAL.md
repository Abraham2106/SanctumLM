# GOAL — Objetivo vigente

- **Fecha de snapshot:** 2026-10-17
- **Estado:** G0 y **G1 aprobados** (ADR-0006–0011 accepted)
- **Objetivo del producto:** aplicación de escritorio local-first de investigación y estudio con IA, inspirada en NotebookLM, con fronteras verificables entre lo local y lo remoto ([CHARTER](../docs/00-project/CHARTER.md)).
- **Objetivo inmediato:** **OLA 4 tramo 3** (TASK-0034 E2E UI↔IPC, TASK-0035 jobs SQLite) sobre `main` @ `84f2ca0`; tramo 2 núcleo chat (0030..0032) integrado; UI aún mock fuera de Tauri; **TASK-0005** / `WBS-4.1`; **sin declarar G2**.
- **Objetivo de programa (sesión larga):** ≥ 22/26 R-PROD (001..015 y 101..111) implementados, con pruebas que pasan y revisados. Techo 24/26. R-PROD-012 OAuth real y R-PROD-107 instaladores 3 SO solo si el humano autoriza.
- **Restricciones:** privacidad por defecto, `cloud_fallback=false`, offline real, español primero; sin push/PR/publicar/firmar sin autorización; AGENTS.md prevalece.
- **Criterio de éxito de la fase anterior (G1):** cumplido — ADRs de producto con campo Aprobación y evidencia humana 2026-10-16.
- **Fuente:** CHARTER v1.0, SRS v1.0 e [investigación](../docs/00-project/research/INVESTIGACION_NOTEBOOK_IA_LOCAL_HIBRIDO.md).
