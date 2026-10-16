//! Índice vectorial — Plan B (ADR-0007 §3): stub hasta sqlite-vec en build de producto.

use core_domain::{ChunkId, NotebookId, SourceId};
use serde::{Deserialize, Serialize};

/// Consulta semántica (mismos filtros de autorización que la rama léxica).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorQuery {
    pub notebook_id: Option<NotebookId>,
    pub embedding_space_id: String,
    pub limit: usize,
    /// Vector de consulta; ignorado por el stub.
    pub query_vector: Vec<f32>,
    pub selected_source_ids: Vec<SourceId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorHit {
    pub chunk_id: ChunkId,
    pub source_id: SourceId,
    pub distance: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorSearchResult {
    pub hits: Vec<VectorHit>,
    /// `false` en Plan B: búsqueda semántica no disponible (sin panic ni load_extension).
    pub semantic_available: bool,
}

/// Abstracción sobre sqlite-vec / backends futuros (ADR-0007 §2).
pub trait VectorIndex: Send + Sync {
    fn search(&self, query: VectorQuery) -> VectorSearchResult;
}

/// Implementación no-op: importación y chat con citas FTS siguen operativos.
#[derive(Debug, Default, Clone, Copy)]
pub struct StubVectorIndex;

impl VectorIndex for StubVectorIndex {
    fn search(&self, query: VectorQuery) -> VectorSearchResult {
        if query.notebook_id.is_none() {
            return VectorSearchResult {
                hits: Vec::new(),
                semantic_available: false,
            };
        }
        VectorSearchResult {
            hits: Vec::new(),
            semantic_available: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stub_does_not_panic_and_reports_unavailable() {
        let index = StubVectorIndex;
        let result = index.search(VectorQuery {
            notebook_id: Some(NotebookId("nb-1".into())),
            embedding_space_id: "ollama:nomic".into(),
            limit: 5,
            query_vector: vec![0.1, 0.2],
            selected_source_ids: vec![SourceId("src-1".into())],
        });
        assert!(result.hits.is_empty());
        assert!(!result.semantic_available);
    }

    #[test]
    fn stub_general_chat_returns_empty() {
        let index = StubVectorIndex;
        let result = index.search(VectorQuery {
            notebook_id: None,
            embedding_space_id: "space".into(),
            limit: 5,
            query_vector: vec![],
            selected_source_ids: vec![],
        });
        assert!(result.hits.is_empty());
    }
}
