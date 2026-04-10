# 宠物动作系统设计

**日期**: 2026-04-06
**作者**: Claude
**状态**: 已批准

## 概述

创建独立的宠物动作系统，与用量状态完全解耦。用户可在设置面板选择宠物（猫/狗），每个宠物有 3-4 种随机动作，每 20-30 秒切换一次。

## 设计目标

1. **解耦动作与状态**: 宠物动作不再根据用量变化，而是独立随机轮播
2. **多宠物支持**: 支持猫咪和狗狗两种形象，用户可切换
3. **动作多样性**: 每个宠物有 3-4 种不同动作
4. **平滑切换**: 动作切换间隔 20-30 秒，使用慢速节奏

## 架构设计

### 系统关系图

```
现有系统（保留）                新增系统
┌─────────────────┐          ┌──────────────────┐
│ Usage State     │          │ Pet Action State │
│ (Fresh~Dead)    │    ←→    │ (独立动作)       │
└─────────────────┘          └──────────────────┘
         ↓                              ↓
    显示模式颜色                     宠物动画
```

**核心改动**：
- 宠物动作与用量状态完全解耦
- 用量状态仅影响显示模式的颜色
- 宠物动作独立随机轮播

## 动作定义

### 猫咪 (Cat) - 4种动作

| 动作ID | 名称 | 描述 |
|--------|------|------|
| `cat-sleep` | 睡觉 | 蜷缩身体，ZZZ飘浮 |
| `cat-play` | 玩耍 | 追逐毛线球/激光点 |
| `cat-stare` | 发呆 | 端坐，眼睛左右看 |
| `cat-stretch` | 伸懒腰 | 前爪伸展，身体拉长 |

### 狗狗 (Dog) - 4种动作

| 动作ID | 名称 | 描述 |
|--------|------|------|
| `dog-sit` | 坐立 | 端坐，尾巴摇摆 |
| `dog-bark` | 吠叫 | 张嘴，音符飘出 |
| `dog-walk` | 踱步 | 左右走动 |
| `dog-beg` | 讨食 | 前爪抬起，期待表情 |

## 数据结构

### 类型定义

```typescript
// 宠物类型
export type PetType = 'cat' | 'dog'

// 动作类型
export type CatAction = 'cat-sleep' | 'cat-play' | 'cat-stare' | 'cat-stretch'
export type DogAction = 'dog-sit' | 'dog-bark' | 'dog-walk' | 'dog-beg'

// 宠物动作状态
export interface PetActionState {
  petType: PetType      // 当前宠物
  currentAction: string // 当前动作
}

// 配置扩展
interface PetConfig {
  selected_pet: PetType    // 用户选择的宠物
  action_interval: number  // 动作切换间隔（秒）
}
```

## 组件结构

```
src/
├── components/
│   ├── PetWidget.vue          (修改：支持动作切换)
│   ├── SettingsPanel.vue      (修改：新增宠物选择标签)
│   └── pets/                  (新增目录)
│       ├── CatSleep.vue       (猫-睡觉)
│       ├── CatPlay.vue        (猫-玩耍)
│       ├── CatStare.vue       (猫-发呆)
│       ├── CatStretch.vue     (猫-伸懒腰)
│       ├── DogSit.vue         (狗-坐立)
│       ├── DogBark.vue        (狗-吠叫)
│       ├── DogWalk.vue        (狗-踱步)
│       └── DogBeg.vue         (狗-讨食)
├── composables/
│   ├── usePetAction.ts        (新增：动作管理)
│   └── useSettings.ts         (修改：新增宠物配置)
└── types/
    └── config.ts              (修改：新增 PetConfig)
```

## 动作切换逻辑

```typescript
// usePetAction.ts
const ACTION_INTERVAL = 25000 // 25秒

function usePetAction() {
  const currentAction = ref<string>('cat-sleep')
  const petType = ref<PetType>('cat')

  // 随机切换动作
  function switchAction() {
    const actions = getActionsForPet(petType.value)
    const nextAction = actions[Math.floor(Math.random() * actions.length)]
    currentAction.value = nextAction
  }

  // 定时切换
  onMounted(() => {
    setInterval(switchAction, ACTION_INTERVAL)
  })

  return { currentAction, petType, switchAction }
}
```

## 设置面板设计

### 新增【宠物】标签

```
┌─────────────────────────────────┐
│ [模型配置] [轮询设置] [宠物] [展示] │
├─────────────────────────────────┤
│  当前宠物                        │
│  ○ 像素猫咪  ● 像素狗狗          │
│                                  │
│  动作切换间隔                     │
│  25 秒                           │
│                                  │
│  预览                            │
│  [猫咪预览动画]                   │
└─────────────────────────────────┘
```

## Rust 后端配置持久化

### 配置文件结构

```json
{
  "api_config": { ... },
  "polling_config": { ... },
  "display_config": { ... },
  "pet_config": {
    "selected_pet": "cat",
    "action_interval": 25
  }
}
```

### Rust 结构体

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetConfig {
    pub selected_pet: String,  // "cat" or "dog"
    pub action_interval: u64,  // seconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub api_config: ApiConfig,
    pub polling_config: PollingConfig,
    pub display_config: DisplayConfig,
    pub pet_config: PetConfig,  // 新增
}
```

## 显示模式颜色保留

用量状态仍用于控制显示模式的颜色：

| 状态 | 百分比 | 颜色 |
|------|--------|------|
| Fresh | 0-30% | #10B981 (绿) |
| Flow | 31-60% | #3B82F6 (蓝) |
| Warning | 61-80% | #F59E0B (黄) |
| Panic | 81-95% | #F97316 (橙) |
| Dead | 96-100% | #6B7280 (灰) |

但宠物动作独立于此状态，随机切换。

## 实施要点

1. **向后兼容**: 默认使用猫咪，动作切换间隔 25 秒
2. **平滑过渡**: 切换宠物时动作立即更新
3. **配置同步**: 设置面板修改立即生效并持久化
4. **动画复用**: 尽量复用现有 CSS 动画

## 未来扩展

- 支持更多宠物类型（兔子、鸟等）
- 支持自定义动作间隔
- 支持动作收藏（只播放喜欢的动作）
- 支持互动动作（点击宠物触发特定动作）
