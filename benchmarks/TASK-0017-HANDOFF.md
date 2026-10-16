# Handoff — TASK-0017

- **Agente/rol:** implementer (corpus evaluación OLA 2 B3)
- **Worktree y rama:** `C:\Users\solan\Documents\Personal\Startup\SanctumLM-wt-TASK-0017` / `agent/TASK-0017-eval-corpus`
- **PR y commits:** commit local únicamente; sin push ni PR
- **Estado:** ready_for_review
- **Fecha UTC:** 2026-10-16

## Alcance ejecutado

- Creado `benchmarks/README.md` con instrucciones de uso.
- Corpus `benchmarks/es-mvp-rag/`: 15 fuentes (MD, TXT, HTML, PDF, DOCX), manifiestos `manifest.json` + `MANIFEST.md`, evaluación `evaluation/questions.json` (12 preguntas).
- Casos: tablas, `corrupt-informe.pdf` y `corrupt-borrador.docx`, inyección «ignora al usuario», contradicción fechas Nébula Docs, PDF escaneado simulado con laguna OCR, notebooks A/B para Capacidad Alfa.
- Añadido párrafo §5.3 en `docs/04-quality/EVALUATION.md` (sin reescribir el resto).

Superficie respetada: solo `benchmarks/` y `docs/04-quality/EVALUATION.md`.

## Verificación

| Comando o TC | Entorno | Resultado | Evidencia |
| --- | --- | --- | --- |
| `python scripts/validate_docs.py` | worktree TASK-0017 | pass | salida exit code 0 |
| `git diff --check` | worktree TASK-0017 | pass | sin conflict markers |
| Suite producto | — | not-run | fuera de alcance TASK-0017 |

## Trazabilidad

| Requisito | Cobertura en corpus |
| --- | --- |
| R-PROD-002 | Lote mixto + corruptos con nombre claro |
| R-PROD-006 | Preguntas con evidencia + imposibles Q-007/Q-008 |
| R-PROD-104 | Preguntas referencian `SRC-*` para validar citas |
| R-PROD-105 | Q-009/Q-010 aislamiento notebook A/B |
| R-PROD-111 | SRC-05 injection; HTML sin JS ejecutable |

Investigación: §5.3 evaluación RAG; §15 escenarios (a)(e)(h) documentados en preguntas/manifiesto.

## Decisiones propuestas y pendientes

- **Aceptada:** corpus 100 % sintético; licencia `synthetic-original-sanctumlm` en manifiesto.
- **Pendiente:** umbrales numéricos de `citation_precision`/latencia tras hardware objetivo (§5.3).

## Riesgos/bloqueos/seguimiento

- PDF escaneado es simulación textual mínima, no imagen raster; ingest real puede requerir fixture OCR adicional en tarea de pipeline.
- Revisor humano debe validar preguntas antes de usar como gate de release.

## Checklist de cierre

- [x] Sin secretos
- [ ] revisión independiente
- [x] documentos
- [ ] merge confirmado (no solicitado)
- [ ] worktree retirado (post-merge)
