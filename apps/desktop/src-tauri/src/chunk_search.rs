use std::sync::{Arc, Mutex};

use retrieval::search::{ChunkHit, ChunkSearch, SearchQuery};
use rusqlite::Connection;
use storage::search_sources_fts;

pub struct StorageChunkSearch {
    conn: Arc<Mutex<Connection>>,
}

impl StorageChunkSearch {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

impl ChunkSearch for StorageChunkSearch {
    fn search(&self, query: SearchQuery) -> Vec<ChunkHit> {
        let Some(notebook_id) = query.notebook_id else {
            return Vec::new();
        };
        search_sources_fts(
            &self.conn,
            &notebook_id,
            &query.query,
            &query.selected_source_ids,
            query.limit,
        )
        .into_iter()
        .map(|hit| ChunkHit {
            chunk_id: hit.chunk_id,
            source_id: hit.source_id,
            source_version_id: hit.source_version_id,
            quote: hit.text,
            score: hit.rank,
        })
        .collect()
    }
}
