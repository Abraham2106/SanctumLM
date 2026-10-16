-- sanctumlm migration 0002 — chats, messages, citations (ADR-0007 §Migración chats)
PRAGMA foreign_keys = ON;

CREATE TABLE chats (
  id TEXT PRIMARY KEY,
  notebook_id TEXT REFERENCES notebooks(id) ON DELETE CASCADE,
  mode TEXT NOT NULL CHECK (mode IN ('general', 'notebook')),
  title TEXT NOT NULL DEFAULT '',
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE INDEX idx_chats_notebook ON chats(notebook_id);
CREATE INDEX idx_chats_mode ON chats(mode);

CREATE TABLE messages (
  id TEXT PRIMARY KEY,
  chat_id TEXT NOT NULL REFERENCES chats(id) ON DELETE CASCADE,
  role TEXT NOT NULL CHECK (role IN ('system', 'user', 'assistant')),
  body TEXT NOT NULL,
  model_id TEXT,
  created_at TEXT NOT NULL
);

CREATE INDEX idx_messages_chat ON messages(chat_id, created_at);

CREATE TABLE citations (
  message_id TEXT NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
  chunk_id INTEGER NOT NULL REFERENCES chunks(id),
  source_version_id TEXT NOT NULL,
  quote TEXT NOT NULL,
  verification_status TEXT NOT NULL DEFAULT 'unverified',
  PRIMARY KEY (message_id, chunk_id)
);
