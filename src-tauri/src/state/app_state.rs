use crate::config::{AppConfig, DatabaseConfig};
use crate::repositories::{SessionRepository, SqliteSessionRepository};
#[cfg(feature = "postgres")]
use crate::repositories::PostgresSessionRepository;
use crate::error::AppResult;
use std::sync::{Arc, Mutex as StdMutex};
use tokio::sync::Mutex as TokioMutex;

pub struct AppState {
    pub is_recording: Arc<StdMutex<bool>>,
    pub current_session_id: Arc<StdMutex<Option<i64>>>,
    pub repository: Arc<TokioMutex<Box<dyn SessionRepository>>>,
}

impl AppState {
    pub async fn new(app_handle: &tauri::AppHandle) -> AppResult<Self> {
        let config = AppConfig::load_from_env(app_handle);
        let repository = Self::create_repository(config.database).await?;
        
        Ok(Self {
            is_recording: Arc::new(StdMutex::new(false)),
            current_session_id: Arc::new(StdMutex::new(None)),
            repository: Arc::new(TokioMutex::new(repository)),
        })
    }
    
    async fn create_repository(
        config: DatabaseConfig,
    ) -> AppResult<Box<dyn SessionRepository>> {
        match config {
            DatabaseConfig::SQLite { path } => {
                let repo = SqliteSessionRepository::new(path)?;
                repo.init().await?;
                Ok(Box::new(repo))
            }
            #[cfg(feature = "postgres")]
            DatabaseConfig::PostgreSQL { connection_string } => {
                let repo = PostgresSessionRepository::new(&connection_string).await?;
                repo.init().await?;
                Ok(Box::new(repo))
            }
        }
    }
}