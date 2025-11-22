use serde::{Deserialize, Serialize};
use super::Action;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventRecord {
    pub id: Option<i64>,
    pub session_id: Option<i64>,
    pub timestamp_ms: u128,
    pub action: Action,
}

impl EventRecord {
    pub fn new(timestamp_ms: u128, action: Action) -> Self {
        Self {
            id: None,
            session_id: None,
            timestamp_ms,
            action,
        }
    }
}