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
