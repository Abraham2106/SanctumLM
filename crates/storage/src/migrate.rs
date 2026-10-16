use rusqlite::{Connection, OptionalExtension};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

const MIGRATION_0001: &str = include_str!("../migrations/0001_notebooks_sources_chunks_fts.sql");
const MIGRATION_0002: &str = include_str!("../migrations/0002_chats_messages_citations.sql");

const TARGET_VERSION: i64 = 2;

/// Aplica migraciones pendientes (0001). Idempotente: no reejecuta una versión ya registrada.
pub fn migrate_to_latest(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    apply_if_pending(conn, 1, MIGRATION_0001)?;
    apply_if_pending(conn, 2, MIGRATION_0002)?;
    Ok(())
}

fn apply_if_pending(conn: &Connection, version: i64, sql: &str) -> rusqlite::Result<()> {
    if migration_applied(conn, version)? {
        return Ok(());
    }
    conn.execute_batch(sql)?;
    let applied_at = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    conn.execute(
        "INSERT INTO schema_migrations (version, applied_at) VALUES (?1, ?2)",
        (version, applied_at.as_str()),
    )?;
    Ok(())
}

fn migration_applied(conn: &Connection, version: i64) -> rusqlite::Result<bool> {
    let exists: i64 = conn.query_row(
        "SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name = 'schema_migrations'",
        [],
        |row| row.get(0),
    )?;
    if exists == 0 {
        return Ok(false);
    }

    conn.query_row(
        "SELECT version FROM schema_migrations WHERE version = ?1",
        [version],
        |row| row.get::<_, i64>(0),
    )
    .optional()
    .map(|v| v.is_some())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn open_test_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("in-memory db");
        conn.execute_batch("PRAGMA foreign_keys = ON;")
            .expect("foreign keys");
        conn
    }

    fn fts_match_count(conn: &Connection, term: &str) -> i64 {
        conn.query_row(
            "SELECT count(*) FROM chunks_fts WHERE chunks_fts MATCH ?1",
            [term],
            |row| row.get(0),
        )
        .expect("fts query")
    }

    /// Inserta notebook → source → source_version → chunk; devuelve `chunk.id`.
    fn insert_public_chunk(conn: &Connection, text: &str) -> i64 {
        let now = OffsetDateTime::now_utc()
            .format(&Rfc3339)
            .expect("rfc3339");

        conn.execute(
            "INSERT INTO notebooks (id, title, created_at, updated_at) VALUES (?1, ?2, ?3, ?3)",
            ("nb-1", "Demo", now.as_str()),
        )
        .expect("notebook");
        conn.execute(
            "INSERT INTO sources (id, notebook_id, kind, sha256, classification, status) \
             VALUES (?1, ?2, 'file', 'abc', 'public', 'ready')",
            ("src-1", "nb-1"),
        )
        .expect("source");
        conn.execute(
            "INSERT INTO source_versions (id, source_id, revision, fetched_at, content_hash, storage_path) \
             VALUES (?1, ?2, 1, ?3, 'hash', 'vault/demo')",
            ("sv-1", "src-1", now.as_str()),
        )
        .expect("source_version");
        conn.execute(
            "INSERT INTO chunks (source_version_id, ordinal, text) VALUES (?1, 0, ?2)",
            ("sv-1", text),
        )
        .expect("chunk");

        conn.last_insert_rowid()
    }

    #[test]
    fn migration_0001_is_idempotent() {
        let conn = open_test_conn();
        migrate_to_latest(&conn).expect("first migrate");
        migrate_to_latest(&conn).expect("second migrate");

        let version: i64 = conn
            .query_row(
                "SELECT count(*) FROM schema_migrations WHERE version = 1",
                [],
                |row| row.get(0),
            )
            .expect("schema_migrations row");
        assert_eq!(version, 1);
    }

    #[test]
    fn migration_0001_creates_core_tables() {
        let conn = open_test_conn();
        migrate_to_latest(&conn).expect("migrate");

        for table in [
            "notebooks",
            "sources",
            "source_versions",
            "chunks",
            "chunks_fts",
        ] {
            let count: i64 = conn
                .query_row(
                    "SELECT count(*) FROM sqlite_master WHERE type IN ('table','virtual table') AND name = ?1",
                    [table],
                    |row| row.get(0),
                )
                .unwrap_or_else(|_| panic!("query {table}"));
            assert_eq!(count, 1, "missing table {table}");
        }
    }

    #[test]
    fn fts_smoke_insert_and_match() {
        let conn = open_test_conn();
        migrate_to_latest(&conn).expect("migrate");

        insert_public_chunk(&conn, "contenido lexico sanctum");
        assert_eq!(fts_match_count(&conn, "lexico"), 1);
    }

    #[test]
    fn fts_update_trigger_reindexes_text() {
        let conn = open_test_conn();
        migrate_to_latest(&conn).expect("migrate");

        let chunk_id = insert_public_chunk(&conn, "tokenviejo lexico");
        assert_eq!(fts_match_count(&conn, "tokenviejo"), 1);

        conn.execute(
            "UPDATE chunks SET text = ?1 WHERE id = ?2",
            ("tokennuevo lexico", chunk_id),
        )
        .expect("update chunk");

        assert_eq!(fts_match_count(&conn, "tokenviejo"), 0);
        assert_eq!(fts_match_count(&conn, "tokennuevo"), 1);
    }

    #[test]
    fn fts_delete_trigger_removes_index_row() {
        let conn = open_test_conn();
        migrate_to_latest(&conn).expect("migrate");

        let chunk_id = insert_public_chunk(&conn, "tokenborrar lexico");
        assert_eq!(fts_match_count(&conn, "tokenborrar"), 1);

        conn.execute("DELETE FROM chunks WHERE id = ?1", [chunk_id])
            .expect("delete chunk");

        assert_eq!(fts_match_count(&conn, "tokenborrar"), 0);
    }
}
