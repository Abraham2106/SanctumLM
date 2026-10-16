//! FTS, vectores, ranking y citas (ADR-0007; sqlite-vec en implementación posterior).
//!
//! Plan B: recuperación léxica vía FTS5 + [`StubVectorIndex`] sin `load_extension`.

pub mod fake;
pub mod fts;
pub mod search;
pub mod vector;

pub use fake::FakeChunkSearch;
pub use fts::FtsChunkSearch;
pub use search::{ChunkHit, ChunkSearch, SearchQuery};
pub use vector::{
    StubVectorIndex, VectorHit, VectorIndex, VectorQuery, VectorSearchResult,
};
