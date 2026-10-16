use std::sync::{Arc, Mutex};

use core_domain::{DataClassification, NotebookId, ProductError, ProductErrorCode};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::repository::{NotebookMeta, NotebookRepository};

#[derive(Debug, Serialize, Deserialize)]
struct NotebookSettings {
    #[serde(default)]
    classification: Option<DataClassification>,
}

pub struct SqliteNotebookRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteNotebookRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    fn now_rfc3339() -> Result<String, ProductError> {
        OffsetDateTime::now_utc().format(&Rfc3339).map_err(|_| {
            ProductError::new(ProductErrorCode::LocalUnavailable, "timestamp")
        })
    }

    fn settings_json(classification: DataClassification) -> Result<String, ProductError> {
        let settings = NotebookSettings {
            classification: Some(classification),
        };
        serde_json::to_string(&settings).map_err(|_| {
            ProductError::new(ProductErrorCode::LocalUnavailable, "settings_json")
        })
    }

    fn classification_from_settings(json: &str) -> DataClassification {
        serde_json::from_str::<NotebookSettings>(json)
            .ok()
            .and_then(|s| s.classification)
            .unwrap_or(DataClassification::Private)
    }
}

impl NotebookRepository for SqliteNotebookRepository {
    fn create(&mut self, title: String, classification: DataClassification) -> NotebookMeta {
        let id = NotebookId(Uuid::new_v4().to_string());
        let now = Self::now_rfc3339().expect("timestamp");
        let settings_json =
            Self::settings_json(classification).expect("settings_json");

        let conn = self.conn.lock().expect("sqlite lock");
        conn.execute(
            "INSERT INTO notebooks (id, title, created_at, updated_at, settings_json) \
             VALUES (?1, ?2, ?3, ?3, ?4)",
            (
                id.0.as_str(),
                title.as_str(),
                now.as_str(),
                settings_json.as_str(),
            ),
        )
        .expect("insert notebook");

        NotebookMeta {
            id,
            title,
            classification,
        }
    }

    fn get(&self, id: &NotebookId) -> Result<NotebookMeta, ProductError> {
        let conn = self.conn.lock().expect("sqlite lock");
        conn.query_row(
            "SELECT title, settings_json FROM notebooks WHERE id = ?1",
            [id.0.as_str()],
            |row| {
                let title: String = row.get(0)?;
                let settings_json: String = row.get(1)?;
                Ok((title, settings_json))
            },
        )
        .map(|(title, settings_json)| NotebookMeta {
            id: id.clone(),
            title,
            classification: Self::classification_from_settings(&settings_json),
        })
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => ProductError::not_found("notebook"),
            _ => ProductError::new(ProductErrorCode::LocalUnavailable, "sqlite"),
        })
    }

    fn list(&self) -> Vec<NotebookMeta> {
        let conn = self.conn.lock().expect("sqlite lock");
        let mut stmt = conn
            .prepare("SELECT id, title, settings_json FROM notebooks ORDER BY created_at")
            .expect("list stmt");

        stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let title: String = row.get(1)?;
            let settings_json: String = row.get(2)?;
            Ok(NotebookMeta {
                id: NotebookId(id),
                title,
                classification: Self::classification_from_settings(&settings_json),
            })
        })
        .expect("list query")
        .filter_map(Result::ok)
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_domain::ProductErrorCode;
    use crate::migrate::migrate_to_latest;

    fn test_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("conn");
        migrate_to_latest(&conn).expect("migrate");
        conn
    }

    #[test]
    fn create_and_get_notebook() {
        let conn = test_conn();
        let mut repo = SqliteNotebookRepository::new(Arc::new(Mutex::new(conn)));
        let created = repo.create("Estudio".into(), DataClassification::Public);
        let fetched = repo.get(&created.id).unwrap();
        assert_eq!(fetched.title, "Estudio");
        assert_eq!(fetched.classification, DataClassification::Public);
    }

    #[test]
    fn list_returns_persisted_notebooks() {
        let conn = test_conn();
        let mut repo = SqliteNotebookRepository::new(Arc::new(Mutex::new(conn)));
        assert!(repo.list().is_empty());

        let a = repo.create("Alpha".into(), DataClassification::Public);
        let b = repo.create("Beta".into(), DataClassification::Public);

        let listed = repo.list();
        assert_eq!(listed.len(), 2);
        assert_eq!(listed[0].id, a.id);
        assert_eq!(listed[0].classification, DataClassification::Public);
        assert_eq!(listed[1].id, b.id);
        assert_eq!(listed[1].title, "Beta");
    }

    #[test]
    fn missing_notebook_is_not_found() {
        let conn = test_conn();
        let repo = SqliteNotebookRepository::new(Arc::new(Mutex::new(conn)));
        let err = repo
            .get(&NotebookId("missing".into()))
            .unwrap_err();
        assert_eq!(err.code, ProductErrorCode::NotFound);
    }
}
