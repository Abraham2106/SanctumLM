use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use storage::{
    open_memory_connection, InMemoryJobStore, SqliteNotebookRepository, SqliteSourceRepository,
};

use crate::import_service::ImportRuntime;

pub struct AppState {
    pub notebooks: Mutex<SqliteNotebookRepository>,
    pub sources: SqliteSourceRepository,
    pub jobs: Arc<InMemoryJobStore>,
    pub import_runtime: ImportRuntime,
    pub conn: Arc<Mutex<Connection>>,
}

impl AppState {
    pub fn in_memory() -> Self {
        let conn = open_memory_connection().expect("open memory db");
        let arc = Arc::new(Mutex::new(conn));
        let staging = std::env::temp_dir().join("sanctumlm-staging-test");
        let _ = std::fs::create_dir_all(&staging);
        Self::from_connection(Arc::clone(&arc), staging, None)
    }

    pub fn from_connection(
        conn: Arc<Mutex<Connection>>,
        staging_root: PathBuf,
        documents_worker: Option<PathBuf>,
    ) -> Self {
        Self {
            notebooks: Mutex::new(SqliteNotebookRepository::new(Arc::clone(&conn))),
            sources: SqliteSourceRepository::new(Arc::clone(&conn)),
            jobs: Arc::new(InMemoryJobStore::new()),
            import_runtime: ImportRuntime::new(
                Arc::clone(&conn),
                staging_root,
                documents_worker,
            ),
            conn,
        }
    }
}
