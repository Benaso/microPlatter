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
    // check and drop quickly so the guard doesn't live across await
    {
        let is_recording_check = state.is_recording.lock().unwrap();
        if *is_recording_check {
            return Err(AppError::AlreadyRecording.to_string());
        }
    }

    // 创建新会话 (await point)
    let repository = state.repository.lock().await;
    let session_id = repository
        .create_session(&session_name, description.as_deref())
        .await
        .map_err(|e| e.to_string())?;
    drop(repository);
    
    // 设置当前会话ID
    {
        let mut current_session = state.current_session_id.lock().unwrap();
        *current_session = Some(session_id);
    }

    {
        let mut is_recording = state.is_recording.lock().unwrap();
        *is_recording = true;
    }
    
    // 启动录制
    let is_recording_clone = state.is_recording.clone();
    std::thread::spawn(move || {
        if let Err(e) = RecorderService::start_recording(is_recording_clone, app_handle) {
            eprintln!("Recording error: {:?}", e);
        }
    });
    
    Ok(session_id)
}

#[tauri::command]
pub async fn stop_recording(
    state: State<'_, AppState>,
) -> Result<String, String> {
    // check and flip the recording flag quickly in a scoped block so the guard
    // doesn't live across the await below
    {
        let mut guard = state.is_recording.lock().unwrap();
        if !*guard {
            return Err(AppError::NotRecording.to_string());
        }
        *guard = false;
    }
    
    // 等待一下确保所有事件都被记录
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // 获取当前会话ID
    let session_id = {
        let current_session = state.current_session_id.lock().unwrap();
        current_session.ok_or(AppError::NoActiveSession.to_string())?
    };
    
    // 保存记录到数据库
    let repository = state.repository.lock().await;
    let count = RecorderService::save_recording(session_id, &**repository)
        .await
        .map_err(|e| e.to_string())?;
    drop(repository);
    
    // 清除当前会话
    let mut current_session = state.current_session_id.lock().unwrap();
    *current_session = None;
    
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