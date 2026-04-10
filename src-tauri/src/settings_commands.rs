use crate::config::{load_config, save_config, AppConfig};
use tauri::{AppHandle, Emitter};

/// 获取当前配置
#[tauri::command]
pub async fn get_config(app: AppHandle) -> Result<AppConfig, String> {
    load_config(&app)
}

/// 保存配置
#[tauri::command]
pub async fn save_config_handler(
    app: AppHandle,
    config: AppConfig,
) -> Result<(), String> {
    save_config(&app, &config)?;

    // 触发配置变更事件
    let _ = app.emit("config-changed", &config);

    Ok(())
}

/// 设置开机自启动
#[tauri::command]
pub async fn set_auto_start(app: AppHandle, enabled: bool) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        use tauri::utils::platform::current_exe;

        let exe_path = current_exe()
            .map_err(|e| format!("Failed to get executable path: {}", e))?;

        let exe_path_str = exe_path.to_string_lossy().to_string();

        // 使用 PowerShell 设置注册表启动项
        let ps_script = if enabled {
            format!(
                "New-ItemProperty -Path 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Run' -Name 'PlanGuard' -Value '{}' -PropertyType String -Force",
                exe_path_str.replace("\\", "\\\\")
            )
        } else {
            "Remove-ItemProperty -Path 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Run' -Name 'PlanGuard' -ErrorAction SilentlyContinue".to_string()
        };

        let result = std::process::Command::new("powershell")
            .args(["-Command", &ps_script])
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .output()
            .map_err(|e| format!("Failed to execute PowerShell: {}", e))?;

        if !result.status.success() {
            let error = String::from_utf8_lossy(&result.stderr);
            return Err(format!("PowerShell command failed: {}", error));
        }

        Ok(())
    }

    #[cfg(target_os = "macos")]
    {
        use std::fs;
        use tauri::utils::platform::current_exe;

        let exe_path = current_exe()
            .map_err(|e| format!("Failed to get executable path: {}", e))?;

        let plist_path = dirs::home_dir()
            .ok_or_else(|| "Failed to get home directory".to_string())?
            .join("Library/LaunchAgents/com.planguard.app.plist");

        if enabled {
            // 创建 LaunchAgent plist 文件
            let plist_content = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.planguard.app</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>
"#,
                exe_path.to_string_lossy()
            );

            fs::write(&plist_path, plist_content)
                .map_err(|e| format!("Failed to write LaunchAgent plist: {}", e))?;
        } else {
            // 删除 LaunchAgent plist 文件
            let _ = fs::remove_file(&plist_path);
        }

        Ok(())
    }

    #[cfg(target_os = "linux")]
    {
        use std::fs;
        use tauri::utils::platform::current_exe;

        let exe_path = current_exe()
            .map_err(|e| format!("Failed to get executable path: {}", e))?;

        let autostart_dir = dirs::home_dir()
            .ok_or_else(|| "Failed to get home directory".to_string())?
            .join(".config/autostart");

        if !autostart_dir.exists() {
            fs::create_dir_all(&autostart_dir)
                .map_err(|e| format!("Failed to create autostart directory: {}", e))?;
        }

        let desktop_file = autostart_dir.join("planguard.desktop");

        if enabled {
            let desktop_content = format!(
                r#"[Desktop Entry]
Type=Application
Name=PlanGuard
Exec={}
Icon=planguard
Terminal=false
X-MultipleArgs=false
Categories=Utility;
"#,
                exe_path.to_string_lossy()
            );

            fs::write(&desktop_file, desktop_content)
                .map_err(|e| format!("Failed to write desktop file: {}", e))?;
        } else {
            let _ = fs::remove_file(&desktop_file);
        }

        Ok(())
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err("Auto-start is not supported on this platform".to_string())
    }
}

/// 测试 API 连接
#[tauri::command]
pub async fn test_api_connection(
    api_url: String,
    api_key: String,
) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to build client: {}", e))?;

    let response = client
        .get(&api_url)
        .header("Authorization", api_key)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.status().is_success() {
        Ok("Connection successful".to_string())
    } else {
        Err(format!("API returned error: {}", response.status()))
    }
}
