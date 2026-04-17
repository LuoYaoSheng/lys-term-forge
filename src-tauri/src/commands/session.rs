use tauri::{AppHandle, State};

use crate::{
    core::session_manager::SessionManager,
    models::dto::{
        SessionOpenRequest, SessionOpenResponse, SessionSendRequest, SessionCloseRequest, SessionInfo
    },
};

#[tauri::command]
pub async fn session_open(
    app: AppHandle,
    manager: State<'_, SessionManager>,
    req: SessionOpenRequest
) -> Result<SessionOpenResponse, String> {
    match manager.open_ssh(
        app,
        req.host,
        req.port,
        req.username,
        req.password,
        req.key_path,
    ).await {
        Ok(session_id) => Ok(SessionOpenResponse { session_id }),
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub async fn session_send(
    manager: State<'_, SessionManager>,
    req: SessionSendRequest
) -> Result<(), String> {
    manager.send(&req.session_id, req.data).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn session_close(
    manager: State<'_, SessionManager>,
    req: SessionCloseRequest
) -> Result<(), String> {
    manager.close(&req.session_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn session_list(
    manager: State<'_, SessionManager>
) -> Result<Vec<SessionInfo>, String> {
    let list = manager.list().await;
    Ok(list.into_iter().map(|(session_id, host, username, status)| SessionInfo {
        session_id, host, username, status
    }).collect())
}

#[tauri::command]
pub async fn session_resize(
    manager: State<'_, SessionManager>,
    session_id: String,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    manager.resize(&session_id, cols, rows).await.map_err(|e| e.to_string())
}
