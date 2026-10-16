# ADR-0007 — Persistencia SQLite, FTS5 e índice vectorial

- **Estado:** proposed
- **Fecha:** 2026-10-16
- **Alcance:** producto — capa de almacenamiento local (metadatos, texto, vectores, archivos)
- **Tarea:** TASK-0010
- **Vínculos:** R-PROD-001, R-PROD-007, R-PROD-103, R-PROD-105, R-PROD-111; WBS-3.2, WBS-4.1
- **Aprobación:** pendiente

## Contexto

La [investigación §4.1–§4.3](../../00-project/research/INVESTIGACION_NOTEBOOK_IA_LOCAL_HIBRIDO.md) recomienda SQLite embebido con **FTS5** para búsqueda léxica y **sqlite-vec** (pre-v1) para similitud vectorial, más un diseño integral de **cifrado en reposo** y claves fuera de la base de datos. SanctumLM es local-first, un usuario por instalación, con aislamiento por notebook (R-PROD-105), integridad versionada (R-PROD-103) y secretos solo en keychain (R-PROD-111).

Fuerzas en tensión:

- Simplicidad operativa (sin servidor de bases de datos) frente a escalabilidad futura (Qdrant/LanceDB pospuestos hasta medir).
- sqlite-vec acelera el MVP al compartir transacciones con FTS5, pero su API y empaquetado aún son inestables.
- Cifrar únicamente el archivo `.db` no protege originales, miniaturas, cachés ni temporales ([§4.1](../../00-project/research/INVESTIGACION_NOTEBOOK_IA_LOCAL_HIBRIDO.md)).
- `OPEN-010` sigue **abierta**: el nivel obligatorio de cifrado en MVP requiere decisión humana + revisión de security antes de implementar.

## Decisión propuesta

### 1. Motor relacional

Adoptar **SQLite 3** como única base de metadatos e índices en el MVP, con:

- **FTS5** obligatorio (`chunks_fts`) para recuperación léxica/BM25.
- **WAL** activado, `PRAGMA foreign_keys = ON`, migraciones versionadas y reversibles (criterio de resiliencia R-PROD-106).
- Un único archivo de base de datos por instalación (ruta de datos de la app), más directorio de **vault** para blobs de archivos.

### 2. Índice vectorial (sqlite-vec)

- Fijar el binding Rust **`sqlite-vec = 0.1.9`** enlazado **estáticamente / embebido** en el binario (`sqlite3_auto_extension` + símbolos del crate). **Prohibido en producto:** `load_extension` desde rutas de usuario, carpetas descargables o variables de entorno; solo el artefacto de build aprobado registra sqlite-vec. El spike TEMP usó `rusqlite` con feature `load_extension` únicamente para validar el crate en Windows — no es patrón de producción. Evidencia: [spike Windows](../../04-quality/evidence/spike-sqlite-vec-windows.md).
- **Rechazar por ahora** `0.1.10-alpha.4` en Windows (fallo de compilación del crate). Evidencia detallada en el mismo documento.
- Encapsular sqlite-vec detrás de un **`VectorIndex` trait** (nombre provisional) en `crates/retrieval/`: cargar extensión, crear tablas `vec0`, insertar/buscar, ocultar SQL específico y permitir sustituir el backend sin cambiar el núcleo RAG.
- **No mezclar embeddings de distintos modelos ni dimensiones en el mismo índice físico.** Cada espacio de embedding se identifica por `(model_id, dimension)` y materializa su propia tabla virtual `vec0` (o equivalente), creada bajo transacción al registrar un nuevo par modelo/dimensión. Las consultas semánticas deben incluir siempre el `embedding_space_id` derivado del modelo activo de indexación/consulta; cambiar el modelo implica reindexar en un espacio nuevo, no reutilizar el anterior.
- **LanceDB y Qdrant:** pospuestos; no forman parte del MVP. Si sqlite-vec deja de ser viable, la primera alternativa externa se evaluará con ADR dedicado y pruebas de portabilidad.

### 3. Plan B — FTS5-only

Si sqlite-vec falla en alguna plataforma objetivo o en CI antes de G2:

- Entregar recuperación **solo léxica** (FTS5 + ranking BM25) en MVP.
- Mantener el trait `VectorIndex` con implementación **no-op / stub** que registre «búsqueda semántica no disponible» y no bloquee importación ni chat con citas basadas en FTS.
- No almacenar vectores hasta que un backend vectorial aprobado esté operativo.

### 4. Esquema y migración

El esqueleto §4.2–§4.3 de la investigación se consolida en la **migración SQL 0001 propuesta** (texto en este ADR, no en el repositorio de crates hasta ADR `accepted` e implementación TASK dedicada). Incluye entidades de notebook, fuentes versionadas, chunks, FTS5, espacios de embedding, chat/citas/artefactos/jobs y auditoría mínima.

### 5. Cifrado en reposo (propuesta pendiente de OPEN-010)

Propuesta arquitectónica para revisión humana/security (no cierra OPEN-010):

| Activo | Tratamiento propuesto |
| --- | --- |
| Archivo SQLite | Cifrado en reposo vía **SQLCipher** (o equivalente auditado) con clave de base de datos (DEK) de 256 bits. |
| Originales importados (`vault/sources/…`) | Cifrado **por archivo** AES-256-GCM (o stream cifrado del SO) con DEK; metadatos en SQLite. |
| Miniaturas y cachés derivadas | Mismo DEK o subclave derivada (HKDF) por categoría; nunca plaintext en disco para clasificación `private`/`restricted`. |
| Temporales de ingestión/conversión | Escritura cifrada en subdirectorio efímero; **borrado seguro** al completar, fallar o cerrar la app. |
| Claves | **DEK** generada en instalación; **envuelta** y persistida solo en **keychain del SO** (Windows Credential Manager / DPAPI, macOS Keychain, Linux Secret Service). Rotación y recuperación: ADR/security posteriores. |
| Logs y SQLite | Prohibido registrar DEK, tokens, payloads de conectores o contenido de fuentes clasificadas. |

**Recomendación para el humano (OPEN-010):** en MVP, exigir cifrado en reposo **por defecto** para notebooks con clasificación `private` (originales + DB + cachés), con opción explícita documentada para importaciones `public`/demostración sin cifrado de archivos (solo metadatos), sujeta a confirmación en G1. Security debe validar SQLCipher vs. cifrado a nivel de archivos del SO en Windows primero (OPEN-009).

**Gate de implementación (OPEN-010):** hasta que `OPEN-010` se cierre con decisión humana + security **y** exista evidencia de **SQLCipher (o equivalente) coexistiendo con FTS5 y sqlite-vec embebido en Windows**, queda **prohibido** persistir en builds de usuario / candidatos G2 contenido clasificado `private` o `restricted` (filas en `sources`, blobs en vault, cachés derivadas). Excepción única: entornos de desarrollo con **datos sintéticos** explícitamente `public` y sin tokens reales. El esqueleto esquema, FTS5 sobre datos de prueba públicos y el spike vectorial siguen permitidos.

### 6. Aislamiento de recuperación (R-PROD-105) y chat general (R-PROD-007)

Toda recuperación — **FTS5 y búsqueda en tablas `vec0`** — debe aplicar el mismo filtro de autorización que [investigación §11.3](../../00-project/research/INVESTIGACION_NOTEBOOK_IA_LOCAL_HIBRIDO.md): `notebook_id` acotado, `selected_source_ids` validados previamente por el núcleo (lista JSON/expansión segura, nunca confiar en IDs del LLM) y `sources.status = 'ready'`. Los candidatos léxicos y vectoriales se obtienen **solo** mediante `JOIN` a `sources`/`source_versions`/`chunks` con esas condiciones; no consultar `chunks_fts` ni `vec0` en aislamiento sin revalidar pertenencia al notebook y a la selección.

Tras obtener un `chunk_id` (p. ej. vecino más cercano), **obligatorio** revalidar con el mismo patrón §11.3 antes de citar o devolver texto:

```sql
SELECT c.id, c.text, c.page, sv.id AS source_version_id
FROM chunks c
JOIN source_versions sv ON sv.id = c.source_version_id
JOIN sources s ON s.id = sv.source_id
WHERE s.notebook_id = :notebook_id
  AND s.id IN (SELECT value FROM json_each(:selected_source_ids))
  AND s.status = 'ready'
  AND c.id = :candidate_chunk_id;
```

La búsqueda vectorial puede ordenar por distancia dentro de un subconjunto ya filtrado (p. ej. `chunk_id IN (...)` derivado del JOIN anterior) o aplicar el JOIN en la consulta compuesta; en ambos casos, un `chunk_id` fuera de notebook/selección **no puede** aparecer en resultados finales.

**Prueba negativa obligatoria:** escenario **(e)** de §15 (dos notebooks con contenido similar/incompatible) debe fallar también en la rama **semántica** (vec0): ningún fragmento del notebook B en respuestas del notebook A, y viceversa.

**R-PROD-007 (chat general):** conversaciones con `chats.notebook_id IS NULL` **no** invocan FTS5 ni índices vectoriales; el motor de recuperación devuelve **cero fragmentos** de fuentes. Prohibido atar un chat general a consultas sobre `chunks`, `chunks_fts` o `vec0`.

### 7. Conectores y secretos (R-PROD-111)

- Tokens OAuth, refresh tokens y credenciales de conectores: **solo keychain del SO** (mismo principio que R-PROD-111).
- La tabla `connectors` almacena **metadatos** (`provider`, `scopes_json`, `policy_json`, identificadores no secretos). **Prohibido** persistir secretos en SQLite, en `settings_json` de `notebooks` o en logs.
- Referencias en SQLite: p. ej. `connector_id` + handle opaco al keychain; la app resuelve el token en memoria solo durante la operación autorizada.

## Alternativas evaluadas

| Opción | Pros | Contras | Veredicto |
| --- | --- | --- | --- |
| **SQLite + FTS5 + sqlite-vec 0.1.9** (propuesta) | Una transacción, backup simple, offline, alineado con investigación | Pre-v1, riesgo de empaquetado multiplataforma | **Propuesta MVP** tras spike Windows pass |
| **SQLite FTS5-only** | Menor riesgo de build, suficiente para citas léxicas tempranas | Sin semántica; KPI RAG degradado | **Plan B** integrado vía trait |
| **LanceDB / Qdrant embebido** | Escala vectorial, APIs estables | Segundo motor, tamaño, sync con SQLite | **Pospuesto** |
| **PostgreSQL / servidor local** | Concurrencia multiusuario | Contradice local-first simple | **Rechazado MVP** |
| **Cifrar solo SQLCipher** | Fácil de vender | Deja originales y temp en claro | **Insuficiente** (investigación §4.1) |

## Consecuencias

**Beneficios:** alineación con R-PROD-001/103/105; un solo backup lógico; pruebas de aislamiento SQL ([§11.3](../../00-project/research/INVESTIGACION_NOTEBOOK_IA_LOCAL_HIBRIDO.md)); spike Windows demuestra viabilidad técnica inmediata.

**Costos:** mantener capa de abstracción sobre sqlite-vec; pruebas de migración y de cifrado en tres SO; reindexado completo al cambiar modelo de embeddings.

**Riesgos residuales:**

- Regresión de crates `sqlite-vec` > 0.1.9 sin QA multiplataforma.
- Complejidad de SQLCipher + extensiones (FTS5 + vec) en un mismo archivo.
- Decisión OPEN-010 retrasada bloquea implementación de cifrado pero **no** el esqueleto esquema/FTS.

**Pruebas previstas al implementar:** migración 0001 idempotente; triggers FTS; escenario (e) aislamiento notebook **léxico y semántico**; chat general sin fragmentos; interrupción de ingestión; spike replicado en macOS/Linux; tests negativos de mezcla `model_id`/dimensión; gate OPEN-010 antes de persistir `private`/`restricted` en builds de usuario.

## Migración 0001 propuesta (SQL)

> **Estado:** propuesta de diseño en ADR; no es un archivo de migración del repositorio hasta aprobación e implementación en `crates/storage`.

```sql
-- sanctumlm migration 0001 (PROPUESTA) — metadatos + FTS5 + registro de espacios vectoriales
PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;

CREATE TABLE schema_migrations (
  version INTEGER PRIMARY KEY,
  applied_at TEXT NOT NULL
);

CREATE TABLE notebooks (
  id TEXT PRIMARY KEY,
  title TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  settings_json TEXT NOT NULL DEFAULT '{}'
);

CREATE TABLE sources (
  id TEXT PRIMARY KEY,
  notebook_id TEXT NOT NULL REFERENCES notebooks(id) ON DELETE CASCADE,
  kind TEXT NOT NULL,
  uri TEXT,
  sha256 TEXT NOT NULL,
  classification TEXT NOT NULL DEFAULT 'private',
  status TEXT NOT NULL,
  CHECK (classification IN ('public', 'private', 'restricted'))
);

CREATE INDEX idx_sources_notebook ON sources(notebook_id);

CREATE TABLE source_versions (
  id TEXT PRIMARY KEY,
  source_id TEXT NOT NULL REFERENCES sources(id) ON DELETE CASCADE,
  revision INTEGER NOT NULL,
  fetched_at TEXT NOT NULL,
  content_hash TEXT NOT NULL,
  storage_path TEXT NOT NULL,
  UNIQUE (source_id, revision)
);

CREATE TABLE chunks (
  id INTEGER PRIMARY KEY,
  source_version_id TEXT NOT NULL REFERENCES source_versions(id) ON DELETE CASCADE,
  ordinal INTEGER NOT NULL,
  page INTEGER,
  text TEXT NOT NULL,
  char_start INTEGER,
  char_end INTEGER
);

CREATE INDEX idx_chunks_source_version ON chunks(source_version_id, ordinal);

CREATE VIRTUAL TABLE chunks_fts USING fts5(
  text,
  content='chunks',
  content_rowid='id',
  tokenize='unicode61'
);

CREATE TRIGGER chunks_ai AFTER INSERT ON chunks BEGIN
  INSERT INTO chunks_fts(rowid, text) VALUES (new.id, new.text);
END;

CREATE TRIGGER chunks_ad AFTER DELETE ON chunks BEGIN
  INSERT INTO chunks_fts(chunks_fts, rowid, text) VALUES ('delete', old.id, old.text);
END;

CREATE TRIGGER chunks_au AFTER UPDATE OF text ON chunks BEGIN
  INSERT INTO chunks_fts(chunks_fts, rowid, text) VALUES ('delete', old.id, old.text);
  INSERT INTO chunks_fts(rowid, text) VALUES (new.id, new.text);
END;

-- Espacios de embedding: un índice vec0 por fila (model_id + dimension únicos)
CREATE TABLE embedding_spaces (
  id TEXT PRIMARY KEY,
  model_id TEXT NOT NULL,
  dimension INTEGER NOT NULL,
  vec_table TEXT NOT NULL UNIQUE,
  created_at TEXT NOT NULL,
  UNIQUE (model_id, dimension)
);

-- Tabla relacional opcional para auditoría / joins (vectores viven en vec0 asociado)
CREATE TABLE chunk_embeddings (
  chunk_id INTEGER NOT NULL REFERENCES chunks(id) ON DELETE CASCADE,
  embedding_space_id TEXT NOT NULL REFERENCES embedding_spaces(id) ON DELETE CASCADE,
  PRIMARY KEY (chunk_id, embedding_space_id)
);

-- Las tablas virtuales vec0 se crean dinámicamente, p. ej.:
-- CREATE VIRTUAL TABLE vec_space_<id> USING vec0(
--   chunk_id INTEGER PRIMARY KEY,
--   embedding float[384]
-- );
-- (dimensión fija por espacio; no compartir tabla entre modelos)

CREATE TABLE notes (
  id TEXT PRIMARY KEY,
  notebook_id TEXT NOT NULL REFERENCES notebooks(id) ON DELETE CASCADE,
  title TEXT NOT NULL,
  body TEXT NOT NULL,
  revision INTEGER NOT NULL DEFAULT 1,
  updated_at TEXT NOT NULL
);

CREATE TABLE chats (
  id TEXT PRIMARY KEY,
  notebook_id TEXT REFERENCES notebooks(id) ON DELETE CASCADE,
  mode TEXT NOT NULL,
  created_at TEXT NOT NULL
);

CREATE TABLE messages (
  id TEXT PRIMARY KEY,
  chat_id TEXT NOT NULL REFERENCES chats(id) ON DELETE CASCADE,
  role TEXT NOT NULL,
  body TEXT NOT NULL,
  model_id TEXT,
  created_at TEXT NOT NULL
);

CREATE TABLE citations (
  message_id TEXT NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
  chunk_id INTEGER NOT NULL REFERENCES chunks(id),
  quote TEXT NOT NULL,
  char_start INTEGER,
  char_end INTEGER,
  verification_status TEXT NOT NULL DEFAULT 'pending',
  PRIMARY KEY (message_id, chunk_id)
);

CREATE TABLE artifacts (
  id TEXT PRIMARY KEY,
  notebook_id TEXT NOT NULL REFERENCES notebooks(id) ON DELETE CASCADE,
  type TEXT NOT NULL,
  spec_json TEXT NOT NULL,
  status TEXT NOT NULL,
  revision INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE jobs (
  id TEXT PRIMARY KEY,
  notebook_id TEXT REFERENCES notebooks(id) ON DELETE SET NULL,
  type TEXT NOT NULL,
  state TEXT NOT NULL,
  progress REAL NOT NULL DEFAULT 0,
  error_code TEXT,
  updated_at TEXT NOT NULL
);

-- Metadatos únicamente; tokens OAuth viven en keychain del SO (R-PROD-111).
CREATE TABLE connectors (
  id TEXT PRIMARY KEY,
  provider TEXT NOT NULL,
  scopes_json TEXT NOT NULL DEFAULT '[]',
  policy_json TEXT NOT NULL DEFAULT '{}'
);

CREATE TABLE audit_events (
  id INTEGER PRIMARY KEY,
  action TEXT NOT NULL,
  destination TEXT,
  policy_decision TEXT,
  timestamp TEXT NOT NULL
);

CREATE INDEX idx_audit_timestamp ON audit_events(timestamp);
```

## Evidencia y aprobación

| Evidencia | Resultado |
| --- | --- |
| [Spike sqlite-vec Windows](../../04-quality/evidence/spike-sqlite-vec-windows.md) | **pass** con `sqlite-vec 0.1.9` |
| `0.1.10-alpha.4` en mismo entorno | **fail** (include ausente) |
| `python scripts/validate_docs.py` | Ejecutar en TASK-0010; se espera aviso de ADR-0007 sin índice hasta integración planner |

Aprobación humana en G1 requerida antes de marcar `accepted`, implementar migraciones en código o fijar dependencias en el workspace.

## Sustitución

Ninguna. Complementa la recomendación de stack del CHARTER e investigación §4. Primer ADR de producto sobre datos; no supersede ADR de proceso (0001–0005).
