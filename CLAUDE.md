# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

PlanGuard 是一个跨平台桌面 API 配额监控应用，通过可爱的电子宠物和多种显示模式，实时展示 API 使用量。

- **Windows**: 96x96 透明悬浮窗，支持手动拖动，点击显示详情面板
- **macOS**: 系统托盘 + 气泡面板（待实现）
- **多提供商支持**: 智谱 (BigModel)、Z.AI 等，可扩展

## 常用命令

```bash
# 开发模式（热重载）
npm run tauri:dev

# 构建（包含前端和 Rust）
npm run tauri:build

# 仅构建前端（跳过类型检查）
npm run build

# 仅构建 Rust 部分
cd src-tauri && cargo build

# 仅运行已编译的二进制文件
./src-tauri/target/debug/plan-guard.exe  # Windows
./src-tauri/target/debug/plan-guard      # macOS/Linux

# 运行 Rust 测试
cd src-tauri && cargo test
```

## 关键依赖

### Rust 端（`src-tauri/Cargo.toml`）
- `tauri 2.0`: 应用框架，包含 tray-icon, image-ico, image-png 功能
- `tokio`: 异步运行时，用于轮询服务
- `reqwest`: HTTP 客户端，调用 API
- `serde/serde_json`: 序列化/反序列化
- `chrono`: 时间处理
- `dirs`: 获取系统配置目录

### Vue 端（`package.json`）
- `vue 3.4`: 前端框架
- `@tauri-apps/api 2.10`: Tauri API
- `tailwindcss 3.4`: CSS 框架
- `vite 5.0`: 构建工具
- `typescript 5.3`: 类型支持

## 核心架构

### 数据流
```
Rust 端轮询 (可配置，默认1分钟) → fetch_usage() → Tauri Event ("usage-update") → Vue 监听 → UI 更新
```

### 状态机
基于 `tokens_percent = used / total * 100`（宠物状态由 5h Token 额度驱动）：

| 状态 | 百分比 | 颜色 | 动画效果 |
|------|--------|------|----------|
| Fresh | 0-30% | #10B981 (绿) | 呼吸动画 |
| Flow | 31-60% | #3B82F6 (蓝) | 敲打动画 |
| Warning | 61-80% | #F59E0B (黄) | 抖动动画 |
| Panic | 81-95% | #F97316 (橙) | 冒汗动画 |
| Dead | 96-100% | #6B7280 (灰) | 灵魂出窍动画 |

### 模块职责

**Rust 端 (`src-tauri/src/`)**
- `main.rs`: 入口，窗口初始化，平台特定配置，注册 Tauri Commands
- `polling.rs`: 后台轮询服务，从配置读取间隔，调用真实 API 获取使用量
- `events.rs`: 事件定义和数据结构 (`UsageData`, `ApiResponse`)
- `commands.rs`: Tauri Commands (`get_current_usage`)
- `config.rs`: 配置文件读写，支持旧配置迁移，默认值管理
- `settings_commands.rs`: 设置相关 Commands（保存配置、测试连接、开机自启）
- `windows.rs`: 窗口管理（info-panel, settings）
- `tray.rs`: 系统托盘支持（Windows 和 macOS）

**Vue 端 (`src/`)**
- `composables/useUsageState.ts`: 状态计算逻辑（百分比、状态映射、颜色）
- `composables/useTauriEvents.ts`: 监听 Rust 端事件 (`usage-update`, `usage-error`)
- `composables/usePlatform.ts`: 平台检测（Windows/macOS）
- `composables/useSettings.ts`: 配置管理，监听配置变化
- `composables/useDisplayMode.ts`: 显示模式管理（5 种模式）
- `composables/usePetAction.ts`: 宠物类型和动作管理
- `composables/useTheme.ts`: 主题管理
- `components/PetWidget.vue`: Windows 悬浮窗组件（含拖动、多宠物、多显示模式）
- `components/PopoverPanel.vue`: macOS 气泡面板（待实现）
- `components/pets/`: 宠物组件目录
  - `JellySpirit.vue`: 果冻精灵（SVG，动态颜色）
  - `PixelGhost.vue`: 像素幽灵（SVG，动态颜色）
  - `DogSit.vue`, `DogBark.vue`, `DogWalk.vue`, `DogBeg.vue`: 狗狗动作
  - `CatGifViewer.vue`: 猫咪 GIF 播放器
- `App.vue`: 根组件，根据平台选择渲染组件

### 跨端通信

**Rust → Vue (Event)**
```rust
app.emit("usage-update", UsageData { used: 65, total: 100 })
```

```typescript
listen('usage-update', (event) => {
  usageData.value = event.payload
})
```

**Vue → Rust (Command)**
```typescript
invoke('get_current_usage')
```

## 窗口系统

应用包含 3 个窗口（配置见 `tauri.conf.json`）：

### 主窗口 (`main`)
- 96x96 像素，透明、无边框、始终置顶
- 跳过任务栏，不支持调整大小
- 渲染 `PetWidget.vue`（Windows）或 `PopoverPanel.vue`（macOS）

### 信息面板 (`info-panel`)
- 280x320 像素，透明、无边框、始终置顶
- 初始隐藏，点击主窗口显示
- 显示详细用量信息和状态
- 通过 `windows::open_info_panel` / `windows::close_info_panel` 控制

### 设置窗口 (`settings`)
- 680x600 像素，不透明、有装饰、可调整大小
- 初始隐藏，显示在任务栏
- 深色主题背景 (#12141c)
- 通过 `windows::open_settings_panel` / `windows::close_settings_panel` 控制

## 平台特定注意事项

### Windows
- 窗口配置：`decorations: false`, `transparent: true`, `alwaysOnTop: true`
- 拖动实现：手动监听 `mousedown/mousemove/mouseup`，使用 `appWindow.setPosition()`
- 窗口大小：96x96 像素

### macOS
- 系统托盘图标需根据状态动态切换
- 不在 dock 栏显示
- 点击托盘显示气泡面板

## 开发约定

### API 数据格式
API 返回的数据结构（见 `events.rs`）：
```rust
UsageData {
    used: u32,           // 5h Token 使用百分比（驱动宠物状态）
    total: u32,          // 总量（固定 100）
    time_percent: u32,   // 月 MCP 额度百分比
    tokens_percent: u32, // 5h Token 额度百分比
    time_remaining: Option<u32>,     // 月 MCP 剩余次数
    tokens_reset_time: Option<i64>,  // 5h Token 下次重置时间（时间戳）
    time_reset_time: Option<i64>,   // 月度额度下次重置时间（时间戳）
    level: Option<String>,           // 会员等级
    usage_details: Vec<UsageDetailData>, // 工具使用详情
}
```

### 配置管理
- 配置文件位置：`~/.config/plan-guard/config.json` (Linux) 或 `%APPDATA%\plan-guard\config.json` (Windows)
- 轮询间隔：从 `config.polling_config.interval_minutes` 读取（默认 1 分钟）
- 所有配置修改通过 `settings_commands.rs` 的 Command 接口
- 配置变更会触发 Tauri 事件，Vue 端自动响应

### 多显示模式
5 种 Token 剩余量显示模式（`display_mode` 配置）：
1. `none`: 不显示
2. `holo-bubble`: 全息气泡
3. `cyber-ring`: 赛博圆环
4. `aura-field`: 光场
5. `energy-core`: 能量核心（像素网格）
6. `status-floater`: 状态浮动条

### 宠物系统
- 宠物类型通过 `pet_config.selected_pet` 配置
- 每种宠物支持多个动作，通过 `usePetAction` 管理
- 宠物颜色动态根据状态变化（`gradientColor`, `gradientStrokeColor`）

### 状态相关逻辑
- 所有状态计算逻辑集中在 `useUsageState.ts`
- 所有跨平台检测使用 `usePlatform.ts`
- 颜色系统与项目状态完全一致（见 COLORS 常量）

## 已知问题

- vue-tsc 在当前环境有兼容性问题，构建时可绕过（使用 `vite build` 跳过类型检查）
- macOS 系统托盘功能部分实现，气泡面板待完善

## 架构扩展点

### 添加新的 API 提供商
在 `src-tauri/src/config.rs` 的 `ModelConfig::api_domain()` 方法中添加新提供商：

```rust
pub fn api_domain(&self) -> &'static str {
    match self.provider.as_str() {
        "bigmodel" => "https://open.bigmodel.cn/",
        "zai" => "https://api.z.ai/",
        "your-provider" => "https://your-api.com/", // 新增
        _ => "https://open.bigmodel.cn/",
    }
}
```

### 添加新的显示模式
1. 在 `PetWidget.vue` 的 template 中添加新的 `v-else-if` 分支
2. 在 `<style scoped>` 中添加对应的 CSS 样式
3. 在 `useDisplayMode.ts` 中更新模式列表

### 添加新的宠物类型
1. 在 `src/components/pets/` 创建新的宠物组件
2. 在 `PetWidget.vue` 中导入并注册组件
3. 在 `usePetAction.ts` 中添加宠物类型和动作逻辑
4. 在配置中添加新宠物的选项
