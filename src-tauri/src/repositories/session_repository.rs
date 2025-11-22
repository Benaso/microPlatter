use crate::models::{EventRecord, Session, SessionInfo};
use crate::error::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait SessionRepository: Send + Sync {
    /// 初始化数据库
    async fn init(&self) -> AppResult<()>;

    /// 创建会话
    async fn create_session(&self, name: &str, description: Option<&str>) -> AppResult<i64>;

    /// 获取会话详情
    async fn get_session(&self, session_id: i64) -> AppResult<Option<Session>>;

    /// 列出所有会话
    async fn list_session(&self) -> AppResult<Vec<SessionInfo>>;

    /// 更新会话
    async fn update_session(
        &self,
        session_id: i64,
        name: &str,
        description: Option<&str>
    ) -> AppResult<()>;

    /// 删除会话
    async fn delete_session(&self, session_id: i64) -> AppResult<()>;

    /// 保存时间记录
    async fn save_events(&self, session_id: i64, events: &[EventRecord]) -> AppResult<()>;

    /// 加载时间记录
    async fn load_events(&self, session_id: i64) -> AppResult<Vec<EventRecord>>;
}