//! Orquestación RAG léxica: contexto FTS, abstención y chat general sin recuperación.

use core_domain::{NotebookId, SourceId};
use serde::{Deserialize, Serialize};

use crate::search::{ChunkHit, ChunkSearch, SearchQuery};

/// Mensaje canónico cuando FTS no devuelve evidencia (preparación R-PROD-006).
pub const ABSTENTION_INSUFFICIENT_EVIDENCE: &str =
    "No hay evidencia suficiente en las fuentes seleccionadas.";

/// Consulta de recuperación para armar el paquete de evidencia del chat notebook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RagQuery {
    pub notebook_id: Option<NotebookId>,
    pub question: String,
    pub selected_source_ids: Vec<SourceId>,
    pub max_chunks: usize,
}

/// Fragmento autorizado incluido en el contexto del modelo.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextBlock {
    pub chunk_id: String,
    pub source_id: String,
    pub source_version_id: String,
    pub text: String,
    pub score: f32,
}

impl From<ChunkHit> for ContextBlock {
    fn from(hit: ChunkHit) -> Self {
        Self {
            chunk_id: hit.chunk_id.0,
            source_id: hit.source_id.0,
            source_version_id: hit.source_version_id,
            text: hit.quote,
            score: hit.score,
        }
    }
}

/// Resultado de recuperación léxica.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RagOutcome {
    /// Chat general (`notebook_id` ausente): cero fragmentos (R-PROD-007).
    GeneralChatNoContext,
    /// Notebook sin hits FTS: abstener con mensaje explícito.
    Abstain { message: String },
    /// Contexto listo para el router / prompt.
    Context { blocks: Vec<ContextBlock> },
}

/// Orquestador FTS-first (Plan B: sin sqlite-vec productivo).
pub struct LexicalRag<'a, S: ChunkSearch + ?Sized> {
    search: &'a S,
    default_limit: usize,
}

impl<'a, S: ChunkSearch + ?Sized> LexicalRag<'a, S> {
    pub fn new(search: &'a S) -> Self {
        Self {
            search,
            default_limit: 8,
        }
    }

    pub fn with_default_limit(search: &'a S, default_limit: usize) -> Self {
        Self {
            search,
            default_limit,
        }
    }

    pub fn retrieve(&self, query: RagQuery) -> RagOutcome {
        let Some(notebook_id) = query.notebook_id else {
            return RagOutcome::GeneralChatNoContext;
        };

        let limit = if query.max_chunks == 0 {
            self.default_limit
        } else {
            query.max_chunks
        };

        let hits = self.search.search(SearchQuery {
            notebook_id: Some(notebook_id),
            query: query.question,
            selected_source_ids: query.selected_source_ids,
            limit,
        });

        if hits.is_empty() {
            return RagOutcome::Abstain {
                message: ABSTENTION_INSUFFICIENT_EVIDENCE.into(),
            };
        }

        RagOutcome::Context {
            blocks: hits.into_iter().map(ContextBlock::from).collect(),
        }
    }
}

/// Serializa bloques para inyectar en el prompt del modelo.
pub fn format_context_prompt(blocks: &[ContextBlock]) -> String {
    blocks
        .iter()
        .enumerate()
        .map(|(i, b)| {
            format!(
                "[{idx}] chunk_id={cid} source_id={sid} source_version_id={svid}\n{text}",
                idx = i + 1,
                cid = b.chunk_id,
                sid = b.source_id,
                svid = b.source_version_id,
                text = b.text
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::FakeChunkSearch;
    use core_domain::ChunkId;

    #[test]
    fn general_chat_returns_no_context() {
        let search = FakeChunkSearch::with_static_hits(vec![]);
        let rag = LexicalRag::new(&search);
        let outcome = rag.retrieve(RagQuery {
            notebook_id: None,
            question: "agua".into(),
            selected_source_ids: vec![],
            max_chunks: 5,
        });
        assert_eq!(outcome, RagOutcome::GeneralChatNoContext);
    }

    #[test]
    fn empty_hits_abstain() {
        let search = FakeChunkSearch::with_static_hits(vec![]);
        let rag = LexicalRag::new(&search);
        let outcome = rag.retrieve(RagQuery {
            notebook_id: Some(NotebookId("nb-1".into())),
            question: "agua".into(),
            selected_source_ids: vec![],
            max_chunks: 5,
        });
        assert_eq!(
            outcome,
            RagOutcome::Abstain {
                message: ABSTENTION_INSUFFICIENT_EVIDENCE.into()
            }
        );
    }

    #[test]
    fn hits_become_context_blocks() {
        let search = FakeChunkSearch::with_static_hits(vec![ChunkHit {
            chunk_id: ChunkId("42".into()),
            source_id: SourceId("src-1".into()),
            source_version_id: "sv-1".into(),
            quote: "evaporacion del agua".into(),
            score: 1.0,
        }]);
        let rag = LexicalRag::new(&search);
        let outcome = rag.retrieve(RagQuery {
            notebook_id: Some(NotebookId("nb-1".into())),
            question: "evaporacion".into(),
            selected_source_ids: vec![SourceId("src-1".into())],
            max_chunks: 3,
        });
        match outcome {
            RagOutcome::Context { blocks } => {
                assert_eq!(blocks.len(), 1);
                assert_eq!(blocks[0].chunk_id, "42");
            }
            other => panic!("expected context, got {other:?}"),
        }
    }
}
