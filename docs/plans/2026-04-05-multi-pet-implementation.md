# 多宠物系统实现计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 将 PlanGuard 从单一猫咪宠物重构为支持多宠物、多动画的配置驱动系统

**Architecture:** 采用配置文件驱动方案，宠物数据存储在 JSON 文件中，通过 usePetManager composable 管理状态和动画切换，使用 Tauri Store 持久化用户设置

**Tech Stack:** Vue 3, TypeScript, Tauri 2, Tauri Plugin Store

---

## Task 1: 添加 Tauri Store 依赖

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Modify: `src-tauri/src/main.rs`

**Step 1: 添加 Cargo.toml 依赖**

```toml
[dependencies]
tauri-plugin-store = "2"
```

**Step 2: 在 main.rs 注册插件**

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Step 3: 在 tauri.conf.json 启用 store 插件**

```json
{
  "plugins": {
    "store": {
      "allow": ["settings.json"]
    }
  }
}
```

**Step 4: 测试构建**

Run: `cd src-tauri && cargo build`
Expected: 编译成功，无错误

**Step 5: Commit**

```bash
git add src-tauri/Cargo.toml src-tauri/src/main.rs src-tauri/tauri.conf.json
git commit -m "feat: add tauri-plugin-store dependency"
```

---

## Task 2: 创建宠物配置目录结构

**Files:**
- Create: `src/assets/pets/index.ts`
- Create: `src/assets/pets/cat.json`

**Step 1: 创建目录**

Run: `mkdir -p src/assets/pets`

**Step 2: 创建类型定义文件**

```typescript
// src/assets/pets/types.ts
export interface Animation {
  id: string
  name: string
  svg: string
  duration: number
}

export interface Pet {
  id: string
  name: string
  description: string
  animations: Animation[]
}
```

**Step 3: 创建猫咪配置文件（迁移现有动画）**

```json
// src/assets/pets/cat.json
{
  "id": "cat",
  "name": "猫咪",
  "description": "可爱的像素猫咪",
  "animations": [
    {
      "id": "sleeping",
      "name": "睡觉",
      "svg": "<svg class=\"pet-svg\" viewBox=\"0 0 80 80\" xmlns=\"http://www.w3.org/2000/svg\"><style>.cat-breathe { animation: breathe 2.8s ease-in-out infinite; transform-origin: 22px 55px; } @keyframes breathe { 0%,100% { transform: scaleY(1); } 50% { transform: scaleY(1.1); } }</style><text class=\"zzz-a\" x=\"44\" y=\"26\" font-size=\"7\" fill=\"#6EE7B7\" font-family=\"monospace\" font-weight=\"bold\">z</text><ellipse class=\"cat-breathe\" cx=\"22\" cy=\"55\" rx=\"18\" ry=\"11\" fill=\"#F4A460\"/></svg>",
      "duration": 2800
    }
  ]
}
```

**Step 4: 创建配置导出文件**

```typescript
// src/assets/pets/index.ts
import catJson from './cat.json'
import type { Pet } from './types'

export const pets: Pet[] = [
  catJson as Pet
]

export const defaultPetId = 'cat'
```

**Step 5: Commit**

```bash
git add src/assets/pets/
git commit -m "feat: create pet configuration structure"
```

---

## Task 3: 实现 usePetManager Composable

**Files:**
- Create: `src/composables/usePetManager.ts`
- Create: `src/composables/usePetManager.spec.ts`

**Step 1: 编写测试**

```typescript
// src/composables/usePetManager.spec.ts
import { describe, it, expect, beforeEach, vi } from 'vitest'
import { usePetManager } from './usePetManager'
import { Store } from 'tauri-plugin-store'

vi.mock('tauri-plugin-store', () => ({
  Store: vi.fn()
}))

describe('usePetManager', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should load pets from config', async () => {
    const { loadPets, availablePets } = usePetManager()
    await loadPets()
    expect(availablePets.value.length).toBeGreaterThan(0)
  })

  it('should select pet by id', () => {
    const { selectPet, selectedPetId } = usePetManager()
    selectPet('cat')
    expect(selectedPetId.value).toBe('cat')
  })

  it('should switch to different animation', () => {
    const { switchAnimation, currentAnimation } = usePetManager()
    const beforeId = currentAnimation.value?.id
    switchAnimation()
    if (beforeId) {
      expect(currentAnimation.value?.id).not.toBe(beforeId)
    }
  })
})
```

**Step 2: 运行测试验证失败**

Run: `npm test`
Expected: FAIL with "usePetManager not defined"

**Step 3: 实现最小可用版本**

```typescript
// src/composables/usePetManager.ts
import { ref, computed, onMounted } from 'vue'
import { Store } from 'tauri-plugin-store'
import { pets, defaultPetId } from '@/assets/pets'
import type { Pet, Animation } from '@/assets/pets/types'

const store = new Store('settings.json')

export function usePetManager() {
  const selectedPetId = ref<string>(defaultPetId)
  const animationInterval = ref<number>(120)
  const availablePets = ref<Pet[]>(pets)
  const currentAnimation = ref<Animation | null>(null)

  const currentPet = computed(() => 
    availablePets.value.find(p => p.id === selectedPetId.value)
  )

  async function loadPets() {
    availablePets.value = pets
    // 加载保存的设置
    const savedPetId = await store.get<string>('petId')
    if (savedPetId) {
      selectedPetId.value = savedPetId
    }
    const savedInterval = await store.get<number>('animationInterval')
    if (savedInterval) {
      animationInterval.value = savedInterval
    }
    // 设置初始动画
    if (currentPet.value && currentPet.value.animations.length > 0) {
      currentAnimation.value = currentPet.value.animations[0]
    }
  }

  function selectPet(id: string) {
    const pet = availablePets.value.find(p => p.id === id)
    if (pet) {
      selectedPetId.value = id
      currentAnimation.value = pet.animations[0] || null
    }
  }

  function switchAnimation() {
    const pet = currentPet.value
    if (!pet || pet.animations.length === 0) return

    let nextAnimation: Animation
    do {
      nextAnimation = pet.animations[
        Math.floor(Math.random() * pet.animations.length)
      ]
    } while (
      nextAnimation.id === currentAnimation.value?.id && 
      pet.animations.length > 1
    )

    currentAnimation.value = nextAnimation
  }

  async function saveSettings() {
    await store.set('petId', selectedPetId.value)
    await store.set('animationInterval', animationInterval.value)
    await store.save()
  }

  return {
    selectedPetId,
    animationInterval,
    availablePets,
    currentPet,
    currentAnimation,
    loadPets,
    selectPet,
    switchAnimation,
    saveSettings
  }
}
```

**Step 4: 运行测试验证通过**

Run: `npm test`
Expected: PASS

**Step 5: Commit**

```bash
git add src/composables/usePetManager.ts src/composables/usePetManager.spec.ts
git commit -m "feat: implement usePetManager composable"
```

---

## Task 4: 重构 PetWidget.vue

**Files:**
- Modify: `src/components/PetWidget.vue`

**Step 1: 备份现有组件**

Run: `cp src/components/PetWidget.vue src/components/PetWidget.vue.bak`

**Step 2: 修改 script 部分**

```vue
<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import { useUsageState } from '../composables/useUsageState'
import { useTauriEvents } from '../composables/useTauriEvents'
import { usePetManager } from '../composables/usePetManager'

const { usageData, setupEventListener } = useTauriEvents()
const { petState } = useUsageState(
  computed(() => usageData.value.used),
  computed(() => usageData.value.total)
)

// 宠物管理
const { 
  currentPet, 
  currentAnimation, 
  switchAnimation,
  loadPets,
  animationInterval
} = usePetManager()

// 动画定时器
let animationTimer: number | null = null

// 设置动画定时器
function setupAnimationTimer() {
  if (animationTimer) clearInterval(animationTimer)
  animationTimer = window.setInterval(() => {
    switchAnimation()
  }, animationInterval.value * 1000)
}

// 监听动画间隔变化
watch(animationInterval, setupAnimationTimer)

// 双指标数据
const timePercent = computed(() => usageData.value.time_percent ?? 0)
const tokensPercent = computed(() => usageData.value.tokens_percent ?? 0)
const timeRemaining = computed(() => usageData.value.time_remaining)

const heartMessages: Record<string, string> = {
  Fresh:   '新的一天，能量满格！冲冲冲！
