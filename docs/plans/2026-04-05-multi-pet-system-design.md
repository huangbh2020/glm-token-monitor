# 多宠物系统设计文档

**日期:** 2026-04-05
**状态:** 已批准

---

## 概述

将当前单一猫咪宠物重构为多宠物系统，支持：
- 3-5 种不同宠物形象
- 每种宠物 7+ 种动画效果
- 动画定时随机切换（1-3 分钟可配置）
- 用户在设置页面选择宠物
- 动画独立于用量状态（用量仅影响颜色/光晕）

---

## 架构设计

### 实现方案

采用**配置文件驱动**方案：
- 宠物数据存储在独立的 JSON 配置文件
- 创建 `usePetManager` composable 管理宠物选择和动画切换
- 使用 Tauri 的 `store` 插件持久化用户选择

### 文件结构

```
src/
├── assets/
│   └── pets/
│       ├── index.ts           # 宠物配置导出
│       ├── cat.json           # 猫咪宠物配置
│       ├── dog.json           # 狗狗宠物配置
│       ├── rabbit.json        # 兔子宠物配置
│       └── ...
├── composables/
│   ├── usePetManager.ts       # 宠物管理（新增）
│   └── useUsageState.ts       # 现有，保持不变
└── components/
    ├── PetWidget.vue          # 重构：使用宠物配置渲染
    └── SettingsPanel.vue      # 添加宠物选择界面
```

### 数据流

```
用户启动应用
    ↓
Tauri Store 读取用户选择 (petId, animationInterval)
    ↓
usePetManager 加载宠物配置
    ↓
PetWidget 根据配置渲染 SVG
    ↓
定时器触发 → 随机切换动画 → SVG 重新渲染
    ↓
用户在设置页面选择新宠物
    ↓
保存到 Tauri Store → 重新加载配置
```

---

## 数据模型

### 宠物配置格式

```typescript
interface Pet {
  id: string              // 宠物唯一标识
  name: string            // 宠物名称
  description: string     // 宠物描述
  preview: string         // 预览图 URL（可选）
  animations: Animation[] // 动画列表
}

interface Animation {
  id: string        // 动画唯一标识
  name: string      // 动画名称
  svg: string       // 完整的 SVG + 内联 CSS
  duration: number  // 动画周期（毫秒）
}
```

### 配置示例

```json
{
  "id": "cat",
  "name": "猫咪",
  "description": "可爱的像素猫咪",
  "animations": [
    {
      "id": "sleeping",
      "name": "睡觉",
      "svg": "<svg class=\"pet-svg\" viewBox=\"0 0 80 80\">...</svg><style>.cat-breathe { animation: breathe 2.8s infinite; }</style>",
      "duration": 2800
    }
  ]
}
```

### Tauri Store 存储格式

```json
{
  "petId": "cat",
  "animationInterval": 120
}
```

---

## 组件设计

### PetWidget.vue 重构

**职责：**
- 从 `usePetManager` 获取当前宠物和动画
- 渲染宠物 SVG（动态注入配置中的 SVG 内容）
- 应用用量状态相关的颜色/光晕样式
- 处理拖拽和点击交互

**关键代码：**
```vue
<script setup>
import { usePetManager } from '@/composables/usePetManager'

const { currentPet, currentAnimation, switchAnimation } = usePetManager()

// 定时切换动画
onMounted(() => {
  const timer = setInterval(switchAnimation, animationInterval.value * 1000)
  onUnmounted(() => clearInterval(timer))
})
</script>

<template>
  <div class="pet-widget" :class="[`pet-${petState.toLowerCase()}`]">
    <!-- 动态渲染 SVG -->
    <div v-html="currentAnimation?.svg" class="pet-svg"></div>
  </div>
</template>
```

### SettingsPanel.vue 扩展

**新增功能：**
- 宠物选择网格（显示所有可用宠物的预览）
- 动画间隔滑块（60-180 秒可调）
- 宠物描述和名称展示

---

## 状态管理

### usePetManager Composable

**职责：**
- 管理当前选中的宠物和动画
- 提供动画切换逻辑
- 处理用户设置的持久化

**接口：**
```typescript
interface PetManagerState {
  selectedPetId: string      // 当前宠物ID
  animationInterval: number  // 动画切换间隔（秒）
  availablePets: Pet[]       // 所有可用宠物
  currentAnimation: Animation // 当前播放的动画
}

export function usePetManager(): {
  // 状态
  selectedPetId: Ref<string>
  animationInterval: Ref<number>
  availablePets: Ref<Pet[]>
  currentAnimation: Ref<Animation | null>

  // 方法
  loadPets: () => Promise<void>
  selectPet: (id: string) => void
  switchAnimation: () => void
  saveSettings: () => Promise<void>
  loadSettings: () => Promise<void>
}
```

### 动画切换逻辑

```typescript
function switchAnimation() {
  const pet = availablePets.value.find(p => p.id === selectedPetId.value)
  if (!pet || pet.animations.length === 0) return

  // 随机选择一个与当前不同的动画
  let nextAnimation
  do {
    nextAnimation = pet.animations[
      Math.floor(Math.random() * pet.animations.length)
    ]
  } while (nextAnimation.id === currentAnimation.value?.id && pet.animations.length > 1)

  currentAnimation.value = nextAnimation
}
```

---

## 持久化

### Tauri Store 配置

**依赖添加：**
```toml
# src-tauri/Cargo.toml
[dependencies]
tauri-plugin-store = "2"
```

**插件注册：**
```rust
// src-tauri/src/main.rs
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**使用方式：**
```typescript
import { Store } from 'tauri-plugin-store'

const store = new Store('settings.json')

// 保存
await store.set('petId', 'cat')
await store.save()

// 读取
const petId = await store.get<string>('petId')
```

---

## 动画系统

### SVG 动画处理

**方案：内联 CSS 动画**
- 动画效果通过 `<style>` 标签内联在 SVG 中
- 每个动画配置包含完整的 SVG + CSS
- 优点：自包含，无需外部样式文件

### 动画过渡效果

```css
.pet-svg {
  transition: opacity 0.3s ease;
}

.pet-svg.fading-out {
  opacity: 0;
}
```

---

## 测试策略

### 单元测试

```typescript
// composables/usePetManager.spec.ts
describe('usePetManager', () => {
  it('should load pets from config', async () => {
    const { loadPets, availablePets } = usePetManager()
    await loadPets()
    expect(availablePets.value.length).toBeGreaterThan(0)
  })

  it('should switch to different animation', () => {
    const { switchAnimation, currentAnimation } = usePetManager()
    const beforeId = currentAnimation.value.id
    switchAnimation()
    expect(currentAnimation.value.id).not.toBe(beforeId)
  })

  it('should save settings to store', async () => {
    const { selectPet, saveSettings } = usePetManager()
    selectPet('dog')
    await saveSettings()
    // 验证 Tauri Store 调用
  })
})
```

### 集成测试

- 验证宠物切换后 SVG 正确渲染
- 验证动画定时器正常工作
- 验证设置页面选择后持久化成功

### 手动测试清单

- [ ] 启动应用，默认宠物正确显示
- [ ] 等待动画切换，验证动画变化
- [ ] 打开设置，选择不同宠物，验证切换
- [ ] 调整动画间隔，验证定时器更新
- [ ] 重启应用，验证选择被记住

---

## 初始宠物计划

### 第一批宠物（3-5 种）

1. **猫咪** - 现有，迁移到新系统
   - 睡觉、打字、暴躁、惊恐、幽灵等 7+ 动画

2. **狗狗** - 新增
   - 摇尾巴、接飞盘、看门、睡觉等 7+ 动画

3. **兔子** - 新增
   - 吃胡萝卜、跳跃、抖耳朵、发呆等 7+ 动画

4. **机器人** - 新增（可选）
   - 充电、计算、移动、故障等 7+ 动画

5. **史莱姆** - 新增（可选）
   - 跳动、变形、流动、分裂等 7+ 动画

---

## 实现注意事项

1. **向后兼容**：现有猫咪动画需要迁移到新配置格式
2. **默认值**：未设置时默认使用猫咪，动画间隔 120 秒
3. **错误处理**：配置加载失败时回退到默认宠物
4. **性能**：SVG 使用 `v-html` 渲染，注意 XSS 风险（配置文件可信）
5. **动画平滑**：切换时添加淡入淡出过渡效果

---

## 后续扩展可能性

- 支持用户自定义宠物（插件系统）
- 动画市场/分享功能
- 宠物配件系统
- 宠物互动/进化机制
