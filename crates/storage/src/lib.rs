//! SQLite, cifrado, migraciones y vault de archivos (ADR-0007; implementación en C1).
//!
//! Producto: `rusqlite` con SQLite embebido (`bundled`); **sin** `load_extension` ni SQLCipher
//! hasta cierre de OPEN-010.

pub mod memory;
pub mod migrate;
pub mod repository;
pub mod sqlite;
pub mod sqlite_notebook;

pub use memory::InMemoryNotebookRepository;
pub use migrate::migrate_to_latest;
pub use repository::{NotebookMeta, NotebookRepository};
pub use sqlite::{notebook_repository, open_connection, open_memory_connection};
pub use sqlite_notebook::SqliteNotebookRepository;
