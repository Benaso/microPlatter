use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DatabaseConfig {
    SQLite { 
        path: String 
    },
    #[cfg(feature = "postgres")]
    PostgreSQL { 
        connection_string: String 
    },
}

impl AppConfig {
    pub fn load_from_env(app_handle: &tauri::AppHandle) -> Self {
        // 从环境变量读取数据库配置
        if let Ok(pg_conn) = std::env::var("DATABASE_URL") {
            #[cfg(feature = "postgres")]
            return AppConfig {
                database: DatabaseConfig::PostgreSQL {
                    connection_string: pg_conn,
                },
            };
        }
        
        // 默认使用 SQLite
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        let db_path = current_dir.join("recordings.db");
        
        AppConfig {
            database: DatabaseConfig::SQLite {
                path: db_path.to_string_lossy().to_string(),
            },
        }
    }
}