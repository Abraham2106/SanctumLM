# Benchmarks — corpus y evaluación RAG (MVP)

Conjuntos de documentos **sintéticos o de dominio público** para medir importación, citas, abstención y casos adversariales. No incluye datos personales ni contenido con licencia restrictiva.

## Conjunto principal (`es-mvp-rag/`)

Corpus en **español** alineado con la investigación §5.3 y la batería §15:

| Artefacto | Descripción |
| --- | --- |
| `es-mvp-rag/manifest.json` | Manifiesto machine-readable: `id`, idioma, etiquetas, SHA-256 |
| `es-mvp-rag/sources/` | 10–20 fuentes en PDF, TXT, MD, DOCX, HTML |
| `es-mvp-rag/evaluation/questions.json` | Preguntas con evidencia esperada, imposibles (abstención) y aislamiento notebook |

### Uso previsto

1. Importar el lote mixto (`sources/`) en un notebook de prueba y comprobar hash, estado de extracción y error específico en archivos `corrupt-*` (R-PROD-002).
2. Ejecutar preguntas de `evaluation/questions.json` contra el chat con fuentes; verificar citas a `source_id`/segmento y abstención en ítems `impossible` (R-PROD-006, R-PROD-104).
3. Para escenario §15(e): cargar `notebook-a/` y `notebook-b/` en notebooks distintos; las preguntas `isolation_*` no deben filtrar evidencia cruzada (R-PROD-105).
4. Tratar `05-injection-*.txt` y HTML importado como **datos no confiables**; no ejecutar scripts embebidos (R-PROD-111).

### Métricas (§5.3)

Medir de forma auxiliar: `citation_precision`, `citation_recall`, coincidencia de hechos soportados, calidad de abstención y latencia. La revisión humana sigue siendo la fuente de verdad para alucinaciones y citas.

### Licencia del corpus

Textos marcados como **sintéticos originales** para SanctumLM. Tablas y cifras son ficticias. No descargar modelos ni usar APIs de pago para regenerar este paquete.
