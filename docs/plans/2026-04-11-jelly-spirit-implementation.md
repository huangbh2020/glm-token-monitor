# 果冻精灵宠物组件实现计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 创建简约的果冻精灵 SVG 组件，替换复杂的猫咪形象

**Architecture:** 单文件 Vue 组件，使用 SVG + SMIL 动画，通过 props 接收状态并动态渲染颜色和动画

**Tech Stack:** Vue 3 Composition API, SVG, SMIL Animation

---

### Task 1: 创建 JellySpirit.vue 组件骨架

**Files:**
- Create: `src/components/pets/JellySpirit.vue`

**Step 1: 创建组件基础结构**

```vue
<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  state: 'Fresh' | 'Flow' | 'Warning' | 'Panic' | 'Dead'
  width?: number
  height?: number
}

const props = withDefaults(defineProps<Props>(), {
  width: 100,
  height: 100
})

// 颜色配置
const colors = {
  Fresh:  { fill: '#10B981', stroke: '#059669', eye: '#1F2937' },
  Flow:   { fill: '#3B82F6', stroke: '#2563EB', eye: '#1F2937' },
  Warning:{ fill: '#F59E0B', stroke: '#D97706', eye: '#1F2937' },
  Panic:  { fill: '#EF4444', stroke: '#DC2626', eye: '#1F2937' },
  Dead:   { fill: '#9CA3AF', stroke: '#6B7280', eye: '#6B7280' }
}

const currentColors = computed(() => colors[props.state])
</script>

<template>
  <div class="spirit-container" :style="{ width: `${width}px`, height: `${height}px` }">
    <svg
      :width="width"
      :height="height"
      viewBox="0 0 64 64"
      xmlns="http://www.w3.org/2000/svg"
      class="spirit-svg"
    >
      <!-- 身体 -->
      <ellipse cx="32" cy="36" rx="24" ry="26" :fill="currentColors.fill" :stroke="currentColors.stroke" stroke-width="2"/>
      <!-- 高光 -->
      <ellipse cx="26" cy="20" rx="6" ry="4" fill="white" opacity="0.3"/>
      <!-- 左眼 -->
      <circle cx="26" cy="32" r="3" :fill="currentColors.eye"/>
      <!-- 右眼 -->
      <circle cx="38" cy="32" r="3" :fill="currentColors.eye"/>
    </svg>
  </div>
</template>

<style scoped>
.spirit-container {
  display: flex;
  align-items: center;
  justify-content: center;
}

.spirit-svg {
  display: block;
}
</style>
```

**Step 2: 验证组件可以渲染**

运行开发服务器查看效果：
```bash
npm run tauri:dev
```

**Step 3: Commit**

```bash
git add src/components/pets/JellySpirit.vue
git commit -m "feat: add JellySpirit pet component skeleton"
```

---

### Task 2: 添加 Fresh 状态动画（呼吸 + 慢眨眼）

**Files:**
- Modify: `src/components/pets/JellySpirit.vue`

**Step 1: 添加呼吸动画到身体**

在 `<ellipse>` 身体元素中添加：

```vue
<ellipse cx="32" cy="36" rx="24" ry="26" :fill="currentColors.fill" :stroke="currentColors.stroke" stroke-width="2">
  <animateTransform
    v-if="state === 'Fresh'"
    attributeName="transform"
    type="scale"
    values="1,1; 1,1.05; 1,1"
    dur="3s"
    repeatCount="indefinite"
    calcMode="spline"
    keyTimes="0;0.5;1"
    keySplines="0.4 0 0.6 1; 0.4 0 0.6 1"
  />
</ellipse>
```

**Step 2: 添加眼睛眨眼动画**

修改眼睛元素：

```vue
<!-- 左眼 -->
<circle cx="26" cy="32" r="3" :fill="currentColors.eye">
  <animate
    v-if="state === 'Fresh'"
    attributeName="r"
    values="3;0.3;3"
    dur="4s"
    repeatCount="indefinite"
  />
</circle>
<!-- 右眼 -->
<circle cx="38" cy="32" r="3" :fill="currentColors.eye">
  <animate
    v-if="state === 'Fresh'"
    attributeName="r"
    values="3;0.3;3"
    dur="4s"
    begin="0.1s"
    repeatCount="indefinite"
  />
</circle>
```

**Step 3: 验证动画效果**

刷新页面查看呼吸和眨眼动画。

**Step 4: Commit**

```bash
git add src/components/pets/JellySpirit.vue
git commit -m "feat: add Fresh state animations to JellySpirit"
```

---

### Task 3: 添加 Flow 状态动画（摇摆 + 正常眨眼）

**Files:**
- Modify: `src/components/pets/JellySpirit.vue`

**Step 1: 添加轻微摇摆动画**

用 `<g>` 包裹身体和眼睛，添加旋转动画：

```vue
<g>
  <animateTransform
    v-if="state === 'Flow'"
    attributeName="transform"
    type="rotate"
    values="-3 32 36; 3 32 36; -3 32 36"
    dur="2s"
    repeatCount="indefinite"
    calcMode="spline"
    keyTimes="0;0.5;1"
    keySplines="0.45 0 0.55 1; 0.45 0 0.55 1"
  />
  <!-- 身体和眼睛元素 -->
</g>
```

**Step 2: 添加 Flow 状态眨眼**

```vue
<circle cx="26" cy="32" r="3" :fill="currentColors.eye">
  <animate
    v-if="state === 'Flow'"
    attributeName="r"
    values="3;0.3;3"
    dur="3s"
    repeatCount="indefinite"
  />
</circle>
```

**Step 3: Commit**

```bash
git add src/components/pets/JellySpirit.vue
git commit -m "feat: add Flow state animations to JellySpirit"
```

---

### Task 4: 添加 Warning 状态动画（抖动 + 眼睛放大）

**Files:**
- Modify: `src/components/pets/JellySpirit.vue`

**Step 1: 添加抖动动画**

```vue
<animateTransform
  v-if="state === 'Warning'"
  attributeName="transform"
  type="translate"
  values="0,0; 1,0; 0,0; -1,0; 0,0"
  dur="0.3s"
  repeatCount="indefinite"
/>
```

**Step 2: 添加眼睛放大动画**

```vue
<circle cx="26" cy="32" r="3" :fill="currentColors.eye">
  <animate
    v-if="state === 'Warning'"
    attributeName="r"
    values="4;4;4"
    dur="1s"
  />
</circle>
```

**Step 3: Commit**

```bash
git add src/components/pets/JellySpirit.vue
git commit -m "feat: add Warning state animations to JellySpirit"
```

---

### Task 5: 添加 Panic 状态动画（快速弹跳 + 眼睛瞪大）

**Files:**
- Modify: `src/components/pets/JellySpirit.vue`

**Step 1: 添加快速弹跳**

```vue
<animateTransform
  v-if="state === 'Panic'"
  attributeName="transform"
  type="translate"
  values="0,0; 0,-4; 0,0"
  dur="0.25s"
  repeatCount="indefinite"
  calcMode="spline"
  keyTimes="0;0.5;1"
  keySplines="0.5 0 0.5 1; 0.5 0 0.5 1"
/>
```

**Step 2: 添加眼睛最大状态**

```vue
<circle cx="26" cy="32" r="3" :fill="currentColors.eye">
  <animate
    v-if="state === 'Panic'"
    attributeName="r"
    values="5;5;5"
    dur="1s"
  />
</circle>
```

**Step 3: 添加小星星特效（可选）**

在 Panic 状态眼睛旁边添加小星星装饰。

**Step 4: Commit**

```bash
git add src/components/pets/JellySpirit.vue
git commit -m "feat: add Panic state animations to JellySpirit"
```

---

### Task 6: 集成到 PetWidget.vue

**Files:**
- Modify: `src/components/PetWidget.vue`

**Step 1: 导入 JellySpirit 组件**

```typescript
import JellySpirit from './pets/JellySpirit.vue'
```

**Step 2: 替换现有宠物渲染**

找到当前渲染 `CatGifViewer` 的位置，替换为：

```vue
<!-- 果冻精灵 -->
<JellySpirit
  v-if="petType === 'spirit'"
  :state="petState"
  :width="100"
  :height="100"
/>
```

**Step 3: 添加 petType 支持**

在 `usePetAction` composable 中添加 `spirit` 类型支持。

**Step 4: 测试各状态切换**

手动切换状态验证颜色和动画正确。

**Step 5: Commit**

```bash
git add src/components/PetWidget.vue
git commit -m "feat: integrate JellySpirit into PetWidget"
```

---

### Task 7: 清理旧宠物组件（可选）

**Files:**
- Consider removing: `src/components/pets/CatGifViewer.vue`
- Consider removing: `src/components/pets/Dog*.vue`
- Consider removing: `src/assets/pets/*.gif`

**Step 1: 确认新组件工作正常后，删除旧文件**

**Step 2: 更新导入引用**

**Step 3: Commit**

```bash
git add -A
git commit -m "refactor: remove old pet components, use JellySpirit"
```
