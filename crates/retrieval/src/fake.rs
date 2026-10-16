use crate::search::{ChunkHit, ChunkSearch, SearchQuery};

/// Buscador determinista para tests (no RAG real).
pub struct FakeChunkSearch {
    hits: Vec<ChunkHit>,
}

impl FakeChunkSearch {
    pub fn with_static_hits(hits: Vec<ChunkHit>) -> Self {
        Self { hits }
    }
}

impl ChunkSearch for FakeChunkSearch {
    fn search(&self, query: SearchQuery) -> Vec<ChunkHit> {
        self.hits
            .iter()
            .filter(|h| {
                query.selected_source_ids.is_empty()
                    || query
                        .selected_source_ids
                        .iter()
                        .any(|s| s.0 == h.source_id.0)
            })
            .filter(|h| {
                h.quote
                    .to_lowercase()
                    .contains(&query.query.to_lowercase())
            })
            .take(query.limit)
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_domain::{ChunkId, NotebookId, SourceId};

    #[test]
    fn filters_by_source_scope() {
        let search = FakeChunkSearch::with_static_hits(vec![
            ChunkHit {
                chunk_id: ChunkId("c1".into()),
                source_id: SourceId("s1".into()),
                quote: "agua evaporación".into(),
                score: 1.0,
            },
            ChunkHit {
                chunk_id: ChunkId("c2".into()),
                source_id: SourceId("s2".into()),
                quote: "agua lluvia".into(),
                score: 0.9,
            },
        ]);
        let hits = search.search(SearchQuery {
            notebook_id: NotebookId("nb-1".into()),
            query: "agua".into(),
            selected_source_ids: vec![SourceId("s1".into())],
            limit: 10,
        });
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].chunk_id.0, "c1");
    }
}
