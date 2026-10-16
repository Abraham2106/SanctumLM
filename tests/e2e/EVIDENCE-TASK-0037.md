# Evidencia TASK-0037 — import → FTS → chat notebook

**Estado TC-PROD:** preparación ejecutada a nivel núcleo; casos formales siguen **not-run** para G2. **No** declara G2 ni 26/26 `R-PROD`.

## Comando canónico

```bash
./scripts/run-e2e-vertical-slice.sh
```

## Precondiciones

- Rust toolchain estable (≥ 1.85 recomendado; Tauri 2.12+ requiere Cargo reciente).
- Linux: librerías GTK/WebKit para compilar `sanctumlm-desktop` (mismo requisito que `cargo test -p sanctumlm-desktop`).
- Corpus versionado en `benchmarks/es-mvp-rag/sources/` (solo lectura).

## Pasos automatizados (núcleo IPC)

1. Crear notebook en SQLite (memoria de prueba).
2. Importar texto del manifiesto `01-politica-vacaciones.md` como `inline_text` y esperar job `completed`.
3. Comprobar fuente `ready` y prefijo SHA-256.
4. `search_sources` FTS con consulta acotada al notebook.
5. `chat_send` modo `notebook` con adaptador fake que emite JSON de cita; validar citas persistidas.
6. (Adicional) Aislamiento FTS entre notebooks A/B del benchmark §15(e).
7. (Adicional) Chat `general` sin invocar FTS (coherencia R-PROD-007 parcial).

## Resultado esperado

```
test e2e_task0037_import_fts_notebook_chat ... ok
test e2e_task0037_notebook_fts_isolation ... ok
test e2e_task0037_general_chat_skips_notebook_fts ... ok
```

Trazas `[e2e-0037]` en stderr con `notebook_id`, `source_id`, `fts_hits`, `citations`.

## Matriz parcial R-PROD

| R-PROD | Evidencia en esta tarea | Límite conocido |
| --- | --- | --- |
| 001 | `notebook_create` + listados | Sin reinicio de app ni CRUD completo |
| 002 | Import inline → `ready` + hash | Sin corruptos ni sidecar archivo |
| 006 | Chat notebook + citas validadas (fake LLM) | Sin batería `questions.json` ni Ollama |
| 105 | FTS no cruza `notebook_id` | Dos notebooks en misma DB de prueba |

## Registro de ejecución (plantilla)

| Campo | Valor |
| --- | --- |
| task-id | TASK-0037 |
| commit | _(rellenar tras merge)_ |
| comando | `./scripts/run-e2e-vertical-slice.sh` |
| resultado | pass / fail / not-run |
| log | `tests/e2e/artifacts/e2e-vertical-slice-*.log` |
| ejecutor | _(agente/humano)_ |
| fecha UTC | _(ISO-8601)_ |
