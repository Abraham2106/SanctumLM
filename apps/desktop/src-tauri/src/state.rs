use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use ai_router::config::InferenceConfig;
use ai_router::{FakeModelAdapter, ModelRouter, OllamaLoopbackAdapter};
use retrieval::citations::CitationValidator;
use retrieval::search::ChunkSearch;
use rusqlite::Connection;
use storage::{
    open_memory_connection, SqliteChatRepository, SqliteJobRepository, SqliteNotebookRepository,
    SqliteSourceRepository,
};

use crate::chunk_search::StorageChunkSearch;
use crate::chat_service::ChatRuntime;
use crate::import_service::ImportRuntime;

pub struct AppState {
    pub notebooks: Mutex<SqliteNotebookRepository>,
    pub sources: SqliteSourceRepository,
    pub jobs: SqliteJobRepository,
    pub import_runtime: ImportRuntime,
    pub conn: Arc<Mutex<Connection>>,
    pub chat: ChatRuntime,
}

impl AppState {
    pub fn in_memory() -> Self {
        let conn = open_memory_connection().expect("open memory db");
        let arc = Arc::new(Mutex::new(conn));
        let staging = std::env::temp_dir().join("sanctumlm-staging-test");
        let _ = std::fs::create_dir_all(&staging);
        Self::from_connection_with_router(
            Arc::clone(&arc),
            staging,
            None,
            ModelRouter::local_only(FakeModelAdapter::ollama_loopback_stub()),
            Arc::new(StorageChunkSearch::new(Arc::clone(&arc))),
        )
    }

    pub fn from_connection(
        conn: Arc<Mutex<Connection>>,
        staging_root: PathBuf,
        documents_worker: Option<PathBuf>,
    ) -> Self {
        let router = production_router();
        let search: Arc<dyn ChunkSearch + Send + Sync> =
            Arc::new(StorageChunkSearch::new(Arc::clone(&conn)));
        Self::from_connection_with_router(conn, staging_root, documents_worker, router, search)
    }

    pub fn from_connection_with_router(
        conn: Arc<Mutex<Connection>>,
        staging_root: PathBuf,
        documents_worker: Option<PathBuf>,
        router: ModelRouter,
        search: Arc<dyn ChunkSearch + Send + Sync>,
    ) -> Self {
        let chat_repo = SqliteChatRepository::new(Arc::clone(&conn));
        let citations = CitationValidator::from_shared(Arc::clone(&conn));
        Self {
            notebooks: Mutex::new(SqliteNotebookRepository::new(Arc::clone(&conn))),
            sources: SqliteSourceRepository::new(Arc::clone(&conn)),
            jobs: SqliteJobRepository::new(Arc::clone(&conn)),
            import_runtime: ImportRuntime::new(
                Arc::clone(&conn),
                staging_root,
                documents_worker,
            ),
            chat: ChatRuntime {
                router: Mutex::new(router),
                search,
                chats: chat_repo,
                citations,
            },
            conn,
        }
    }
}

fn production_router() -> ModelRouter {
    match OllamaLoopbackAdapter::from_config(&InferenceConfig::default()) {
        Ok(adapter) => ModelRouter::local_only(adapter),
        Err(_) => ModelRouter::local_only(FakeModelAdapter::ollama_loopback_stub()),
    }
}
