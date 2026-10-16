//! Validación de citas propuestas por el modelo contra chunks autorizados (R-PROD-104/105).

use std::sync::{Arc, Mutex};

use core_domain::{ChunkId, NotebookId, SourceId};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

/// Estados alineados con contratos IPC (`verification`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationVerification {
    Backed,
    Inference,
    Discrepancy,
    Unverified,
}

/// Cita extraída del texto del modelo (no confiable hasta validar).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationProposal {
    pub chunk_id: ChunkId,
    pub source_version_id: Option<String>,
    pub quote: String,
}

/// Cita tras comprobación determinista en SQLite.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatedCitation {
    pub chunk_id: ChunkId,
    pub source_id: SourceId,
    pub source_version_id: String,
    pub quote: String,
    pub verification: CitationVerification,
}

/// Alcance de autorización fijado por el núcleo (no el LLM).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CitationScope {
    pub notebook_id: NotebookId,
    pub selected_source_ids: Vec<SourceId>,
}

/// Valida propuestas contra el patrón §11.3 (JOIN sources / versiones / chunks).
pub struct CitationValidator {
    conn: Arc<Mutex<Connection>>,
}

impl CitationValidator {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    pub fn from_shared(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    pub fn validate(&self, scope: &CitationScope, proposals: &[CitationProposal]) -> Vec<ValidatedCitation> {
        proposals
            .iter()
            .filter_map(|p| self.validate_one(scope, p))
            .collect()
    }

    fn validate_one(
        &self,
        scope: &CitationScope,
        proposal: &CitationProposal,
    ) -> Option<ValidatedCitation> {
        let chunk_key = parse_chunk_rowid(&proposal.chunk_id)?;
        let authorized = self.fetch_authorized_chunk(scope, chunk_key)?;

        if let Some(expected_sv) = &proposal.source_version_id {
            if expected_sv != &authorized.source_version_id {
                return Some(ValidatedCitation {
                    chunk_id: proposal.chunk_id.clone(),
                    source_id: authorized.source_id,
                    source_version_id: authorized.source_version_id,
                    quote: proposal.quote.clone(),
                    verification: CitationVerification::Discrepancy,
                });
            }
        }

        let verification = if quote_matches_chunk(&proposal.quote, &authorized.text) {
            CitationVerification::Backed
        } else {
            CitationVerification::Discrepancy
        };

        let quote = if verification == CitationVerification::Backed {
            extract_matching_quote(&proposal.quote, &authorized.text)
                .unwrap_or_else(|| proposal.quote.clone())
        } else {
            proposal.quote.clone()
        };

        Some(ValidatedCitation {
            chunk_id: ChunkId(authorized.chunk_id),
            source_id: authorized.source_id,
            source_version_id: authorized.source_version_id,
            quote,
            verification,
        })
    }

    fn fetch_authorized_chunk(
        &self,
        scope: &CitationScope,
        chunk_rowid: i64,
    ) -> Option<AuthorizedRow> {
        let source_filter_json = selected_sources_json(&scope.selected_source_ids);
        let sql = if scope.selected_source_ids.is_empty() {
            "SELECT c.id, c.text, sv.id, s.id \
             FROM chunks c \
             JOIN source_versions sv ON sv.id = c.source_version_id \
             JOIN sources s ON s.id = sv.source_id \
             WHERE s.notebook_id = ?1 \
               AND s.status = 'ready' \
               AND c.id = ?2"
        } else {
            "SELECT c.id, c.text, sv.id, s.id \
             FROM chunks c \
             JOIN source_versions sv ON sv.id = c.source_version_id \
             JOIN sources s ON s.id = sv.source_id \
             WHERE s.notebook_id = ?1 \
               AND s.status = 'ready' \
               AND s.id IN (SELECT value FROM json_each(?2)) \
               AND c.id = ?3"
        };

        let conn = self.conn.lock().expect("sqlite lock");
        if scope.selected_source_ids.is_empty() {
            conn.query_row(
                sql,
                params![scope.notebook_id.0, chunk_rowid],
                map_authorized_row,
            )
            .ok()
        } else {
            conn.query_row(
                sql,
                params![scope.notebook_id.0, source_filter_json, chunk_rowid],
                map_authorized_row,
            )
            .ok()
        }
    }
}

struct AuthorizedRow {
    chunk_id: String,
    text: String,
    source_version_id: String,
    source_id: SourceId,
}

fn map_authorized_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<AuthorizedRow> {
    let chunk_id: i64 = row.get(0)?;
    let text: String = row.get(1)?;
    let source_version_id: String = row.get(2)?;
    let source_id: String = row.get(3)?;
    Ok(AuthorizedRow {
        chunk_id: chunk_id.to_string(),
        text,
        source_version_id,
        source_id: SourceId(source_id),
    })
}

fn parse_chunk_rowid(id: &ChunkId) -> Option<i64> {
    id.0.parse().ok()
}

fn selected_sources_json(ids: &[SourceId]) -> String {
    let raw: Vec<&str> = ids.iter().map(|s| s.0.as_str()).collect();
    serde_json::to_string(&raw).unwrap_or_else(|_| "[]".into())
}

fn normalize_ws(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn quote_matches_chunk(proposed: &str, chunk_text: &str) -> bool {
    let p = normalize_ws(proposed);
    let c = normalize_ws(chunk_text);
    if p.is_empty() {
        return false;
    }
    c.to_lowercase().contains(&p.to_lowercase())
}

fn extract_matching_quote(proposed: &str, chunk_text: &str) -> Option<String> {
    let p = normalize_ws(proposed);
    if p.is_empty() {
        return None;
    }
    if chunk_text.contains(&p) {
        return Some(p);
    }
    let lower_chunk = chunk_text.to_lowercase();
    let lower_p = p.to_lowercase();
    let start = lower_chunk.find(&lower_p)?;
    Some(chunk_text[start..start + p.len()].to_string())
}

/// Extrae propuestas de cita embebidas en JSON del modelo (`chunk_id` + `quote`).
pub fn extract_citation_proposals(model_text: &str) -> Vec<CitationProposal> {
    let mut out = Vec::new();
    for segment in model_text.split('{') {
        if !segment.contains("chunk_id") {
            continue;
        }
        let Some(chunk_id) = extract_json_string_field(segment, "chunk_id")
            .or_else(|| extract_json_number_field(segment, "chunk_id"))
        else {
            continue;
        };
        let quote = extract_json_string_field(segment, "quote").unwrap_or_default();
        let source_version_id = extract_json_string_field(segment, "source_version_id");
        out.push(CitationProposal {
            chunk_id: ChunkId(chunk_id),
            source_version_id,
            quote,
        });
    }
    out
}

fn extract_json_string_field(segment: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{key}\"");
    let after_key = segment.split(&pattern).nth(1)?;
    let rest = after_key.split(':').nth(1)?.trim_start();
    if !rest.starts_with('"') {
        return None;
    }
    let mut value = String::new();
    let mut escaped = false;
    for ch in rest[1..].chars() {
        if escaped {
            value.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == '"' {
            break;
        }
        value.push(ch);
    }
    Some(value)
}

fn extract_json_number_field(segment: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{key}\"");
    let key_pos = segment.find(&pattern)?;
    let after_key = &segment[key_pos + pattern.len()..];
    let colon = after_key.find(':')?;
    let rest = after_key[colon + 1..].trim_start();
    let num: String = rest
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    if num.is_empty() {
        None
    } else {
        Some(num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fts::FtsChunkSearch;
    use core_domain::NotebookId;

    const FIXTURE_TS: &str = "2026-01-01T00:00:00Z";

    fn seed_chunk(conn: &Connection, notebook: &str, source: &str, text: &str) -> i64 {
        conn.execute(
            "INSERT OR IGNORE INTO notebooks (id, title, created_at, updated_at) \
             VALUES (?1, ?1, ?2, ?2)",
            (notebook, FIXTURE_TS),
        )
        .expect("notebook");
        conn.execute(
            "INSERT INTO sources (id, notebook_id, kind, sha256, classification, status) \
             VALUES (?1, ?2, 'file', 'abc', 'public', 'ready')",
            (source, notebook),
        )
        .expect("source");
        let sv_id = format!("sv-{source}");
        conn.execute(
            "INSERT INTO source_versions (id, source_id, revision, fetched_at, content_hash, storage_path) \
             VALUES (?1, ?2, 1, ?3, 'hash', 'vault/demo')",
            (sv_id.as_str(), source, FIXTURE_TS),
        )
        .expect("sv");
        conn.execute(
            "INSERT INTO chunks (source_version_id, ordinal, text) VALUES (?1, 0, ?2)",
            (sv_id.as_str(), text),
        )
        .expect("chunk");
        conn.last_insert_rowid()
    }

    #[test]
    fn invented_chunk_id_is_omitted() {
        let search = FtsChunkSearch::open_in_memory().expect("db");
        let conn_arc = search.connection();
        let chunk_id = {
            let conn = conn_arc.lock().expect("lock");
            seed_chunk(&conn, "nb-a", "src-a", "evaporacion cuenca atlantico")
        };
        let validator = CitationValidator::from_shared(conn_arc);
        let validated = validator.validate(
            &CitationScope {
                notebook_id: NotebookId("nb-a".into()),
                selected_source_ids: vec![SourceId("src-a".into())],
            },
            &[
                CitationProposal {
                    chunk_id: ChunkId("999999".into()),
                    source_version_id: None,
                    quote: "evaporacion".into(),
                },
                CitationProposal {
                    chunk_id: ChunkId(chunk_id.to_string()),
                    source_version_id: None,
                    quote: "evaporacion cuenca".into(),
                },
            ],
        );
        assert_eq!(validated.len(), 1);
        assert_eq!(validated[0].verification, CitationVerification::Backed);
    }

    #[test]
    fn wrong_notebook_chunk_rejected() {
        let search = FtsChunkSearch::open_in_memory().expect("db");
        let conn_arc = search.connection();
        let chunk_b = {
            let conn = conn_arc.lock().expect("lock");
            seed_chunk(&conn, "nb-b", "src-b", "evaporacion pacifico")
        };
        let validator = CitationValidator::from_shared(conn_arc);
        let validated = validator.validate(
            &CitationScope {
                notebook_id: NotebookId("nb-a".into()),
                selected_source_ids: vec![SourceId("src-a".into())],
            },
            &[CitationProposal {
                chunk_id: ChunkId(chunk_b.to_string()),
                source_version_id: None,
                quote: "evaporacion".into(),
            }],
        );
        assert!(validated.is_empty());
    }

    #[test]
    fn quote_mismatch_is_discrepancy() {
        let search = FtsChunkSearch::open_in_memory().expect("db");
        let conn_arc = search.connection();
        let chunk_id = {
            let conn = conn_arc.lock().expect("lock");
            seed_chunk(&conn, "nb-a", "src-a", "evaporacion cuenca atlantico")
        };
        let validator = CitationValidator::from_shared(conn_arc);
        let validated = validator.validate(
            &CitationScope {
                notebook_id: NotebookId("nb-a".into()),
                selected_source_ids: vec![],
            },
            &[CitationProposal {
                chunk_id: ChunkId(chunk_id.to_string()),
                source_version_id: None,
                quote: "texto inventado".into(),
            }],
        );
        assert_eq!(validated.len(), 1);
        assert_eq!(validated[0].verification, CitationVerification::Discrepancy);
    }

    #[test]
    fn extract_proposals_from_json_blob() {
        let text = r#"Respuesta breve. {"chunk_id": "7", "source_version_id": "sv-1", "quote": "agua dulce"}"#;
        let proposals = extract_citation_proposals(text);
        assert_eq!(proposals.len(), 1);
        assert_eq!(proposals[0].chunk_id.0, "7");
        assert_eq!(proposals[0].quote, "agua dulce");
    }
}
