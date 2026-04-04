# 独立信息面板窗口设计

**日期:** 2026-04-04
**状态:** 已批准

## 概述

将当前内嵌在 `PetWidget.vue` 中的信息面板重构为独立的 Tauri 窗口，提供更好的用户体验和代码组织。

## 架构设计

### 整体架构

```
主窗口 (PetWidget.vue)
    │
    │ click → invoke('open_info_panel')
    ↓
Rust 后端 (main.rs)
    │
    │ create WebviewWindow
    ↓
副窗口 (info-panel-entry.ts)
    │
    └── InfoPanel.vue
```

### 文件结构

```
src/
├── components/
│   ├── PetWidget.vue          (修改：移除内嵌面板)
│   └── InfoPanel.vue          (新建：副窗口内容)
├── info-panel-entry.ts        (新建：副窗口 Vue 入口)
├── main.ts                    (现有：主窗口入口)
src-tauri/
├── src/
│   ├── main.rs                (修改：添加副窗口创建逻辑)
│   └── windows.rs             (新建：窗口管理模块)
└── tauri.conf.json            (修改：注册副窗口)
```

## 组件设计

### InfoPanel.vue

**职责：** 展示用量详情的独立窗口组件

**Props：**
```typescript
interface Props {
  initialUsage?: UsageData
  initialPetState?: string
}
```

**数据获取：**
1. 从 props 获取初始数据
2. 监听 `usage-update` 事件持续更新
3. 提供 `invoke('get_current_usage')` 手动刷新

**UI 元素：**
- 状态心语
- 月度额度 + 进度条
- 5小时额度 + 进度条
- 最后更新时间
- 刷新按钮
- 关闭按钮 (×)

### PetWidget.vue 修改

**移除内容：**
- `showSidePanel` 状态
- `toggleSidePanel` 函数
- 内嵌的 `.side-info-panel` 模板
- 内嵌面板相关样式
- 窗口大小调整逻辑

**新增内容：**
- 点击事件改为 `invoke('open_info_panel')`

## 窗口配置

### Tauri 配置 (tauri.conf.json)

```json
{
  "windows": [
    {
      "label": "main",
      "title": "PlanGuard",
      "width": 96,
      "height": 96,
      "decorations": false,
      "transparent": true,
      "alwaysOnTop": true
    },
    {
      "label": "info-panel",
      "title": "用量详情",
      "width": 280,
      "height": 320,
      "decorations": false,
      "transparent": true,
      "alwaysOnTop": true,
      "visible": false,
      "skipTaskbar": true
    }
  ]
}
```

### 窗口定位逻辑

```rust
// 获取主窗口位置
let main_pos = main_window.current_position()?;

// 计算副窗口位置（主窗口右侧，间隔 8px）
let info_x = main_pos.x + 96 + 8;
let info_y = main_pos.y;

// 如果超出屏幕右侧，则显示在左侧
if info_x + 280 > screen_width {
    info_x = main_pos.x - 280 - 8;
}
```

## 事件通信

### Rust → Vue (事件流)

```
polling.rs (轮询服务)
    │
    │ emit_all("usage-update", data)
    ↓
├──→ 主窗口 (PetWidget) → 更新宠物状态
└──→ 副窗口 (InfoPanel) → 更新用量数据
```

### Vue → Rust (命令流)

```typescript
// 主窗口请求打开副窗口
invoke('open_info_panel')

// 副窗口请求关闭
invoke('close_info_panel')

// 副窗口手动刷新
invoke('get_current_usage')
```

### Rust 命令实现

```rust
#[tauri::command]
async fn open_info_panel(
  main_window: Window,
  info_panel: WebviewWindow
) -> Result<(), String> {
  // 计算位置
  // 显示窗口
  // 设置焦点
}

#[tauri::command]
async fn close_info_panel(info_panel: WebviewWindow) {
  info_panel.hide();
}
```

## 实施步骤

### Phase 1: 创建新组件
1. 创建 `src/components/InfoPanel.vue`
2. 创建 `src/info-panel-entry.ts`
3. 修改 `src-tauri/tauri.conf.json` 注册副窗口

### Phase 2: 窗口管理
4. 创建 `src-tauri/src/windows.rs` 窗口管理模块
5. 修改 `src-tauri/src/main.rs` 集成窗口管理

### Phase 3: 事件通信
6. 添加 `open_info_panel` 和 `close_info_panel` 命令
7. 确保 `usage-update` 事件广播到所有窗口

### Phase 4: 清理重构
8. 移除 `PetWidget.vue` 中的内嵌面板代码
9. 修改点击事件为调用新窗口命令

### Phase 5: 测试验证
10. 测试窗口打开/关闭
11. 测试数据同步更新
12. 测试窗口定位逻辑
