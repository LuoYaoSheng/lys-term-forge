use tauri::{State, Manager};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedConnection {
    pub id: String,
    pub name: String,
    pub mode: String, // "fake" | "ssh"
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Option<String>, // Encrypted in production
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
        inner.data.connections.clone()
    }

    pub async fn save(&self, conn: SavedConnection) -> anyhow::Result<()> {
        let mut inner = self.inner.lock().await;

        // Update or add
        if let Some(existing) = inner.data.connections.iter_mut().find(|c| c.id == conn.id) {
            *existing = conn;
        } else {
            inner.data.connections.push(conn);
        }

        // Persist
        self.persist(&inner).await
    }

    pub async fn delete(&self, id: &str) -> anyhow::Result<()> {
        let mut inner = self.inner.lock().await;
        inner.data.connections.retain(|c| c.id != id);
        self.persist(&inner).await
    }

    async fn persist(&self, inner: &StoreInner) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(&inner.data)?;
        fs::write(&inner.path, json)?;
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
