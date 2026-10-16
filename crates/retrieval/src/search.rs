use core_domain::{ChunkId, NotebookId, SourceId};
use serde::{Deserialize, Serialize};

/// Resultado de búsqueda léxica (BM25 vía FTS5).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChunkHit {
    pub chunk_id: ChunkId,
    pub source_id: SourceId,
    pub source_version_id: String,
    pub quote: String,
    pub score: f32,
}

/// Consulta de recuperación acotada a un notebook (`None` = chat general, R-PROD-007).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchQuery {
    pub notebook_id: Option<NotebookId>,
    pub query: String,
    pub selected_source_ids: Vec<SourceId>,
    pub limit: usize,
}

pub trait ChunkSearch: Send + Sync {
    fn search(&self, query: SearchQuery) -> Vec<ChunkHit>;
}
