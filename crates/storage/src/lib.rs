//! SQLite, cifrado, migraciones y vault de archivos (ADR-0007; implementación en C1).
//!
//! Producto: `rusqlite` con SQLite embebido (`bundled`); **sin** `load_extension` ni SQLCipher
//! hasta cierre de OPEN-010.

pub mod import_inline;
pub mod job_memory;
pub mod memory;
pub mod migrate;
pub mod repository;
pub mod source_meta;
pub mod sqlite;
pub mod sqlite_notebook;
pub mod sqlite_fts;
pub mod sqlite_source;

pub use import_inline::{chunk_inline_text, sha256_hex, validate_inline_size};
pub use job_memory::{parse_job_id, CancelFlag, InMemoryJobStore};
pub use memory::InMemoryNotebookRepository;
pub use migrate::migrate_to_latest;
pub use repository::{NotebookMeta, NotebookRepository};
pub use source_meta::{SourceMeta, SourceStatus};
pub use sqlite::{notebook_repository, open_connection, open_memory_connection};
pub use sqlite_notebook::SqliteNotebookRepository;
pub use sqlite_fts::{search_sources_fts, FtsHit};
pub use sqlite_source::{PersistedChunk, SqliteSourceRepository};
