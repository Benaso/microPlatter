use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] anyhow::Error),
    
    #[error("Already recording")]
    AlreadyRecording,
    
    #[error("Not recording")]
    NotRecording,
    
    #[error("No active session")]
    NoActiveSession,
    
    #[error("Session not found: {0}")]
    SessionNotFound(i64),
    
    #[error("Recording error: {0}")]
    RecordingError(String),
    
    #[error("Playback error: {0}")]
    PlaybackError(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type AppResult<T> = Result<T, AppError>;

impl From<AppError> for String {
    fn from(err: AppError) -> String {
        err.to_string()
    }
}

// 从 rusqlite::Error 转换
impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Database(err.into())
    }
}

// Allow using `?` with std::io::Error where AppError is expected
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Database(err.into())
    }
}