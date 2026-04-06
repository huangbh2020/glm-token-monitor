# 容量展示模式系统设计文档

**日期**: 2026-04-06
**项目**: PlanGuard (GLM Token Monitor)
**作者**: Claude Code

---

## 1. 概述

### 1.1 目标

为 PlanGuard 添加多种容量展示模式，允许用户在设置面板中选择不同的视觉展示方式。

### 1.2 当前状态

- 仅支持一种展示方式：像素气泡（右上角显示 `5h: 85%`）
- 配置文件中无展示模式相关字段

### 1.3 目标状态

支持 5 种展示模式：
1. **像素气泡** - 当前默认方式
2. **圆环进度** - 围绕宠物的环形进度条
3. **呼吸灯效** - 宠物周围的光晕脉动效果
4. **胶囊指示器** - 底部居中的进度条
5. **头顶数字** - 宠物头顶的表情+数字

---

## 2. 架构设计

### 2.1 类型定义

```typescript
// src/types/config.ts
export type DisplayMode =
  | 'bubble'      // 像素气泡（当前默认）
  | 'ring'        // 圆环进度
  | 'breathing'   // 呼吸灯效
  | 'capsule'     // 胶囊指示器
  | 'emoji'       // 头顶数字

export interface DisplayConfig {
  display_mode: DisplayMode
}

export interface AppConfig {
  api_config: ApiConfig
  polling_config: PollingConfig
  display_config: DisplayConfig  // 新增
}
```

### 2.2 渲染策略

在 `PetWidget.vue` 中使用条件渲染：

```vue
<!-- 像素气泡 -->
<div v-if="displayMode === 'bubble'" class="pixel-bubble">
  5h: <span class="bubble-val">{{ 100 - tokensPercent }}%</span>
</div>

<!-- 圆环进度 -->
<div v-else-if="displayMode === 'ring'" class="ring-display">
  <svg viewBox="0 0 96 96">
    <circle class="ring-bg" cx="48" cy="48" r="42"/>
    <circle class="ring-progress" cx="48" cy="48" r="42"
      :stroke-dasharray="circumference"
      :stroke-dashoffset="progressOffset"/>
  </svg>
</div>

<!-- 其他模式... -->
```

### 2.3 组件结构

```
PetWidget.vue
├── 宠物 SVG (现有)
├── DisplayModeRenderer (新增)
│   ├── bubble (像素气泡) - 现有代码
│   ├── ring (圆环进度)
│   ├── breathing (呼吸灯效)
│   ├── capsule (胶囊指示器)
│   └── emoji (头顶数字)
└── 心语面板 (现有)
```

---

## 3. 展示模式详细设计

### 3.1 像素气泡 (bubble) - 当前默认

**位置**: 右上角
**样式**: 深色背景 + 硬阴影 + 像素边框
**显示内容**: `5h: 85%`

```css
.pixel-bubble {
  position: absolute;
  top: 10px;
  right: 6px;
  background: #0F172A;
  border: 2px solid #334155;
  box-shadow: 2px 2px 0 rgba(0, 0, 0, 0.8);
  padding: 3px 6px;
  font-family: ui-monospace, monospace;
  font-size: 10px;
  font-weight: 700;
}
```

### 3.2 圆环进度 (ring)

**位置**: 围绕宠物主体，作为背景层
**实现**: SVG 圆环，stroke-dasharray 控制进度
**颜色**: 跟随宠物状态色（绿/蓝/黄/橙/灰）

```vue
<svg class="ring-svg" viewBox="0 0 96 96">
  <!-- 背景圈 -->
  <circle cx="48" cy="48" r="42" fill="none" stroke="#1E293B" stroke-width="4"/>
  <!-- 进度圈 -->
  <circle cx="48" cy="48" r="42" fill="none" :stroke="stateColor" stroke-width="4"
    stroke-dasharray="264" stroke-dashoffset="calc(264 * (1 - tokensPercent / 100))"
    transform="rotate(-90 48 48)" stroke-linecap="round"/>
  <!-- 中心文字 -->
  <text x="48" y="52" text-anchor="middle" :fill="stateColor" font-size="12" font-weight="bold">
    {{ tokensPercent }}%
  </text>
</svg>
```

### 3.3 呼吸灯效 (breathing)

**位置**: 宠物周围的光晕效果
**实现**: CSS animation + box-shadow
**特性**:
- 颜色深浅表示容量
- 脉动频率表示状态
- Panic 状态快速闪烁

```css
.breathing-glow {
  position: absolute;
  inset: 0;
  border-radius: 50%;
  background: radial-gradient(circle, var(--glow-color) 0%, transparent 70%);
  animation: breathe var(--breath-duration) ease-in-out infinite;
}

@keyframes breathe {
  0%, 100% { opacity: 0.3; transform: scale(1); }
  50% { opacity: 0.7; transform: scale(1.1); }
}
```

### 3.4 胶囊指示器 (capsule)

**位置**: 底部居中
**样式**: 圆角矩形，类似 iOS 电池指示器
**显示**: 百分比 + 进度条

```vue
<div class="capsule-indicator">
  <div class="capsule-body">
    <div class="capsule-fill" :style="{ width: tokensPercent + '%' }"></div>
  </div>
  <span class="capsule-text">{{ tokensPercent }}%</span>
</div>
```

```css
.capsule-indicator {
  position: absolute;
  bottom: 8px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: 4px;
}

.capsule-body {
  width: 48px;
  height: 8px;
  background: #1E293B;
  border-radius: 4px;
  overflow: hidden;
  border: 1px solid #334155;
}

.capsule-fill {
  height: 100%;
  background: var(--state-color);
  transition: width 0.4s ease;
}
```

### 3.5 头顶数字 (emoji)

**位置**: 宠物正上方
**内容**: 表情符号 + 百分比
**表情规则**:
- 0-30%: 😊 (开心)
- 31-60%: 😐 (平静)
- 61-80%: 😟 (担心)
- 81-95%: 😱 (恐慌)
- 96-100%: 💀 (死亡)

```vue
<div class="emoji-display">
  <span class="emoji-icon">{{ statusEmoji }}</span>
  <span class="emoji-percent">{{ tokensPercent }}%</span>
</div>
```

```typescript
const statusEmoji = computed(() => {
  const p = tokensPercent.value
  if (p <= 30) return '😊'
  if (p <= 60) return '😐'
  if (p <= 80) return '😟'
  if (p <= 95) return '😱'
  return '💀'
})
```

---

## 4. 配置结构

### 4.1 Rust 端

```rust
// src-tauri/src/config.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    /// 展示模式：bubble, ring, breathing, capsule, emoji
    #[serde(default = "default_display_mode")]
    pub display_mode: String,
}

fn default_display_mode() -> String {
    "bubble".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_config: ApiConfig,
    pub polling_config: PollingConfig,
    pub display_config: DisplayConfig,
}
```

### 4.2 默认配置

```json
{
  "api_config": { ... },
  "polling_config": { ... },
  "display_config": {
    "display_mode": "bubble"
  }
}
```

---

## 5. 数据流

### 5.1 启动流程

```
Tauri 启动
  → 加载 config.json
  → display_mode: "bubble"
  → Vue PetWidget 挂载
  → 读取 display_config
  → 条件渲染对应 UI
```

### 5.2 配置更新流程

```
用户打开设置面板
  → 选择新展示模式 (如: ring)
  → 保存配置 (saveConfig)
  → 写入 config.json
  → Rust 发送 config-changed 事件
  → Vue 监听事件
  → 重新渲染 PetWidget
```

### 5.3 Vue 端状态管理

```typescript
// src/composables/useDisplayMode.ts

import { computed } from 'vue'
import { useSettings } from './useSettings'

export function useDisplayMode() {
  const { config } = useSettings()

  const displayMode = computed<DisplayMode>(() => {
    return config.value.display_config?.display_mode || 'bubble'
  })

  return {
    displayMode
  }
}
```

---

## 6. 设置面板 UI 设计

### 6.1 新增标签页

```
┌─────────────────────────────────────────────┐
│ 设置                          [X]            │
├─────────────────────────────────────────────┤
│ [模型配置] [轮询设置] [展示设置] ◀─ 新增    │
├─────────────────────────────────────────────┤
│                                             │
│  ┌─────────────────────────────────────┐   │
│  │ 展示模式                             │   │
│  │ ┌─────────────────────────────────┐ │   │
│  │ │ ◉ 像素气泡  [▶ 预览]            │ │   │
│  │ │ ○ 圆环进度  [▶ 预览]            │ │   │
│  │ │ ○ 呼吸灯效  [▶ 预览]            │ │   │
│  │ │ ○ 胶囊指示器 [▶ 预览]           │ │   │
│  │ │ ○ 头顶数字  [▶ 预览]            │ │   │
│  │ └─────────────────────────────────┘ │   │
│  └─────────────────────────────────────┘   │
│                                             │
│  ┌─────────────────────────────────────┐   │
│  │ 实时预览                             │   │
│  │  ┌───────┐                          │   │
│  │  │ 🐱    │  ← 当前选中模式的预览   │   │
│  │  │  ⭕85%│                          │   │
│  │  └───────┘                          │   │
│  └─────────────────────────────────────┘   │
│                                             │
│  [取消]                    [保存]           │
└─────────────────────────────────────────────┘
```

### 6.2 交互设计

1. **单选按钮组**: 使用 RPG 风格的单选按钮
2. **预览按钮**: 点击 [▶ 预览] 可临时切换查看效果（3秒后自动恢复）
3. **实时预览区**: 显示当前选中模式的效果（带动画）
4. **保存后生效**: 点击保存后写入配置文件

### 6.3 预览功能实现

```typescript
// 临时预览状态
const previewMode = ref<DisplayMode | null>(null)

// 实际使用的模式（预览时显示预览值，否则显示配置值）
const activeDisplayMode = computed(() => {
  return previewMode.value || displayMode.value
})

// 开始预览
function startPreview(mode: DisplayMode) {
  previewMode.value = mode
  setTimeout(() => {
    previewMode.value = null
  }, 3000) // 3秒后自动恢复
}
```

---

## 7. 实现清单

### 7.1 Rust 端

- [ ] 更新 `Config` 结构体，添加 `display_config` 字段
- [ ] 实现 `DisplayConfig` 序列化/反序列化
- [ ] 添加默认值处理
- [ ] 更新配置迁移逻辑（处理旧配置文件）

### 7.2 Vue 端

- [ ] 创建 `src/types/config.ts`，添加类型定义
- [ ] 创建 `src/composables/useDisplayMode.ts`
- [ ] 更新 `src/composables/useSettings.ts`，添加 `display_config` 处理
- [ ] 更新 `PetWidget.vue`：
  - [ ] 导入 `useDisplayMode`
  - [ ] 实现圆环进度 UI
  - [ ] 实现呼吸灯效 UI
  - [ ] 实现胶囊指示器 UI
  - [ ] 实现头顶数字 UI
  - [ ] 添加条件渲染逻辑
- [ ] 更新 `SettingsPanel.vue`：
  - [ ] 添加"展示设置"标签页
  - [ ] 实现模式选择 UI
  - [ ] 实现预览功能
  - [ ] 实现实时预览区

### 7.3 样式文件

- [ ] 添加圆环进度样式
- [ ] 添加呼吸灯效样式
- [ ] 添加胶囊指示器样式
- [ ] 添加头顶数字样式
- [ ] 更新设置面板样式

---

## 8. 测试计划

### 8.1 单元测试

- [ ] `DisplayConfig` 序列化/反序列化测试
- [ ] `useDisplayMode` composable 测试
- [ ] 各展示模式组件渲染测试

### 8.2 集成测试

- [ ] 配置保存/加载流程测试
- [ ] 模式切换功能测试
- [ ] 预览功能测试

### 8.3 视觉测试

- [ ] 5种展示模式在不同状态下的视觉效果
- [ ] 动画流畅度测试
- [ ] 颜色主题一致性测试

---

## 9. 技术约束

1. **展示内容**: 仅显示 5h Token 剩余百分比
2. **切换方式**: 手动切换（在设置面板中）
3. **窗口大小**: 保持 96x96 像素（除扩展状态外）
4. **颜色主题**: 必须跟随现有的宠物状态颜色系统
5. **性能**: 动画不应影响应用性能

---

## 10. 未来扩展

- [ ] 支持用户自定义展示模式
- [ ] 支持同时显示多个指标（月度 + 5h）
- [ ] 支持展示模式插件系统
- [ ] 支持不同平台使用不同默认模式

---

**文档版本**: 1.0
**最后更新**: 2026-04-06
