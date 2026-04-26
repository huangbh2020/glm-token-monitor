use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter, Manager};

const PROFILES_FILE: &str = "claude-api-profiles.json";
const META_FILE: &str = "_meta.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetaConfig {
    #[serde(default)]
    pub applied_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeApiProfile {
    pub id: String,
    pub name: String,
    pub inference_gateway_base_url: String,
    pub inference_gateway_api_key: String,
    pub inference_models: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileStore {
    #[serde(default)]
    pub profiles: Vec<ClaudeApiProfile>,
    #[serde(default)]
    pub active_profile_id: Option<String>,
    #[serde(default)]
    pub claude_config_path: Option<String>,
}

impl Default for ProfileStore {
    fn default() -> Self {
        Self {
            profiles: Vec::new(),
            active_profile_id: None,
            claude_config_path: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeConfigFields {
    pub inference_gateway_base_url: String,
    pub inference_gateway_api_key: String,
    pub inference_models: Vec<String>,
}

fn profiles_file_path(app: &AppHandle, store: Option<&ProfileStore>) -> Result<PathBuf, String> {
    // 优先使用 store 中配置的 Claude 配置路径的父目录
    let target_dir = if let Some(ref store) = store {
        if let Some(ref path) = store.claude_config_path {
            let p = PathBuf::from(path);
            if let Some(parent) = p.parent() {
                parent.to_path_buf()
            } else {
                p
            }
        } else {
            auto_detect_claude_config_dir()
        }
    } else {
        // 尝试从默认位置加载 store 来确定路径
        let default_path = default_profiles_file_path(app)?;
        if let Ok(content) = fs::read_to_string(&default_path) {
            if let Ok(store) = serde_json::from_str::<ProfileStore>(&content) {
                if let Some(ref path) = store.claude_config_path {
                    if let Some(parent) = PathBuf::from(path).parent() {
                        parent.to_path_buf()
                    } else {
                        auto_detect_claude_config_dir()
                    }
                } else {
                    auto_detect_claude_config_dir()
                }
            } else {
                auto_detect_claude_config_dir()
            }
        } else {
            auto_detect_claude_config_dir()
        }
    };

    if !target_dir.exists() {
        fs::create_dir_all(&target_dir)
            .map_err(|e| format!("Failed to create config dir: {}", e))?;
    }
    Ok(target_dir.join(PROFILES_FILE))
}

fn default_profiles_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("Failed to get config dir: {}", e))?;
    Ok(config_dir.join(PROFILES_FILE))
}

fn auto_detect_claude_config_dir() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let base = home.join("Library/Application Support/Claude-3p/configLibrary");
    if base.exists() {
        base
    } else {
        home.join(".config")
    }
}

pub fn load_profiles(app: &AppHandle) -> Result<ProfileStore, String> {
    // 先尝试从默认位置加载（向后兼容）
    let default_path = default_profiles_file_path(app)?;
    if default_path.exists() {
        let content = fs::read_to_string(&default_path)
            .map_err(|e| format!("Failed to read profiles file: {}", e))?;
        let store: ProfileStore = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse profiles: {}", e))?;
        // 迁移到新的位置
        save_profiles(app, &store)?;
        // 删除旧文件
        let _ = fs::remove_file(&default_path);
        return Ok(store);
    }

    // 从新位置加载
    let path = profiles_file_path(app, None)?;
    if !path.exists() {
        let store = ProfileStore::default();
        save_profiles(app, &store)?;
        return Ok(store);
    }
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read profiles file: {}", e))?;
    let mut store: ProfileStore = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse profiles: {}", e))?;

    // 从 _meta.json 同步 applied_id
    let meta = load_meta(&store)?;
    if let Some(applied_id) = meta.applied_id {
        // 检查 profile 是否存在
        if store.profiles.iter().any(|p| p.id == applied_id) {
            store.active_profile_id = Some(applied_id);
        }
    }

    Ok(store)
}

pub fn save_profiles(app: &AppHandle, store: &ProfileStore) -> Result<(), String> {
    let path = profiles_file_path(app, Some(store))?;
    let content = serde_json::to_string_pretty(store)
        .map_err(|e| format!("Failed to serialize profiles: {}", e))?;
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write profiles file: {}", e))?;
    Ok(())
}

fn meta_file_path(store: &ProfileStore) -> Result<PathBuf, String> {
    let claude_dir = if let Some(ref path) = store.claude_config_path {
        PathBuf::from(path)
            .parent()
            .map(|p| p.to_path_buf())
            .ok_or_else(|| "Invalid Claude config path".to_string())?
    } else {
        auto_detect_claude_config_dir()
    };
    Ok(claude_dir.join(META_FILE))
}

pub fn load_meta(store: &ProfileStore) -> Result<MetaConfig, String> {
    let path = meta_file_path(store)?;
    if !path.exists() {
        return Ok(MetaConfig::default());
    }
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read meta file: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("Failed to parse meta: {}", e))
}

pub fn save_meta(store: &ProfileStore, meta: &MetaConfig) -> Result<(), String> {
    let path = meta_file_path(store)?;
    let content = serde_json::to_string_pretty(meta)
        .map_err(|e| format!("Failed to serialize meta: {}", e))?;
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write meta file: {}", e))?;
    Ok(())
}

fn resolve_claude_config_path(store: &ProfileStore) -> Result<PathBuf, String> {
    if let Some(ref path) = store.claude_config_path {
        let p = PathBuf::from(path);
        if p.exists() {
            return Ok(p);
        }
    }
    // Auto-detect macOS default path
    let home = dirs::home_dir()
        .ok_or_else(|| "Failed to get home directory".to_string())?;
    let base = home.join("Library/Application Support/Claude-3p/configLibrary");
    if base.exists() {
        let entries = fs::read_dir(&base)
            .map_err(|e| format!("Failed to read Claude config dir: {}", e))?;
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json") {
                    return Ok(path);
                }
            }
        }
    }
    Err("Claude Code 配置文件未找到，请在设置中手动指定路径".to_string())
}

pub fn read_claude_config_fields(path: &Path) -> Result<ClaudeConfigFields, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read Claude config: {}", e))?;
    let json: Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse Claude config: {}", e))?;
    Ok(ClaudeConfigFields {
        inference_gateway_base_url: json
            .get("inferenceGatewayBaseUrl")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        inference_gateway_api_key: json
            .get("inferenceGatewayApiKey")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        inference_models: json
            .get("inferenceModels")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default(),
    })
}

pub fn write_claude_config_fields(path: &Path, fields: &ClaudeConfigFields) -> Result<(), String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read Claude config: {}", e))?;
    let mut json: Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse Claude config: {}", e))?;

    json["inferenceGatewayBaseUrl"] = Value::String(fields.inference_gateway_base_url.clone());
    json["inferenceGatewayApiKey"] = Value::String(fields.inference_gateway_api_key.clone());
    json["inferenceModels"] = serde_json::to_value(&fields.inference_models)
        .map_err(|e| format!("Failed to serialize models: {}", e))?;

    let updated = serde_json::to_string_pretty(&json)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    fs::write(path, updated)
        .map_err(|e| format!("Failed to write Claude config: {}", e))?;
    Ok(())
}

pub fn switch_profile(app: &AppHandle, profile_id: String) -> Result<(), String> {
    let mut store = load_profiles(app)?;
    let profile = store
        .profiles
        .iter()
        .find(|p| p.id == profile_id)
        .ok_or_else(|| format!("Profile not found: {}", profile_id))?
        .clone();

    let claude_path = resolve_claude_config_path(&store)?;
    let fields = ClaudeConfigFields {
        inference_gateway_base_url: profile.inference_gateway_base_url.clone(),
        inference_gateway_api_key: profile.inference_gateway_api_key.clone(),
        inference_models: profile.inference_models.clone(),
    };

    write_claude_config_fields(&claude_path, &fields)?;

    store.active_profile_id = Some(profile_id);
    save_profiles(app, &store)?;

    Ok(())
}

// Tauri Commands

#[tauri::command]
pub async fn get_profiles(app: AppHandle) -> Result<ProfileStore, String> {
    load_profiles(&app)
}

#[tauri::command]
pub async fn save_profile_handler(
    app: AppHandle,
    profile: ClaudeApiProfile,
) -> Result<ProfileStore, String> {
    let mut store = load_profiles(&app)?;
    if let Some(existing) = store.profiles.iter_mut().find(|p| p.id == profile.id) {
        *existing = profile;
    } else {
        store.profiles.push(profile);
    }
    save_profiles(&app, &store)?;
    Ok(store)
}

#[tauri::command]
pub async fn delete_profile_handler(
    app: AppHandle,
    id: String,
) -> Result<ProfileStore, String> {
    let mut store = load_profiles(&app)?;
    store.profiles.retain(|p| p.id != id);
    if store.active_profile_id.as_deref() == Some(&id) {
        store.active_profile_id = None;
    }
    save_profiles(&app, &store)?;
    Ok(store)
}

#[tauri::command]
pub async fn switch_profile_handler(app: AppHandle, id: String) -> Result<(), String> {
    switch_profile(&app, id)?;
    let store = load_profiles(&app)?;
    let _ = app.emit("claude-profile-changed", &store);
    Ok(())
}

#[tauri::command]
pub async fn get_claude_config_path_cmd(app: AppHandle) -> Result<String, String> {
    let store = load_profiles(&app)?;
    let path = resolve_claude_config_path(&store)?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn set_claude_config_path_handler(
    app: AppHandle,
    path: String,
) -> Result<(), String> {
    let mut store = load_profiles(&app)?;
    store.claude_config_path = Some(path);
    save_profiles(&app, &store)?;
    Ok(())
}
