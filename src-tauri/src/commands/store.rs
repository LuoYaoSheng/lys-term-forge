use tauri::State;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::warn;

use crate::core::crypto;

/// Wire format for the connection store JSON file.
/// Passwords are stored encrypted (base64-encoded AES-256-GCM ciphertext).
#[derive(Clone, Serialize, Deserialize)]
pub struct SavedConnection {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    /// Encrypted password (base64-encoded), transparently encrypted/decrypted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

// Manual Debug impl to mask password
impl std::fmt::Debug for SavedConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SavedConnection")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("host", &self.host)
            .field("port", &self.port)
            .field("username", &self.username)
            .field("password", &self.password.as_ref().map(|_| "***"))
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStore {
    pub connections: Vec<SavedConnection>,
}

struct StoreInner {
    path: PathBuf,
    data: ConnectionStore,
}

pub struct ConnectionStoreManager {
    inner: Arc<Mutex<StoreInner>>,
}

impl ConnectionStoreManager {
    pub fn new() -> anyhow::Result<Self> {
        let path = Self::store_path()?;
        let data = if path.exists() {
            let content = fs::read_to_string(&path)?;
            serde_json::from_str(&content).unwrap_or_else(|_| ConnectionStore {
                connections: Vec::new(),
            })
        } else {
            ConnectionStore {
                connections: Vec::new(),
            }
        };

        Ok(Self {
            inner: Arc::new(Mutex::new(StoreInner { path, data })),
        })
    }

    fn store_path() -> anyhow::Result<PathBuf> {
        let mut path = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("No home directory"))?;
        path.push(".termforge");
        fs::create_dir_all(&path)?;
        path.push("connections.json");
        Ok(path)
    }

    pub async fn list(&self) -> Vec<SavedConnection> {
        let inner = self.inner.lock().await;
        // Decrypt passwords for local consumption
        inner.data.connections.iter().map(|c| {
            let mut conn = c.clone();
            if let Some(ref encrypted) = conn.password {
                match crypto::decrypt(encrypted) {
                    Ok(decrypted) => conn.password = Some(decrypted),
                    Err(e) => {
                        warn!(id = %conn.id, error = %e, "Failed to decrypt stored password");
                        conn.password = None;
                    }
                }
            }
            conn
        }).collect()
    }

    pub async fn save(&self, mut conn: SavedConnection) -> anyhow::Result<()> {
        // Encrypt password before persisting
        if let Some(ref pw) = conn.password {
            if !pw.is_empty() {
                match crypto::encrypt(pw) {
                    Ok(encrypted) => conn.password = Some(encrypted),
                    Err(e) => {
                        warn!(error = %e, "Failed to encrypt password — storing without encryption");
                    }
                }
            }
        }

        let mut inner = self.inner.lock().await;

        // Update or add
        if let Some(existing) = inner.data.connections.iter_mut().find(|c| c.id == conn.id) {
            *existing = conn;
        } else {
            inner.data.connections.push(conn);
        }

        self.persist(&inner).await
    }

    pub async fn delete(&self, id: &str) -> anyhow::Result<()> {
        let mut inner = self.inner.lock().await;
        inner.data.connections.retain(|c| c.id != id);
        self.persist(&inner).await
    }

    async fn persist(&self, inner: &StoreInner) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(&inner.data)?;
        fs::write(&inner.path, &json)?;

        // Set file permissions to owner-only (0o600) on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = fs::Permissions::from_mode(0o600);
            fs::set_permissions(&inner.path, perms)?;
        }

        Ok(())
    }
}

impl Default for ConnectionStoreManager {
    fn default() -> Self {
        Self::new().expect("Failed to create ConnectionStoreManager")
    }
}

#[tauri::command]
pub async fn connection_list(
    store: State<'_, ConnectionStoreManager>
) -> Result<Vec<SavedConnection>, String> {
    Ok(store.list().await)
}

#[tauri::command]
pub async fn connection_save(
    store: State<'_, ConnectionStoreManager>,
    conn: SavedConnection
) -> Result<(), String> {
    store.save(conn).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn connection_delete(
    store: State<'_, ConnectionStoreManager>,
    id: String
) -> Result<(), String> {
    store.delete(&id).await.map_err(|e| e.to_string())
}
