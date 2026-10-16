# workers/documents — ingesta aislada

Sidecar restringido para conversión de documentos (Docling / MarkItDown), según investigación §3.3 y ADR-0009.

## Aislamiento (OLA 2 esqueleto)

- Este paquete es un **stub**: no descarga ni ejecuta Docling ni otros conversores.
- La ingesta real correrá en proceso separado con límites de red, filesystem y tiempo; el núcleo Rust encola trabajos y consume resultados por IPC/cola.
- No añadir dependencias de conversión hasta una tarea de implementación explícita con revisión de security.

## Ejecución local (desarrollo)

```text
cargo run -p documents-worker
```

Salida esperada: mensaje de stub en stderr; código de salida 0.
