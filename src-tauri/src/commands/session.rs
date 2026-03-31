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
    match req.mode.as_str() {
        "fake" => {
            let session_id = manager
                .open_fake(app, req.host, req.username)
                .await;
            Ok(SessionOpenResponse { session_id })
        }
        "ssh" => {
            match manager.open_ssh(
                app,
                req.host,
                req.port,
                req.username,
                req.password,
            ).await {
                Ok(session_id) => Ok(SessionOpenResponse { session_id }),
                Err(e) => Err(e),
            }
        }
        _ => Err("Invalid mode. Use 'fake' or 'ssh'".into())
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
