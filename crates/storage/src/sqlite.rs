use std::path::Path;
use std::sync::{Arc, Mutex};

use rusqlite::Connection;

use crate::migrate::migrate_to_latest;
use crate::sqlite_notebook::SqliteNotebookRepository;

/// Abre una base en disco con WAL y migraciones aplicadas (sin SQLCipher; OPEN-010).
pub fn open_connection(path: impl AsRef<Path>) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")?;
    migrate_to_latest(&conn)?;
    Ok(conn)
}

/// Conexión en memoria para tests (sin WAL).
pub fn open_memory_connection() -> rusqlite::Result<Connection> {
    let conn = Connection::open_in_memory()?;
    migrate_to_latest(&conn)?;
    Ok(conn)
}

/// Repositorio de notebooks respaldado por SQLite compartiendo una conexión mutex.
pub fn notebook_repository(
    conn: Connection,
) -> rusqlite::Result<SqliteNotebookRepository> {
    migrate_to_latest(&conn)?;
    Ok(SqliteNotebookRepository::new(Arc::new(Mutex::new(conn))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn open_connection_enables_wal_and_foreign_keys() {
        let dir = std::env::temp_dir().join(format!(
            "sanctumlm-storage-test-{}",
            uuid::Uuid::new_v4()
        ));
        fs::create_dir_all(&dir).expect("temp dir");
        let path = dir.join("test.db");

        let conn = open_connection(&path).expect("open_connection");
        let fk: i64 = conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .expect("foreign_keys pragma");
        let journal_mode: String = conn
            .query_row("PRAGMA journal_mode", [], |row| row.get(0))
            .expect("journal_mode pragma");

        assert_eq!(fk, 1);
        assert_eq!(journal_mode.to_lowercase(), "wal");

        drop(conn);
        let _ = fs::remove_dir_all(&dir);
    }
}
