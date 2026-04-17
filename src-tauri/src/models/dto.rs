use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SessionOpenRequest {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Option<String>,
    #[serde(default)]
    pub key_path: Option<String>,
}

// Manual Debug impl to mask sensitive fields
impl std::fmt::Debug for SessionOpenRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionOpenRequest")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("username", &self.username)
            .field("password", &"***")
            .field("key_path", &self.key_path)
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionOpenResponse {
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSendRequest {
    pub session_id: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCloseRequest {
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub session_id: String,
    pub host: String,
    pub username: String,
    pub status: String,
}
