use tauri::{WebviewWindow, Manager};

/// 打开信息面板窗口
#[tauri::command]
pub async fn open_info_panel(
    main_window: WebviewWindow,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    // 获取或创建 info-panel 窗口
    let info_panel = match app_handle.get_webview_window("info-panel") {
        Some(window) => window,
        None => return Err("Info panel window not found".to_string()),
    };

    // 获取主窗口位置（使用逻辑坐标）
    let main_pos = main_window
        .outer_position()
        .map_err(|e| format!("Failed to get main window position: {}", e))?;
    let scale_factor = main_window
        .current_monitor()
        .map_err(|e| format!("Failed to get monitor: {}", e))?
        .ok_or("Failed to get monitor")?
        .scale_factor();
    let main_pos_logical: tauri::LogicalPosition<f64> = main_pos.to_logical(scale_factor);

    // 获取主窗口实际大小（使用逻辑坐标）
    let main_size = main_window
        .inner_size()
        .map_err(|e| format!("Failed to get main window size: {}", e))?;
    let main_size_logical: tauri::LogicalSize<f64> = main_size.to_logical(scale_factor);

    // 计算副窗口尺寸和间距
    let panel_width = 280.0;
    let panel_height = 320.0;
    let gap = 8.0;

    // 获取主显示器信息
    let (screen_width, screen_height) = match main_window.current_monitor() {
        Ok(Some(monitor)) => {
            let size = monitor.size();
            let logical = size.to_logical(monitor.scale_factor());
            (logical.width, logical.height)
        }
        _ => (1920.0, 1080.0), // 默认分辨率
    };

    // 计算右侧位置
    let right_x = main_pos_logical.x + main_size_logical.width + gap;
    let can_fit_right = right_x + panel_width <= screen_width;

    // 计算左侧位置
    let left_x = main_pos_logical.x - panel_width - gap;
    let can_fit_left = left_x >= 0.0;

    // 选择最佳位置：优先右侧，其次左侧，都不可行则选择显示更多的
    let info_x: f64 = if can_fit_right {
        right_x
    } else if can_fit_left {
        left_x
    } else {
        // 两侧都无法完整显示，选择显示更多的一侧
        let right_visible = screen_width - right_x;
        let left_visible = left_x + panel_width;
        if right_visible >= left_visible {
            right_x
        } else {
            left_x
        }
    };

    // 计算垂直位置，确保不超出屏幕
    let mut info_y: f64 = main_pos_logical.y;
    if info_y + panel_height > screen_height {
        // 底部超出，贴底显示
        info_y = screen_height - panel_height;
    }
    if info_y < 0.0 {
        info_y = 0.0;
    }

    // 设置窗口位置（使用逻辑坐标）
    info_panel
        .set_position(tauri::Position::Logical(tauri::LogicalPosition {
            x: info_x.max(0.0),
            y: info_y.max(0.0),
        }))
        .map_err(|e| format!("Failed to set info panel position: {}", e))?;

    // 显示窗口
    info_panel.show().map_err(|e| format!("Failed to show info panel: {}", e))?;

    // 触发数据刷新（异步，不阻塞窗口显示）
    let app_handle_for_refresh = app_handle.clone();
    tokio::spawn(async move {
        crate::polling::emit_usage(&app_handle_for_refresh).await;
    });

    // 设置焦点
    info_panel.set_focus().map_err(|e| format!("Failed to focus info panel: {}", e))?;

    Ok(())
}

/// 关闭信息面板窗口
#[tauri::command]
pub async fn close_info_panel(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(info_panel) = app_handle.get_webview_window("info-panel") {
        info_panel.hide().map_err(|e| format!("Failed to hide info panel: {}", e))?;
    }
    Ok(())
}
