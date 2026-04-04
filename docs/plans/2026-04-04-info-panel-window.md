# 独立信息面板窗口实施计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**目标:** 将内嵌在 PetWidget 中的信息面板重构为独立的 Tauri 窗口，点击宠物时打开独立窗口展示用量详情。

**架构:** 使用 Tauri 多窗口架构，主窗口负责展示宠物，副窗口 (info-panel) 负责展示用量详情。通过 Tauri Events 实现数据同步，通过 Commands 实现窗口控制。

**技术栈:** Vue 3, TypeScript, Tauri 2.x, Rust

---

## Task 1: 创建 InfoPanel 组件

**Files:**
- Create: `src/components/InfoPanel.vue`

**Step 1: 创建 InfoPanel.vue 基础结构**

```vue
<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useUsageState } from '../composables/useUsageState'
import { useTauriEvents } from '../composables/useTauriEvents'

const { usageData, setupEventListener } = useTauriEvents()
const { petState } = useUsageState(
  computed(() => usageData.value.used),
  computed(() => usageData.value.total)
)

// 双指标数据
const timePercent = computed(() => usageData.value.time_percent ?? 0)
const tokensPercent = computed(() => usageData.value.tokens_percent ?? 0)
const timeRemaining = computed(() => usageData.value.time_remaining)

// 心语映射
const heartMessages: Record<string, string> = {
  Fresh:   '新的一天，能量满格！冲冲冲！',
  Flow:    '代码写得正顺手，不要打扰我~',
  Warning: '用量有点多了，要省着点...',
  Panic:   '要炸了！谁在疯狂 Call API？！',
  Dead:    '系统崩溃... 请充值续命...',
}
const heartMsg = computed(() => heartMessages[petState.value])

// 状态
const isRefreshing = ref(false)
const lastUpdateTime = ref<string>('')
const fetchError = ref<string>('')

// 手动刷新数据
async function refreshUsageData() {
  try {
    isRefreshing.value = true
    const { invoke } = await import('@tauri-apps/api/core')
    const data = await invoke<typeof usageData.value>('get_current_usage')
    usageData.value = data
    const now = new Date()
    lastUpdateTime.value = `${now.getHours().toString().padStart(2,'0')}:${now.getMinutes().toString().padStart(2,'0')}:${now.getSeconds().toString().padStart(2,'0')}`
    fetchError.value = ''
  } catch (err) {
    fetchError.value = String(err)
    console.error('Refresh failed:', err)
  } finally {
    isRefreshing.value = false
  }
}

// 关闭窗口
async function closeWindow() {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('close_info_panel')
  } catch (err) {
    console.error('Close window failed:', err)
  }
}

onMounted(async () => {
  await setupEventListener()
  refreshUsageData()
})
</script>

<template>
  <div class="info-panel" :class="`panel-${petState.toLowerCase()}`">
    <div class="panel-header">
      <span class="panel-title">📊 用量详情</span>
      <button class="panel-close" @click="closeWindow">×</button>
    </div>

    <div class="panel-content">
      <!-- 状态心语 -->
      <div class="panel-section">
        <div class="section-label">💭 状态</div>
        <div class="heart-text-large">{{ heartMsg }}</div>
      </div>

      <!-- 月度额度 -->
      <div class="panel-section">
        <div class="section-label">🗓️ 月度额度</div>
        <div class="metric-row">
          <div class="metric-info">
            <span class="metric-percent">{{ timePercent }}%</span>
            <span class="metric-remaining">剩余 {{ timeRemaining }} 次</span>
          </div>
          <div class="progress-bar">
            <div class="progress-fill" :class="`progress-${petState.toLowerCase()}`" :style="{ width: timePercent + '%' }"></div>
          </div>
        </div>
      </div>

      <!-- 5小时额度 -->
      <div class="panel-section">
        <div class="section-label">⏱️ 5小时额度</div>
        <div class="metric-row">
          <div class="metric-info">
            <span class="metric-percent">{{ tokensPercent }}%</span>
            <span class="metric-remaining">剩余 {{ 100 - tokensPercent }}%</span>
          </div>
          <div class="progress-bar">
            <div class="progress-fill progress-tokens" :style="{ width: tokensPercent + '%' }"></div>
          </div>
        </div>
      </div>

      <!-- 最后更新时间 -->
      <div class="panel-section">
        <div class="section-label">🔄 最后更新</div>
        <div class="update-time">{{ lastUpdateTime || '加载中...' }}</div>
      </div>

      <!-- 错误提示 -->
      <div v-if="fetchError" class="panel-section">
        <div class="error-message">⚠ {{ fetchError }}</div>
      </div>

      <!-- 刷新按钮 -->
      <button class="refresh-btn" @click="refreshUsageData" :disabled="isRefreshing">
        {{ isRefreshing ? '刷新中...' : '🔄 刷新' }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.info-panel {
  width: 100vw;
  height: 100vh;
  background: rgba(15, 23, 42, 0.95);
  backdrop-filter: blur(10px);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  font-family: 'PingFang SC', 'Microsoft YaHei', sans-serif;
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 14px;
  background: rgba(30, 41, 59, 0.8);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.panel-title {
  font-size: 12px;
  font-weight: 600;
  color: #E2E8F0;
}

.panel-close {
  width: 24px;
  height: 24px;
  border: none;
  background: rgba(239, 68, 68, 0.2);
  color: #F87171;
  border-radius: 4px;
  cursor: pointer;
  font-size: 18px;
  line-height: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.panel-close:hover {
  background: rgba(239, 68, 68, 0.4);
  transform: scale(1.1);
}

.panel-content {
  flex: 1;
  padding: 14px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.panel-section {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.section-label {
  font-size: 10px;
  color: #94A3B8;
  font-weight: 500;
}

.heart-text-large {
  font-size: 12px;
  color: #CBD5E1;
  line-height: 1.5;
}

.metric-row {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.metric-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.metric-percent {
  font-size: 16px;
  font-weight: 700;
  font-family: ui-monospace, monospace;
}

.metric-remaining {
  font-size: 10px;
  color: #94A3B8;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  border-radius: 4px;
  transition: width 0.4s ease;
}

.progress-fresh { background: linear-gradient(90deg, #10B981, #34D399); }
.progress-flow { background: linear-gradient(90deg, #3B82F6, #60A5FA); }
.progress-warning { background: linear-gradient(90deg, #F59E0B, #FBBF24); }
.progress-panic { background: linear-gradient(90deg, #EF4444, #F87171); }
.progress-dead { background: linear-gradient(90deg, #6B7280, #9CA3AF); }
.progress-tokens { background: linear-gradient(90deg, #3B82F6, #60A5FA); }

.update-time {
  font-size: 10px;
  color: #64748B;
  font-family: ui-monospace, monospace;
}

.error-message {
  font-size: 10px;
  color: #F87171;
  background: rgba(239, 68, 68, 0.1);
  padding: 6px 8px;
  border-radius: 4px;
  line-height: 1.4;
}

.refresh-btn {
  margin-top: auto;
  padding: 8px 12px;
  background: rgba(59, 130, 246, 0.2);
  color: #60A5FA;
  border: 1px solid rgba(59, 130, 246, 0.3);
  border-radius: 6px;
  font-size: 11px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.refresh-btn:hover:not(:disabled) {
  background: rgba(59, 130, 246, 0.3);
}

.refresh-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>
```

**Step 2: 验证文件创建**

Run: `ls -la src/components/InfoPanel.vue`
Expected: 文件存在

**Step 3: 提交**

```bash
git add src/components/InfoPanel.vue
git commit -m "feat: create InfoPanel component for independent window"
```

---

## Task 2: 创建副窗口 Vue 入口文件

**Files:**
- Create: `src/info-panel-entry.ts`

**Step 1: 创建入口文件**

```typescript
import { createApp } from 'vue'
import InfoPanel from './components/InfoPanel.vue'
import './style.css'

const app = createApp(InfoPanel)
app.mount('#app')
```

**Step 2: 验证文件创建**

Run: `ls -la src/info-panel-entry.ts`
Expected: 文件存在

**Step 3: 提交**

```bash
git add src/info-panel-entry.ts
git commit -m "feat: create entry point for info panel window"
```

---

## Task 3: 注册副窗口到 Tauri 配置

**Files:**
- Modify: `src-tauri/tauri.conf.json`

**Step 1: 读取当前配置**

Run: `cat src-tauri/tauri.conf.json`
Expected: 查看当前 windows 配置

**Step 2: 在 windows 数组中添加 info-panel 窗口配置**

在 `windows` 数组中，现有 `"label": "main"` 的配置后面添加：

```json
{
  "label": "info-panel",
  "title": "用量详情",
  "width": 280,
  "height": 320,
  "decorations": false,
  "transparent": true,
  "alwaysOnTop": true,
  "visible": false,
  "skipTaskbar": true,
  "url": "info-panel.html"
}
```

**Step 3: 验证配置文件语法**

Run: `cd src-tauri && cargo check`
Expected: 编译检查通过

**Step 4: 提交**

```bash
git add src-tauri/tauri.conf.json
git commit -m "feat: register info-panel window in Tauri config"
```

---

## Task 4: 创建窗口管理模块

**Files:**
- Create: `src-tauri/src/windows.rs`

**Step 1: 创建窗口管理模块**

```rust
use tauri::{WebviewWindow, Manager, Result};

/// 打开信息面板窗口
#[tauri::command]
pub async fn open_info_panel(
    main_window: WebviewWindow,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    // 获取或创建 info-panel 窗口
    let info_panel = match app_handle.get_webview_window("info-panel") {
        Some(window) => window,
        None => return Err("Info panel window not found".to_string()),
    };

    // 获取主窗口位置
    let main_pos = main_window
        .current_position()
        .map_err(|e| format!("Failed to get main window position: {}", e))?;

    // 计算副窗口位置（主窗口右侧，间隔 8px）
    let panel_width = 280;
    let main_width = 96;
    let gap = 8;
    let mut info_x = main_pos.x + main_width as i32 + gap;
    let info_y = main_pos.y;

    // 获取主显示器信息
    if let Some(monitor) = main_window.current_monitor().map_err(|e| format!("Failed to get monitor: {}", e))? {
        let screen_size = monitor.size();
        // 如果超出屏幕右侧，则显示在左侧
        if info_x + panel_width > screen_size.width as i32 {
            info_x = main_pos.x - panel_width - gap;
        }
    }

    // 设置窗口位置
    info_panel
        .set_position(tauri::Position::Physical(tauri::PhysicalPosition {
            x: info_x.max(0),
            y: info_y.max(0),
        }))
        .map_err(|e| format!("Failed to set info panel position: {}", e))?;

    // 显示窗口并设置焦点
    info_panel.show().map_err(|e| format!("Failed to show info panel: {}", e))?;
    info_panel.set_focus().map_err(|e| format!("Failed to focus info panel: {}", e))?;

    Ok(())
}

/// 关闭信息面板窗口
#[tauri::command]
pub async fn close_info_panel(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(info_panel) = app_handle.get_webview_window("info-panel") {
        info_panel.hide().map_err(|e| format!("Failed to hide info panel: {}", e))?;
    }
    Ok(())
}
```

**Step 2: 验证文件创建**

Run: `ls -la src-tauri/src/windows.rs`
Expected: 文件存在

**Step 3: 提交**

```bash
git add src-tauri/src/windows.rs
git commit -m "feat: create window management module"
```

---

## Task 5: 集成窗口管理到 main.rs

**Files:**
- Modify: `src-tauri/src/main.rs`

**Step 1: 在文件顶部添加模块声明**

在现有 `mod` 声明后添加：

```rust
mod windows;
```

**Step 2: 在 invoke_handler 中注册新命令**

找到 `invoke_handler` 宏调用，在现有命令后添加：

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // ... 其他配置 ...
        .invoke_handler(tauri::generate_handler![
            // ... 现有命令 ...
            get_current_usage,
            windows::open_info_panel,
            windows::close_info_panel,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Step 3: 验证编译**

Run: `cd src-tauri && cargo check`
Expected: 编译检查通过

**Step 4: 提交**

```bash
git add src-tauri/src/main.rs
git commit -m "feat: integrate window management commands"
```

---

## Task 6: 确保 usage-update 事件广播到所有窗口

**Files:**
- Modify: `src-tauri/src/polling.rs`

**Step 1: 检查当前事件发送方式**

Run: `grep -n "emit.*usage-update" src-tauri/src/polling.rs`
Expected: 查看当前事件发送代码

**Step 2: 修改事件发送为广播所有窗口**

如果当前使用 `app.emit()`，确保改为 `app.emit_all()` 以确保所有窗口都能收到事件：

```rust
app.emit_all("usage-update", usage_data)?;
```

**Step 3: 验证编译**

Run: `cd src-tauri && cargo check`
Expected: 编译检查通过

**Step 4: 提交**

```bash
git add src-tauri/src/polling.rs
git commit -m "fix: broadcast usage-update to all windows"
```

---

## Task 7: 修改 PetWidget 点击事件

**Files:**
- Modify: `src/components/PetWidget.vue`

**Step 1: 移除侧边面板相关状态**

删除以下代码：

```typescript
// 删除这些行
const showSidePanel = ref(false)
```

```typescript
// 删除 toggleSidePanel 函数
async function toggleSidePanel() {
  // ... 删除整个函数
}
```

**Step 2: 修改点击处理函数**

将 `handleClick` 函数简化为：

```typescript
// 点击处理（区分拖动和点击）
const handleClick = async (event: MouseEvent) => {
  const dragDuration = Date.now() - dragStartTime
  const dragDistance = Math.sqrt(
    Math.pow(event.clientX - dragStartPos.x, 2) +
    Math.pow(event.clientY - dragStartPos.y, 2)
  )

  // 如果移动距离小于5px且持续时间小于300ms，认为是点击而非拖动
  if (dragDistance < 5 && dragDuration < 300) {
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('open_info_panel')
    } catch (err) {
      console.error('Open info panel failed:', err)
    }
  }
}
```

**Step 3: 移除侧边面板模板**

删除从 `<!-- 右侧信息面板 -->` 到对应 `</transition>` 的整个代码块。

**Step 4: 移除侧边面板相关样式**

删除 `.side-info-panel`, `.panel-header`, `.panel-content`, `.panel-section` 等与侧边面板相关的所有 CSS 样式。

**Step 5: 验证编译**

Run: `npm run build`
Expected: 构建成功

**Step 6: 提交**

```bash
git add src/components/PetWidget.vue
git commit -m "refactor: replace embedded panel with window invocation"
```

---

## Task 8: 配置副窗口的 HTML 入口

**Files:**
- Create: `src-tauri/info-panel.html`

**Step 1: 创建副窗口 HTML 文件**

```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>用量详情</title>
</head>
<body>
  <div id="app"></div>
  <script type="module" src="/info-panel-entry.ts"></script>
</body>
</html>
```

**Step 2: 验证文件创建**

Run: `ls -la src-tauri/info-panel.html`
Expected: 文件存在

**Step 3: 提交**

```bash
git add src-tauri/info-panel.html
git commit -m "feat: create HTML entry for info panel window"
```

---

## Task 9: 构建并测试

**Step 1: 构建应用**

Run: `npm run tauri build`
Expected: 构建成功，无错误

**Step 2: 运行开发模式测试**

Run: `npm run tauri:dev`
Expected: 应用启动

**Step 3: 手动测试窗口打开/关闭**

1. 点击宠物 widget
2. 验证信息面板窗口在主窗口右侧打开
3. 验证窗口位置正确（不超出屏幕）
4. 点击关闭按钮 (×)
5. 验证窗口关闭

**Step 4: 测试数据同步**

1. 打开信息面板窗口
2. 等待轮询更新（5分钟）或点击刷新按钮
3. 验证两个窗口的数据同步更新

**Step 5: 记录测试结果**

如果测试通过，提交最终版本：

```bash
git commit --allow-empty -m "test: verify info panel window functionality"
```

---

## 测试检查清单

- [ ] 点击宠物能打开信息面板窗口
- [ ] 信息面板窗口位置正确（主窗口右侧，间隔 8px）
- [ ] 窗口超出屏幕时自动调整到左侧
- [ ] 关闭按钮 (×) 能关闭窗口
- [ ] 数据在两个窗口间同步更新
- [ ] 刷新按钮能手动刷新数据
- [ ] 错误状态正确显示
- [ ] 窗口样式符合设计规范
