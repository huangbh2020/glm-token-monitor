# 信息面板重构 + 热力图 实现计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 在宠物面板中添加"查看更多"按钮，点击展开详细信息面板，展示 Token 总用量、各模型消耗明细、以及 GitHub 风格的用量热力图（支持今天/7天/30天切换）。

**Architecture:** 新增 Rust Command `get_model_usage` 按需调用 `/api/monitor/usage/model-usage` API；Vue 端新增 `useModelUsage` composable 和 `UsageHeatmap` 组件；PetWidget/PopoverPanel 添加展开模式，窗口动态调整大小。

**Tech Stack:** Rust (reqwest, serde), Vue 3, TypeScript, Tauri 2.0

---

### Task 1: 新增 Rust 数据结构

**Files:**
- Modify: `src-tauri/src/events.rs`

**Step 1: 在 events.rs 末尾添加 model-usage API 的响应结构体**

```rust
// ── Model Usage API 响应结构 ──

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelUsageResponse {
    pub code: u32,
    pub data: Option<ModelUsageData>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelUsageData {
    pub x_time: Vec<String>,
    pub model_call_count: Vec<u64>,
    pub tokens_usage: Vec<u64>,
    pub total_usage: TotalUsage,
    pub model_data_list: Vec<ModelDataEntry>,
    pub model_summary_list: Vec<ModelSummary>,
    pub granularity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalUsage {
    pub total_model_call_count: u64,
    pub total_tokens_usage: u64,
    pub model_summary_list: Vec<ModelSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelSummary {
    pub model_name: String,
    pub total_tokens: u64,
    pub sort_order: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelDataEntry {
    pub model_name: String,
    pub sort_order: u32,
    pub tokens_usage: Vec<u64>,
    pub total_tokens: u64,
}
```

**Step 2: 编译验证**

Run: `cd src-tauri && cargo build`
Expected: 编译成功，无错误

**Step 3: Commit**

```bash
git add src-tauri/src/events.rs
git commit -m "feat: add model-usage API response data structures"
```

---

### Task 2: 新增 Rust API 调用函数

**Files:**
- Modify: `src-tauri/src/polling.rs`

**Step 1: 在 polling.rs 中添加 fetch_model_usage 函数**

在 `format_time` 函数之后、`fetch_usage` 函数之前添加：

```rust
/// 获取模型用量详情（按需调用，非轮询）
pub async fn fetch_model_usage(
    app: &AppHandle,
    start_time: &str,
    end_time: &str,
) -> Result<crate::events::ModelUsageData, String> {
    let model_config = get_active_model_config(app)?;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let url = format!("{}api/monitor/usage/model-usage", model_config.api_domain());

    let response = client
        .get(&url)
        .query(&[("startTime", start_time), ("endTime", end_time)])
        .header("Authorization", &model_config.api_key)
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    let api_resp: crate::events::ModelUsageResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if !api_resp.success {
        return Err(format!("API returned error code: {}", api_resp.code));
    }

    api_resp.data.ok_or("API response missing data field".to_string())
}
```

**Step 2: 编译验证**

Run: `cd src-tauri && cargo build`
Expected: 编译成功

**Step 3: Commit**

```bash
git add src-tauri/src/polling.rs
git commit -m "feat: add fetch_model_usage API call function"
```

---

### Task 3: 新增 Tauri Command

**Files:**
- Modify: `src-tauri/src/commands.rs`
- Modify: `src-tauri/src/main.rs`

**Step 1: 在 commands.rs 中添加 get_model_usage command**

```rust
use crate::events::{ModelUsageData, UsageData};
use crate::polling::{fetch_model_usage, fetch_usage};
use tauri::AppHandle;

/// 手动获取当前使用量（前端触发，用于按需刷新）
#[tauri::command]
pub async fn get_current_usage(app: AppHandle) -> Result<UsageData, String> {
    fetch_usage(&app).await
}

/// 获取模型用量详情（按时间范围）
#[tauri::command]
pub async fn get_model_usage(
    app: AppHandle,
    time_range: String,
) -> Result<ModelUsageData, String> {
    let now = chrono::Local::now();
    let (start_time, end_time) = match time_range.as_str() {
        "today" => {
            let start = now.format("%Y-%m-%d 00:00:00").to_string();
            let end = now.format("%Y-%m-%d 23:59:59").to_string();
            (start, end)
        }
        "7days" => {
            let start = (now - chrono::Duration::days(6))
                .format("%Y-%m-%d 00:00:00")
                .to_string();
            let end = now.format("%Y-%m-%d 23:59:59").to_string();
            (start, end)
        }
        "30days" => {
            let start = (now - chrono::Duration::days(29))
                .format("%Y-%m-%d 00:00:00")
                .to_string();
            let end = now.format("%Y-%m-%d 23:59:59").to_string();
            (start, end)
        }
        _ => return Err(format!("Invalid time_range: {}", time_range)),
    };

    fetch_model_usage(&app, &start_time, &end_time).await
}
```

**Step 2: 在 main.rs 中注册新 command**

在 `invoke_handler` 的 `generate_handler!` 宏中添加 `commands::get_model_usage`：

```rust
.invoke_handler(tauri::generate_handler![
    commands::get_current_usage,
    commands::get_model_usage,  // 新增
    // ... 其他 commands
])
```

**Step 3: 编译验证**

Run: `cd src-tauri && cargo build`
Expected: 编译成功

**Step 4: Commit**

```bash
git add src-tauri/src/commands.rs src-tauri/src/main.rs
git commit -m "feat: add get_model_usage tauri command with time range support"
```

---

### Task 4: 新增 useModelUsage composable

**Files:**
- Create: `src/composables/useModelUsage.ts`

**Step 1: 创建 composable**

```typescript
import { ref, type Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface ModelSummary {
  model_name: string
  total_tokens: number
  sort_order: number
}

export interface TotalUsage {
  total_model_call_count: number
  total_tokens_usage: number
  model_summary_list: ModelSummary[]
}

export interface ModelDataEntry {
  model_name: string
  sort_order: number
  tokens_usage: number[]
  total_tokens: number
}

export interface ModelUsageData {
  x_time: string[]
  model_call_count: number[]
  tokens_usage: number[]
  total_usage: TotalUsage
  model_data_list: ModelDataEntry[]
  model_summary_list: ModelSummary[]
  granularity: string
}

export type TimeRange = 'today' | '7days' | '30days'

export function useModelUsage() {
  const modelUsageData: Ref<ModelUsageData | null> = ref(null)
  const isLoading = ref(false)
  const error = ref<string>('')
  const activeTab: Ref<TimeRange> = ref('today')

  async function fetchModelUsage(timeRange: TimeRange) {
    isLoading.value = true
    error.value = ''
    try {
      const data = await invoke<ModelUsageData>('get_model_usage', {
        timeRange,
      })
      modelUsageData.value = data
      activeTab.value = timeRange
    } catch (err) {
      error.value = String(err)
      console.error('Failed to fetch model usage:', err)
    } finally {
      isLoading.value = false
    }
  }

  return {
    modelUsageData,
    isLoading,
    error,
    activeTab,
    fetchModelUsage,
  }
}
```

**Step 2: TypeScript 编译验证**

Run: `npm run build`
Expected: 编译成功（或 vue-tsc 兼容性问题可忽略，vite build 成功即可）

**Step 3: Commit**

```bash
git add src/composables/useModelUsage.ts
git commit -m "feat: add useModelUsage composable for model usage data"
```

---

### Task 5: 新增 UsageHeatmap 组件

**Files:**
- Create: `src/components/UsageHeatmap.vue`

**Step 1: 创建热力图组件**

```vue
<script setup lang="ts">
import { computed } from 'vue'
import type { ModelUsageData, TimeRange } from '../composables/useModelUsage'

const props = defineProps<{
  data: ModelUsageData
  timeRange: TimeRange
}>()

// 将数据按天分组
const dailyData = computed(() => {
  const { x_time, tokens_usage } = props.data
  const days: { label: string; hours: number[] }[] = []

  if (props.timeRange === 'today') {
    // 今天：直接取 24 小时
    const today = new Date()
    const label = `${today.getMonth() + 1}/${today.getDate()}`
    days.push({ label, hours: tokens_usage.slice(0, 24) })
  } else {
    // 7天/30天：按日期分组
    const dateMap = new Map<string, number[]>()
    x_time.forEach((time, i) => {
      const date = time.split(' ')[0] // "2026-04-29"
      if (!dateMap.has(date)) {
        dateMap.set(date, new Array(24).fill(0))
      }
      const hour = parseInt(time.split(' ')[1]?.split(':')[0] || '0')
      dateMap.get(date)![hour] = tokens_usage[i] || 0
    })

    dateMap.forEach((hours, date) => {
      const d = new Date(date)
      const label = `${d.getMonth() + 1}/${d.getDate()}`
      days.push({ label, hours })
    })
  }

  return days
})

// 计算色阶（基于当天最大值的相对值）
const maxUsage = computed(() => {
  let max = 0
  dailyData.value.forEach(day => {
    day.hours.forEach(v => { if (v > max) max = v })
  })
  return max || 1
})

function getColor(value: number): string {
  if (value === 0) return 'var(--heatmap-empty)'
  const ratio = value / maxUsage.value
  if (ratio < 0.25) return 'var(--heatmap-low)'
  if (ratio < 0.5) return 'var(--heatmap-mid)'
  if (ratio < 0.75) return 'var(--heatmap-high)'
  return 'var(--heatmap-max)'
}

function formatTokens(value: number): string {
  if (value === 0) return '0'
  if (value >= 1_000_000) return `${(value / 1_000_000).toFixed(1)}M`
  if (value >= 1_000) return `${(value / 1_000).toFixed(1)}K`
  return String(value)
}

// 小时标签
const hourLabels = ['0', '', '', '3', '', '', '6', '', '', '9', '', '', '12', '', '', '15', '', '', '18', '', '', '21', '', '']
</script>

<template>
  <div class="heatmap-container">
    <!-- 小时标签 -->
    <div class="hour-labels">
      <span class="day-label-spacer"></span>
      <span v-for="(h, i) in hourLabels" :key="i" class="hour-label">{{ h }}</span>
    </div>
    <!-- 热力图网格 -->
    <div class="heatmap-grid" :class="`range-${timeRange}`">
      <div v-for="(day, di) in dailyData" :key="di" class="heatmap-row">
        <span class="day-label">{{ day.label }}</span>
        <span
          v-for="(val, hi) in day.hours"
          :key="hi"
          class="heatmap-cell"
          :style="{ background: getColor(val) }"
          :title="`${day.label} ${hi}:00 - ${formatTokens(val)} tokens`"
        ></span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.heatmap-container {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.hour-labels {
  display: flex;
  align-items: center;
  gap: 2px;
  margin-bottom: 2px;
}

.day-label-spacer {
  width: 32px;
  flex-shrink: 0;
}

.hour-label {
  width: 8px;
  font-size: 8px;
  color: var(--text-muted, #71717a);
  text-align: center;
  line-height: 1;
}

.heatmap-grid {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.heatmap-grid.range-30days {
  max-height: 200px;
  overflow-y: auto;
  padding-right: 4px;
}

.heatmap-row {
  display: flex;
  align-items: center;
  gap: 2px;
}

.day-label {
  width: 32px;
  font-size: 9px;
  color: var(--text-muted, #71717a);
  text-align: right;
  padding-right: 4px;
  flex-shrink: 0;
  font-family: ui-monospace, monospace;
}

.heatmap-cell {
  width: 8px;
  height: 8px;
  border-radius: 2px;
  cursor: pointer;
  transition: transform 0.1s ease;
  flex-shrink: 0;
}

.heatmap-cell:hover {
  transform: scale(1.4);
  outline: 1px solid rgba(255, 255, 255, 0.3);
}

/* 色阶变量 */
:root {
  --heatmap-empty: #161b22;
  --heatmap-low: #0e4429;
  --heatmap-mid: #006d32;
  --heatmap-high: #26a641;
  --heatmap-max: #39d353;
}

[data-theme="light"] {
  --heatmap-empty: #ebedf0;
  --heatmap-low: #9be9a8;
  --heatmap-mid: #40c463;
  --heatmap-high: #30a14e;
  --heatmap-max: #216e39;
}
</style>
```

**Step 2: 编译验证**

Run: `npm run build`
Expected: 编译成功

**Step 3: Commit**

```bash
git add src/components/UsageHeatmap.vue
git commit -m "feat: add UsageHeatmap component with GitHub-style heatmap"
```

---

### Task 6: 修改 PetWidget.vue 添加展开面板

**Files:**
- Modify: `src/components/PetWidget.vue`

**Step 1: 添加 import 和 composable**

在 `<script setup>` 中添加：

```typescript
import { useModelUsage } from '../composables/useModelUsage'
import UsageHeatmap from './UsageHeatmap.vue'

const { modelUsageData, isLoading: isModelLoading, error: modelError, activeTab, fetchModelUsage } = useModelUsage()
```

**Step 2: 添加展开状态和方法**

```typescript
const isExpanded = ref(false)

async function toggleExpanded() {
  if (!isExpanded.value) {
    isExpanded.value = true
    // 默认加载今天的数据
    if (!modelUsageData.value) {
      await fetchModelUsage('today')
    }
  } else {
    isExpanded.value = false
  }
}

async function switchTab(tab: 'today' | '7days' | '30days') {
  await fetchModelUsage(tab)
}

// 格式化 Token 数量
function formatTokens(value: number): string {
  if (value >= 1_000_000) return `${(value / 1_000_000).toFixed(1)}M`
  if (value >= 1_000) return `${(value / 1_000).toFixed(1)}K`
  return String(value)
}
```

**Step 3: 调整窗口尺寸 watch**

修改已有的 `watch([hasApiKey, showInfoPanel, displayMode], ...)` 为：

```typescript
watch([hasApiKey, showInfoPanel, displayMode, isExpanded], async ([hasKey, showPanel, mode, expanded]) => {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    if (!hasKey) {
      await invoke('resize_main_window', { width: 160, height: 180 })
    } else if (expanded) {
      // 展开模式：根据 tab 调整高度
      const heights = { today: 380, '7days': 440, '30days': 520 }
      await invoke('resize_main_window', { width: 340, height: heights[activeTab.value] || 380 })
    } else if (showPanel) {
      await invoke('resize_main_window', { width: 160, height: 240 })
    } else if (mode === 'pedestal') {
      await invoke('resize_main_window', { width: 160, height: 150 })
    } else {
      await invoke('resize_main_window', { width: 120, height: 120 })
    }
  } catch (err) {
    console.error('Failed to resize window:', err)
  }
}, { immediate: true })
```

**Step 4: 在模板中添加展开面板**

在 `</transition>` (info-bubble 的 transition) 之后，添加展开面板：

```vue
<!-- 展开详情面板 -->
<transition name="panel-expand">
  <div v-if="isExpanded && hasApiKey" class="expanded-panel" :data-theme="currentTheme"
    @mousedown.stop @click.stop @dblclick.stop>
    <!-- 顶部栏 -->
    <div class="expanded-header">
      <span class="expanded-title">用量详情</span>
      <button class="info-btn close" @click="isExpanded = false" title="收起">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <path d="M18 6L6 18M6 6l12 12"/>
        </svg>
      </button>
    </div>

    <!-- 加载状态 -->
    <div v-if="isModelLoading" class="loading-state">
      <svg class="spinning" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M23 4v6h-6M1 20v-6h6"/>
        <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/>
      </svg>
      <span>加载中...</span>
    </div>

    <!-- 错误状态 -->
    <div v-else-if="modelError" class="error-state">
      <span>{{ modelError.slice(0, 30) }}</span>
    </div>

    <!-- 数据展示 -->
    <template v-else-if="modelUsageData">
      <!-- 概览 -->
      <div class="overview-section">
        <div class="overview-item">
          <span class="overview-label">总用量</span>
          <span class="overview-value">{{ formatTokens(modelUsageData.total_usage.total_tokens_usage) }}</span>
        </div>
        <div class="overview-item">
          <span class="overview-label">调用次数</span>
          <span class="overview-value">{{ modelUsageData.total_usage.total_model_call_count }}</span>
        </div>
      </div>

      <!-- 模型明细 -->
      <div class="models-section">
        <div v-for="m in modelUsageData.model_summary_list" :key="m.model_name" class="model-row">
          <span class="model-name">{{ m.model_name }}</span>
          <span class="model-tokens">{{ formatTokens(m.total_tokens) }}</span>
        </div>
      </div>

      <!-- Tab 栏 -->
      <div class="tab-bar">
        <button
          v-for="tab in (['today', '7days', '30days'] as const)"
          :key="tab"
          class="tab-btn"
          :class="{ active: activeTab === tab }"
          @click="switchTab(tab)"
        >
          {{ tab === 'today' ? '今天' : tab === '7days' ? '7天' : '30天' }}
        </button>
      </div>

      <!-- 热力图 -->
      <UsageHeatmap :data="modelUsageData" :time-range="activeTab" />
    </template>
  </div>
</transition>
```

**Step 5: 在 info-bubble 中添加"查看更多"按钮**

在 info-bubble 的 `</div>` 结束标签之前，`</div>` 之后添加：

```vue
<!-- 在 info-bubble 的数据区域之后 -->
<div class="view-more-row">
  <button class="view-more-btn" @click.stop="toggleExpanded">
    {{ isExpanded ? '收起' : '查看更多' }}
    <svg :class="{ rotated: isExpanded }" width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <polyline points="6 9 12 15 18 9"></polyline>
    </svg>
  </button>
</div>
```

**Step 6: 添加展开面板的 CSS 样式**

在 `<style scoped>` 中添加：

```css
/* ── 查看更多按钮 ── */
.view-more-row {
  margin-top: 8px;
  text-align: center;
}

.view-more-btn {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px 12px;
  background: rgba(59, 130, 246, 0.1);
  border: 1px solid rgba(59, 130, 246, 0.2);
  border-radius: 10px;
  color: #60a5fa;
  font-size: 10px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.view-more-btn:hover {
  background: rgba(59, 130, 246, 0.2);
  border-color: rgba(59, 130, 246, 0.4);
}

.view-more-btn svg {
  transition: transform 0.2s ease;
}

.view-more-btn svg.rotated {
  transform: rotate(180deg);
}

/* ── 展开详情面板 ── */
.expanded-panel {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  width: 320px;
  background: rgba(15, 15, 17, 0.96);
  border: 1px solid rgba(59, 130, 246, 0.3);
  border-radius: 12px;
  padding: 12px;
  z-index: 200;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5), 0 0 12px rgba(59, 130, 246, 0.15);
  backdrop-filter: blur(12px);
  pointer-events: auto;
  max-height: calc(100% - 20px);
  overflow-y: auto;
}

.expanded-panel[data-theme="light"] {
  background: rgba(255, 255, 255, 0.98);
  border-color: rgba(59, 130, 246, 0.25);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.12);
}

.expanded-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 10px;
}

.expanded-title {
  font-size: 13px;
  font-weight: 600;
  color: #e4e4e7;
}

.expanded-panel[data-theme="light"] .expanded-title {
  color: #1c1c1e;
}

/* 概览 */
.overview-section {
  display: flex;
  gap: 12px;
  margin-bottom: 10px;
}

.overview-item {
  flex: 1;
  background: rgba(255, 255, 255, 0.04);
  border-radius: 8px;
  padding: 8px 10px;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.expanded-panel[data-theme="light"] .overview-item {
  background: rgba(0, 0, 0, 0.03);
}

.overview-label {
  font-size: 9px;
  color: #71717a;
  font-weight: 500;
}

.overview-value {
  font-size: 16px;
  font-weight: 700;
  color: #e4e4e7;
  font-family: ui-monospace, monospace;
}

.expanded-panel[data-theme="light"] .overview-value {
  color: #1c1c1e;
}

/* 模型明细 */
.models-section {
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin-bottom: 10px;
}

.model-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 10px;
  background: rgba(255, 255, 255, 0.03);
  border-radius: 6px;
}

.expanded-panel[data-theme="light"] .model-row {
  background: rgba(0, 0, 0, 0.02);
}

.model-name {
  font-size: 11px;
  font-weight: 500;
  color: #a1a1aa;
}

.expanded-panel[data-theme="light"] .model-name {
  color: #6b7280;
}

.model-tokens {
  font-size: 12px;
  font-weight: 600;
  color: #e4e4e7;
  font-family: ui-monospace, monospace;
}

.expanded-panel[data-theme="light"] .model-tokens {
  color: #1c1c1e;
}

/* Tab 栏 */
.tab-bar {
  display: flex;
  gap: 4px;
  margin-bottom: 8px;
  padding: 3px;
  background: rgba(255, 255, 255, 0.04);
  border-radius: 8px;
}

.expanded-panel[data-theme="light"] .tab-bar {
  background: rgba(0, 0, 0, 0.03);
}

.tab-btn {
  flex: 1;
  padding: 5px 0;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: #71717a;
  font-size: 11px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.tab-btn.active {
  background: rgba(59, 130, 246, 0.2);
  color: #60a5fa;
}

.tab-btn:hover:not(.active) {
  background: rgba(255, 255, 255, 0.06);
  color: #a1a1aa;
}

.expanded-panel[data-theme="light"] .tab-btn.active {
  background: rgba(59, 130, 246, 0.15);
  color: #2563eb;
}

.expanded-panel[data-theme="light"] .tab-btn:hover:not(.active) {
  background: rgba(0, 0, 0, 0.04);
}

/* 加载和错误状态 */
.loading-state, .error-state {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 20px;
  color: #71717a;
  font-size: 11px;
}

.error-state {
  color: #fca5a5;
}

/* 展开动画 */
.panel-expand-enter-active,
.panel-expand-leave-active {
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.panel-expand-enter-from,
.panel-expand-leave-to {
  opacity: 0;
  transform: translate(-50%, -50%) scale(0.9);
}
```

**Step 7: 编译验证**

Run: `npm run build`
Expected: 编译成功

**Step 8: Commit**

```bash
git add src/components/PetWidget.vue
git commit -m "feat: add expanded detail panel with heatmap to PetWidget"
```

---

### Task 7: 修改 PopoverPanel.vue 添加展开面板

**Files:**
- Modify: `src/components/PopoverPanel.vue`

**Step 1: 添加 import 和 composable**

与 Task 6 相同的 import 和 composable 初始化。

**Step 2: 添加展开状态和方法**

与 Task 6 相同的 `isExpanded`、`toggleExpanded`、`switchTab`、`formatTokens`。

**Step 3: 调整窗口尺寸 watch**

与 Task 6 相同的逻辑，添加 `isExpanded` 到 watch 依赖。

**Step 4: 在模板中添加"查看更多"按钮和展开面板**

与 Task 6 相同的模板结构。

**Step 5: 添加 CSS 样式**

与 Task 6 相同的 CSS。

**Step 6: 编译验证**

Run: `npm run build`
Expected: 编译成功

**Step 7: Commit**

```bash
git add src/components/PopoverPanel.vue
git commit -m "feat: add expanded detail panel with heatmap to PopoverPanel"
```

---

### Task 8: 端到端测试

**Step 1: 启动开发模式**

Run: `npm run tauri:dev`
Expected: 应用启动，宠物正常显示

**Step 2: 测试信息面板**

- 点击宠物 → 信息面板气泡出现
- 确认底部显示"查看更多"按钮

**Step 3: 测试展开面板**

- 点击"查看更多" → 窗口扩大，显示加载状态
- 确认概览数据（总用量、调用次数）正确显示
- 确认模型明细列表正确显示
- 确认热力图正确渲染（今天视图，1行24列）

**Step 4: 测试 Tab 切换**

- 点击"7天" Tab → 热力图切换为7行
- 点击"30天" Tab → 热力图切换为30行（可滚动）
- 点击"今天" Tab → 回到1行

**Step 5: 测试收起**

- 点击"收起"按钮 → 窗口恢复原大小
- 确认宠物正常显示

**Step 6: 测试 macOS 端（如可用）**

- 重复 Step 2-5 在 PopoverPanel 上

**Step 7: Commit（如有修复）**

```bash
git add -A
git commit -m "fix: e2e testing fixes for info panel heatmap"
```

---

### Task 9: 最终构建验证

**Step 1: 前端构建**

Run: `npm run build`
Expected: 构建成功

**Step 2: Rust 构建**

Run: `cd src-tauri && cargo build`
Expected: 编译成功，无警告

**Step 3: 完整构建**

Run: `npm run tauri:build`
Expected: 构建成功

**Step 4: Final Commit**

```bash
git add -A
git commit -m "feat: complete info panel refactor with model usage heatmap"
```
