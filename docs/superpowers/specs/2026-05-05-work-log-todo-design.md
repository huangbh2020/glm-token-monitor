# 工作日志 Todo 功能设计

## Context

用户每天都有很多琐碎的工作事项需要记录，且需要应付日会和周会汇报。现有 Todo 软件过于复杂，需要一个**极简的工作日志工具**：快速记录做了什么、按天整理流水记录、开会时能看能复制。集成在现有宠物监控应用中，纯本地存储。

## 方案选型

采用**每日清单模式（方案 A）**：每天的记录是一个扁平清单，每条有两个状态——待办（未勾选）和已完成（勾选）。一个列表同时解决事前规划和事后记录。

## 1. UI 入口与窗口布局

### 1.1 快速录入浮窗（QuickCapture）
- **触发方式**：全局快捷键（默认 `Cmd+Shift+T` / `Ctrl+Shift+T`）或宠物右键菜单
- **形态**：类似 Spotlight 的小浮窗，屏幕居中，毛玻璃效果，只有一个输入框
- **交互**：输入文字 → 回车提交 → 自动关闭 → 宠物身上闪一个小动画确认
- **默认状态**：新条目默认标记为"已完成"（事后记录场景为主）；输入 `~` 前缀则创建为待办
- **Escape** 关闭浮窗

### 1.2 Todo 面板窗口（todo-panel）
- **触发方式**：双击宠物 / 托盘菜单"工作日志"
- **形态**：新窗口，300x450 左右，毛玻璃效果，与 info-panel 风格一致
- **布局**：
  - 顶部：日期切换器（← 今天 →）
  - 中间：当天清单列表，每项可勾选/取消/删除，底部内嵌输入框
  - 底部：快捷操作栏（复制今日记录、复制本周记录）

### 1.3 宠物上的 Todo 指示
- 宠物旁边显示小圆形气泡，显示当前未完成待办数量
- 无未完成项时自动隐藏

## 2. 数据模型与存储

### 2.1 存储结构
每天一个 JSON 文件，按日期命名：

```
<storage_path>/
  ├── 2025-05-05.json
  ├── 2025-05-04.json
  └── ...
```

默认存储路径：`~/.config/plan-guard/todo/`，用户可在设置面板自定义。

### 2.2 单日数据格式

```json
{
  "date": "2025-05-05",
  "items": [
    {
      "id": "a1b2c3",
      "text": "修复了登录页的 Bug",
      "done": true,
      "done_at": 1746424320,
      "created_at": 1746424320
    },
    {
      "id": "d4e5f6",
      "text": "继续开发 B 功能",
      "done": false,
      "done_at": null,
      "created_at": 1746430000
    }
  ]
}
```

- `id`：6 位随机字符串
- `done`：是否已完成
- `done_at`：完成时间戳（Unix 秒），未完成为 null
- `created_at`：创建时间戳

### 2.3 排序规则
未完成在前（按 created_at 升序），已完成在后（按 done_at 倒序）。

### 2.4 自动清理
保留最近 90 天数据，更早的自动归档（可在设置中配置保留天数）。

### 2.5 复制格式

**复制今日记录**：
```
2025-05-05
✅ 修复了登录页的 Bug
✅ 和产品对齐了需求 A
☐ 继续开发 B 功能
✅ 回复了客户反馈邮件
☐ 准备周会 PPT
```

**复制本周记录**：按天分组输出所有已完成项。

## 3. Rust 后端

### 3.1 新增模块
`src-tauri/src/todo.rs`：负责读写每日 JSON 文件、条目增删改查、格式化文本生成、自动清理。

### 3.2 Tauri Commands

| Command | 说明 |
|---------|------|
| `todo_add_item(text, is_done)` | 添加条目，is_done 默认 true |
| `todo_toggle_item(id, date)` | 切换完成状态 |
| `todo_delete_item(id, date)` | 删除条目 |
| `todo_get_day(date)` | 获取指定日期的列表，默认今天 |
| `todo_get_week()` | 获取本周数据 |
| `todo_copy_day(date)` | 返回当日格式化文本 |
| `todo_copy_week()` | 返回本周格式化文本 |

### 3.3 配置扩展
在现有 `config.json` 中新增：

```json
{
  "todo_config": {
    "storage_path": null,
    "retention_days": 90
  }
}
```

`storage_path` 为 null 时使用默认路径。

### 3.4 全局快捷键
通过 Tauri 全局快捷键 API 注册 `Cmd+Shift+T`（macOS）/ `Ctrl+Shift+T`（Windows），触发时 emit `show-quick-capture` 事件给前端。

## 4. 前端组件

### 4.1 新增文件

```
src/components/
  ├── QuickCapture.vue            # 快速录入浮窗
  ├── TodoPanel.vue               # Todo 面板窗口
  └── todo/
      ├── TodoList.vue            # 清单列表（含勾选、删除、输入框）
      ├── TodoItem.vue            # 单条条目
      ├── DaySwitcher.vue         # 顶部日期切换器
      └── TodoBadge.vue           # 宠物上的未完成数气泡

src/composables/
  └── useTodo.ts                  # Todo 状态与 Tauri Command 封装
```

### 4.2 各组件职责

**QuickCapture.vue**：居中浮窗，一个输入框。监听 `show-quick-capture` 事件显隐。回车提交后清空并关闭，Escape 关闭。

**TodoPanel.vue**：新窗口根组件。布局：DaySwitcher → TodoList → 底部操作栏（复制今日 / 复制本周）。

**TodoList.vue**：渲染 TodoItem 列表 + 底部内嵌输入框。新条目默认已完成。

**TodoItem.vue**：左侧复选框（圆形、状态色）+ 中间文本 + 右侧完成时间（`HH:mm`）。悬停显示删除按钮。

**TodoBadge.vue**：小圆形气泡显示未完成数，无未完成项时隐藏。集成在 PetWidget.vue 中。

**useTodo.ts**：封装所有 todo Tauri Commands 调用，提供响应式数据（`todayItems`、`pendingCount`）和方法（`addItem`、`toggleItem`、`deleteItem`、`copyDay`、`copyWeek`）。

## 5. 关键修改点

### 现有文件修改
- `src-tauri/src/main.rs`：注册 todo 模块和全局快捷键
- `src-tauri/src/config.rs`：扩展 Config 结构体，增加 `todo_config`
- `src-tauri/src/tray.rs`：托盘菜单增加"工作日志"选项
- `src-tauri/src/windows.rs`：新增 todo-panel 窗口管理
- `src/components/PetWidget.vue`：集成 TodoBadge、双击打开 TodoPanel、QuickCapture
- `src/components/PopoverPanel.vue`：macOS 端集成 TodoBadge 和入口
- `src-tauri/tauri.conf.json`：注册 todo-panel 窗口
- 设置面板：新增 Todo 存储路径配置项

## 6. 验证方式

1. 快捷键唤起 QuickCapture → 输入文字 → 回车 → 确认条目已添加到今日列表
2. 点击宠物托盘菜单 → 打开 Todo 面板 → 查看今日列表 → 勾选/取消/删除条目
3. 点击"复制今日记录" → 粘贴到文本编辑器 → 确认格式正确
4. 点击"复制本周记录" → 确认按天分组、只含已完成项
5. 添加待办条目（`~` 前缀）→ 确认气泡显示未完成数 → 完成后气泡更新
6. 设置面板修改存储路径 → 确认数据迁移到新路径
7. 跨天验证：添加昨天和明天的条目 → 通过日期切换器查看
