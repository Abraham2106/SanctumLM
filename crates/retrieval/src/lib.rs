//! FTS, vectores, ranking y citas (ADR-0007; sqlite-vec en implementación posterior).
//!
//! Plan B: recuperación léxica vía FTS5 + [`StubVectorIndex`] sin `load_extension`.

pub mod citations;
pub mod fake;
pub mod fts;
pub mod rag;
pub mod search;
pub mod vector;

pub use citations::{
    extract_citation_proposals, CitationProposal, CitationScope, CitationValidator,
    CitationVerification, ValidatedCitation,
};
pub use fake::FakeChunkSearch;
pub use fts::FtsChunkSearch;
pub use rag::{
    format_context_prompt, LexicalRag, RagOutcome, RagQuery, ABSTENTION_INSUFFICIENT_EVIDENCE,
    ContextBlock,
};
pub use search::{ChunkHit, ChunkSearch, SearchQuery};
pub use vector::{
    StubVectorIndex, VectorHit, VectorIndex, VectorQuery, VectorSearchResult,
};
