// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::command;

// 定义一个命令，可以被前端 invoke 调用
#[command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Greetings from Rust 👋", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}