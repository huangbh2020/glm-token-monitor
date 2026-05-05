#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod events;
mod polling;
mod commands;
mod settings_commands;
mod tray;
mod windows;
mod claude_profile;
mod todo;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::get_current_usage,
            commands::get_model_usage,
            windows::open_info_panel,
            windows::close_info_panel,
            windows::open_settings_panel,
            windows::close_settings_panel,
            windows::resize_main_window,
            settings_commands::get_config,
            settings_commands::save_config_handler,
            settings_commands::test_api_connection,
            settings_commands::set_auto_start,
            claude_profile::get_profiles,
            claude_profile::save_profile_handler,
            claude_profile::delete_profile_handler,
            claude_profile::switch_profile_handler,
            claude_profile::get_claude_config_path_cmd,
            claude_profile::set_claude_config_path_handler,
            commands::get_cursor_position,
            todo::todo_add_item,
            todo::todo_toggle_item,
            todo::todo_delete_item,
            todo::todo_get_day,
            todo::todo_get_week,
            todo::todo_copy_day,
            todo::todo_copy_week,
            todo::todo_get_pending_count,
            windows::open_todo_panel,
            windows::close_todo_panel,
        ])
        .setup(|app| {
            // Windows 平台：设置完全透明无边框窗口
            #[cfg(target_os = "windows")]
            {
                use tauri::Manager;
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.set_decorations(false);
                }
            }

            // macOS 平台：透明无边框窗口，不在 Dock 栏显示图标
            #[cfg(target_os = "macos")]
            {
                use tauri::Manager;
                // 隐藏 Dock 图标（Accessory 模式：仅在系统托盘显示）
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.set_decorations(false);
                }
            }

            // 初始化配置（确保配置文件存在）
            if let Err(e) = config::load_config(app.handle()) {
                eprintln!("Failed to initialize config: {}", e);
            }

            // 清理过期 todo 数据
            todo::run_cleanup(app.handle());

            // 注册全局快捷键：Cmd+Shift+T (macOS) / Ctrl+Shift+T (Windows)
            {
                use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};
                let shortcut = Shortcut::new(
                    Some(Modifiers::SUPER | Modifiers::SHIFT),
                    Code::KeyT,
                );
                if let Err(e) = app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, _event| {
                    use tauri::Emitter;
                    let _ = _app.emit("show-quick-capture", ());
                }) {
                    eprintln!("Failed to register global shortcut: {}", e);
                }
            }

            // 创建系统托盘（Windows 和 macOS）
            let app_handle = app.handle().clone();
            if let Err(e) = tray::create_system_tray(&app_handle) {
                eprintln!("System tray error: {}", e);
            }
            tray::setup_tray_menu_handler(app.handle());

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
