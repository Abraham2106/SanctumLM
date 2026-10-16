use core_domain::{DataClassification, NotebookId, ProductError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookMeta {
    pub id: NotebookId,
    pub title: String,
    pub classification: DataClassification,
}

pub trait NotebookRepository: Send + Sync {
    fn create(&mut self, title: String, classification: DataClassification) -> NotebookMeta;

    fn get(&self, id: &NotebookId) -> Result<NotebookMeta, ProductError>;

    fn list(&self) -> Vec<NotebookMeta>;
}
