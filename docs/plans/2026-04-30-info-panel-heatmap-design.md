# 信息面板重构 + 热力图设计

## 目标

1. 在宠物面板（PetWidget / PopoverPanel）中添加"查看更多"按钮，点击后内嵌展开详细信息面板
2. 重构信息面板，展示 Token 总用量、各模型消耗明细
3. 新增 GitHub 风格热力图，展示按小时粒度的用量分布
4. 支持今天/7天/30天三种时间范围切换

## 数据源

### 新增 API 端点
```
GET /api/monitor/usage/model-usage?startTime=YYYY-MM-DD+HH:MM:SS&endTime=YYYY-MM-DD+HH:MM:SS
```

返回结构：
- `x_time`: 时间轴数组（小时标签）
- `modelCallCount`: 每小时调用次数
- `tokensUsage`: 每小时 Token 用量
- `totalUsage.totalModelCallCount`: 总调用次数
- `totalUsage.totalTokensUsage`: 总 Token 用量
- `totalUsage.modelSummaryList`: 各模型汇总 [{modelName, totalTokens}]
- `modelDataList`: 各模型明细 [{modelName, tokensUsage[], totalTokens}]
- `granularity`: 粒度（"hourly"）

### 时间范围
| Tab | startTime | endTime |
|-----|-----------|---------|
| 今天 | 当天 00:00:00 | 当天 23:59:59 |
| 7天 | 7天前 00:00:00 | 当天 23:59:59 |
| 30天 | 30天前 00:00:00 | 当天 23:59:59 |

## 架构变更

### Rust 端

#### 1. 新增数据结构 (`events.rs`)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelUsageResponse {
    pub code: u32,
    pub data: Option<ModelUsageData>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct TotalUsage {
    pub total_model_call_count: u64,
    pub total_tokens_usage: u64,
    pub model_summary_list: Vec<ModelSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSummary {
    pub model_name: String,
    pub total_tokens: u64,
    pub sort_order: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDataEntry {
    pub model_name: String,
    pub sort_order: u32,
    pub tokens_usage: Vec<u64>,
    pub total_tokens: u64,
}
```

#### 2. 新增 API 调用 (`polling.rs`)
```rust
pub async fn fetch_model_usage(
    app: &AppHandle,
    start_time: &str,
    end_time: &str,
) -> Result<ModelUsageData, String>
```

#### 3. 新增 Tauri Command (`commands.rs`)
```rust
#[tauri::command]
pub async fn get_model_usage(
    app: AppHandle,
    time_range: String,  // "today" | "7days" | "30days"
) -> Result<ModelUsageData, String>
```

根据 time_range 计算 startTime/endTime，调用 fetch_model_usage。

### Vue 端

#### 1. 新增 composable (`composables/useModelUsage.ts`)
- `modelUsageData`: 响应式数据
- `isLoading`: 加载状态
- `activeTab`: 当前选中的时间范围
- `fetchModelUsage(timeRange)`: 获取数据

#### 2. 新增组件 (`components/UsageHeatmap.vue`)
热力图组件，props:
- `data`: ModelUsageData
- `timeRange`: "today" | "7days" | "30days"

布局：
- 24列（0-23小时）x N行（天数）
- 今天：1行 x 24列
- 7天：7行 x 24列
- 30天：30行 x 24列（可纵向滚动）
- 每个格子 8x8px，间距 2px
- 左侧显示日期标签
- 顶部显示小时标签（0, 3, 6, 9, 12, 15, 18, 21）

色阶（基于 tokensUsage 相对值）：
- 0: #161b22（无用量，深色背景）
- 低: #0e4429（深绿）
- 中: #006d32（绿）
- 高: #26a641（亮绿）
- 极高: #39d353（最亮绿）

#### 3. 修改 PetWidget.vue / PopoverPanel.vue
- 在现有信息面板气泡底部添加"查看更多"按钮
- 点击后展开更大面板，显示：
  - 概览区：总 Token 用量、总调用次数
  - 模型明细：各模型消耗列表
  - Tab 栏：今天 / 7天 / 30天
  - 热力图区域

#### 4. 窗口尺寸调整
展开后的窗口尺寸：
- 今天：340 x 380px
- 7天：340 x 440px
- 30天：340 x 520px

## 文件变更清单

| 文件 | 变更类型 | 说明 |
|------|----------|------|
| `src-tauri/src/events.rs` | 修改 | 新增 ModelUsageResponse 等数据结构 |
| `src-tauri/src/polling.rs` | 修改 | 新增 fetch_model_usage 函数 |
| `src-tauri/src/commands.rs` | 修改 | 新增 get_model_usage command |
| `src-tauri/src/main.rs` | 修改 | 注册新 command |
| `src/composables/useModelUsage.ts` | 新增 | 模型用量数据 composable |
| `src/components/UsageHeatmap.vue` | 新增 | 热力图组件 |
| `src/components/PetWidget.vue` | 修改 | 添加"查看更多"按钮 + 展开面板 |
| `src/components/PopoverPanel.vue` | 修改 | 同上（macOS 端） |

## 交互流程

```
用户点击宠物 → 显示信息面板气泡（现有）
                → 气泡底部显示"查看更多"按钮
用户点击"查看更多" → 面板扩展为大面板
                    → 调用 get_model_usage("today")
                    → 显示概览 + 模型明细 + 热力图
用户切换 Tab → 调用 get_model_usage(新范围)
             → 更新热力图和概览数据
用户点击"收起" → 回到小面板状态
```
