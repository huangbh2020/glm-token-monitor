#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod events;
mod polling;
mod commands;
mod settings_commands;
mod tray;
mod windows;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_current_usage,
            windows::open_info_panel,
            windows::close_info_panel,
            windows::open_settings_panel,
            windows::close_settings_panel,
            windows::resize_main_window,
            settings_commands::get_config,
            settings_commands::save_config_handler,
            settings_commands::test_api_connection,
            settings_commands::set_auto_start,
        ])
        .setup(|app| {
            // Windows 平台：设置完全透明无边框窗口
            #[cfg(target_os = "windows")]
            {
                use tauri::Manager;
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.set_decorations(false);
                    // 不要使用 set_ignore_cursor_events，它会阻止鼠标事件
                }
            }

            // macOS 平台：透明无边框窗口（拖动通过 JS startDragging 实现）
            #[cfg(target_os = "macos")]
            {
                use tauri::Manager;
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.set_decorations(false);
                }
            }

            // 初始化配置（确保配置文件存在）
            if let Err(e) = config::load_config(app.handle()) {
                eprintln!("Failed to initialize config: {}", e);
            }

            // 创建系统托盘（Windows 和 macOS）
            let app_handle = app.handle().clone();
            if let Err(e) = tray::create_system_tray(&app_handle) {
                eprintln!("System tray error: {}", e);
            }

            // 启动轮询服务
            tauri::async_runtime::spawn(async move {
                if let Err(e) = polling::start_polling_loop(app_handle).await {
                    eprintln!("Polling loop error: {}", e);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
