use std::sync::{Arc, Mutex};

use core_domain::{ChunkId, NotebookId, SourceId};
use rusqlite::{params, Connection};

#[derive(Debug, Clone)]
pub struct FtsHit {
    pub chunk_id: ChunkId,
    pub source_id: SourceId,
    pub text: String,
    pub page: Option<i32>,
    pub char_start: Option<i32>,
    pub char_end: Option<i32>,
    pub rank: f32,
}

pub fn search_sources_fts(
    conn: &Arc<Mutex<Connection>>,
    notebook_id: &NotebookId,
    query: &str,
    selected_source_ids: &[SourceId],
    limit: usize,
) -> Vec<FtsHit> {
    let Some(match_expr) = fts_match_expression(query) else {
        return Vec::new();
    };
    if limit == 0 {
        return Vec::new();
    }

    let source_filter_json = selected_sources_json(selected_source_ids);
    let limit_i = i64::try_from(limit).unwrap_or(i64::MAX);

    let sql = if selected_source_ids.is_empty() {
        "SELECT c.id, s.id, c.text, c.page, c.char_start, c.char_end, bm25(chunks_fts) AS rank \
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
        "SELECT c.id, s.id, c.text, c.page, c.char_start, c.char_end, bm25(chunks_fts) AS rank \
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

    let conn = conn.lock().expect("sqlite lock");
    let mut stmt = match conn.prepare(sql) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };

    let map_row = |row: &rusqlite::Row<'_>| {
        let chunk_id: i64 = row.get(0)?;
        let source_id: String = row.get(1)?;
        let text: String = row.get(2)?;
        let page: Option<i32> = row.get(3)?;
        let char_start: Option<i32> = row.get(4)?;
        let char_end: Option<i32> = row.get(5)?;
        let rank: f64 = row.get(6)?;
        Ok(FtsHit {
            chunk_id: ChunkId(chunk_id.to_string()),
            source_id: SourceId(source_id),
            text,
            page,
            char_start,
            char_end,
            rank: (-rank) as f32,
        })
    };

    if selected_source_ids.is_empty() {
        stmt.query_map(params![match_expr, notebook_id.0, limit_i], map_row)
    } else {
        stmt.query_map(
            params![match_expr, notebook_id.0, source_filter_json, limit_i],
            map_row,
        )
    }
    .into_iter()
    .flatten()
    .filter_map(Result::ok)
    .collect()
}

fn selected_sources_json(ids: &[SourceId]) -> String {
    let raw: Vec<&str> = ids.iter().map(|s| s.0.as_str()).collect();
    serde_json::to_string(&raw).unwrap_or_else(|_| "[]".into())
}

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
