//! FTS, vectores, ranking y citas (ADR-0007; sqlite-vec en implementación posterior).

pub mod fake;
pub mod search;

pub use fake::FakeChunkSearch;
pub use search::{ChunkHit, ChunkSearch, SearchQuery};
