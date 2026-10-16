//! Búsqueda FTS5 con filtro de autorización ADR-0007 §6 (JOIN sources).

use std::sync::{Arc, Mutex};

use core_domain::{ChunkId, SourceId};
use rusqlite::{params, Connection};
use storage::open_memory_connection;

use crate::search::{ChunkHit, ChunkSearch, SearchQuery};

/// Motor léxico sobre `chunks_fts` con revalidación por notebook y fuentes listas.
pub struct FtsChunkSearch {
    conn: Arc<Mutex<Connection>>,
}

impl FtsChunkSearch {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    /// Conexión en memoria con migración 0001 (tests y fixtures).
    pub fn open_in_memory() -> rusqlite::Result<Self> {
        Ok(Self::new(open_memory_connection()?))
    }

    pub fn connection(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.conn)
    }
}

impl ChunkSearch for FtsChunkSearch {
    fn search(&self, query: SearchQuery) -> Vec<ChunkHit> {
        let Some(notebook_id) = query.notebook_id else {
            return Vec::new();
        };
        let Some(match_expr) = fts_match_expression(&query.query) else {
            return Vec::new();
        };
        if query.limit == 0 {
            return Vec::new();
        }

        let source_filter_json = selected_sources_json(&query.selected_source_ids);
        let limit = i64::try_from(query.limit).unwrap_or(i64::MAX);

        let sql = if query.selected_source_ids.is_empty() {
            "SELECT c.id, s.id, sv.id, c.text, bm25(chunks_fts) AS rank \
             FROM chunks_fts \
             JOIN chunks c ON c.id = chunks_fts.rowid \
             JOIN source_versions sv ON sv.id = c.source_version_id \
             JOIN sources s ON s.id = sv.source_id \
             WHERE chunks_fts MATCH ?1 \
               AND s.notebook_id = ?2 \
               AND s.status = 'ready' \
             ORDER BY rank \
             LIMIT ?3"
        } else {
            "SELECT c.id, s.id, sv.id, c.text, bm25(chunks_fts) AS rank \
             FROM chunks_fts \
             JOIN chunks c ON c.id = chunks_fts.rowid \
             JOIN source_versions sv ON sv.id = c.source_version_id \
             JOIN sources s ON s.id = sv.source_id \
             WHERE chunks_fts MATCH ?1 \
               AND s.notebook_id = ?2 \
               AND s.status = 'ready' \
               AND s.id IN (SELECT value FROM json_each(?3)) \
             ORDER BY rank \
             LIMIT ?4"
        };

        let conn = self.conn.lock().expect("sqlite lock");
        let mut stmt = match conn.prepare(sql) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        let map_row = |row: &rusqlite::Row<'_>| {
            let chunk_id: i64 = row.get(0)?;
            let source_id: String = row.get(1)?;
            let source_version_id: String = row.get(2)?;
            let text: String = row.get(3)?;
            let rank: f64 = row.get(4)?;
            Ok(ChunkHit {
                chunk_id: ChunkId(chunk_id.to_string()),
                source_id: SourceId(source_id),
                source_version_id,
                quote: text,
                score: (-rank) as f32,
            })
        };

        if query.selected_source_ids.is_empty() {
            stmt.query_map(params![match_expr, notebook_id.0, limit], map_row)
        } else {
            stmt.query_map(
                params![match_expr, notebook_id.0, source_filter_json, limit],
                map_row,
            )
        }
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .collect()
    }
}

fn selected_sources_json(ids: &[SourceId]) -> String {
    let raw: Vec<&str> = ids.iter().map(|s| s.0.as_str()).collect();
    serde_json::to_string(&raw).unwrap_or_else(|_| "[]".into())
}

/// Tokeniza la consulta del usuario en términos FTS5 AND escapados.
fn fts_match_expression(raw: &str) -> Option<String> {
    let tokens: Vec<String> = raw
        .split_whitespace()
        .filter(|t| !t.is_empty())
        .map(|t| {
            let escaped = t.replace('"', "\"\"");
            format!("\"{escaped}\"")
        })
        .collect();
    if tokens.is_empty() {
        None
    } else {
        Some(tokens.join(" AND "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_domain::NotebookId;

    const FIXTURE_TS: &str = "2026-01-01T00:00:00Z";

    fn insert_chunk(
        conn: &Connection,
        notebook_id: &str,
        source_id: &str,
        status: &str,
        text: &str,
    ) {
        conn.execute(
            "INSERT OR IGNORE INTO notebooks (id, title, created_at, updated_at) \
             VALUES (?1, ?2, ?3, ?3)",
            (notebook_id, notebook_id, FIXTURE_TS),
        )
        .expect("notebook");
        conn.execute(
            "INSERT INTO sources (id, notebook_id, kind, sha256, classification, status) \
             VALUES (?1, ?2, 'file', 'abc', 'public', ?3)",
            (source_id, notebook_id, status),
        )
        .expect("source");
        let sv_id = format!("sv-{source_id}");
        conn.execute(
            "INSERT INTO source_versions (id, source_id, revision, fetched_at, content_hash, storage_path) \
             VALUES (?1, ?2, 1, ?3, 'hash', 'vault/demo')",
            (sv_id.as_str(), source_id, FIXTURE_TS),
        )
        .expect("source_version");
        conn.execute(
            "INSERT INTO chunks (source_version_id, ordinal, text) VALUES (?1, 0, ?2)",
            (sv_id.as_str(), text),
        )
        .expect("chunk");
    }

    #[test]
    fn fts_returns_matching_chunk_for_notebook_and_source() {
        let search = FtsChunkSearch::open_in_memory().expect("db");
        {
            let conn = search.conn.lock().expect("lock");
            insert_chunk(
                &conn,
                "nb-a",
                "src-a",
                "ready",
                "evaporacion del agua dulce",
            );
        }

        let hits = search.search(SearchQuery {
            notebook_id: Some(NotebookId("nb-a".into())),
            query: "evaporacion".into(),
            selected_source_ids: vec![SourceId("src-a".into())],
            limit: 10,
        });

        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].source_id.0, "src-a");
        assert!(hits[0].quote.contains("evaporacion"));
    }

    #[test]
    fn scenario_e_no_cross_notebook_leakage() {
        let search = FtsChunkSearch::open_in_memory().expect("db");
        {
            let conn = search.conn.lock().expect("lock");
            insert_chunk(
                &conn,
                "nb-a",
                "src-a",
                "ready",
                "evaporacion cuenca atlantico",
            );
            insert_chunk(
                &conn,
                "nb-b",
                "src-b",
                "ready",
                "evaporacion cuenca pacifico",
            );
        }

        let hits = search.search(SearchQuery {
            notebook_id: Some(NotebookId("nb-a".into())),
            query: "evaporacion".into(),
            selected_source_ids: vec![SourceId("src-a".into())],
            limit: 10,
        });

        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].source_id.0, "src-a");
        assert!(!hits[0].quote.contains("pacifico"));
    }

    #[test]
    fn general_chat_without_notebook_returns_empty() {
        let search = FtsChunkSearch::open_in_memory().expect("db");
        {
            let conn = search.conn.lock().expect("lock");
            insert_chunk(&conn, "nb-a", "src-a", "ready", "evaporacion agua");
        }

        let hits = search.search(SearchQuery {
            notebook_id: None,
            query: "evaporacion".into(),
            selected_source_ids: vec![SourceId("src-a".into())],
            limit: 10,
        });

        assert!(hits.is_empty());
    }

    #[test]
    fn non_ready_sources_are_excluded() {
        let search = FtsChunkSearch::open_in_memory().expect("db");
        {
            let conn = search.conn.lock().expect("lock");
            insert_chunk(
                &conn,
                "nb-a",
                "src-a",
                "processing",
                "evaporacion pendiente",
            );
        }

        let hits = search.search(SearchQuery {
            notebook_id: Some(NotebookId("nb-a".into())),
            query: "evaporacion".into(),
            selected_source_ids: vec![SourceId("src-a".into())],
            limit: 10,
        });

        assert!(hits.is_empty());
    }
}
