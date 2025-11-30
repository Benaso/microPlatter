#[cfg(feature = "postgres")]
use super::SessionRepository;
use crate::models::{EventRecord, Session, SessionInfo};
use crate::error::{AppError, AppResult};
use async_trait::async_trait;
use chrono::Utc;
use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

pub struct PostgresSessionRepository {
    pool: Pool,
}

impl PostgresSessionRepository {
    pub async fn new(connection_string: &str) -> AppResult<Self> {
        let mut config = Config::new();
        config.url = Some(connection_string.to_string());
        
        let pool = config.create_pool(Some(Runtime::Tokio1), NoTls)
            .map_err(|e| AppError::Database(e.into()))?;
        
        Ok(Self { pool })
    }
}

#[async_trait]
impl SessionRepository for PostgresSessionRepository {
    async fn init(&self) -> AppResult<()> {
        let client = self.pool.get().await
            .map_err(|e| AppError::Database(e.into()))?;
        
        client.execute(
            "CREATE TABLE IF NOT EXISTS sessions (
                id BIGSERIAL PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL,
                event_count BIGINT DEFAULT 0,
                time_cost float64 DEFAULT 0.0
            )",
            &[],
        ).await.map_err(|e| AppError::Database(e.into()))?;
        
        client.execute(
            "CREATE TABLE IF NOT EXISTS events (
                id BIGSERIAL PRIMARY KEY,
                session_id BIGINT NOT NULL,
                timestamp_ms BIGINT NOT NULL,
                action_type TEXT NOT NULL,
                action_data JSONB NOT NULL,
                FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
            )",
            &[],
        ).await.map_err(|e| AppError::Database(e.into()))?;
        
        client.execute(
            "CREATE INDEX IF NOT EXISTS idx_events_session ON events(session_id)",
            &[],
        ).await.map_err(|e| AppError::Database(e.into()))?;
        
        client.execute(
            "CREATE INDEX IF NOT EXISTS idx_events_timestamp 
             ON events(session_id, timestamp_ms)",
            &[],
        ).await.map_err(|e| AppError::Database(e.into()))?;
        
        Ok(())
    }
    
    async fn create_session(&self, name: &str, description: Option<&str>) -> AppResult<i64> {
        let client = self.pool.get().await
            .map_err(|e| AppError::Database(e.into()))?;
        let created_at = Utc::now();
        
        let row = client.query_one(
            "INSERT INTO sessions (name, description, created_at) 
             VALUES ($1, $2, $3) RETURNING id",
            &[&name, &description, &created_at],
        ).await.map_err(|e| AppError::Database(e.into()))?;
        
        Ok(row.get(0))
    }
    
    async fn get_session(&self, session_id: i64) -> AppResult<Option<Session>> {
        let client = self.pool.get().await
            .map_err(|e| AppError::Database(e.into()))?;
        
        let rows = client.query(
            "SELECT id, name, description, created_at, event_count 
             FROM sessions WHERE id = $1",
            &[&session_id],
        ).await.map_err(|e| AppError::Database(e.into()))?;
        
        if let Some(row) = rows.first() {
            Ok(Some(Session {
                id: row.get(0),
                name: row.get(1),
                description: row.get(2),
                created_at: row.get(3),
                event_count: row.get(4),
            }))
        } else {
            Ok(None)
        }
    }
    
    async fn list_sessions(&self) -> AppResult<Vec<SessionInfo>> {
        let client = self.pool.get().await
            .map_err(|e| AppError::Database(e.into()))?;
        
        let rows = client.query(
            "SELECT id, name, description, 
                    to_char(created_at, 'YYYY-MM-DD\"T\"HH24:MI:SS\"Z\"'), 
                    event_count 
             FROM sessions ORDER BY created_at DESC",
            &[],
        ).await.map_err(|e| AppError::Database(e.into()))?;
        
        let sessions = rows.iter().map(|row| SessionInfo {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            created_at: row.get(3),
            event_count: row.get(4),
        }).collect();
        
        Ok(sessions)
    }
    
    async fn update_session(
        &self, 
        session_id: i64, 
        name: &str, 
        description: Option<&str>
    ) -> AppResult<()> {
        let client = self.pool.get().await
            .map_err(|e| AppError::Database(e.into()))?;
        
        client.execute(
            "UPDATE sessions SET name = $1, description = $2 WHERE id = $3",
            &[&name, &description, &session_id],
        ).await.map_err(|e| AppError::Database(e.into()))?;
        
        Ok(())
    }
    
    async fn delete_session(&self, session_id: i64) -> AppResult<()> {
        let client = self.pool.get().await
            .map_err(|e| AppError::Database(e.into()))?;
        
        client.execute("DELETE FROM sessions WHERE id = $1", &[&session_id])
            .await.map_err(|e| AppError::Database(e.into()))?;
        
        Ok(())
    }
    
    async fn save_events(&self, session_id: i64, events: &[EventRecord]) -> AppResult<()> {
        let mut client = self.pool.get().await
            .map_err(|e| AppError::Database(e.into()))?;
        let tx = client.transaction().await
            .map_err(|e| AppError::Database(e.into()))?;
        
        for event in events {
            let action_type = event.action.action_type();
            let action_data = serde_json::to_value(&event.action)?;
            
            tx.execute(
                "INSERT INTO events (session_id, timestamp_ms, action_type, action_data) 
                 VALUES ($1, $2, $3, $4)",
                &[&session_id, &(event.timestamp_ms as i64), &action_type, &action_data],
            ).await.map_err(|e| AppError::Database(e.into()))?;
        }
        
        tx.execute(
            "UPDATE sessions SET event_count = $1 WHERE id = $2",
            &[&(events.len() as i64), &session_id],
        ).await.map_err(|e| AppError::Database(e.into()))?;
        
        tx.commit().await.map_err(|e| AppError::Database(e.into()))?;
        Ok(())
    }
    
    async fn load_events(&self, session_id: i64) -> AppResult<Vec<EventRecord>> {
        let client = self.pool.get().await
            .map_err(|e| AppError::Database(e.into()))?;
        
        let rows = client.query(
            "SELECT id, timestamp_ms, action_data FROM events 
             WHERE session_id = $1 ORDER BY timestamp_ms ASC",
            &[&session_id],
        ).await.map_err(|e| AppError::Database(e.into()))?;
        
        let mut records = Vec::new();
        for row in rows {
            let id: i64 = row.get(0);
            let timestamp_ms: i64 = row.get(1);
            let action_data: serde_json::Value = row.get(2);
            let action = serde_json::from_value(action_data)?;
            
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