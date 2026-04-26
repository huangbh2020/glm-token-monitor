use tauri::{AppHandle, Manager};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu, CheckMenuItem};
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};

/// 创建系统托盘（Windows 和 macOS）
/// 不注册事件处理器，事件处理器由 setup_tray_menu_handler 单独注册
pub fn create_system_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 构建配置切换子菜单
    let profile_submenu = build_profile_submenu(app)?;

    // 创建管理配置项
    let manage_profiles_item = MenuItem::with_id(app, "manage_profiles", "管理 API 配置", true, None::<&str>)?;

    // 创建分隔符
    let separator1 = PredefinedMenuItem::separator(app)?;
    let separator2 = PredefinedMenuItem::separator(app)?;

    // 创建其他菜单项
    let show_item = MenuItem::with_id(app, "show", "显示宠物", true, None::<&str>)?;
    let hide_item = MenuItem::with_id(app, "hide", "隐藏宠物", true, None::<&str>)?;
    let refresh_item = MenuItem::with_id(app, "refresh", "刷新数据", true, None::<&str>)?;
    let settings_item = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[
        &profile_submenu,
        &manage_profiles_item,
        &separator1,
        &show_item,
        &hide_item,
        &refresh_item,
        &separator2,
        &settings_item,
        &quit_item,
    ])?;

    // 使用 Tauri 默认图标作为托盘图标
    let icon = app.default_window_icon().ok_or("Failed to get default icon")?.clone();

    let _tray = TrayIconBuilder::with_id("main-tray")
        .icon(icon)
        .menu(&menu)
        .tooltip("glm-token-monitor - Token Monitor")
        .build(app)?;

    Ok(())
}

/// 构建配置切换子菜单
/// 读取所有配置文件，为每个配置创建一个 CheckMenuItem
/// 当前激活的配置会在标签前显示 "✓ "
pub fn build_profile_submenu(app: &AppHandle) -> Result<Submenu<tauri::Wry>, Box<dyn std::error::Error>> {
    let store = crate::claude_profile::load_profiles(app)
        .unwrap_or_else(|_| crate::claude_profile::ProfileStore::default());

    let submenu_title = "切换 API 配置";

    if store.profiles.is_empty() {
        // 空状态：显示禁用的 "（无配置）" 项
        let empty_item = MenuItem::with_id(app, "profile_empty", "（无配置）", false, None::<&str>)?;
        return Ok(Submenu::with_items(app, submenu_title, true, &[&empty_item])?);
    }

    // 为每个配置创建 CheckMenuItem
    let mut menu_items: Vec<Box<dyn tauri::menu::IsMenuItem<tauri::Wry>>> = Vec::new();
    for profile in &store.profiles {
        let is_active = store.active_profile_id.as_deref() == Some(&profile.id);
        let label = if is_active {
            format!("✓ {}", profile.name)
        } else {
            profile.name.clone()
        };
        let item_id = format!("profile_{}", profile.id);
        let item = CheckMenuItem::with_id(
            app,
            item_id,
            label,
            true,
            is_active,
            None::<&str>,
        )?;
        menu_items.push(Box::new(item));
    }

    let menu_refs: Vec<&dyn tauri::menu::IsMenuItem<tauri::Wry>> = menu_items.iter().map(|item| item.as_ref()).collect();
    Ok(Submenu::with_items(app, submenu_title, true, &menu_refs)?)
}

/// 刷新托盘菜单
/// 重新构建配置子菜单并更新托盘菜单
pub fn refresh_tray_menu(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let profile_submenu = build_profile_submenu(app)?;

    let manage_profiles_item = MenuItem::with_id(app, "manage_profiles", "管理 API 配置", true, None::<&str>)?;
    let separator1 = PredefinedMenuItem::separator(app)?;
    let separator2 = PredefinedMenuItem::separator(app)?;
    let show_item = MenuItem::with_id(app, "show", "显示宠物", true, None::<&str>)?;
    let hide_item = MenuItem::with_id(app, "hide", "隐藏宠物", true, None::<&str>)?;
    let refresh_item = MenuItem::with_id(app, "refresh", "刷新数据", true, None::<&str>)?;
    let settings_item = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[
        &profile_submenu,
        &manage_profiles_item,
        &separator1,
        &show_item,
        &hide_item,
        &refresh_item,
        &separator2,
        &settings_item,
        &quit_item,
    ])?;

    if let Some(tray) = app.tray_by_id("main-tray") {
        tray.set_menu(Some(menu))?;
    }

    Ok(())
}

/// 设置托盘菜单事件处理器
/// 注册 on_menu_event 和 on_tray_icon_event 处理器
/// 此函数应该在 create_system_tray 之后调用，且只应调用一次
pub fn setup_tray_menu_handler(app: &AppHandle) {
    // 处理托盘菜单事件
    app.on_menu_event(|app, event| {
        let id = event.id.as_ref();
        match id {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "hide" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }
            "refresh" => {
                // 触发数据刷新
                let app_handle = app.clone();
                tauri::async_runtime::spawn(async move {
                    crate::polling::emit_usage(&app_handle).await;
                });
            }
            "settings" => {
                // 打开设置窗口
                if let Some(window) = app.get_webview_window("settings") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "manage_profiles" => {
                // 打开设置窗口（配置管理在设置中）
                if let Some(window) = app.get_webview_window("settings") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {
                // 处理配置切换事件（id 格式为 "profile_{id}"）
                if let Some(profile_id) = id.strip_prefix("profile_") {
                    let profile_id = profile_id.to_string();
                    let app_handle_for_handler = app.clone();
                    let app_handle_for_refresh = app.clone();
                    tauri::async_runtime::spawn(async move {
                        match crate::claude_profile::switch_profile_handler(app_handle_for_handler, profile_id).await {
                            Ok(_) => {
                                // 刷新托盘菜单以更新勾选状态
                                if let Err(e) = refresh_tray_menu(&app_handle_for_refresh) {
                                    eprintln!("Failed to refresh tray menu: {}", e);
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to switch profile: {}", e);
                            }
                        }
                    });
                }
            }
        }
    });

    // 处理托盘图标点击事件
    let app_handle = app.clone();
    app.on_tray_icon_event(move |_app, event| {
        match event {
            TrayIconEvent::Click {
                id: _,
                position: _,
                rect: _,
                button,
                button_state: _,
            } => {
                if button == MouseButton::Left {
                    // 左键点击切换主窗口显示/隐藏
                    if let Some(window) = app_handle.get_webview_window("main") {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
            }
            _ => {}
        }
    });
}
