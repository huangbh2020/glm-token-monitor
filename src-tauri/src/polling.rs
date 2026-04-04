use crate::events::{ApiResponse, UsageData, EVENT_USAGE_UPDATE};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter};
use tokio::time::interval;

const API_BASE: &str = "https://open.bigmodel.cn/api/monitor/usage/quota/limit";
// TODO: 生产环境应从配置文件或环境变量读取
const AUTH_TOKEN: &str = "5eaeeccb0b654f3b9309ca932d8a2425.a1jmribMBQlQ1yjx";

/// 格式化时间为 API 需要的格式：YYYY-MM-DD HH:MM:SS
fn format_time(secs: i64) -> String {
    if let Some(datetime) = chrono::DateTime::from_timestamp(secs, 0) {
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    } else {
        String::new()
    }
}

/// 请求真实 API 获取用量数据
pub async fn fetch_usage() -> Result<UsageData, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    // 计算时间：当前时间和昨天当前时间
    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Failed to get current time: {}", e))?
        .as_secs() as i64;
    let yesterday_secs = now_secs - 86400; // 24 小时前

    let end_time = format_time(now_secs);
    let start_time = format_time(yesterday_secs);

    let response = client
        .get(API_BASE)
        .query(&[("startTime", start_time), ("endTime", end_time)])
        .header("Authorization", AUTH_TOKEN)
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    let api_resp: ApiResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if !api_resp.success {
        return Err(format!("API returned error code: {}", api_resp.code));
    }

    let data = api_resp.data.ok_or("API response missing data field")?;

    let mut time_percent: u32 = 0;
    let mut time_remaining: Option<u32> = None;
    let mut time_reset_time: Option<i64> = None;
    let mut tokens_percent: u32 = 0;
    let mut tokens_reset_time: Option<i64> = None;
    let mut usage_details: Vec<crate::events::UsageDetailData> = Vec::new();

    for item in &data.limits {
        match item.limit_type.as_str() {
            "TIME_LIMIT" => {
                time_percent = item.percentage.unwrap_or(0);
                time_remaining = item.remaining;
                time_reset_time = item.next_reset_time;
                // 提取工具使用详情
                if let Some(details) = &item.usage_details {
                    for detail in details {
                        usage_details.push(crate::events::UsageDetailData {
                            model_code: detail.model_code.clone(),
                            usage: detail.usage,
                        });
                    }
                }
            }
            "TOKENS_LIMIT" => {
                tokens_percent = item.percentage.unwrap_or(0);
                tokens_reset_time = item.next_reset_time;
            }
            _ => {}
        }
    }

    Ok(UsageData {
        // 宠物状态由 5h Token 额度（tokens_percent）驱动
        used: tokens_percent,
        total: 100,
        time_percent,
        tokens_percent,
        time_remaining,
        tokens_reset_time,
        time_reset_time,
        level: data.level,
        usage_details,
    })
}

/// 启动轮询循环（每 5 分钟刷新一次）
pub async fn start_polling_loop(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 启动时立即拉取一次，无需等待 5 分钟
    emit_usage(&app).await;

    let mut timer = interval(Duration::from_secs(300)); // 5 分钟
    timer.tick().await; // 跳过第一个立即触发的 tick

    loop {
        timer.tick().await;
        emit_usage(&app).await;
    }
}

pub async fn emit_usage(app: &AppHandle) {
    match fetch_usage().await {
        Ok(data) => {
            if let Err(e) = app.emit(EVENT_USAGE_UPDATE, data) {
                eprintln!("Failed to emit usage update: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch usage: {}", e);
            let _ = app.emit("usage-error", format!("Usage fetch failed: {}", e));
        }
    }
}
