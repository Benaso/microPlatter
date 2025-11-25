use crate::state::AppState;
use crate::services::{RecorderService, PlayerService};
use crate::error::AppError;
use tauri::State;

#[tauri::command]
pub async fn start_recording(
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
    session_name: String,
    description: Option<String>,
) -> Result<i64, String> {
    // Check and set recording flag briefly, release guard before any await
    {
        let mut is_recording = state.is_recording.lock().unwrap();
        if *is_recording {
            return Err(AppError::AlreadyRecording.to_string());
        }
        *is_recording = true;
    }

    // 创建新会话 (await while not holding std mutex guards)
    let session_id = {
        let repository = state.repository.lock().await;
        repository
            .create_session(&session_name, description.as_deref())
            .await
            .map_err(|e| e.to_string())?
    };

    // 设置当前会话ID
    {
        let mut current_session = state.current_session_id.lock().unwrap();
        *current_session = Some(session_id);
    }
    
    // 启动录制
    let is_recording_clone = state.is_recording.clone();
    std::thread::spawn(move || {
        if let Err(e) = RecorderService::start_recording(is_recording_clone, app_handle) {
            eprintln!("Recording error: {:?}", e);
        }
    });
    println!("Started recording session: {}", session_id);

    Ok(session_id)
}

#[tauri::command]
pub async fn stop_recording(
    state: State<'_, AppState>,
) -> Result<String, String> {
    {
        let mut is_recording = state.is_recording.lock().unwrap();
        if !*is_recording {
            return Err(AppError::NotRecording.to_string());
        }
        *is_recording = false;
    }

    // 等待一下确保所有事件都被记录
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    // 获取当前会话ID
    let session_id = {
        let current_session = state.current_session_id.lock().unwrap();
        current_session.ok_or(AppError::NoActiveSession.to_string())?
    };
    
    // 保存记录到数据库 (don't hold std guards across await)
    let count = {
        let repository = state.repository.lock().await;
        RecorderService::save_recording(session_id, &**repository)
            .await
            .map_err(|e| e.to_string())?
    };
    
    // 清除当前会话
    {
        let mut current_session = state.current_session_id.lock().unwrap();
        *current_session = None;
    }
    println!("Saved {} events to session {}", count, session_id);

    Ok(format!("Saved {} events to session {}", count, session_id))
}

#[tauri::command]
pub async fn play_recording(
    state: State<'_, AppState>,
    session_id: i64,
) -> Result<String, String> {
    let repository = state.repository.lock().await;
    PlayerService::play_session(session_id, &**repository)
        .await
        .map_err(|e| e.to_string())?;
    Ok("Playback completed".to_string())
}

#[tauri::command]
pub async fn get_recording_status(
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let is_recording = state.is_recording.lock().unwrap();
    Ok(*is_recording)
}