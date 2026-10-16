use core_domain::{DataClassification, NotebookId, ProductError};

use crate::repository::{NotebookMeta, NotebookRepository};

/// Repositorio in-memory para tests de contrato (sin SQLite real).
pub struct InMemoryNotebookRepository {
    notebooks: Vec<NotebookMeta>,
}

impl InMemoryNotebookRepository {
    pub fn new() -> Self {
        Self {
            notebooks: Vec::new(),
        }
    }
}

impl Default for InMemoryNotebookRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl NotebookRepository for InMemoryNotebookRepository {
    fn create(&mut self, title: String, classification: DataClassification) -> NotebookMeta {
        let id = NotebookId(format!("nb-{}", self.notebooks.len() + 1));
        let meta = NotebookMeta {
            id: id.clone(),
            title,
            classification,
        };
        self.notebooks.push(meta.clone());
        meta
    }

    fn get(&self, id: &NotebookId) -> Result<NotebookMeta, ProductError> {
        self.notebooks
            .iter()
            .find(|n| &n.id == id)
            .cloned()
            .ok_or_else(|| ProductError::not_found("notebook"))
    }

    fn list(&self) -> Vec<NotebookMeta> {
        self.notebooks.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_domain::ProductErrorCode;

    #[test]
    fn create_and_get_notebook() {
        let mut repo = InMemoryNotebookRepository::new();
        let created = repo.create(
            "Estudio".into(),
            DataClassification::Private,
        );
        let fetched = repo.get(&created.id).unwrap();
        assert_eq!(fetched.title, "Estudio");
    }

    #[test]
    fn missing_notebook_is_not_found() {
        let repo = InMemoryNotebookRepository::new();
        let err = repo
            .get(&NotebookId("missing".into()))
            .unwrap_err();
        assert_eq!(err.code, ProductErrorCode::NotFound);
    }
}
