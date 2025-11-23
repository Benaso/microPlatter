use crate::state::AppState;
use crate::models::SessionInfo;
use tauri::State;

#[tauri::command]
pub async fn list_sessions(
    state: State<'_, AppState>,
) -> Result<Vec<SessionInfo>, String> {
    let repository = state.repository.lock().await;
    repository.list_sessions()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_session(
    state: State<'_, AppState>,
    session_id: i64,
) -> Result<Option<SessionInfo>, String> {
    let repository = state.repository.lock().await;
    let session = repository.get_session(session_id)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(session.map(|s| SessionInfo {
        id: s.id,
        name: s.name,
        description: s.description,
        created_at: s.created_at.to_rfc3339(),
        event_count: s.event_count,
    }))
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