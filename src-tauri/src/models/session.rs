use serde::{Deserialize, Serialize};
use chrono::{DataTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub create_at: DataTime<Utc>,
    pub event_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub create_at: String,
    pub event_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSessionDto {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSessionDto {
    pub name: String,
    pub description: Option<String>,
}