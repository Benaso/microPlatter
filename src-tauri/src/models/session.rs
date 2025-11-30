use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 领域实体 - 数据库/内部使用
#[derive(Debug, Clone)]
pub struct Session {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub event_count: i64,
    pub time_cost: f64,
}

/// API 响应 - 返回给前端
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    /// ISO 8601 格式，如: "2023-12-19T10:20:34Z"
    pub created_at: String,
    pub event_count: i64,
    pub time_cost: f64,
}

/// API 请求 - 创建会话
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    pub name: String,
    pub description: Option<String>,
}

/// API 请求 - 更新会话
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSessionRequest {
    pub name: String,
    pub description: Option<String>,
}

impl From<Session> for SessionResponse {
    fn from(session: Session) -> Self {
        Self {
            id: session.id,
            name: session.name,
            description: session.description,
            created_at: session.created_at.to_rfc3339(),
            event_count: session.event_count,
            time_cost: session.time_cost,
        }
    }
}