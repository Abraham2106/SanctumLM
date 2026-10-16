use core_domain::ProductError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebSearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
}

pub trait PublicWebSearch: Send + Sync {
    fn search(&self, query: &str, limit: usize) -> Result<Vec<WebSearchResult>, ProductError>;
}
