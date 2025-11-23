mod commands;
mod services;
mod repositories;
mod models;
mod config;
mod state;
mod error;

use state::AppState;
use commands::*;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                match AppState::new(&handle).await {
                    Ok(state) => {
                        handle.manage(state);
                    }
                    Err(e) => {
                        eprintln!("Failed to initialize app state: {}", e);
                        std::process::exit(1);
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 录制命令
            start_recording,
            stop_recording,
            play_recording,
            get_recording_status,
            // 会话命令
            list_sessions,
            get_session,
            update_session,
            delete_session,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}