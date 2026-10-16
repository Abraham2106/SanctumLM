# workers/documents — sidecar de ingesta (esqueleto OLA 3)

Proceso aislado para extracción de documentos según [ADR-0009](../../docs/03-architecture/adr/ADR-0009-ingestion.md). En esta fase: **stub** sin MarkItDown, Docling ni OCR.

## Checklist de aislamiento (ADR-0009)

| Control | Estado en este crate |
| --- | --- |
| Proceso separado del núcleo | Binario `documents-worker` invocable por spawn |
| Sin red en extracción | No hay crates ni sockets de red |
| Sin keychain / secretos | No lee credenciales ni variables sensibles |
| Entrada solo en staging | **PARCIAL** — `validate_staging_root` / `validate_staging_path` rechazan `..` y exigen prefijo bajo root normalizado; el extractor recibe solo `PathBuf` validado. Pendiente (tarea posterior): `canonicalize` bajo root acordado u `open` sin follow-symlink (junctions/symlinks Windows). Tests: `staging::tests` (`..` en root e ítem). |
| Salida JSON tipada | `IpcResult<IngestSidecarResponse>` (ver `src/ipc.rs`) |
| Reinicio aislado | Fallo de un ítem no aborta el lote (R-PROD-002) |

## IPC (stdin / stdout)

Un único JSON por invocación en **stdin**; una línea JSON en **stdout**:

```json
{
  "staging_root": "/path/to/staging",
  "items": [
    {
      "item_id": "file-1",
      "staging_path": "/path/to/staging/doc.txt",
      "mime": "text/plain",
      "max_bytes": 1000000
    }
  ]
}
```

Respuesta exitosa (forma `IpcResult`):

```json
{
  "ok": true,
  "data": {
    "items": [
      {
        "item_id": "file-1",
        "state": "completed",
        "output": {
          "normalized_text": "contenido-stub:text/plain",
          "chunks": [{ "text": "...", "page": 1, "char_start": 0, "char_end": 28 }],
          "warnings": [],
          "extraction_metadata": {
            "adapter_id": "stub-native",
            "adapter_version": "0.0.0-skeleton",
            "duration_ms": 0,
            "sidecar_exit_code": 0
          }
        }
      }
    ]
  }
}
```

Ítems rechazados (ruta fuera de staging, etc.) llevan `"state": "failed"` y `error_code` (`POLICY_DENIED`, `EXTRACTION_CORRUPT`, …) sin detener el resto del lote.

## Desarrollo

```text
cargo test -p documents-worker
cargo run -p documents-worker   # espera JSON en stdin
```

## Fuera de alcance (TASK-0024)

- Parsers PDF/DOCX/HTML reales
- Acceso a red, keychain, SQLite del núcleo
- Spawn desde Tauri (tarea de integración posterior)
