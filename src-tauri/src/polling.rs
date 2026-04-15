use crate::events::{ApiResponse, UsageData, EVENT_USAGE_UPDATE};
use crate::config::get_active_model_config;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter};
use tokio::time::interval;

/// 格式化时间为 API 需要的格式：YYYY-MM-DD HH:MM:SS
fn format_time(secs: i64) -> String {
    if let Some(datetime) = chrono::DateTime::from_timestamp(secs, 0) {
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    } else {
        String::new()
    }
}

/// 请求真实 API 获取用量数据
pub async fn fetch_usage(app: &AppHandle) -> Result<UsageData, String> {
    // 从配置读取 API 参数
    let model_config = get_active_model_config(app)?;
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
        .get(&model_config.api_url())
        .query(&[("startTime", start_time), ("endTime", end_time)])
        .header("Authorization", &model_config.api_key)
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
    let mut weekly_tokens_percent: u32 = 0;
    let mut weekly_tokens_reset_time: Option<i64> = None;
    let mut usage_details: Vec<crate::events::UsageDetailData> = Vec::new();

    for item in &data.limits {
        let unit = item.unit.unwrap_or(0);
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
                match unit {
                    6 => {
                        // 周 Token 限制
                        weekly_tokens_percent = item.percentage.unwrap_or(0);
                        weekly_tokens_reset_time = item.next_reset_time;
                    }
                    _ => {
                        // 5h Token 限制（unit=3 或其他）
                        tokens_percent = item.percentage.unwrap_or(0);
                        tokens_reset_time = item.next_reset_time;
                    }
                }
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
        weekly_tokens_percent,
        time_remaining,
        tokens_reset_time,
        weekly_tokens_reset_time,
        time_reset_time,
        level: data.level,
        usage_details,
    })
}

/// 启动轮询循环（从配置读取间隔）
pub async fn start_polling_loop(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 启动时立即拉取一次
    emit_usage(&app).await;

    // 从配置读取轮询间隔
    let config = crate::config::load_config(&app)?;
    let interval_secs = config.polling_config.interval_minutes * 60;

    let mut timer = interval(Duration::from_secs(interval_secs));
    timer.tick().await; // 跳过第一个立即触发的 tick

    loop {
        timer.tick().await;
        emit_usage(&app).await;
    }
}

pub async fn emit_usage(app: &AppHandle) {
    match fetch_usage(app).await {
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
