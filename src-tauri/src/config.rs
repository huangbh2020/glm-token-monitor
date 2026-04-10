use serde::{Deserialize, Serialize};
use std::fs;
use tauri::{AppHandle, Manager};

const CONFIG_FILE: &str = "config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub api_config: ApiConfig,
    pub polling_config: PollingConfig,
    #[serde(default)]
    pub display_config: DisplayConfig,
    #[serde(default)]
    pub pet_config: PetConfig,
    #[serde(default)]
    pub basic_config: BasicConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub display_mode: String,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            display_mode: "holo-bubble".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetConfig {
    pub selected_pet: String,
    pub action_interval: u64,
}

impl Default for PetConfig {
    fn default() -> Self {
        Self {
            selected_pet: "cat".to_string(),
            action_interval: 25,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicConfig {
    #[serde(default = "default_enable_glow")]
    pub enable_glow: bool,
    #[serde(default)]
    pub auto_start: bool,
    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_enable_glow() -> bool {
    true
}

fn default_theme() -> String {
    "dark".to_string()
}

impl Default for BasicConfig {
    fn default() -> Self {
        Self {
            enable_glow: true,
            auto_start: false,
            theme: "dark".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub current_model: String,
    pub models: Vec<ModelConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub provider: String,
    pub name: String,
    pub api_key: String,
    pub enabled: bool,
}

impl ModelConfig {
    /// 获取 API 域名（硬编码）
    pub fn api_domain(&self) -> &'static str {
        match self.provider.as_str() {
            "bigmodel" => "https://open.bigmodel.cn/",
            "zai" => "https://api.z.ai/",
            _ => "https://open.bigmodel.cn/",
        }
    }

    /// 获取完整的 API URL
    pub fn api_url(&self) -> String {
        format!("{}api/monitor/usage/quota/limit", self.api_domain())
    }
}

// 旧版配置结构（用于迁移）
#[derive(Debug, Clone, Deserialize)]
struct LegacyModelConfig {
    pub provider: String,
    pub name: String,
    pub api_url: String,
    pub api_key: String,
    pub enabled: bool,
}

impl From<LegacyModelConfig> for ModelConfig {
    fn from(legacy: LegacyModelConfig) -> Self {
        // 从完整 URL 中提取域名并丢弃（使用硬编码的域名）
        Self {
            provider: legacy.provider,
            name: legacy.name,
            api_key: legacy.api_key,
            enabled: legacy.enabled,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollingConfig {
    pub interval_minutes: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_config: ApiConfig::default(),
            polling_config: PollingConfig::default(),
            display_config: DisplayConfig::default(),
            pet_config: PetConfig::default(),
            basic_config: BasicConfig::default(),
        }
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            current_model: "bigmodel".to_string(),
            models: vec![
                ModelConfig {
                    provider: "bigmodel".to_string(),
                    name: "BigModel".to_string(),
                    api_key: "".to_string(),
                    enabled: true,
                },
                ModelConfig {
                    provider: "zai".to_string(),
                    name: "Z.AI".to_string(),
                    api_key: "".to_string(),
                    enabled: true,
                },
            ],
        }
    }
}

impl Default for PollingConfig {
    fn default() -> Self {
        Self {
            interval_minutes: 1,
        }
    }
}

pub fn load_config(app: &AppHandle) -> Result<AppConfig, String> {
    let config_path = app.path().app_config_dir()
        .map_err(|e| format!("Failed to get config dir: {}", e))?;

    if !config_path.exists() {
        fs::create_dir_all(&config_path)
            .map_err(|e| format!("Failed to create config dir: {}", e))?;
    }

    let config_file = config_path.join(CONFIG_FILE);

    if !config_file.exists() {
        let default_config = AppConfig::default();
        save_config(app, &default_config)?;
        return Ok(default_config);
    }

    let content = fs::read_to_string(&config_file)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    // 尝试解析新格式
    match serde_json::from_str::<AppConfig>(&content) {
        Ok(config) => Ok(config),
        Err(_) => {
            // 新格式解析失败，尝试迁移旧格式
            #[derive(Deserialize)]
            struct LegacyAppConfig {
                pub api_config: LegacyApiConfig,
                pub polling_config: PollingConfig,
                pub display_config: Option<DisplayConfig>,
            }
            #[derive(Deserialize)]
            struct LegacyApiConfig {
                pub current_model: String,
                pub models: Vec<LegacyModelConfig>,
            }

            if let Ok(legacy_config) = serde_json::from_str::<LegacyAppConfig>(&content) {
                // 迁移旧配置到新格式
                let migrated_config = AppConfig {
                    api_config: ApiConfig {
                        current_model: legacy_config.api_config.current_model,
                        models: legacy_config.api_config.models
                            .into_iter()
                            .map(|m| ModelConfig::from(m))
                            .collect(),
                    },
                    polling_config: legacy_config.polling_config,
                    display_config: legacy_config.display_config.unwrap_or_default(),
                    pet_config: PetConfig::default(),
                    basic_config: BasicConfig::default(),
                };
                // 保存迁移后的配置
                save_config(app, &migrated_config)?;
                Ok(migrated_config)
            } else {
                Err("Failed to parse config: invalid format".to_string())
            }
        }
    }
}

pub fn save_config(app: &AppHandle, config: &AppConfig) -> Result<(), String> {
    let config_path = app.path().app_config_dir()
        .map_err(|e| format!("Failed to get config dir: {}", e))?;
    let config_file = config_path.join(CONFIG_FILE);

    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(&config_file, content)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    Ok(())
}

pub fn get_active_model_config(app: &AppHandle) -> Result<ModelConfig, String> {
    let config = load_config(app)?;
    config.api_config.models
        .into_iter()
        .find(|m| m.provider == config.api_config.current_model)
        .ok_or_else(|| "Active model not found".to_string())
}
