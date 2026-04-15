use serde::{Deserialize, Serialize};

/// 事件名称常量
pub const EVENT_USAGE_UPDATE: &str = "usage-update";

/// 推送给前端的使用量数据（驱动宠物状态 + 信息面板）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageData {
    /// 5h Token 已用百分比（驱动宠物状态，0-100）
    pub used: u32,
    /// 固定为 100
    pub total: u32,
    /// 月 MCP 额度百分比（TIME_LIMIT.percentage）
    pub time_percent: u32,
    /// 5h Token 额度百分比（TOKENS_LIMIT unit=3 percentage）
    pub tokens_percent: u32,
    /// 周 Token 额度百分比（TOKENS_LIMIT unit=6 percentage）
    pub weekly_tokens_percent: u32,
    /// 月 MCP 剩余次数
    pub time_remaining: Option<u32>,
    /// 5h Token 下次重置时间（时间戳，毫秒）
    pub tokens_reset_time: Option<i64>,
    /// 周 Token 下次重置时间（时间戳，毫秒）
    pub weekly_tokens_reset_time: Option<i64>,
    /// 月度额度下次重置时间（时间戳，毫秒）
    pub time_reset_time: Option<i64>,
    /// 会员等级
    pub level: String,
    /// 工具使用详情
    pub usage_details: Vec<UsageDetailData>,
}

/// 工具使用详情（用于前端显示）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageDetailData {
    pub model_code: String,
    pub usage: u32,
}

impl UsageData {
    pub fn percent(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        (self.used as f64 / self.total as f64) * 100.0
    }
}

// ── API 响应结构（反序列化用） ──

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub code: u32,
    pub data: Option<ApiData>,
    pub success: bool,
}

#[derive(Debug, Deserialize)]
pub struct ApiData {
    pub limits: Vec<LimitItem>,
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct LimitItem {
    #[serde(rename = "type")]
    pub limit_type: String,
    pub unit: Option<u32>,
    pub percentage: Option<u32>,
    pub usage: Option<u32>,
    pub number: Option<u32>,
    pub remaining: Option<u32>,
    #[serde(alias = "nextResetTime")]
    pub next_reset_time: Option<i64>,
    #[serde(alias = "usageDetails")]
    pub usage_details: Option<Vec<UsageDetail>>,
}

#[derive(Debug, Deserialize)]
pub struct UsageDetail {
    #[serde(alias = "modelCode")]
    pub model_code: String,
    pub usage: u32,
}
