use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use core_domain::{ProductError, ProductErrorCode};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatMode {
    General,
    Notebook,
}

impl ChatMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChatMode::General => "general",
            ChatMode::Notebook => "notebook",
        }
    }

    pub fn from_db(s: &str) -> Option<Self> {
        match s {
            "general" => Some(ChatMode::General),
            "notebook" => Some(ChatMode::Notebook),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatMessageRole {
    System,
    User,
    Assistant,
}

impl ChatMessageRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChatMessageRole::System => "system",
            ChatMessageRole::User => "user",
            ChatMessageRole::Assistant => "assistant",
        }
    }

    pub fn from_db(s: &str) -> Option<Self> {
        match s {
            "system" => Some(ChatMessageRole::System),
            "user" => Some(ChatMessageRole::User),
            "assistant" => Some(ChatMessageRole::Assistant),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredCitation {
    pub chunk_id: String,
    pub source_version_id: String,
    pub quote: String,
    pub verification: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatRecord {
    pub chat_id: String,
    pub mode: ChatMode,
    pub notebook_id: Option<String>,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageRecord {
    pub message_id: String,
    pub chat_id: String,
    pub role: ChatMessageRole,
    pub content: String,
    pub model_id: Option<String>,
    pub citations: Vec<StoredCitation>,
    pub created_at: String,
}

pub struct SqliteChatRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteChatRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    fn now_rfc3339() -> Result<String, ProductError> {
        OffsetDateTime::now_utc().format(&Rfc3339).map_err(|_| {
            ProductError::new(ProductErrorCode::LocalUnavailable, "timestamp")
        })
    }

    pub fn create_chat(
        &self,
        mode: ChatMode,
        notebook_id: Option<&str>,
        title: Option<&str>,
    ) -> Result<ChatRecord, ProductError> {
        match mode {
            ChatMode::Notebook if notebook_id.is_none() => {
                return Err(ProductError::new(
                    ProductErrorCode::ValidationFailed,
                    "notebook_id es obligatorio en modo notebook",
                ));
            }
            ChatMode::General if notebook_id.is_some() => {
                return Err(ProductError::new(
                    ProductErrorCode::ValidationFailed,
                    "notebook_id no debe enviarse en modo general",
                ));
            }
            _ => {}
        }

        let id = Uuid::new_v4().to_string();
        let now = Self::now_rfc3339()?;
        let title = title.unwrap_or("Nuevo chat").to_string();

        let conn = self.conn.lock().expect("sqlite lock");
        conn.execute(
            "INSERT INTO chats (id, notebook_id, mode, title, created_at, updated_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
            params![
                id,
                notebook_id,
                mode.as_str(),
                title,
                now.as_str(),
            ],
        )
        .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "insert chat"))?;

        Ok(ChatRecord {
            chat_id: id,
            mode,
            notebook_id: notebook_id.map(str::to_string),
            title,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn get_chat(&self, chat_id: &str) -> Result<ChatRecord, ProductError> {
        let conn = self.conn.lock().expect("sqlite lock");
        conn.query_row(
            "SELECT mode, notebook_id, title, created_at, updated_at FROM chats WHERE id = ?1",
            [chat_id],
            |row| {
                let mode_str: String = row.get(0)?;
                let notebook_id: Option<String> = row.get(1)?;
                let title: String = row.get(2)?;
                let created_at: String = row.get(3)?;
                let updated_at: String = row.get(4)?;
                Ok((
                    mode_str,
                    notebook_id,
                    title,
                    created_at,
                    updated_at,
                ))
            },
        )
        .map(|(mode_str, notebook_id, title, created_at, updated_at)| {
            let mode = ChatMode::from_db(&mode_str).unwrap_or(ChatMode::General);
            ChatRecord {
                chat_id: chat_id.to_string(),
                mode,
                notebook_id,
                title,
                created_at,
                updated_at,
            }
        })
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => ProductError::not_found("chat"),
            _ => ProductError::new(ProductErrorCode::LocalUnavailable, "sqlite"),
        })
    }

    pub fn touch_chat(&self, chat_id: &str) -> Result<(), ProductError> {
        let now = Self::now_rfc3339()?;
        let conn = self.conn.lock().expect("sqlite lock");
        let updated = conn
            .execute(
                "UPDATE chats SET updated_at = ?1 WHERE id = ?2",
                params![now.as_str(), chat_id],
            )
            .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "update chat"))?;
        if updated == 0 {
            return Err(ProductError::not_found("chat"));
        }
        Ok(())
    }

    pub fn list_chats(
        &self,
        mode: Option<ChatMode>,
        notebook_id: Option<&str>,
    ) -> Result<Vec<ChatRecord>, ProductError> {
        let conn = self.conn.lock().expect("sqlite lock");
        let mut sql =
            String::from("SELECT id, mode, notebook_id, title, created_at, updated_at FROM chats WHERE 1=1");
        let mut bind_mode: Option<String> = None;
        if let Some(m) = mode {
            bind_mode = Some(m.as_str().to_string());
            sql.push_str(" AND mode = ?");
        }
        if notebook_id.is_some() {
            sql.push_str(" AND notebook_id = ?");
        }
        sql.push_str(" ORDER BY updated_at DESC");

        let mut stmt = conn
            .prepare(&sql)
            .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "prepare"))?;

        let rows = match (&bind_mode, notebook_id) {
            (Some(m), Some(nb)) => stmt.query_map(params![m, nb], map_chat_row),
            (Some(m), None) => stmt.query_map(params![m], map_chat_row),
            (None, Some(nb)) => stmt.query_map(params![nb], map_chat_row),
            (None, None) => stmt.query_map([], map_chat_row),
        }
        .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "query"))?;

        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "sqlite"))
    }

    pub fn insert_message(
        &self,
        chat_id: &str,
        role: ChatMessageRole,
        content: &str,
        model_id: Option<&str>,
        citations: &[StoredCitation],
    ) -> Result<MessageRecord, ProductError> {
        let message_id = Uuid::new_v4().to_string();
        let now = Self::now_rfc3339()?;
        let conn = self.conn.lock().expect("sqlite lock");

        conn.execute(
            "INSERT INTO messages (id, chat_id, role, body, model_id, created_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                message_id,
                chat_id,
                role.as_str(),
                content,
                model_id,
                now.as_str(),
            ],
        )
        .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "insert message"))?;

        for cite in citations {
            let chunk_id: i64 = cite.chunk_id.parse().map_err(|_| {
                ProductError::new(ProductErrorCode::ValidationFailed, "chunk_id inválido")
            })?;
            conn.execute(
                "INSERT INTO citations (message_id, chunk_id, source_version_id, quote, verification_status) \
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    message_id,
                    chunk_id,
                    cite.source_version_id,
                    cite.quote,
                    cite.verification,
                ],
            )
            .map_err(|_| {
                ProductError::new(ProductErrorCode::LocalUnavailable, "insert citation")
            })?;
        }

        drop(conn);
        self.touch_chat(chat_id)?;

        Ok(MessageRecord {
            message_id,
            chat_id: chat_id.to_string(),
            role,
            content: content.to_string(),
            model_id: model_id.map(str::to_string),
            citations: citations.to_vec(),
            created_at: now,
        })
    }

    pub fn messages_for_chats(
        &self,
        chat_ids: &[String],
    ) -> Result<HashMap<String, Vec<MessageRecord>>, ProductError> {
        let mut out: HashMap<String, Vec<MessageRecord>> = HashMap::new();
        if chat_ids.is_empty() {
            return Ok(out);
        }

        let placeholders = chat_ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");
        let sql = format!(
            "SELECT id, chat_id, role, body, model_id, created_at FROM messages \
             WHERE chat_id IN ({placeholders}) ORDER BY created_at ASC"
        );

        let conn = self.conn.lock().expect("sqlite lock");
        let mut stmt = conn
            .prepare(&sql)
            .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "prepare"))?;

        let mut args: Vec<&dyn rusqlite::ToSql> =
            chat_ids.iter().map(|s| s as &dyn rusqlite::ToSql).collect();
        let rows = stmt
            .query_map(rusqlite::params_from_iter(args.drain(..)), |row| {
                let message_id: String = row.get(0)?;
                let chat_id: String = row.get(1)?;
                let role_str: String = row.get(2)?;
                let body: String = row.get(3)?;
                let model_id: Option<String> = row.get(4)?;
                let created_at: String = row.get(5)?;
                let role = ChatMessageRole::from_db(&role_str).unwrap_or(ChatMessageRole::User);
                Ok((
                    message_id,
                    chat_id,
                    role,
                    body,
                    model_id,
                    created_at,
                ))
            })
            .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "query"))?;

        let mut message_ids: Vec<String> = Vec::new();
        for row in rows {
            let (message_id, chat_id, role, body, model_id, created_at) =
                row.map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "row"))?;
            message_ids.push(message_id.clone());
            out.entry(chat_id.clone()).or_default().push(MessageRecord {
                message_id,
                chat_id,
                role,
                content: body,
                model_id,
                citations: Vec::new(),
                created_at,
            });
        }

        if message_ids.is_empty() {
            return Ok(out);
        }

        let cite_placeholders = message_ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");
        let cite_sql = format!(
            "SELECT message_id, chunk_id, source_version_id, quote, verification_status \
             FROM citations WHERE message_id IN ({cite_placeholders})"
        );
        let mut cite_stmt = conn
            .prepare(&cite_sql)
            .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "prepare"))?;
        let cite_rows = cite_stmt
            .query_map(
                rusqlite::params_from_iter(message_ids.iter()),
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        StoredCitation {
                            chunk_id: row.get::<_, i64>(1)?.to_string(),
                            source_version_id: row.get(2)?,
                            quote: row.get(3)?,
                            verification: row.get(4)?,
                        },
                    ))
                },
            )
            .map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "query"))?;

        let mut cites_by_message: HashMap<String, Vec<StoredCitation>> = HashMap::new();
        for row in cite_rows {
            let (mid, cite) =
                row.map_err(|_| ProductError::new(ProductErrorCode::LocalUnavailable, "row"))?;
            cites_by_message.entry(mid).or_default().push(cite);
        }

        for messages in out.values_mut() {
            for msg in messages.iter_mut() {
                if let Some(cites) = cites_by_message.remove(&msg.message_id) {
                    msg.citations = cites;
                }
            }
        }

        Ok(out)
    }
}

fn map_chat_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ChatRecord> {
    let chat_id: String = row.get(0)?;
    let mode_str: String = row.get(1)?;
    let notebook_id: Option<String> = row.get(2)?;
    let title: String = row.get(3)?;
    let created_at: String = row.get(4)?;
    let updated_at: String = row.get(5)?;
    Ok(ChatRecord {
        chat_id,
        mode: ChatMode::from_db(&mode_str).unwrap_or(ChatMode::General),
        notebook_id,
        title,
        created_at,
        updated_at,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::open_memory_connection;

    #[test]
    fn create_and_list_general_chat() {
        let conn = open_memory_connection().expect("db");
        let repo = SqliteChatRepository::new(Arc::new(Mutex::new(conn)));
        let chat = repo
            .create_chat(ChatMode::General, None, Some("General"))
            .expect("create");
        assert_eq!(chat.mode, ChatMode::General);
        assert!(chat.notebook_id.is_none());

        let listed = repo.list_chats(Some(ChatMode::General), None).expect("list");
        assert_eq!(listed.len(), 1);
    }
}
