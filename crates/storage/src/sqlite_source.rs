use std::sync::{Arc, Mutex};

use core_domain::{
    DataClassification, NotebookId, ProductError, ProductErrorCode, SourceId, SourceVersionId,
};
use rusqlite::{params, Connection};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::source_meta::{
    classification_from_db, classification_to_db, decode_source_uri, encode_source_uri,
    kind_from_db, kind_to_db, sha256_prefix, status_from_db, status_to_db, SourceKind, SourceMeta,
    SourceStatus,
};

#[derive(Debug, Clone)]
pub struct PersistedChunk {
    pub text: String,
    pub page: Option<i32>,
    pub char_start: Option<i32>,
    pub char_end: Option<i32>,
}

pub struct SqliteSourceRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteSourceRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    fn now_rfc3339() -> Result<String, ProductError> {
        OffsetDateTime::now_utc()
            .format(&Rfc3339)
            .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "timestamp"))
    }

    pub fn list_by_notebook(&self, notebook_id: &NotebookId) -> Result<Vec<SourceMeta>, ProductError> {
        let conn = self.conn.lock().expect("sqlite lock");
        let mut stmt = conn
            .prepare(
                "SELECT s.id, s.notebook_id, s.kind, s.uri, s.sha256, s.classification, s.status, \
                 COALESCE((
                   SELECT sv.fetched_at FROM source_versions sv \
                   WHERE sv.source_id = s.id ORDER BY sv.revision DESC LIMIT 1
                 ), '') AS updated_at \
                 FROM sources s \
                 WHERE s.notebook_id = ?1 \
                 ORDER BY updated_at DESC, s.id",
            )
            .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "list sources"))?;

        let rows = stmt
            .query_map([notebook_id.0.as_str()], |row| {
                let id: String = row.get(0)?;
                let notebook_id: String = row.get(1)?;
                let kind: String = row.get(2)?;
                let uri: Option<String> = row.get(3)?;
                let sha256: String = row.get(4)?;
                let classification: String = row.get(5)?;
                let status: String = row.get(6)?;
                let updated_at: String = row.get(7)?;
                let (display_name, mime) =
                    decode_source_uri(uri.as_deref(), &id);
                Ok(SourceMeta {
                    source_id: SourceId(id),
                    notebook_id: NotebookId(notebook_id),
                    display_name,
                    kind: kind_from_db(&kind),
                    mime,
                    status: status_from_db(&status),
                    classification: classification_from_db(&classification),
                    sha256_prefix: sha256_prefix(&sha256),
                    updated_at,
                })
            })
            .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "list sources"))?;

        Ok(rows.filter_map(Result::ok).collect())
    }

    pub fn find_ready_by_sha256(
        &self,
        notebook_id: &NotebookId,
        sha256: &str,
    ) -> Result<Option<SourceId>, ProductError> {
        let conn = self.conn.lock().expect("sqlite lock");
        let found = conn
            .query_row(
                "SELECT id FROM sources \
                 WHERE notebook_id = ?1 AND sha256 = ?2 AND status = 'ready' \
                 LIMIT 1",
                params![notebook_id.0.as_str(), sha256],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "dedupe lookup"))?;

        Ok(found.map(SourceId))
    }

    pub fn insert_processing_source(
        &self,
        notebook_id: &NotebookId,
        display_name: &str,
        mime: Option<&str>,
        sha256: &str,
        classification: DataClassification,
    ) -> Result<SourceId, ProductError> {
        let source_id = SourceId(Uuid::new_v4().to_string());
        let uri = encode_source_uri(display_name, mime);
        let conn = self.conn.lock().expect("sqlite lock");
        conn.execute(
            "INSERT INTO sources (id, notebook_id, kind, uri, sha256, classification, status) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                source_id.0.as_str(),
                notebook_id.0.as_str(),
                kind_to_db(SourceKind::Paste),
                uri.as_str(),
                sha256,
                classification_to_db(classification),
                status_to_db(SourceStatus::Processing),
            ],
        )
        .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "insert source"))?;
        Ok(source_id)
    }

    pub fn persist_ready_version(
        &self,
        source_id: &SourceId,
        content_hash: &str,
        storage_path: &str,
        chunks: &[PersistedChunk],
    ) -> Result<SourceVersionId, ProductError> {
        let now = Self::now_rfc3339()?;
        let version_id = SourceVersionId(Uuid::new_v4());
        let conn = self.conn.lock().expect("sqlite lock");

        let tx = conn
            .unchecked_transaction()
            .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "tx"))?;

        tx.execute(
            "INSERT INTO source_versions (id, source_id, revision, fetched_at, content_hash, storage_path) \
             VALUES (?1, ?2, \
               COALESCE((SELECT MAX(revision) FROM source_versions WHERE source_id = ?2), 0) + 1, \
               ?3, ?4, ?5)",
            params![
                version_id.0.to_string(),
                source_id.0.as_str(),
                now.as_str(),
                content_hash,
                storage_path,
            ],
        )
        .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "insert version"))?;

        for (ordinal, chunk) in chunks.iter().enumerate() {
            tx.execute(
                "INSERT INTO chunks (source_version_id, ordinal, page, text, char_start, char_end) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    version_id.0.to_string(),
                    ordinal as i64,
                    chunk.page,
                    chunk.text.as_str(),
                    chunk.char_start,
                    chunk.char_end,
                ],
            )
            .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "insert chunk"))?;
        }

        tx.execute(
            "UPDATE sources SET status = ?1 WHERE id = ?2",
            params![status_to_db(SourceStatus::Ready), source_id.0.as_str()],
        )
        .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "update source"))?;

        tx.commit()
            .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "commit"))?;

        Ok(version_id)
    }

    pub fn mark_failed(&self, source_id: &SourceId) -> Result<(), ProductError> {
        let conn = self.conn.lock().expect("sqlite lock");
        conn.execute(
            "UPDATE sources SET status = ?1 WHERE id = ?2",
            params![status_to_db(SourceStatus::Failed), source_id.0.as_str()],
        )
        .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "mark failed"))?;
        Ok(())
    }

    pub fn count_sources_by_sha256(
        &self,
        notebook_id: &NotebookId,
        sha256: &str,
        status: SourceStatus,
    ) -> Result<i64, ProductError> {
        let conn = self.conn.lock().expect("sqlite lock");
        conn.query_row(
            "SELECT count(*) FROM sources \
             WHERE notebook_id = ?1 AND sha256 = ?2 AND status = ?3",
            params![
                notebook_id.0.as_str(),
                sha256,
                status_to_db(status),
            ],
            |row| row.get(0),
        )
        .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "count sources"))
    }
}

use rusqlite::OptionalExtension;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::migrate::migrate_to_latest;
    use crate::repository::NotebookRepository;
    use crate::sqlite_notebook::SqliteNotebookRepository;

    fn fixture_repo() -> (SqliteSourceRepository, NotebookId) {
        let conn = Connection::open_in_memory().expect("conn");
        migrate_to_latest(&conn).expect("migrate");
        let arc = Arc::new(Mutex::new(conn));
        let mut notebooks = SqliteNotebookRepository::new(Arc::clone(&arc));
        let nb = notebooks.create("Test".into(), DataClassification::Public);
        (SqliteSourceRepository::new(arc), nb.id)
    }

    #[test]
    fn ready_dedupe_lookup_finds_existing() {
        let (repo, nb) = fixture_repo();
        let hash = "deadbeef".repeat(8);
        let id = repo
            .insert_processing_source(&nb, "a.txt", Some("text/plain"), &hash, DataClassification::Public)
            .unwrap();
        repo.persist_ready_version(
            &id,
            &hash,
            "staging/a.txt",
            &[PersistedChunk {
                text: "hola lexico".into(),
                page: None,
                char_start: Some(0),
                char_end: Some(11),
            }],
        )
        .unwrap();

        let dup = repo.find_ready_by_sha256(&nb, &hash).unwrap();
        assert_eq!(dup, Some(id));
    }

    #[test]
    fn retry_same_hash_does_not_duplicate_ready_rows() {
        let (repo, nb) = fixture_repo();
        let hash = "abc123".repeat(10);
        let id1 = repo
            .insert_processing_source(&nb, "doc.md", Some("text/markdown"), &hash, DataClassification::Public)
            .unwrap();
        repo.persist_ready_version(
            &id1,
            &hash,
            "staging/doc.md",
            &[PersistedChunk {
                text: "evaporacion agua".into(),
                page: None,
                char_start: Some(0),
                char_end: Some(16),
            }],
        )
        .unwrap();

        let existing = repo.find_ready_by_sha256(&nb, &hash).unwrap();
        assert_eq!(existing, Some(id1.clone()));

        let ready_count = repo
            .count_sources_by_sha256(&nb, &hash, SourceStatus::Ready)
            .unwrap();
        assert_eq!(ready_count, 1);
    }
}
