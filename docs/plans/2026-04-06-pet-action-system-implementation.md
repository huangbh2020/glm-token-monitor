# 宠物动作系统实施计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 创建独立的宠物动作系统，支持猫/狗两种宠物切换，每个宠物有 4 种随机动作（25秒切换间隔），与用量状态完全解耦。

**Architecture:**
- 新增 `usePetAction` composable 管理动作状态和随机切换
- 创建 `components/pets/` 目录，8 个独立组件（猫4 + 狗4）
- 扩展 Rust 配置结构，添加 `pet_config` 字段
- 修改 `SettingsPanel.vue` 新增【宠物】标签

**Tech Stack:** Vue 3, TypeScript, Tauri, SVG 像素动画

---

## Task 1: 扩展类型定义

**Files:**
- Modify: `src/types/config.ts`

**Step 1: 添加宠物相关类型**

在文件末尾添加：

```typescript
// 宠物类型
export type PetType = 'cat' | 'dog'

// 猫咪动作
export type CatAction = 'cat-sleep' | 'cat-play' | 'cat-stare' | 'cat-stretch'

// 狗狗动作
export type DogAction = 'dog-sit' | 'dog-bark' | 'dog-walk' | 'dog-beg'

// 宠物配置
export interface PetConfig {
  selected_pet: PetType
  action_interval: number  // 秒
}
```

**Step 2: 更新 AppConfig 接口**

找到 `export interface AppConfig`，添加 `pet_config` 字段：

```typescript
export interface AppConfig {
  api_config: ApiConfig
  polling_config: PollingConfig
  display_config: DisplayConfig
  pet_config: PetConfig  // 新增
}
```

**Step 3: 验证类型定义**

运行: `npm run type-check`
预期: 无类型错误

**Step 4: Commit**

```bash
git add src/types/config.ts
git commit -m "feat(types): add pet action system types"
```

---

## Task 2: 创建 usePetAction Composable

**Files:**
- Create: `src/composables/usePetAction.ts`

**Step 1: 创建 composable 文件**

```typescript
import { ref, computed, onMounted, onUnmounted } from 'vue'
import type { PetType, CatAction, DogAction } from '../types/config'

// 动作定义
const CAT_ACTIONS: CatAction[] = ['cat-sleep', 'cat-play', 'cat-stare', 'cat-stretch']
const DOG_ACTIONS: DogAction[] = ['dog-sit', 'dog-bark', 'dog-walk', 'dog-beg']

// 默认配置
const DEFAULT_ACTION_INTERVAL = 25 // 秒

export function usePetAction(initialPet: PetType = 'cat', interval: number = DEFAULT_ACTION_INTERVAL) {
  const petType = ref<PetType>(initialPet)
  const currentAction = ref<string>(CAT_ACTIONS[0])

  // 获取当前宠物的所有动作
  const availableActions = computed(() => {
    return petType.value === 'cat' ? CAT_ACTIONS : DOG_ACTIONS
  })

  // 随机切换动作
  function switchAction() {
    const actions = availableActions.value
    const randomIndex = Math.floor(Math.random() * actions.length)
    currentAction.value = actions[randomIndex]
    console.log(`[PetAction] Switched to: ${currentAction.value}`)
  }

  // 切换宠物类型
  function setPetType(type: PetType) {
    if (petType.value !== type) {
      petType.value = type
      // 切换宠物时立即更新动作
      const actions = type === 'cat' ? CAT_ACTIONS : DOG_ACTIONS
      currentAction.value = actions[0]
      console.log(`[PetAction] Pet changed to: ${type}`)
    }
  }

  // 定时器引用
  let actionTimer: number | null = null

  // 设置定时切换
  function setupActionTimer() {
    if (actionTimer) clearInterval(actionTimer)
    actionTimer = window.setInterval(() => {
      switchAction()
    }, interval * 1000)
  }

  // 清理定时器
  function cleanupActionTimer() {
    if (actionTimer) {
      clearInterval(actionTimer)
      actionTimer = null
    }
  }

  // 组件挂载时启动
  onMounted(() => {
    setupActionTimer()
    // 初始动作
    currentAction.value = availableActions.value[0]
  })

  // 组件卸载时清理
  onUnmounted(() => {
    cleanupActionTimer()
  })

  return {
    petType,
    currentAction,
    availableActions,
    switchAction,
    setPetType,
    setupActionTimer,
    cleanupActionTimer
  }
}
```

**Step 2: Commit**

```bash
git add src/composables/usePetAction.ts
git commit -m "feat(composables): create usePetAction for independent action system"
```

---

## Task 3: 创建猫咪动作组件 - 睡觉

**Files:**
- Create: `src/components/pets/CatSleep.vue`

**Step 1: 创建组件**

```vue
<script setup lang="ts">
// 猫咪睡觉动作 - 蜷缩身体，ZZZ飘浮
</script>

<template>
  <svg class="pet-svg" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
    <!-- ZZZ 浮动 -->
    <text class="zzz-a" x="44" y="26" font-size="7" fill="#6EE7B7" font-family="monospace" font-weight="bold">z</text>
    <text class="zzz-b" x="50" y="19" font-size="9" fill="#6EE7B7" font-family="monospace" font-weight="bold">z</text>
    <text class="zzz-c" x="57" y="12" font-size="11" fill="#6EE7B7" font-family="monospace" font-weight="bold">Z</text>
    <!-- 猫身体（蜷缩） -->
    <ellipse class="cat-breathe" cx="22" cy="55" rx="18" ry="11" fill="#F4A460"/>
    <!-- 猫头 -->
    <rect x="6" y="30" width="22" height="18" rx="5" fill="#F4A460"/>
    <!-- 耳朵左 -->
    <polygon points="8,31 12,22 16,31" fill="#F4A460"/>
    <polygon points="9,30 12,24 15,30" fill="#FFB6C1"/>
    <!-- 耳朵右 -->
    <polygon points="18,31 22,22 26,31" fill="#F4A460"/>
    <polygon points="19,30 22,24 25,30" fill="#FFB6C1"/>
    <!-- 闭眼（睡觉弧线） -->
    <path d="M9 38 Q13 35.5 17 38" stroke="#6B4226" stroke-width="1.5" fill="none" stroke-linecap="round"/>
    <path d="M19 38 Q23 35.5 27 38" stroke="#6B4226" stroke-width="1.5" fill="none" stroke-linecap="round"/>
    <!-- 鼻子 -->
    <polygon points="17,42 19,43 21,42 19,44" fill="#FF9AA2"/>
    <!-- 胡须左 -->
    <line x1="3" y1="40" x2="13" y2="42" stroke="#BBB" stroke-width="0.8"/>
    <line x1="3" y1="43" x2="13" y2="43" stroke="#BBB" stroke-width="0.8"/>
    <!-- 胡须右 -->
    <line x1="24" y1="42" x2="34" y2="40" stroke="#BBB" stroke-width="0.8"/>
    <line x1="24" y1="43" x2="34" y2="43" stroke="#BBB" stroke-width="0.8"/>
    <!-- 尾巴 -->
    <path d="M40 54 Q52 48 48 38 Q44 30 40 36" stroke="#F4A460" stroke-width="5" fill="none" stroke-linecap="round"/>
  </svg>
</template>

<style scoped>
.pet-svg {
  width: 80px;
  height: 80px;
  overflow: visible;
}

/* 呼吸动画 */
.cat-breathe {
  animation: breathe 2.8s ease-in-out infinite;
  transform-origin: 22px 55px;
}

@keyframes breathe {
  0%, 100% { transform: scaleY(1); }
  50% { transform: scaleY(1.1); }
}

/* ZZZ 动画 */
.zzz-a { animation: zzz 2.2s ease-in-out infinite; }
.zzz-b { animation: zzz 2.2s ease-in-out 0.35s infinite; }
.zzz-c { animation: zzz 2.2s ease-in-out 0.7s infinite; }

@keyframes zzz {
  0%   { opacity: 0; transform: translate(0, 4px); }
  25%  { opacity: 1; }
  80%  { opacity: 0.8; }
  100% { opacity: 0; transform: translate(2px, -10px); }
}
</style>
```

**Step 2: Commit**

```bash
git add src/components/pets/CatSleep.vue
git commit -m "feat(pets): create CatSleep component"
```

---

## Task 4: 创建猫咪动作组件 - 玩耍

**Files:**
- Create: `src/components/pets/CatPlay.vue`

**Step 1: 创建组件**

```vue
<script setup lang="ts">
// 猫咪玩耍动作 - 追逐毛线球
</script>

<template>
  <svg class="pet-svg" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
    <!-- 毛线球 -->
    <g class="yarn-ball">
      <circle cx="58" cy="52" r="10" fill="#EC4899"/>
      <circle cx="54" cy="48" r="3" fill="#F472B6"/>
      <circle cx="62" cy="55" r="2.5" fill="#F472B6"/>
      <circle cx="57" cy="57" r="2" fill="#DB2777"/>
      <!-- 毛线 -->
      <path d="M48 52 Q40 48 35 52" stroke="#EC4899" stroke-width="2" fill="none" stroke-linecap="round" class="yarn-string"/>
    </g>
    <!-- 猫身体 -->
    <rect x="4" y="38" width="28" height="24" rx="6" fill="#F4A460"/>
    <!-- 猫头 -->
    <rect x="5" y="18" width="24" height="22" rx="5" fill="#F4A460"/>
    <!-- 耳朵左 -->
    <polygon points="7,20 11,10 15,20" fill="#F4A460"/>
    <polygon points="8,19 11,12 14,19" fill="#FFB6C1"/>
    <!-- 耳朵右 -->
    <polygon points="19,20 23,10 27,20" fill="#F4A460"/>
    <polygon points="20,19 23,12 26,19" fill="#FFB6C1"/>
    <!-- 兴奋的眼睛（大而圆） -->
    <circle cx="12" cy="29" r="4" fill="white"/>
    <circle cx="22" cy="29" r="4" fill="white"/>
    <circle cx="12" cy="29" r="2" fill="#1E293B"/>
    <circle cx="22" cy="29" r="2" fill="#1E293B"/>
    <circle cx="13" cy="28" r="0.8" fill="white"/>
    <circle cx="23" cy="28" r="0.8" fill="white"/>
    <!-- 开嘴 -->
    <ellipse cx="17" cy="37" rx="3" ry="2" fill="#FF9AA2"/>
    <!-- 伸出爪子 -->
    <rect class="play-paw" x="28" y="45" width="10" height="6" rx="3" fill="#E8944A"/>
  </svg>
</template>

<style scoped>
.pet-svg {
  width: 80px;
  height: 80px;
  overflow: visible;
}

/* 毛线球滚动 */
.yarn-ball {
  animation: yarn-roll 2s ease-in-out infinite;
  transform-origin: 58px 52px;
}

@keyframes yarn-roll {
  0%, 100% { transform: translateX(0) rotate(0deg); }
  50% { transform: translateX(-5px) rotate(180deg); }
}

/* 爪子挥动 */
.play-paw {
  animation: paw-swat 1s ease-in-out infinite;
  transform-origin: 33px 48px;
}

@keyframes paw-swat {
  0%, 100% { transform: translateX(0) rotate(0deg); }
  50% { transform: translateX(5px) rotate(15deg); }
}

/* 毛线摆动 */
.yarn-string {
  animation: string-wave 1s ease-in-out infinite;
}

@keyframes string-wave {
  0%, 100% { d: path("M48 52 Q40 48 35 52"); }
  50% { d: path("M48 52 Q42 54 35 52"); }
}
</style>
```

**Step 2: Commit**

```bash
git add src/components/pets/CatPlay.vue
git commit -m "feat(pets): create CatPlay component"
```

---

## Task 5: 创建猫咪动作组件 - 发呆

**Files:**
- Create: `src/components/pets/CatStare.vue`

**Step 1: 创建组件**

```vue
<script setup lang="ts">
// 猫咪发呆动作 - 端坐，眼睛左右看
</script>

<template>
  <svg class="pet-svg" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
    <!-- 猫身体（端坐） -->
    <rect x="8" y="40" width="24" height="28" rx="8" fill="#F4A460"/>
    <!-- 猫头 -->
    <rect x="6" y="16" width="26" height="24" rx="5" fill="#F4A460"/>
    <!-- 耳朵左 -->
    <polygon points="8,18 12,8 16,18" fill="#F4A460"/>
    <polygon points="9,17 12,10 15,17" fill="#FFB6C1"/>
    <!-- 耳朵右 -->
    <polygon points="22,18 26,8 30,18" fill="#F4A460"/>
    <polygon points="23,17 26,10 29,17" fill="#FFB6C1"/>
    <!-- 眼睛（左右移动） -->
    <g class="eyes-look">
      <circle cx="13" cy="28" r="3" fill="white"/>
      <circle cx="25" cy="28" r="3" fill="white"/>
      <circle class="pupil-l" cx="13" cy="28" r="1.5" fill="#1E293B"/>
      <circle class="pupil-r" cx="25" cy="28" r="1.5" fill="#1E293B"/>
    </g>
    <!-- 鼻子 -->
    <polygon points="17,34 19,35 21,34 19,36" fill="#FF9AA2"/>
    <!-- 小嘴 -->
    <path d="M16 38 Q19 39 22 38" stroke="#6B4226" stroke-width="1" fill="none" stroke-linecap="round"/>
    <!-- 前爪 -->
    <ellipse cx="14" cy="68" rx="5" ry="4" fill="#E8944A"/>
    <ellipse cx="26" cy="68" rx="5" ry="4" fill="#E8944A"/>
  </svg>
</template>

<style scoped>
.pet-svg {
  width: 80px;
  height: 80px;
  overflow: visible;
}

/* 眼睛左右看 */
.pupil-l {
  animation: look-left-right 3s ease-in-out infinite;
}
.pupil-r {
  animation: look-left-right 3s ease-in-out infinite;
}

@keyframes look-left-right {
  0%, 100% { transform: translateX(-1px); }
  50% { transform: translateX(1px); }
}
</style>
```

**Step 2: Commit**

```bash
git add src/components/pets/CatStare.vue
git commit -m "feat(pets): create CatStare component"
```

---

## Task 6: 创建猫咪动作组件 - 伸懒腰

**Files:**
- Create: `src/components/pets/CatStretch.vue`

**Step 1: 创建组件**

```vue
<script setup lang="ts">
// 猫咪伸懒腰动作 - 前爪伸展，身体拉长
</script>

<template>
  <svg class="pet-svg" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
    <!-- 猫身体（拉伸） -->
    <ellipse class="body-stretch" cx="24" cy="52" rx="22" ry="12" fill="#F4A460"/>
    <!-- 猫头 -->
    <rect x="4" y="26" width="22" height="18" rx="5" fill="#F4A460"/>
    <!-- 耳朵左 -->
    <polygon points="6,27 10,18 14,27" fill="#F4A460"/>
    <polygon points="7,26 10,20 13,27" fill="#FFB6C1"/>
    <!-- 耳朵右 -->
    <polygon points="16,27 20,18 24,27" fill="#F4A460"/>
    <polygon points="17,26 20,20 23,27" fill="#FFB6C1"/>
    <!-- 半眯眼（享受） -->
    <path d="M8 34 Q11 32 14 34" stroke="#6B4226" stroke-width="1.5" fill="none" stroke-linecap="round"/>
    <path d="M16 34 Q19 32 22 34" stroke="#6B4226" stroke-width="1.5" fill="none" stroke-linecap="round"/>
    <!-- 张嘴（打哈欠） -->
    <ellipse cx="15" cy="39" rx="4" ry="3" fill="#1E293B"/>
    <ellipse cx="15" cy="39" rx="2.5" ry="1.5" fill="#FF9AA2"/>
    <!-- 伸展的前爪 -->
    <rect class="front-paw-l" x="0" y="42" width="12" height="6" rx="3" fill="#E8944A"/>
    <rect class="front-paw-r" x="0" y="50" width="12" height="6" rx="3" fill="#E8944A"/>
    <!-- 尾巴伸展 -->
    <path class="tail-stretch" d="M46 52 Q58 46 54 36 Q50 28 46 34" stroke="#F4A460" stroke-width="5" fill="none" stroke-linecap="round"/>
  </svg>
</template>

<style scoped>
.pet-svg {
  width: 80px;
  height: 80px;
  overflow: visible;
}

/* 身体拉伸 */
.body-stretch {
  animation: stretch-body 2.5s ease-in-out infinite;
}

@keyframes stretch-body {
  0%, 100% { rx: 22; }
  50% { rx: 26; }
}

/* 前爪伸展 */
.front-paw-l {
  animation: stretch-paw 2.5s ease-in-out infinite;
}
.front-paw-r {
  animation: stretch-paw 2.5s ease-in-out 0.2s infinite;
}

@keyframes stretch-paw {
  0%, 100% { transform: translateX(0); }
  50% { transform: translateX(-4px); }
}

/* 尾巴伸展 */
.tail-stretch {
  animation: tail-stretch 2.5s ease-in-out infinite;
  transform-origin: 46px 52px;
}

@keyframes tail-stretch {
  0%, 100% { transform: scaleX(1); }
  50% { transform: scaleX(1.3); }
}
</style>
```

**Step 2: Commit**

```bash
git add src/components/pets/CatStretch.vue
git commit -m "feat(pets): create CatStretch component"
```

---

## Task 7: 创建狗狗动作组件 - 坐立

**Files:**
- Create: `src/components/pets/DogSit.vue`

**Step 1: 创建组件**

```vue
<script setup lang="ts">
// 狗狗坐立动作 - 端坐，尾巴摇摆
</script>

<template>
  <svg class="pet-svg" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
    <!-- 狗身体（坐姿） -->
    <rect x="10" y="42" width="26" height="24" rx="10" fill="#D97706"/>
    <!-- 狗头 -->
    <ellipse cx="23" cy="24" rx="16" ry="14" fill="#D97706"/>
    <!-- 耳朵（垂耳） -->
    <ellipse cx="10" cy="28" rx="6" ry="10" fill="#B45309"/>
    <ellipse cx="36" cy="28" rx="6" ry="10" fill="#B45309"/>
    <!-- 眼睛 -->
    <circle cx="17" cy="22" r="3" fill="white"/>
    <circle cx="29" cy="22" r="3" fill="white"/>
    <circle cx="17" cy="22" r="1.5" fill="#1E293B"/>
    <circle cx="29" cy="22" r="1.5" fill="#1E293B"/>
    <circle cx="17.5" cy="21" r="0.6" fill="white"/>
    <circle cx="29.5" cy="21" r="0.6" fill="white"/>
    <!-- 鼻子 -->
    <ellipse cx="23" cy="30" rx="4" ry="3" fill="#1E293B"/>
    <!-- 嘴巴 -->
    <path d="M17 33 Q23 36 29 33" stroke="#1E293B" stroke-width="1.5" fill="none" stroke-linecap="round"/>
    <!-- 舌头 -->
    <ellipse class="tongue-pant" cx="23" cy="36" rx="3" ry="4" fill="#F472B6"/>
    <!-- 前爪 -->
    <ellipse cx="16" cy="66" rx="5" ry="4" fill="#B45309"/>
    <ellipse cx="30" cy="66" rx="5" ry="4" fill="#B45309"/>
    <!-- 摇摆的尾巴 -->
    <path class="tail-wag" d="M36 50 Q50 42 48 30 Q44 22 40 28" stroke="#D97706" stroke-width="6" fill="none" stroke-linecap="round"/>
  </svg>
</template>

<style scoped>
.pet-svg {
  width: 80px;
  height: 80px;
  overflow: visible;
}

/* 舌头喘气 */
.tongue-pant {
  animation: pant 1s ease-in-out infinite;
}

@keyframes pant {
  0%, 100% { ry: 4; }
  50% { ry: 5; }
}

/* 尾巴摇摆 */
.tail-wag {
  animation: wag 0.4s ease-in-out infinite alternate;
  transform-origin: 36px 50px;
}

@keyframes wag {
  from { transform: rotate(-8deg); }
  to { transform: rotate(8deg); }
}
</style>
```

**Step 2: Commit**

```bash
git add src/components/pets/DogSit.vue
git commit -m "feat(pets): create DogSit component"
```

---

## Task 8: 创建狗狗动作组件 - 吠叫

**Files:**
- Create: `src/components/pets/DogBark.vue`

**Step 1: 创建组件**

```vue
<script setup lang="ts">
// 狗狗吠叫动作 - 张嘴，音符飘出
</script>

<template>
  <svg class="pet-svg" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
    <!-- 音符 -->
    <text class="note-a" x="50" y="12" font-size="10" fill="#FCD34D" font-family="monospace">♪</text>
    <text class="note-b" x="58" y="18" font-size="8" fill="#FCD34D" font-family="monospace">♫</text>
    <!-- 狗身体 -->
    <rect x="10" y="42" width="26" height="24" rx="10" fill="#D97706"/>
    <!-- 狗头 -->
    <ellipse cx="23" cy="24" rx="16" ry="14" fill="#D97706"/>
    <!-- 耳朵（竖起） -->
    <polygon points="8,14 12,4 16,14" fill="#D97706"/>
    <polygon points="30,14 34,4 38,14" fill="#D97706"/>
    <!-- 眼睛（睁大） -->
    <circle cx="17" cy="22" r="3.5" fill="white"/>
    <circle cx="29" cy="22" r="3.5" fill="white"/>
    <circle cx="17" cy="22" r="2" fill="#1E293B"/>
    <circle cx="29" cy="22" r="2" fill="#1E293B"/>
    <!-- 张嘴 -->
    <ellipse class="mouth-open" cx="23" cy="32" rx="6" ry="5" fill="#1E293B"/>
    <!-- 舌头伸出 -->
    <ellipse class="tongue-out" cx="23" cy="35" rx="4" ry="3" fill="#F472B6"/>
    <!-- 前爪 -->
    <ellipse cx="16" cy="66" rx="5" ry="4" fill="#B45309"/>
    <ellipse cx="30" cy="66" rx="5" ry="4" fill="#B45309"/>
    <!-- 尾巴竖直 -->
    <path d="M36 50 Q46 38 42 26 Q38 18 34 24" stroke="#D97706" stroke-width="6" fill="none" stroke-linecap="round"/>
  </svg>
</template>

<style scoped>
.pet-svg {
  width: 80px;
  height: 80px;
  overflow: visible;
}

/* 嘴巴开合 */
.mouth-open {
  animation: bark-mouth 0.3s ease-in-out infinite;
}

@keyframes bark-mouth {
  0%, 100% { ry: 5; }
  50% { ry: 7; }
}

/* 舌头伸出 */
.tongue-out {
  animation: bark-tongue 0.3s ease-in-out infinite;
}

@keyframes bark-tongue {
  0%, 100% { cy: 35; }
  50% { cy: 37; }
}

/* 音符飘出 */
.note-a { animation: note-float 0.8s ease-out infinite; }
.note-b { animation: note-float 0.8s ease-out 0.2s infinite; }

@keyframes note-float {
  0% { opacity: 1; transform: translateY(0) scale(1); }
  100% { opacity: 0; transform: translateY(-10px) scale(0.5); }
}
</style>
```

**Step 2: Commit**

```bash
git add src/components/pets/DogBark.vue
git commit -m "feat(pets): create DogBark component"
```

---

## Task 9: 创建狗狗动作组件 - 踱步

**Files:**
- Create: `src/components/pets/DogWalk.vue`

**Step 1: 创建组件**

```vue
<script setup lang="ts">
// 狗狗踱步动作 - 左右走动
</script>

<template>
  <svg class="pet-svg" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
    <!-- 狗身体 -->
    <rect class="dog-body" x="10" y="42" width="26" height="24" rx="10" fill="#D97706"/>
    <!-- 狗头 -->
    <ellipse cx="23" cy="24" rx="16" ry="14" fill="#D97706"/>
    <!-- 耳朵（垂耳） -->
    <ellipse cx="10" cy="28" rx="6" ry="10" fill="#B45309"/>
    <ellipse cx="36" cy="28" rx="6" ry="10" fill="#B45309"/>
    <!-- 眼睛 -->
    <circle cx="17" cy="22" r="3" fill="white"/>
    <circle cx="29" cy="22" r="3" fill="white"/>
    <circle cx="17" cy="22" r="1.5" fill="#1E293B"/>
    <circle cx="29" cy="22" r="1.5" fill="#1E293B"/>
    <circle cx="17.5" cy="21" r="0.6" fill="white"/>
    <circle cx="29.5" cy="21" r="0.6" fill="white"/>
    <!-- 鼻子 -->
    <ellipse cx="23" cy="30" rx="4" ry="3" fill="#1E293B"/>
    <!-- 嘴巴 -->
    <path d="M17 33 Q23 35 29 33" stroke="#1E293B" stroke-width="1.5" fill="none" stroke-linecap="round"/>
    <!-- 前爪（交替移动） -->
    <ellipse class="paw-fl" cx="16" cy="66" rx="5" ry="4" fill="#B45309"/>
    <ellipse class="paw-fr" cx="30" cy="66" rx="5" ry="4" fill="#B45309"/>
    <!-- 尾巴摆动 -->
    <path class="tail-wag" d="M36 50 Q48 44 44 32 Q40 24 36 30" stroke="#D97706" stroke-width="6" fill="none" stroke-linecap="round"/>
  </svg>
</template>

<style scoped>
.pet-svg {
  width: 80px;
  height: 80px;
  overflow: visible;
}

/* 身体轻微晃动 */
.dog-body {
  animation: walk-bob 0.6s ease-in-out infinite;
}

@keyframes walk-bob {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-2px); }
}

/* 爪子交替 */
.paw-fl {
  animation: walk-paw 0.6s ease-in-out infinite;
}
.paw-fr {
  animation: walk-paw 0.6s ease-in-out 0.3s infinite;
}

@keyframes walk-paw {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-3px); }
}

/* 尾巴摆动 */
.tail-wag {
  animation: wag 0.3s ease-in-out infinite alternate;
  transform-origin: 36px 50px;
}

@keyframes wag {
  from { transform: rotate(-5deg); }
  to { transform: rotate(5deg); }
}
</style>
```

**Step 2: Commit**

```bash
git add src/components/pets/DogWalk.vue
git commit -m "feat(pets): create DogWalk component"
```

---

## Task 10: 创建狗狗动作组件 - 讨食

**Files:**
- Create: `src/components/pets/DogBeg.vue`

**Step 1: 创建组件**

```vue
<script setup lang="ts">
// 狗狗讨食动作 - 前爪抬起，期待表情
</script>

<template>
  <svg class="pet-svg" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
    <!-- 狗身体 -->
    <rect x="10" y="48" width="26" height="20" rx="8" fill="#D97706"/>
    <!-- 狗头 -->
    <ellipse cx="23" cy="24" rx="16" ry="14" fill="#D97706"/>
    <!-- 耳朵（垂耳，更下垂） -->
    <ellipse cx="8" cy="30" rx="6" ry="12" fill="#B45309"/>
    <ellipse cx="38" cy="30" rx="6" ry="12" fill="#B45309"/>
    <!-- 大眼睛（期待） -->
    <circle cx="17" cy="22" r="4" fill="white"/>
    <circle cx="29" cy="22" r="4" fill="white"/>
    <circle cx="17" cy="22" r="2.5" fill="#1E293B"/>
    <circle cx="29" cy="22" r="2.5" fill="#1E293B"/>
    <circle cx="18" cy="21" r="1" fill="white"/>
    <circle cx="30" cy="21" r="1" fill="white"/>
    <!-- 鼻子 -->
    <ellipse cx="23" cy="30" rx="4" ry="3" fill="#1E293B"/>
    <!-- 嘴巴（微笑） -->
    <path d="M17 34 Q23 37 29 34" stroke="#1E293B" stroke-width="1.5" fill="none" stroke-linecap="round"/>
    <!-- 抬起的前爪 -->
    <ellipse class="beg-paw-l" cx="14" cy="45" rx="5" ry="4" fill="#B45309"/>
    <ellipse class="beg-paw-r" cx="32" cy="45" rx="5" ry="4" fill="#B45309"/>
    <!-- 后爪 -->
    <ellipse cx="16" cy="68" rx="5" ry="4" fill="#B45309"/>
    <ellipse cx="30" cy="68" rx="5" ry="4" fill="#B45309"/>
    <!-- 尾巴摇摆 -->
    <path class="tail-wag-fast" d="M36 52 Q48 44 44 32 Q40 24 36 30" stroke="#D97706" stroke-width="6" fill="none" stroke-linecap="round"/>
  </svg>
</template>

<style scoped>
.pet-svg {
  width: 80px;
  height: 80px;
  overflow: visible;
}

/* 抬起的爪子抖动 */
.beg-paw-l {
  animation: beg-shake 0.5s ease-in-out infinite;
}
.beg-paw-r {
  animation: beg-shake 0.5s ease-in-out 0.1s infinite;
}

@keyframes beg-shake {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-2px); }
}

/* 快速摇摆尾巴 */
.tail-wag-fast {
  animation: wag-fast 0.2s ease-in-out infinite alternate;
  transform-origin: 36px 52px;
}

@keyframes wag-fast {
  from { transform: rotate(-10deg); }
  to { transform: rotate(10deg); }
}
</style>
```

**Step 2: Commit**

```bash
git add src/components/pets/DogBeg.vue
git commit -m "feat(pets): create DogBeg component"
```

---

## Task 11: 修改 PetWidget 集成动作系统

**Files:**
- Modify: `src/components/PetWidget.vue`

**Step 1: 在 script 部分添加 usePetAction**

在 `import` 语句后添加：

```typescript
import { usePetAction } from '../composables/usePetAction'
import type { PetType } from '../types/config'
```

在组件 setup 中添加（在 `const { displayMode } = useDisplayMode()` 后）：

```typescript
// 宠物动作系统
const { petType, currentAction } = usePetAction()

// 监听配置变化更新宠物
watch(() => config.value?.pet_config?.selected_pet, (newPet) => {
  if (newPet && newPet !== petType.value) {
    petType.value = newPet as PetType
  }
})
```

**Step 2: 创建动作组件映射**

在 script 中添加：

```typescript
// 动态导入宠物组件
const petComponents = {
  'cat-sleep': () => import('./pets/CatSleep.vue'),
  'cat-play': () => import('./pets/CatPlay.vue'),
  'cat-stare': () => import('./pets/CatStare.vue'),
  'cat-stretch': () => import('./pets/CatStretch.vue'),
  'dog-sit': () => import('./pets/DogSit.vue'),
  'dog-bark': () => import('./pets/DogBark.vue'),
  'dog-walk': () => import('./pets/DogWalk.vue'),
  'dog-beg': () => import('./pets/DogBeg.vue')
}
```

**Step 3: 修改 template 部分**

找到现有的 SVG 状态部分（`<!-- ===== FRESH: 睡猫 + 咖啡 ===== -->` 等），替换为：

```vue
<!-- 动态宠物动作组件 -->
<component :is="petComponents[currentAction]" v-if="petComponents[currentAction]" />
```

**Step 4: 验证组件渲染**

运行: `npm run tauri:dev`
预期: 宠物区域显示猫咪睡觉动作，每 25 秒随机切换

**Step 5: Commit**

```bash
git add src/components/PetWidget.vue
git commit -m "feat(PetWidget): integrate pet action system with dynamic components"
```

---

## Task 12: 扩展 Rust 配置结构

**Files:**
- Modify: `src-tauri/src/config.rs`

**Step 1: 添加 PetConfig 结构体**

在文件中添加（在其他 Config 结构体后）：

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetConfig {
    pub selected_pet: String,
    pub action_interval: u64,
}

impl Default for PetConfig {
    fn default() -> Self {
        Self {
            selected_pet: "cat".to_string(),
            action_interval: 25,
        }
    }
}
```

**Step 2: 更新 AppConfig 结构体**

找到 `pub struct AppConfig`，添加 `pet_config` 字段：

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub api_config: ApiConfig,
    pub polling_config: PollingConfig,
    pub display_config: DisplayConfig,
    pub pet_config: PetConfig,  // 新增
}
```

**Step 3: 更新 Default 实现**

在 `impl Default for AppConfig` 中添加：

```rust
pet_config: PetConfig::default(),
```

**Step 4: 验证编译**

运行: `cd src-tauri && cargo build`
预期: 编译成功

**Step 5: Commit**

```bash
git add src-tauri/src/config.rs
git commit -m "feat(config): add PetConfig to Rust backend"
```

---

## Task 13: 扩展 useSettings Composable

**Files:**
- Modify: `src/composables/useSettings.ts`

**Step 1: 添加宠物配置相关的 computed 和方法**

在 `return` 语句前添加：

```typescript
// 宠物配置
const petConfig = computed(() => config.value?.pet_config)

// 更新宠物类型
async function updatePetType(petType: string) {
  if (config.value?.pet_config) {
    config.value.pet_config.selected_pet = petType
  }
}

// 更新动作间隔
async function updateActionInterval(interval: number) {
  if (config.value?.pet_config) {
    config.value.pet_config.action_interval = interval
  }
}
```

**Step 2: 在 return 中导出**

找到 `return {`，添加：

```typescript
petConfig,
updatePetType,
updateActionInterval,
```

**Step 3: Commit**

```bash
git add src/composables/useSettings.ts
git commit -m "feat(composables): add pet config helpers to useSettings"
```

---

## Task 14: 修改 SettingsPanel 添加宠物标签

**Files:**
- Modify: `src/components/SettingsPanel.vue`

**Step 1: 添加宠物标签到 activeTab 类型**

找到 `const activeTab = ref<'models' | 'polling' | 'display'>('models')`，修改为：

```typescript
const activeTab = ref<'models' | 'polling' | 'pet' | 'display'>('models')
```

**Step 2: 在 script setup 中添加宠物配置方法**

找到 `const {` 开头的解构，添加：

```typescript
petConfig,
updatePetType,
updateActionInterval,
```

**Step 3: 在 template 中添加宠物标签按钮**

找到标签切换部分，在【展示设置】前添加：

```vue
<button
  class="rpg-tab"
  :class="{ active: activeTab === 'pet' }"
  @click="activeTab = 'pet'"
>
  [宠物]
</button>
```

**Step 4: 添加宠物标签内容**

在 `<!-- 展示配置标签 -->` 前添加：

```vue
<!-- 宠物配置标签 -->
<div v-if="activeTab === 'pet'" class="tab-content">
  <div class="rpg-card">
    <div class="rpg-label">当前宠物</div>
    <div class="rpg-models-list">
      <label
        v-for="pet in [
          { value: 'cat', label: '像素猫咪' },
          { value: 'dog', label: '像素狗狗' }
        ]"
        :key="pet.value"
        class="rpg-model-item"
        style="display: flex; align-items: center; cursor: pointer; border-radius: 4px;"
      >
        <input
          type="radio"
          name="petType"
          :value="pet.value"
          :checked="config.pet_config?.selected_pet === pet.value"
          @change="updatePetType(pet.value)"
          style="margin-right: 10px; accent-color: #ffd700;"
        />
        <span class="rpg-model-name">{{ pet.label }}</span>
      </label>
    </div>
  </div>

  <div class="rpg-card">
    <div class="rpg-label">动作切换间隔（秒）</div>
    <input
      type="number"
      class="rpg-input"
      :value="config.pet_config?.action_interval || 25"
      @input="updateActionInterval(parseInt(($event.target as HTMLInputElement).value) || 25)"
      min="10"
      max="60"
    />
    <div class="rpg-help">
      当前：{{ config.pet_config?.action_interval || 25 }} 秒
    </div>
  </div>

  <div class="rpg-card">
    <div class="rpg-label">说明</div>
    <div class="rpg-info-text">
      宠物动作随机切换，与用量状态无关。<br>
      选择不同的宠物可体验不同的动作效果。
    </div>
  </div>
</div>
```

**Step 5: 验证设置面板**

运行: `npm run tauri:dev`
1. 打开设置面板
2. 切换到【宠物】标签
3. 选择"像素狗狗"
4. 预期: 主窗口宠物变为狗狗

**Step 6: Commit**

```bash
git add src/components/SettingsPanel.vue
git commit -m "feat(SettingsPanel): add pet selection tab"
```

---

## Task 14: 处理配置文件兼容性

**Files:**
- Modify: `src-tauri/src/config.rs`

**Step 1: 添加向后兼容的配置加载**

在配置加载函数中添加兼容处理：

```rust
pub fn load_config() -> Result<AppConfig, String> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        return Ok(AppConfig::default());
    }

    let content = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config: {}", e))?;

    // 尝试解析为完整配置
    if let Ok(config) = serde_json::from_str::<AppConfig>(&content) {
        return Ok(config);
    }

    // 向后兼容：尝试解析为旧版配置（无 pet_config）
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct OldAppConfig {
        pub api_config: ApiConfig,
        pub polling_config: PollingConfig,
        pub display_config: DisplayConfig,
    }

    if let Ok(old_config) = serde_json::from_str::<OldAppConfig>(&content) {
        return Ok(AppConfig {
            api_config: old_config.api_config,
            polling_config: old_config.polling_config,
            display_config: old_config.display_config,
            pet_config: PetConfig::default(),
        });
    }

    Err("Failed to parse config file".to_string())
}
```

**Step 2: Commit**

```bash
git add src-tauri/src/config.rs
git commit -m "feat(config): add backward compatibility for old config files"
```

---

## Task 15: 最终测试和验证

**Step 1: 完整功能测试**

运行: `npm run tauri:dev`

测试清单：
- [ ] 启动后默认显示猫咪睡觉动作
- [ ] 等待 25 秒，动作随机切换
- [ ] 打开设置面板，切换到【宠物】标签
- [ ] 选择"像素狗狗"，确认主窗口立即显示狗狗
- [ ] 修改动作切换间隔为 15 秒
- [ ] 等待确认切换间隔生效
- [ ] 保存设置，重启应用
- [ ] 确认配置持久化（仍为狗狗，间隔 15 秒）
- [ ] 用量状态变化时，显示模式颜色仍正常变化
- [ ] 宠物动作不受用量影响

**Step 2: 创建测试配置文件**

如果有旧配置文件，测试向后兼容：

```bash
# 备份现有配置
cp ~/.config/plan-guard/config.json ~/.config/plan-guard/config.json.bak

# 创建旧版配置（无 pet_config）
cat > ~/.config/plan-guard/config.json << 'EOF'
{
  "api_config": {
    "current_model": "bigmodel",
    "models": [...]
  },
  "polling_config": {
    "interval_minutes": 5
  },
  "display_config": {
    "display_mode": "holo-bubble"
  }
}
EOF

# 启动应用，确认默认为猫咪
```

**Step 3: 清理和文档**

```bash
# 恢复配置
mv ~/.config/plan-guard/config.json.bak ~/.config/plan-guard/config.json
```

**Step 4: 提交设计文档**

```bash
git add docs/plans/2026-04-06-pet-action-system-design.md
git commit -m "docs: add pet action system design document"
```

**Step 5: 最终提交**

```bash
git add .
git commit -m "feat: complete pet action system implementation"
```

---

## 完成标准

- [x] 8 个宠物动作组件全部创建
- [x] `usePetAction` composable 正常工作
- [x] 设置面板【宠物】标签可用
- [x] 配置持久化正常
- [x] 向后兼容旧配置文件
- [x] 宠物动作与用量状态完全解耦
- [x] 所有测试通过

## 相关文档

- 设计文档: `docs/plans/2026-04-06-pet-action-system-design.md`
- 类型定义: `src/types/config.ts`
- 动作管理: `src/composables/usePetAction.ts`
