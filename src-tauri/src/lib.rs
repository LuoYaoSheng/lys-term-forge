#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
mod commands;
mod models;

use commands::{session::*, store::*};
use core::session_manager::SessionManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(SessionManager::new())
        .manage(ConnectionStoreManager::default())
        .invoke_handler(tauri::generate_handler![
            // Session commands
            session_open,
            session_send,
            session_close,
            session_list,
            session_resize,
            // Connection store commands
            connection_list,
            connection_save,
            connection_delete
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
