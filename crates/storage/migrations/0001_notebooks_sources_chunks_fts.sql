-- sanctumlm migration 0001 — notebooks, sources, chunks, FTS5 (ADR-0007 §Migración 0001)
PRAGMA foreign_keys = ON;

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
