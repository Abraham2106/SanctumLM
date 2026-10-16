//! SQLite, cifrado, migraciones y vault de archivos (ADR-0007; implementación en C1).

pub mod memory;
pub mod repository;

pub use memory::InMemoryNotebookRepository;
pub use repository::{NotebookMeta, NotebookRepository};
