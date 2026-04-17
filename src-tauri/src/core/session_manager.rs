use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tauri::{AppHandle, Emitter};
use tracing::{info, warn};

use crate::core::ssh::SSHSession;
use crate::models::events::AppEvent;

#[derive(Clone)]
pub struct SessionManager {
    inner: Arc<Mutex<Inner>>,
}

struct Inner {
    sessions: HashMap<String, SessionHandle>,
}

struct SessionHandle {
    host: String,
    username: String,
    status: String,
    session: SSHSession,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner {
                sessions: HashMap::new(),
            })),
        }
    }

    pub async fn list(&self) -> Vec<(String, String, String, String)> {
        let inner = self.inner.lock().await;
        inner
            .sessions
            .iter()
            .map(|(id, s)| (id.clone(), s.host.clone(), s.username.clone(), s.status.clone()))
            .collect()
    }

    pub async fn open_ssh(
        &self,
        app: AppHandle,
        host: String,
        port: u16,
        username: String,
        password: Option<String>,
        key_path: Option<String>,
    ) -> Result<String, String> {
        let session_id = format!("ssh_{}", nanoid::nanoid!(10));

        // SSHSession::new() is blocking — run on the blocking thread pool
        let ssh_session = tokio::task::spawn_blocking({
            let app = app.clone();
            let session_id = session_id.clone();
            let host = host.clone();
            let username = username.clone();
            let password = password.clone();
            let key_path = key_path.clone();
            move || {
                SSHSession::new(app, session_id, host, port, username, password, key_path)
            }
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
        .map_err(|e| format!("Connection failed: {}", e))?;

        {
            let mut inner = self.inner.lock().await;
            inner.sessions.insert(
                session_id.clone(),
                SessionHandle {
                    host: host.clone(),
                    username: username.clone(),
                    status: "connected".into(),
                    session: ssh_session,
                },
            );
        }

        emit_status(&app, &session_id, "connected", None);
        info!(session_id = %session_id, "SSH session opened successfully");

        Ok(session_id)
    }

    /// Send data to a session.
    pub async fn send(&self, session_id: &str, data: String) -> anyhow::Result<()> {
        let inner = self.inner.lock().await;
        let s = inner
            .sessions
            .get(session_id)
            .ok_or_else(|| anyhow::anyhow!("session not found"))?;
        s.session.send(&data)?;
        Ok(())
    }

    /// Resize the PTY for a session.
    pub async fn resize(&self, session_id: &str, cols: u16, rows: u16) -> anyhow::Result<()> {
        let inner = self.inner.lock().await;
        let s = inner
            .sessions
            .get(session_id)
            .ok_or_else(|| anyhow::anyhow!("session not found"))?;
        s.session.resize(cols, rows)?;
        Ok(())
    }

    pub async fn close(&self, session_id: &str) -> anyhow::Result<()> {
        let mut inner = self.inner.lock().await;
        if let Some(s) = inner.sessions.remove(session_id) {
            info!(session_id = %session_id, "Closing SSH session");
            s.session.close()?;
        } else {
            warn!(session_id = %session_id, "Attempted to close non-existent session");
        }
        Ok(())
    }
}

fn emit_status(app: &AppHandle, session_id: &str, status: &str, msg: Option<String>) {
    let _ = app.emit(
        "app_event",
        AppEvent::TerminalStatus {
            session_id: session_id.to_string(),
            status: status.to_string(),
            msg,
        },
    );
}
