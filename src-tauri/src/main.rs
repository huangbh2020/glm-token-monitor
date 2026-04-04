#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod events;
mod polling;
use events::UsageData;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
