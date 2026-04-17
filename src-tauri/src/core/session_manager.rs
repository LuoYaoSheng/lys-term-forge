use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, mpsc};
use tauri::{AppHandle, Emitter};

use crate::core::ssh::SSHSession;
use crate::models::events::AppEvent;

#[derive(Clone)]
pub struct SessionManager {
    inner: Arc<Mutex<Inner>>,
}

struct Inner {
    sessions: HashMap<String, SessionHandle>,
}

enum SessionBackend {
    Fake(mpsc::Sender<String>),
    Real(Arc<SSHSession>),
}

struct SessionHandle {
    host: String,
    username: String,
    status: String,
    backend: SessionBackend,
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
        inner.sessions.iter().map(|(id, s)| {
            (id.clone(), s.host.clone(), s.username.clone(), s.status.clone())
        }).collect()
    }

    pub async fn open_fake(&self, app: AppHandle, host: String, username: String) -> String {
        let session_id = format!("sess_{}", nanoid::nanoid!(10));
        let (tx, mut rx) = mpsc::channel::<String>(200);

        {
            let mut inner = self.inner.lock().await;
            inner.sessions.insert(session_id.clone(), SessionHandle {
                host: host.clone(),
                username: username.clone(),
                status: "connected".into(),
                backend: SessionBackend::Fake(tx),
            });
        }

        emit_status(&app, &session_id, "connected", None);

        // Fake session task: echo input + periodic heartbeat
        tokio::spawn({
            let app = app.clone();
            let session_id = session_id.clone();
            async move {
                // Wait for frontend event listener to be ready
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;

                // Initial banner
                emit_data(&app, &session_id, "Welcome to TermForge (fake session)\r\n");
                emit_data(&app, &session_id, "Type anything and press Enter...\r\n\r\n");

                let mut ticker = tokio::time::interval(std::time::Duration::from_secs(5));
                loop {
                    tokio::select! {
                        _ = ticker.tick() => {
                            emit_data(&app, &session_id, "\r[fake] heartbeat...\r\n");
                        }
                        maybe = rx.recv() => {
                            match maybe {
                                Some(data) => {
                                    // Echo back the input
                                    emit_data(&app, &session_id, &format!("You typed: {}\r\n", data));
                                }
                                None => break, // channel closed
                            }
                        }
                    }
                }
                emit_status(&app, &session_id, "closed", Some("session task ended".into()));
            }
        });

        session_id
    }

    pub async fn open_ssh(
        &self,
        app: AppHandle,
        host: String,
        port: u16,
        username: String,
        password: Option<String>,
    ) -> Result<String, String> {
        let session_id = format!("ssh_{}", nanoid::nanoid!(10));

        // Try to connect
        let ssh_session = match SSHSession::new(
            host.clone(),
            port,
            username.clone(),
            password.clone(),
            None, // key_path
        ).await {
            Ok(session) => Arc::new(session),
            Err(e) => {
                return Err(format!("Connection failed: {}", e));
            }
        };

        {
            let mut inner = self.inner.lock().await;
            inner.sessions.insert(session_id.clone(), SessionHandle {
                host: host.clone(),
                username: username.clone(),
                status: "connected".into(),
                backend: SessionBackend::Real(ssh_session.clone()),
            });
        }

        emit_status(&app, &session_id, "connected", None);

        // Spawn SSH output reader task
        let ssh_session_clone = ssh_session.clone();
        tokio::spawn({
            let app = app.clone();
            let session_id = session_id.clone();
            async move {
                // Wait for frontend event listener to be ready
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;

                emit_data(&app, &session_id, &format!("Connected to {}@{}\r\n\r\n", username, host));

                // Read output from SSH session
                loop {
                    match ssh_session_clone.read().await {
                        Ok(Some(data)) => {
                            // Convert bytes to UTF-8 string, replacing invalid sequences
                            let chunk = String::from_utf8_lossy(&data);
                            emit_data(&app, &session_id, &chunk);
                        }
                        Ok(None) => {
                            // No data available, wait a bit before trying again
                            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                        }
                        Err(e) => {
                            emit_status(&app, &session_id, "error", Some(format!("Read error: {}", e)));
                            break;
                        }
                    }
                }
            }
        });

        Ok(session_id)
    }

    pub async fn send(&self, session_id: &str, data: String) -> anyhow::Result<()> {
        let inner = self.inner.lock().await;
        let s = inner.sessions.get(session_id).ok_or_else(|| anyhow::anyhow!("session not found"))?;

        match &s.backend {
            SessionBackend::Fake(tx) => {
                tx.send(data).await.map_err(|_| anyhow::anyhow!("session channel closed"))?;
            }
            SessionBackend::Real(ssh_session) => {
                ssh_session.send(&data).await?;
            }
        }
        Ok(())
    }

    pub async fn close(&self, session_id: &str) -> anyhow::Result<()> {
        let mut inner = self.inner.lock().await;
        if let Some(s) = inner.sessions.remove(session_id) {
            if let SessionBackend::Real(ssh_session) = s.backend {
                ssh_session.close().await?;
            }
        }
        Ok(())
    }
}

fn emit_data(app: &AppHandle, session_id: &str, chunk: &str) {
    let _ = app.emit("app_event", AppEvent::TerminalData {
        session_id: session_id.to_string(),
        chunk: chunk.to_string(),
    });
}

fn emit_status(app: &AppHandle, session_id: &str, status: &str, msg: Option<String>) {
    let _ = app.emit("app_event", AppEvent::TerminalStatus {
        session_id: session_id.to_string(),
        status: status.to_string(),
        msg,
    });
}
