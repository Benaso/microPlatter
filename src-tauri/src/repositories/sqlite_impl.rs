use super::SessionRepository;
use crate::models::{EventRecord, Session, SessionInfo};
use crate::error::{AppError, AppResult};
use async_trait::async_trait;
use chrono::Utc;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

pub struct SqliteSessionRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteSessionRepository {
    pub fn new(db_path: String) -> AppResult<Self> {
        if let Some(parent) = std::path::Path::new(&db_path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(db_path)
            .map_err(|e| AppError::Database(e.into()))?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
}

#[async_trait]
impl SessionRepository for SqliteSessionRepository {
    async fn init(&self) -> AppResult<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                created_at TEXT NOT NULL,
                event_count INTEGER DEFAULT 0
            )",
            [],
        ).map_err(|e| AppError::Database(e.into()))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id INTEGER NOT NULL,
                timestamp_ms INTEGER NOT NULL,
                action_type TEXT NOT NULL,
                action_data TEXT NOT NULL,
                FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
            )",
            [],
        ).map_err(|e| AppError::Database(e.into()))?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_events_session ON events(session_id)",
            [],
        ).map_err(|e| AppError::Database(e.into()))?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idex_events_timestamp
            ON events(session_id, timestamp_ms)",
            [],
        ).map_err(|e| AppError::Database(e.into()))?;

        Ok(())
    }

    async fn create_session(&self, name: &str, description: Option<&str>) -> AppResult<i64> {
        let conn = self.lock().unwrap();
        let created_at = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO sessions (name, description, created_at) VALUES (?1, ?2, ?3)",
            params![name, description, created_at],
        ).map_err(|e| AppError::Database(e.into()))?;

        Ok(conn.last_insert_rowid())
    }

    async fn get_session(&self, session_id: i64) -> AppResult<Option<Session>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, created_at, event_count 
             FROM sessions WHERE id = ?1"
        ).map_err(|e| AppError::Database(e.into()))?;

        let mut rows = stmt.query([session_id])
            .map_err(|e| AppError::Database(e.into()))?;

        if let Some(row) = rows.next().map_err(|e| AppError::Database(e.into()))? {
            let create_at_str: String = row.get(3)
                .map_err(|e| AppError::Database(e.into()))?;
            let create_at = chrono::DataTime::parse_from_rfc3339(&create_at_str)
                .map_err(|e| AppError::Database(e.into()))?
                .with_timezone(&Utc);

            Ok(Some(Session {
                id: row.get(0).map_err(|e| AppError::Database(e.into()))?,
                name: row.get(1).map_err(|e| AppError::Database(e.into()))?,
                description: row.get(2).map_err(|e| AppError::Database(e.into()))?,
                create_at,
                event_count: row.get(4).map_err(|e| AppError::Database(e.into()))?,
            }))
        } else {
            Ok(None)
        }
    }

    async fn list_session(&self) -> AppResult<Vec<SessionInfo>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, created_at, event_count 
             FROM sessions ORDER BY created_at DESC"
        ).map_err(|e| AppError::Database(e.into()))?;

        let sessions = stmt.query_map([], |row| {
            Ok(SessionInfo {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                create_at: row.get(3)?,
                event_count: row.get(4)?,
            })
        }).map_err(|e| AppError::Database(e.into()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| AppError::Database(e.into()))?;

        Ok(sessions)
    }

    async fn update_session(
        &self,
        session_id: i64,
        name: &str,
        description: Option<&str>
    ) -> AppResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE sessions SET name = ?1, description = ?2 WHERE id = ?3",
            params![name, description, session_id],
        ).map_err(|e| AppError::Database(e.into()))?;
        Ok(())
    }

    async fn delete_session(&self, session_id: i64) -> AppResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM sessions WHERE id = ?1", [session_id])
            .map_err(|e| AppError::Database(e.into()))?;
        Ok(())
    }

    async fn save_events(&self, session_id: i64, events: &[EventRecord]) -> AppResult<()> {
        let conn = self.conn.lock().unwrap();
        let tx = conn.unchecked_transaction()
            .map_err(|e| AppError::Database(e.into()))?;

        for event in events {
            let action_type = event.action.action_type();
            let action_data = serde_json::to_string(&event.action)?;

            tx.execute(
                "INSERT INTO events (session_id, timestamp_ms, action_type, action_data) 
                 VALUES (?1, ?2, ?3, ?4)",
                params![session_id, event.timestamp_ms as i64, action_type, action_data],
            ).map_err(|e| AppError::Database(e.into()))?;
        }

        tx.execute(
            "UPDATE sessions SET event_count = ?1 WHERE id = ?2",
            params![events.len(), session_id],
        ).map_err(|e| AppError::Database(e.into()))?;
        
        tx.commit().map_err(|e| AppError::Database(e.into()))?;
        Ok(())
    }

    async fn load_events(&self, session_id: i64) -> AppResult<Vec<EventRecord>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, timestamp_ms, action_data FROM events 
             WHERE session_id = ?1 ORDER BY timestamp_ms ASC"
        ).map_err(|e| AppError::Database(e.into()))?;
        
        let events = stmt.query_map([session_id], |row| {
            let id: i64 = row.get(0)?;
            let timestamp_ms: i64 = row.get(1)?;
            let action_data: String = row.get(2)?;
            Ok((id, timestamp_ms, action_data))
        }).map_err(|e| AppError::Database(e.into()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| AppError::Database(e.into()))?;
        
        let mut records = Vec::new();
        for (id, timestamp_ms, action_data) in events {
            let action = serde_json::from_str(&action_data)?;
            records.push(EventRecord {
                id: Some(id),
                session_id: Some(session_id),
                timestamp_ms: timestamp_ms as u128,
                action,
            });
        }
        
        Ok(records)
    }
}