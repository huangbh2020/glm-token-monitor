use crate::events::{ModelUsageData, UsageData};
use crate::polling::{fetch_model_usage, fetch_usage};
use tauri::AppHandle;

/// 手动获取当前使用量（前端触发，用于按需刷新）
#[tauri::command]
pub async fn get_current_usage(app: AppHandle) -> Result<UsageData, String> {
    fetch_usage(&app).await
}

/// 获取模型用量详情（按时间范围）
#[tauri::command]
pub async fn get_model_usage(
    app: AppHandle,
    time_range: String,
) -> Result<ModelUsageData, String> {
    let now = chrono::Local::now();
    let (start_time, end_time) = match time_range.as_str() {
        "today" => {
            let start = now.format("%Y-%m-%d 00:00:00").to_string();
            let end = now.format("%Y-%m-%d 23:59:59").to_string();
            (start, end)
        }
        "7days" => {
            let start = (now - chrono::Duration::days(6))
                .format("%Y-%m-%d 00:00:00")
                .to_string();
            let end = now.format("%Y-%m-%d 23:59:59").to_string();
            (start, end)
        }
        "30days" => {
            let start = (now - chrono::Duration::days(29))
                .format("%Y-%m-%d 00:00:00")
                .to_string();
            let end = now.format("%Y-%m-%d 23:59:59").to_string();
            (start, end)
        }
        _ => return Err(format!("Invalid time_range: {}", time_range)),
    };

    fetch_model_usage(&app, &start_time, &end_time).await
}
