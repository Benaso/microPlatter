use crate::state::AppState;
use crate::models::SessionResponse;
use tauri::State;

#[tauri::command]
pub async fn list_sessions(
    state: State<'_, AppState>,
) -> Result<Vec<SessionResponse>, String> {
    let repository = state.repository.lock().await;
    let sessions = repository.list_sessions()
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(sessions.into_iter().map(SessionResponse::from).collect())
}

#[tauri::command]
pub async fn get_session(
    state: State<'_, AppState>,
    session_id: i64,
) -> Result<Option<SessionResponse>, String> {
    let repository = state.repository.lock().await;
    let session = repository.get_session(session_id)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(session.map(SessionResponse::from))
}

#[tauri::command]
pub async fn update_session(
    state: State<'_, AppState>,
    session_id: i64,
    name: String,
    description: Option<String>,
) -> Result<String, String> {
    let repository = state.repository.lock().await;
    repository.update_session(session_id, &name, description.as_deref())
        .await
        .map_err(|e| e.to_string())?;
    Ok(format!("Session {} updated", session_id))
}

#[tauri::command]
pub async fn delete_session(
    state: State<'_, AppState>,
    session_id: i64,
) -> Result<String, String> {
    let repository = state.repository.lock().await;
    repository.delete_session(session_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(format!("Session {} deleted", session_id))
}