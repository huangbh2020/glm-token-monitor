use serde::{Deserialize, Serialize};

/// 事件名称常量
pub const EVENT_USAGE_UPDATE: &str = "usage-update";

/// 使用量数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageData {
    pub used: u32,
    pub total: u32,
}

impl UsageData {
    /// 计算使用百分比
    pub fn percent(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        (self.used as f64 / self.total as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_calculation() {
        let data = UsageData { used: 50, total: 100 };
        assert_eq!(data.percent(), 50.0);
    }

    #[test]
    fn test_percent_zero_total() {
        let data = UsageData { used: 10, total: 0 };
        assert_eq!(data.percent(), 0.0);
    }
}
