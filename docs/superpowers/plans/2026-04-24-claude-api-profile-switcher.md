# Claude API 配置切换功能实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 在 plan-guard 中新增 Claude Code 第三方 API 配置管理功能，支持保存多套 API 配置并通过托盘菜单一键切换。

**Architecture:** Rust 端新增 `claude_profile.rs` 模块管理 profile 数据和 Claude Code 配置文件的读写切换。托盘菜单动态生成子菜单用于快速切换。Vue 设置面板新增 "API 配置" 标签页管理配置的增删改。

**Tech Stack:** Rust (serde, serde_json, uuid), Vue 3 + TypeScript, Tauri 2.0

---

## 文件结构

| 操作 | 文件 | 职责 |
|------|------|------|
| 创建 | `src-tauri/src/claude_profile.rs` | Profile 数据结构、文件读写、切换逻辑、Tauri Commands |
| 修改 | `src-tauri/src/main.rs` | 注册新模块和新 Commands |
| 修改 | `src-tauri/src/tray.rs` | 托盘菜单新增动态子菜单 |
| 修改 | `src-tauri/Cargo.toml` | 添加 uuid 依赖 |
| 创建 | `src/composables/useClaudeProfiles.ts` | Vue 端 profile 管理 composable |
| 修改 | `src/types/config.ts` | 添加 profile 相关类型 |
| 修改 | `src/components/SettingsPanel.vue` | 新增 "API 配置" 标签页 |

---

### Task 1: 添加 Rust 依赖

**Files:**
- Modify: `src-tauri/Cargo.toml`

- [ ] **Step 1: 在 Cargo.toml 中添加 uuid 依赖**

在 `[dependencies]` 中添加：

```toml
uuid = { version = "1", features = ["v4"] }
```

- [ ] **Step 2: 验证编译**

Run: `cd src-tauri && cargo check`
Expected: 编译成功（无新 warning 或 error）

- [ ] **Step 3: 提交**

```bash
git add src-tauri/Cargo.toml
git commit -m "chore: 添加 uuid 依赖用于 profile ID 生成"
```

---

### Task 2: 创建 claude_profile.rs — 数据结构与文件读写

**Files:**
- Create: `src-tauri/src/claude_profile.rs`

- [ ] **Step 1: 创建数据结构和文件读写函数**

创建 `src-tauri/src/claude_profile.rs`，包含完整的数据结构定义和文件 I/O：

```rust
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

const PROFILES_FILE: &str = "claude-api-profiles.json";

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

fn profiles_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    let config_dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("Failed to get config dir: {}", e))?;
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Failed to create config dir: {}", e))?;
    }
    Ok(config_dir.join(PROFILES_FILE))
}

pub fn load_profiles(app: &AppHandle) -> Result<ProfileStore, String> {
    let path = profiles_file_path(app)?;
    if !path.exists() {
        let store = ProfileStore::default();
        save_profiles(app, &store)?;
        return Ok(store);
    }
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read profiles file: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("Failed to parse profiles: {}", e))
}

pub fn save_profiles(app: &AppHandle, store: &ProfileStore) -> Result<(), String> {
    let path = profiles_file_path(app)?;
    let content = serde_json::to_string_pretty(store)
        .map_err(|e| format!("Failed to serialize profiles: {}", e))?;
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write profiles file: {}", e))?;
    Ok(())
}
```

- [ ] **Step 2: 验证编译**

Run: `cd src-tauri && cargo check`
Expected: 编译成功

- [ ] **Step 3: 提交**

```bash
git add src-tauri/src/claude_profile.rs
git commit -m "feat: 创建 claude_profile 模块 — 数据结构与文件读写"
```

---

### Task 3: 实现 Claude Code 配置文件切换逻辑

**Files:**
- Modify: `src-tauri/src/claude_profile.rs`

- [ ] **Step 1: 添加切换函数和 Claude Code 配置读写逻辑**

在 `claude_profile.rs` 末尾追加：

```rust
use serde_json::Value;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeConfigFields {
    pub inference_gateway_base_url: String,
    pub inference_gateway_api_key: String,
    pub inference_models: Vec<String>,
}

fn resolve_claude_config_path(store: &ProfileStore) -> Result<PathBuf, String> {
    if let Some(ref path) = store.claude_config_path {
        let p = PathBuf::from(path);
        if p.exists() {
            return Ok(p);
        }
    }
    // 自动探测 macOS 默认路径
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
```

- [ ] **Step 2: 验证编译**

Run: `cd src-tauri && cargo check`
Expected: 编译成功

- [ ] **Step 3: 提交**

```bash
git add src-tauri/src/claude_profile.rs
git commit -m "feat: 实现 Claude Code 配置切换逻辑"
```

---

### Task 4: 注册 Tauri Commands

**Files:**
- Modify: `src-tauri/src/claude_profile.rs` (追加 Commands)
- Modify: `src-tauri/src/main.rs` (注册模块和 Commands)

- [ ] **Step 1: 在 claude_profile.rs 末尾追加 Tauri Command 函数**

```rust
use tauri::Emitter;

#[derive(Debug, Clone, Serialize)]
pub struct ProfileSwitchResult {
    pub active_profile_id: String,
    pub profile_name: String,
}

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
```

- [ ] **Step 2: 修改 main.rs 注册模块和 Commands**

在 `main.rs` 顶部 `mod` 声明中添加：

```rust
mod claude_profile;
```

在 `invoke_handler` 宏中追加：

```rust
claude_profile::get_profiles,
claude_profile::save_profile_handler,
claude_profile::delete_profile_handler,
claude_profile::switch_profile_handler,
claude_profile::get_claude_config_path_cmd,
claude_profile::set_claude_config_path_handler,
```

- [ ] **Step 3: 验证编译**

Run: `cd src-tauri && cargo check`
Expected: 编译成功

- [ ] **Step 4: 提交**

```bash
git add src-tauri/src/claude_profile.rs src-tauri/src/main.rs
git commit -m "feat: 注册 Claude profile Tauri Commands"
```

---

### Task 5: 托盘菜单添加动态 API 配置切换子菜单

**Files:**
- Modify: `src-tauri/src/tray.rs`

- [ ] **Step 1: 重构 tray.rs 使用动态菜单构建**

将 `tray.rs` 的 `create_system_tray` 函数改为构建基础菜单（不含 profile 子菜单），在 `on_menu_event` 中处理 profile 切换事件。新增 `update_tray_menu` 函数供外部调用以动态刷新菜单。

完整替换 `tray.rs` 内容为：

```rust
use tauri::{AppHandle, Manager};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu, CheckMenuItem};
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};

pub fn create_system_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show_item = MenuItem::with_id(app, "show", "显示宠物", true, None::<&str>)?;
    let hide_item = MenuItem::with_id(app, "hide", "隐藏宠物", true, None::<&str>)?;
    let refresh_item = MenuItem::with_id(app, "refresh", "刷新数据", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let switch_submenu = build_profile_submenu(app)?;
    let manage_profiles = MenuItem::with_id(app, "manage_profiles", "管理 API 配置", true, None::<&str>)?;
    let separator2 = PredefinedMenuItem::separator(app)?;
    let settings_item = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
    let separator3 = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[
        &show_item,
        &hide_item,
        &refresh_item,
        &separator,
        &switch_submenu,
        &manage_profiles,
        &separator2,
        &settings_item,
        &separator3,
        &quit_item,
    ])?;

    let icon = app.default_window_icon().ok_or("Failed to get default icon")?.clone();

    let _tray = TrayIconBuilder::with_id("main-tray")
        .icon(icon)
        .menu(&menu)
        .tooltip("glm-token-monitor - Token Monitor")
        .build(app)?;

    Ok(())
}

fn build_profile_submenu(app: &AppHandle) -> Result<Submenu<tauri::Wry>, Box<dyn std::error::Error>> {
    let store = crate::claude_profile::load_profiles(app);
    let mut items: Vec<Box<dyn tauri::menu::MenuItem<tauri::Wry>>> = vec![];

    match store {
        Ok(s) => {
            if s.profiles.is_empty() {
                let empty_item = MenuItem::with_id(app, "no_profiles", "（无配置）", false, None::<&str>)?;
                items.push(Box::new(empty_item));
            } else {
                for profile in &s.profiles {
                    let is_active = s.active_profile_id.as_deref() == Some(&profile.id);
                    let label = if is_active {
                        format!("✓ {}", profile.name)
                    } else {
                        profile.name.clone()
                    };
                    let item = CheckMenuItem::with_id(
                        app,
                        &format!("profile_{}", profile.id),
                        &label,
                        true,
                        is_active,
                        None::<&str>,
                    )?;
                    items.push(Box::new(item));
                }
            }
        }
        Err(_) => {
            let err_item = MenuItem::with_id(app, "profile_error", "（加载失败）", false, None::<&str>)?;
            items.push(Box::new(err_item));
        }
    }

    Submenu::with_items(app, "切换 API 配置", true, &items.iter().map(|i| i.as_ref()).collect::<Vec<_>>())
}

pub fn refresh_tray_menu(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(tray) = app.tray_by_id("main-tray") {
        let new_menu = {
            let show_item = MenuItem::with_id(app, "show", "显示宠物", true, None::<&str>)?;
            let hide_item = MenuItem::with_id(app, "hide", "隐藏宠物", true, None::<&str>)?;
            let refresh_item = MenuItem::with_id(app, "refresh", "刷新数据", true, None::<&str>)?;
            let separator = PredefinedMenuItem::separator(app)?;
            let switch_submenu = build_profile_submenu(app)?;
            let manage_profiles = MenuItem::with_id(app, "manage_profiles", "管理 API 配置", true, None::<&str>)?;
            let separator2 = PredefinedMenuItem::separator(app)?;
            let settings_item = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
            let separator3 = PredefinedMenuItem::separator(app)?;
            let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

            Menu::with_items(app, &[
                &show_item,
                &hide_item,
                &refresh_item,
                &separator,
                &switch_submenu,
                &manage_profiles,
                &separator2,
                &settings_item,
                &separator3,
                &quit_item,
            ])?
        };
        tray.set_menu(Some(new_menu))?;
    }
    Ok(())
}

pub fn setup_tray_menu_handler(app: &AppHandle) {
    app.on_menu_event(|app, event| {
        let id = event.id.as_ref();
        if id == "show" {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        } else if id == "hide" {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.hide();
            }
        } else if id == "refresh" {
            let app_handle = app.clone();
            tauri::async_runtime::spawn(async move {
                crate::polling::emit_usage(&app_handle).await;
            });
        } else if id == "settings" || id == "manage_profiles" {
            if let Some(window) = app.get_webview_window("settings") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        } else if id == "quit" {
            app.exit(0);
        } else if id.starts_with("profile_") {
            let profile_id = id.strip_prefix("profile_").unwrap_or("").to_string();
            let app_handle = app.clone();
            tauri::async_runtime::spawn(async move {
                match crate::claude_profile::switch_profile_handler(app_handle.clone(), profile_id).await {
                    Ok(()) => {
                        let _ = crate::tray::refresh_tray_menu(&app_handle);
                    }
                    Err(e) => eprintln!("Switch profile error: {}", e),
                }
            });
        }
    });

    // 托盘图标点击
    let app_handle = app.clone();
    app.on_tray_icon_event(move |_app, event| {
        if let TrayIconEvent::Click { button, .. } = event {
            if button == MouseButton::Left {
                if let Some(window) = app_handle.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        }
    });
}
```

- [ ] **Step 2: 修改 main.rs 适配新的 tray 接口**

将 `main.rs` 的 `setup` 闭包中 tray 相关调用改为：

```rust
// 创建系统托盘
let app_handle = app.handle().clone();
if let Err(e) = tray::create_system_tray(&app_handle) {
    eprintln!("System tray error: {}", e);
}
tray::setup_tray_menu_handler(app.handle());
```

注意：`setup_tray_menu_handler` 必须在 `create_system_tray` 之后调用，因为它需要菜单已存在。同时 `on_menu_event` 和 `on_tray_icon_event` 只能注册一次，所以原 `tray.rs` 中的事件处理全部移到 `setup_tray_menu_handler` 中，`create_system_tray` 不再包含事件处理。

- [ ] **Step 3: 验证编译**

Run: `cd src-tauri && cargo check`
Expected: 编译成功

- [ ] **Step 4: 提交**

```bash
git add src-tauri/src/tray.rs src-tauri/src/main.rs
git commit -m "feat: 托盘菜单添加动态 API 配置切换子菜单"
```

---

### Task 6: 前端 — 添加 TypeScript 类型

**Files:**
- Modify: `src/types/config.ts`

- [ ] **Step 1: 在 config.ts 末尾添加 profile 相关类型**

```typescript
export interface ClaudeApiProfile {
  id: string
  name: string
  inference_gateway_base_url: string
  inference_gateway_api_key: string
  inference_models: string[]
}

export interface ProfileStore {
  profiles: ClaudeApiProfile[]
  active_profile_id: string | null
  claude_config_path: string | null
}
```

- [ ] **Step 2: 提交**

```bash
git add src/types/config.ts
git commit -m "feat: 添加 Claude profile TypeScript 类型定义"
```

---

### Task 7: 前端 — 创建 useClaudeProfiles composable

**Files:**
- Create: `src/composables/useClaudeProfiles.ts`

- [ ] **Step 1: 创建 composable**

```typescript
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { ClaudeApiProfile, ProfileStore } from '../types/config'

const globalStore = ref<ProfileStore>({
  profiles: [],
  active_profile_id: null,
  claude_config_path: null,
})

const globalIsLoading = ref(false)
const globalError = ref<string | null>(null)

export function useClaudeProfiles() {
  const store = globalStore
  const isLoading = globalIsLoading
  const error = globalError

  const activeProfile = ref<ClaudeApiProfile | null>(null)

  async function loadProfiles() {
    try {
      isLoading.value = true
      error.value = null
      const result = await invoke<ProfileStore>('get_profiles')
      store.value = result
      syncActiveProfile()
    } catch (err) {
      error.value = String(err)
    } finally {
      isLoading.value = false
    }
  }

  function syncActiveProfile() {
    activeProfile.value = store.value.profiles.find(
      p => p.id === store.value.active_profile_id
    ) || null
  }

  async function saveProfile(profile: ClaudeApiProfile) {
    try {
      error.value = null
      const result = await invoke<ProfileStore>('save_profile_handler', { profile })
      store.value = result
      syncActiveProfile()
    } catch (err) {
      error.value = String(err)
      throw err
    }
  }

  async function deleteProfile(id: string) {
    try {
      error.value = null
      const result = await invoke<ProfileStore>('delete_profile_handler', { id })
      store.value = result
      syncActiveProfile()
    } catch (err) {
      error.value = String(err)
      throw err
    }
  }

  async function switchProfile(id: string) {
    try {
      error.value = null
      await invoke('switch_profile_handler', { id })
      store.value.active_profile_id = id
      syncActiveProfile()
    } catch (err) {
      error.value = String(err)
      throw err
    }
  }

  async function getClaudeConfigPath(): Promise<string> {
    return await invoke<string>('get_claude_config_path_cmd')
  }

  async function setClaudeConfigPath(path: string) {
    await invoke('set_claude_config_path_handler', { path })
    store.value.claude_config_path = path
  }

  async function setupProfileListener() {
    return await listen<ProfileStore>('claude-profile-changed', (event) => {
      if (event.payload) {
        store.value = event.payload
        syncActiveProfile()
      }
    })
  }

  function createEmptyProfile(): ClaudeApiProfile {
    return {
      id: crypto.randomUUID(),
      name: '',
      inference_gateway_base_url: '',
      inference_gateway_api_key: '',
      inference_models: [],
    }
  }

  return {
    store,
    isLoading,
    error,
    activeProfile,
    loadProfiles,
    saveProfile,
    deleteProfile,
    switchProfile,
    getClaudeConfigPath,
    setClaudeConfigPath,
    setupProfileListener,
    createEmptyProfile,
  }
}
```

- [ ] **Step 2: 提交**

```bash
git add src/composables/useClaudeProfiles.ts
git commit -m "feat: 创建 useClaudeProfiles composable"
```

---

### Task 8: 前端 — SettingsPanel 新增 "API 配置" 标签页

**Files:**
- Modify: `src/components/SettingsPanel.vue`

- [ ] **Step 1: 在 script setup 中引入 composable 和扩展标签类型**

在 `<script setup>` 顶部 import 区追加：

```typescript
import { useClaudeProfiles } from '../composables/useClaudeProfiles'
import type { ClaudeApiProfile } from '../types/config'
```

在 `useTheme()` 之后添加：

```typescript
const {
  store: profileStore,
  isLoading: isProfileLoading,
  error: profileError,
  activeProfile,
  loadProfiles,
  saveProfile,
  deleteProfile,
  switchProfile,
  getClaudeConfigPath,
  setClaudeConfigPath,
  setupProfileListener,
  createEmptyProfile,
} = useClaudeProfiles()
```

修改 `activeTab` 类型：

```typescript
const activeTab = ref<'basic' | 'models' | 'pet' | 'claude-api'>('basic')
```

添加 profile 编辑状态：

```typescript
const editingProfile = ref<ClaudeApiProfile | null>(null)
const isCreatingProfile = ref(false)
const profileModelsInput = ref('')
const claudeConfigPath = ref('')
const showDeleteConfirm = ref<string | null>(null)

function startCreateProfile() {
  editingProfile.value = createEmptyProfile()
  isCreatingProfile.value = true
  profileModelsInput.value = ''
}

function startEditProfile(profile: ClaudeApiProfile) {
  editingProfile.value = { ...profile }
  isCreatingProfile.value = false
  profileModelsInput.value = profile.inference_models.join(', ')
}

function cancelEditProfile() {
  editingProfile.value = null
  isCreatingProfile.value = false
  profileModelsInput.value ''
}

async function saveProfileEdit() {
  if (!editingProfile.value) return
  editingProfile.value.inference_models = profileModelsInput.value
    .split(',')
    .map(s => s.trim())
    .filter(s => s.length > 0)
  await saveProfile(editingProfile.value)
  editingProfile.value = null
  isCreatingProfile.value = false
}

async function handleDeleteProfile(id: string) {
  await deleteProfile(id)
  showDeleteConfirm.value = null
}

async function handleSwitchProfile(id: string) {
  await switchProfile(id)
}
```

在 `onMounted` 中追加：

```typescript
await loadProfiles()
try {
  claudeConfigPath.value = await getClaudeConfigPath()
} catch {}
const unlistenProfile = await setupProfileListener()
const originalCleanup = cleanup
cleanup = () => {
  originalCleanup?.()
  unlistenProfile.then(fn => fn())
}
```

- [ ] **Step 2: 在 template 标签导航中添加新标签按钮**

在 `tab-nav` 中的 tab 数组追加：

```typescript
{ id: 'claude-api', label: 'API 配置' }
```

- [ ] **Step 3: 在 template 内容区添加 API 配置标签页内容**

在 `<!-- 宠物设置 -->` div 之后、`<!-- 错误提示 -->` 之前添加：

```html
<!-- API 配置管理 -->
<div v-if="activeTab === 'claude-api'" class="settings-section">
  <!-- 当前激活配置 -->
  <div v-if="activeProfile" class="setting-item active-profile">
    <div class="setting-info">
      <span class="setting-label">当前配置</span>
      <span class="setting-desc">{{ activeProfile.name }} · {{ activeProfile.inference_gateway_base_url }}</span>
    </div>
    <span class="active-badge">已激活</span>
  </div>

  <!-- Profile 列表 -->
  <div class="profiles-list">
    <div
      v-for="profile in profileStore.profiles"
      :key="profile.id"
      class="profile-card"
      :class="{ active: profile.id === profileStore.active_profile_id }"
    >
      <div class="profile-main">
        <div class="profile-info">
          <span class="profile-name">{{ profile.name }}</span>
          <span class="profile-url">{{ profile.inference_gateway_base_url }}</span>
          <span class="profile-models">{{ profile.inference_models.join(', ') }}</span>
        </div>
        <div class="profile-actions">
          <button
            v-if="profile.id !== profileStore.active_profile_id"
            class="action-btn activate"
            @click="handleSwitchProfile(profile.id)"
          >切换</button>
          <button class="action-btn" @click="startEditProfile(profile)">编辑</button>
          <button
            v-if="!showDeleteConfirm || showDeleteConfirm !== profile.id"
            class="action-btn danger"
            @click="showDeleteConfirm = profile.id"
          >删除</button>
          <template v-else>
            <button class="action-btn danger" @click="handleDeleteProfile(profile.id)">确认</button>
            <button class="action-btn" @click="showDeleteConfirm = null">取消</button>
          </template>
        </div>
      </div>
    </div>
  </div>

  <!-- 新增按钮 -->
  <button v-if="!editingProfile" class="action-btn add" @click="startCreateProfile">
    + 新增配置
  </button>

  <!-- 编辑/新增表单 -->
  <div v-if="editingProfile" class="profile-editor">
    <div class="editor-field">
      <label>名称</label>
      <input v-model="editingProfile.name" class="api-key-input" placeholder="例如：智谱 GLM" />
    </div>
    <div class="editor-field">
      <label>Base URL</label>
      <input v-model="editingProfile.inference_gateway_base_url" class="api-key-input" placeholder="https://open.bigmodel.cn/api/anthropic" />
    </div>
    <div class="editor-field">
      <label>API Key</label>
      <input v-model="editingProfile.inference_gateway_api_key" class="api-key-input" type="password" placeholder="输入 API Key" />
    </div>
    <div class="editor-field">
      <label>Models（逗号分隔）</label>
      <input v-model="profileModelsInput" class="api-key-input" placeholder="glm-5.1, glm-4.7" />
    </div>
    <div class="editor-actions">
      <button class="action-btn save" @click="saveProfileEdit">保存</button>
      <button class="action-btn cancel" @click="cancelEditProfile">取消</button>
    </div>
  </div>

  <!-- Claude 配置路径 -->
  <div class="setting-item full">
    <div class="setting-info">
      <span class="setting-label">Claude 配置路径</span>
      <span class="setting-desc">Claude Code 桌面版配置文件路径</span>
    </div>
    <div class="api-key-input-group">
      <input
        v-model="claudeConfigPath"
        class="api-key-input"
        placeholder="自动探测或手动指定"
        @change="setClaudeConfigPath(claudeConfigPath)"
      />
    </div>
  </div>

  <!-- 错误提示 -->
  <div v-if="profileError" class="error-bar">
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <circle cx="12" cy="12" r="10"/>
      <path d="M12 8v4M12 16h.01"/>
    </svg>
    <span>{{ profileError }}</span>
  </div>
</div>
```

- [ ] **Step 4: 添加 API 配置标签页的样式**

在 `<style scoped>` 末尾追加：

```css
/* ── API 配置标签页 ── */
.active-profile {
  background: #111113;
  border-color: #22c55e;
}
.active-badge {
  font-size: 9px;
  font-weight: 600;
  color: #22c55e;
  background: rgba(34, 197, 94, 0.1);
  padding: 4px 10px;
  border-radius: 4px;
}

.profiles-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.profile-card {
  background: #0a0a0b;
  border: 1px solid #1c1c1e;
  border-radius: 6px;
  overflow: hidden;
  transition: all 0.2s;
}
.profile-card.active {
  border-color: #22c55e;
}

.profile-main {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 14px;
  gap: 12px;
}

.profile-info {
  display: flex;
  flex-direction: column;
  gap: 3px;
  flex: 1;
  min-width: 0;
}

.profile-name {
  font-size: 11px;
  font-weight: 600;
  color: #e4e4e7;
}

.profile-url {
  font-size: 9px;
  color: #52525b;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.profile-models {
  font-size: 8px;
  color: #3f3f46;
}

.profile-actions {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-shrink: 0;
}

.action-btn.activate {
  background: rgba(34, 197, 94, 0.15);
  border-color: #22c55e;
  color: #22c55e;
}
.action-btn.activate:hover {
  background: #22c55e;
  color: #ffffff;
}

.action-btn.danger {
  color: #f87171;
  border-color: #7f1d1d;
}
.action-btn.danger:hover {
  background: #7f1d1d;
  color: #fca5a5;
}

.action-btn.add {
  width: 100%;
  padding: 12px;
  background: #111113;
  border: 1px dashed #27272a;
  border-radius: 6px;
  color: #71717a;
  font-size: 10px;
  text-align: center;
}
.action-btn.add:hover {
  border-color: #3f3f46;
  color: #e4e4e7;
}

.profile-editor {
  background: #111113;
  border: 1px solid #27272a;
  border-radius: 6px;
  padding: 14px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.editor-field {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.editor-field label {
  font-size: 9px;
  font-weight: 600;
  color: #71717a;
  letter-spacing: 0.5px;
}

.editor-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
}

/* 浅色主题 */
.settings-panel[data-theme="light"] .active-profile {
  background: #ffffff;
  border-color: #22c55e;
}
.settings-panel[data-theme="light"] .profile-card {
  background: #fafaf9;
  border-color: #e4e4e7;
}
.settings-panel[data-theme="light"] .profile-card.active {
  border-color: #22c55e;
}
.settings-panel[data-theme="light"] .profile-name {
  color: #1c1c1e;
}
.settings-panel[data-theme="light"] .profile-url {
  color: #737373;
}
.settings-panel[data-theme="light"] .profile-editor {
  background: #ffffff;
  border-color: #e4e4e7;
}
.settings-panel[data-theme="light"] .action-btn.add {
  background: #ffffff;
  border-color: #e4e4e7;
  color: #737373;
}
```

- [ ] **Step 5: 验证前端构建**

Run: `npm run build`
Expected: 构建成功

- [ ] **Step 6: 提交**

```bash
git add src/components/SettingsPanel.vue
git commit -m "feat: SettingsPanel 新增 API 配置管理标签页"
```

---

### Task 9: 集成测试 — 编译与手动验证

**Files:** 无新增

- [ ] **Step 1: 完整编译验证**

Run: `cd src-tauri && cargo build`
Expected: 编译成功

- [ ] **Step 2: 运行应用并测试**

Run: `npm run tauri:dev`

测试清单：
1. 系统托盘右键菜单出现 "切换 API 配置" 子菜单
2. 设置面板出现 "API 配置" 标签页
3. 点击 "新增配置" → 填写表单 → 保存 → 出现在列表
4. 点击 "切换" → Claude Code 配置文件中 3 个字段被更新
5. 托盘子菜单显示 ✓ 标记
6. 删除配置正常工作

- [ ] **Step 3: 提交最终状态**

```bash
git add -A
git commit -m "feat: Claude API 配置切换功能完成"
```
