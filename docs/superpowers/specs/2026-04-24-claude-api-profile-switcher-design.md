# Claude API 配置切换功能设计

## 概述

在 plan-guard 应用中新增 Claude Code 第三方 API 配置管理功能，支持保存多套 API 配置并一键切换。通过托盘菜单快速切换，设置面板管理配置。

## 数据存储

### Profile 存储文件

路径：plan-guard 配置目录下 `claude-api-profiles.json`

```json
{
  "profiles": [
    {
      "id": "uuid-string",
      "name": "智谱 GLM",
      "inferenceGatewayBaseUrl": "https://open.bigmodel.cn/api/anthropic",
      "inferenceGatewayApiKey": "xxx",
      "inferenceModels": ["glm-5.1", "glm-4.7"]
    }
  ],
  "activeProfileId": "uuid-string"
}
```

### Claude Code 配置文件

路径（macOS 默认）：`~/Library/Application Support/Claude-3p/configLibrary/{uuid}.json`

切换时只覆盖三个字段，保留其他字段：
- `inferenceGatewayBaseUrl`
- `inferenceGatewayApiKey`
- `inferenceModels`

Claude Code 配置文件路径存储在 plan-guard 主配置中（`claude_config_path` 字段），支持用户自定义。

## Rust 端

### 新增模块：`src-tauri/src/claude_profile.rs`

**数据结构：**
- `ClaudeApiProfile`：id, name, inferenceGatewayBaseUrl, inferenceGatewayApiKey, inferenceModels
- `ProfileStore`：profiles 列表 + activeProfileId

**职责：**
- 从 `claude-api-profiles.json` 加载/保存 profile 列表
- `switch_profile(id)`：读取目标 profile → 加载 Claude Code 配置 → 覆盖 3 个字段 → 写回
- 切换后通过 Tauri 事件通知前端状态变更

**Tauri Commands：**
- `get_profiles()` — 返回所有 profile 及当前激活 ID
- `save_profile(profile)` — 新增或更新
- `delete_profile(id)` — 删除
- `switch_profile(id)` — 切换激活 profile
- `get_claude_config_path()` — 获取配置路径
- `set_claude_config_path(path)` — 设置配置路径

## 托盘菜单

在 `tray.rs` 托盘菜单中新增：
- **"切换 API 配置" 子菜单**：动态列出所有 profile name，当前激活的前面打勾（✓），点击触发 `switch_profile`
- **"管理 API 配置"**：点击打开设置面板并定位到 API 配置标签页

菜单每次打开时动态读取 profile 列表。

## 设置面板

在 `SettingsPanel.vue` 标签栏新增 **"API 配置"** 标签：

- **配置列表**：卡片式展示所有 profile（name、url、models），当前激活高亮
- **新增按钮**：表单填写 name、url、apiKey、models（逗号分隔）
- **编辑/删除**：每个卡片有编辑和删除按钮
- **切换按钮**：每个卡片有"使用此配置"按钮
- **Claude 配置路径**：底部可编辑路径输入框（默认自动填充）
