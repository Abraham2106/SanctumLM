-- sanctumlm migration 0003 — jobs durables (ADR-0007 §jobs)
PRAGMA foreign_keys = ON;

CREATE TABLE jobs (
  id TEXT PRIMARY KEY,
  notebook_id TEXT REFERENCES notebooks(id) ON DELETE SET NULL,
  type TEXT NOT NULL,
  state TEXT NOT NULL,
  progress REAL NOT NULL DEFAULT 0,
  error_code TEXT,
  updated_at TEXT NOT NULL
);

CREATE INDEX idx_jobs_notebook ON jobs(notebook_id);
CREATE INDEX idx_jobs_state ON jobs(state);
