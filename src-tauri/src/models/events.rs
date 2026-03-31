use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum AppEvent {
    #[serde(rename = "terminal:data")]
    TerminalData { session_id: String, chunk: String },

    #[serde(rename = "terminal:status")]
    TerminalStatus { session_id: String, status: String, msg: Option<String> },
}
