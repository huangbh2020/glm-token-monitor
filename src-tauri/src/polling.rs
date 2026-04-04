use crate::events::{UsageData, EVENT_USAGE_UPDATE};
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::time::interval;

/// Mock 数据 - 返回固定的使用量数据
async fn fetch_usage() -> Result<UsageData, String> {
    // 模拟 API 请求延迟
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Mock 数据：65% 使用量
    Ok(UsageData {
        used: 65,
        total: 100,
    })
}

/// 启动轮询循环
pub async fn start_polling_loop(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let mut timer = interval(Duration::from_secs(300)); // 5 分钟

    loop {
        timer.tick().await;

        match fetch_usage().await {
            Ok(data) => {
                if let Err(e) = app.emit(EVENT_USAGE_UPDATE, data) {
                    eprintln!("Failed to emit usage update: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to fetch usage: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_usage() {
        let result = fetch_usage().await;
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.used, 65);
        assert_eq!(data.total, 100);
    }
}
