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
        
        // 默认使用 SQLite，并选择平台合适的位置以避免在开发时触发 watcher 重建
        // Linux: $XDG_DATA_HOME 或 $HOME/.local/share
        // macOS: ~/Library/Application Support
        // Windows: %APPDATA%
        let db_path = if let Ok(path) = std::env::var("MICROPLATTER_DB_PATH") {
            // 允许通过环境变量覆盖位置（开发/测试友好）
            std::path::PathBuf::from(path)
        } else {
            #[cfg(target_os = "windows")]
            {
                if let Ok(appdata) = std::env::var("APPDATA") {
                    let mut p = std::path::PathBuf::from(appdata);
                    p.push("microplatter");
                    p.push("recordings.db");
                    p
                } else {
                    // fallback to current dir
                    std::env::current_dir().expect("Failed to get current directory").join("recordings.db")
                }
            }

            #[cfg(target_os = "macos")]
            {
                if let Ok(home) = std::env::var("HOME") {
                    let mut p = std::path::PathBuf::from(home);
                    p.push("Library");
                    p.push("Application Support");
                    p.push("microplatter");
                    p.push("recordings.db");
                    p
                } else {
                    std::env::current_dir().expect("Failed to get current directory").join("recordings.db")
                }
            }

            #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
            {
                // Linux / other unix
                if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
                    let mut p = std::path::PathBuf::from(xdg);
                    p.push("microplatter");
                    p.push("recordings.db");
                    p
                } else if let Ok(home) = std::env::var("HOME") {
                    let mut p = std::path::PathBuf::from(home);
                    p.push(".local");
                    p.push("share");
                    p.push("microplatter");
                    p.push("recordings.db");
                    p
                } else {
                    std::env::current_dir().expect("Failed to get current directory").join("recordings.db")
                }
            }
        };

        // Ensure parent directory exists where applicable
        if let Some(parent) = db_path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!("Failed to create database directory {:?}: {:?}", parent, e);
            }
        }

        AppConfig {
            database: DatabaseConfig::SQLite {
                path: db_path.to_string_lossy().to_string(),
            },
        }
    }
}