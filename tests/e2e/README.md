# E2E — tramo vertical núcleo (TASK-0037)

Evidencia **reproducible** del flujo **import → índice FTS → búsqueda → chat notebook** contra el núcleo Tauri (`sanctumlm-desktop`), sin depender de la UI React ni de Ollama en CI.

## Qué cubre (parcial)

| Requisito | Alcance de esta evidencia | TC homólogo (estado) |
| --- | --- | --- |
| R-PROD-001 | Crear notebook y persistencia en SQLite en memoria de prueba | TC-PROD-001 — **not-run** formal (sin reinicio de app) |
| R-PROD-002 | Import `inline_text` → fuente `ready` + prefijo SHA-256 | TC-PROD-002 — **not-run** (sin lote mixto/corruptos en este script) |
| R-PROD-006 | Chat notebook con recuperación FTS y citas validadas (adaptador fake) | TC-PROD-006 — **not-run** formal (sin batería `questions.json` completa) |
| R-PROD-105 | FTS acotado por `notebook_id` (corpus `notebook-a` / `notebook-b`) | TC-PROD-105 — **not-run** formal |

**No** declara gate **G2** ni cobertura **26/26** de `R-PROD`.

## Ejecución automática (recomendada)

Desde la raíz del repositorio:

```bash
chmod +x scripts/run-e2e-vertical-slice.sh
./scripts/run-e2e-vertical-slice.sh
```

Equivalente manual:

```bash
cargo test -p sanctumlm-desktop e2e_task0037 -- --nocapture
```

Salida con trazas `[e2e-0037]` en stderr; el script guarda log y metadatos en `tests/e2e/artifacts/` (local, no versionados).

## Tests

| Test | Archivo |
| --- | --- |
| `e2e_task0037_import_fts_notebook_chat` | `apps/desktop/src-tauri/tests/e2e_task0037_vertical_slice.rs` |
| `e2e_task0037_notebook_fts_isolation` | idem |
| `e2e_task0037_general_chat_skips_notebook_fts` | idem (coherencia R-PROD-007 parcial) |

Corpus: archivos bajo `benchmarks/es-mvp-rag/sources/` (solo lectura; import vía `inline_text`).

## Smoke manual UI (opcional, fuera de este paquete)

1. `cd apps/desktop && npm run tauri dev`
2. Crear notebook → importar TXT/MD → buscar término en panel fuentes
3. Chat notebook con Ollama local (RSK-12): citas visibles si el modelo responde

Registrar evidencia con la plantilla en [TEST_PLAN.md](../../docs/04-quality/TEST_PLAN.md) § plantilla; marcar `not-run` donde no aplique.

## Trazabilidad

- **Tarea:** TASK-0037  
- **WBS:** cierre evidencia OLA 4 (import/FTS/chat), sin ampliar requisitos  
- **ADR:** ADR-0006 (IPC), ADR-0007 (FTS), ADR-0008 (RAG/citas)
