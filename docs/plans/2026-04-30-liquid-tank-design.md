# Liquid Tank Pedestal — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace the flat progress-bar pedestal with a liquid-tank energy slot that animates a wave surface, driven by the existing PetState color system.

**Architecture:** New `LiquidTank.vue` SVG component receives `percent` + `state` from PetWidget, renders a capsule with animated dual-layer sine wave. Shared `getStatusColor()` extracted to `src/utils/statusColor.ts` to eliminate duplication between PetWidget and InfoPanel.

**Tech Stack:** Vue 3 Composition API, inline SVG, CSS keyframe animations

**Design doc:** `docs/plans/2026-04-30-liquid-tank-design.md` (this file)

---

## Task 1: Extract shared `getStatusColor()`

**Files:**
- Create: `src/utils/statusColor.ts`

**Step 1: Create the utility**

```bash
mkdir -p src/utils
```

Create `src/utils/statusColor.ts`:

```typescript
/**
 * Returns a status color based on the used-percentage threshold.
 * Matches the existing PetState color scheme.
 */
export function getStatusColor(percent: number): string {
  if (percent >= 96) return '#6B7280'   // Dead
  if (percent >= 81) return '#F97316'   // Panic
  if (percent >= 61) return '#F59E0B'   // Warning
  return '#3B82F6'                       // Flow / Fresh
}
```

**Step 2: Verify import works**

Run: `cd src && npx tsc --noEmit utils/statusColor.ts 2>/dev/null || echo "OK (vue-tsc quirk)"`
Expected: No blocking errors

**Step 3: Commit**

```bash
git add src/utils/statusColor.ts
git commit -m "refactor: extract getStatusColor to shared utility"
```

---

## Task 2: Create `LiquidTank.vue`

**Files:**
- Create: `src/components/LiquidTank.vue`

**Step 1: Create the component**

Create `src/components/LiquidTank.vue`:

```vue
<script setup lang="ts">
import { computed } from 'vue'
import type { PetState } from '../composables/useUsageState'

const props = defineProps<{
  percent: number
  state: PetState
}>()

const tankW = 130
const tankH = 22
const pad = 2
const innerW = tankW - pad * 2   // 126
const innerH = tankH - pad * 2   // 18

const gradientColors: Record<PetState, [string, string]> = {
  Fresh:   ['#10B981', '#34D399'],
  Flow:    ['#3B82F6', '#60A5FA'],
  Warning: ['#F59E0B', '#FBBF24'],
  Panic:   ['#EF4444', '#F87171'],
  Dead:    ['#4B5563', '#6B7280'],
}

const borderGlows: Record<PetState, string> = {
  Fresh:   'rgba(52,211,153,0.3)',
  Flow:    'rgba(96,165,250,0.3)',
  Warning: 'rgba(251,191,36,0.3)',
  Panic:   'rgba(248,113,113,0.35)',
  Dead:    'rgba(107,114,128,0.2)',
}

const stateAmplitudes: Record<PetState, number> = {
  Fresh: 2, Flow: 2, Warning: 1.5, Panic: 3, Dead: 0.5,
}

const stateDurations: Record<PetState, string> = {
  Fresh: '3s', Flow: '2.5s', Warning: '2s', Panic: '1.2s', Dead: '5s',
}

const colors    = computed(() => gradientColors[props.state])
const glow      = computed(() => borderGlows[props.state])
const amplitude = computed(() => stateAmplitudes[props.state])
const duration  = computed(() => stateDurations[props.state])

const fillHeight = computed(() => {
  const h = (Math.max(0, Math.min(100, props.percent)) / 100) * innerH
  return Math.max(h, 0)
})

const waveY = computed(() => tankH - pad - fillHeight.value)

function wavePath(y: number, amp: number): string {
  const a = Math.min(amp, fillHeight.value / 2)
  return [
    `M -2 ${y}`,
    `C 15 ${y - a}, 35 ${y + a}, 52 ${y}`,
    `C 69 ${y - a}, 89 ${y + a}, 106 ${y}`,
    `C 115 ${y - a}, 126 ${y + a}, 132 ${y}`,
    `L 132 ${tankH} L -2 ${tankH} Z`,
  ].join(' ')
}

const waveFrontPath = computed(() => wavePath(waveY.value, amplitude.value))
const waveBackPath  = computed(() => wavePath(waveY.value, amplitude.value * 0.6))
</script>

<template>
  <div class="liquid-tank-wrap">
    <svg
      :width="tankW"
      :height="tankH"
      :viewBox="`0 0 ${tankW} ${tankH}`"
      xmlns="http://www.w3.org/2000/svg"
      class="liquid-tank"
    >
      <defs>
        <clipPath :id="`tank-clip-${state}`">
          <rect :x="pad" :y="pad" :width="innerW" :height="innerH" :rx="(innerH / 2)" />
        </clipPath>

        <linearGradient :id="`liquid-grad-${state}`" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%"   :stop-color="colors[0]" />
          <stop offset="100%" :stop-color="colors[1]" />
        </linearGradient>
      </defs>

      <!-- Container background -->
      <rect
        :x="pad" :y="pad" :width="innerW" :height="innerH"
        :rx="(innerH / 2)"
        fill="rgba(15,15,17,0.85)"
        class="tank-bg"
      />

      <!-- Liquid (clipped to capsule) -->
      <g :clip-path="`url(#tank-clip-${state})`">
        <path
          :d="waveBackPath"
          :fill="`url(#liquid-grad-${state})`"
          opacity="0.4"
          class="wave wave-back"
          :style="{ animationDuration: `calc(${duration} * 1.4)` }"
        />
        <path
          :d="waveFrontPath"
          :fill="`url(#liquid-grad-${state})`"
          class="wave wave-front"
          :style="{ animationDuration: duration }"
        />
      </g>

      <!-- Border + glow -->
      <rect
        :x="pad" :y="pad" :width="innerW" :height="innerH"
        :rx="(innerH / 2)"
        fill="none"
        stroke="rgba(255,255,255,0.12)"
        stroke-width="1"
        class="tank-stroke"
        :style="{ filter: `drop-shadow(0 0 3px ${glow})` }"
      />
    </svg>
  </div>
</template>

<style scoped>
.liquid-tank-wrap {
  display: flex;
  justify-content: center;
  margin-top: 2px;
  z-index: 10;
}

.liquid-tank {
  display: block;
  overflow: visible;
}

.tank-bg {
  transition: fill 0.6s ease;
}

.tank-stroke {
  transition: stroke 0.6s ease, filter 0.6s ease;
}

.wave {
  will-change: transform;
}
.wave-front {
  animation: waveShift 2.5s linear infinite;
}
.wave-back {
  animation: waveShift 3.5s linear infinite reverse;
}

@keyframes waveShift {
  from { transform: translateX(0); }
  to   { transform: translateX(-52px); }
}
</style>
```

**Step 2: Verify component loads**

Run: `npm run build 2>&1 | tail -5`
Expected: Build succeeds (or only vue-tsc warnings, no hard errors)

**Step 3: Commit**

```bash
git add src/components/LiquidTank.vue
git commit -m "feat: add LiquidTank component with SVG wave animation"
```

---

## Task 3: Update PetWidget.vue

**Files:**
- Modify: `src/components/PetWidget.vue`

**Step 1: Add import**

Add after the existing pet imports (around line 23):

```typescript
import LiquidTank from './LiquidTank.vue'
import { getStatusColor } from '../utils/statusColor'
```

**Step 2: Remove local `getStatusColor()`**

Delete lines 184-189 (the local `getStatusColor` function):

```typescript
// DELETE this block:
function getStatusColor(percent: number): string {
  if (percent >= 96) return '#6B7280'
  if (percent >= 81) return '#F97316'
  if (percent >= 61) return '#F59E0B'
  return '#3B82F6'
}
```

**Step 3: Replace pedestal template**

Replace the entire pedestal block (lines 322-329):

```html
<!-- BEFORE -->
<transition name="pedestal-fade">
  <div v-if="displayMode === 'pedestal' && !showInfoPanel" class="pet-pedestal" :class="`state-${petState.toLowerCase()}`">
    <div class="pedestal-bar">
      <div class="pedestal-fill" :style="{ width: (100 - tokensPercent) + '%' }"></div>
    </div>
    <span class="pedestal-text">{{ 100 - tokensPercent }}%</span>
  </div>
</transition>
```

With:

```html
<!-- AFTER -->
<transition name="pedestal-fade">
  <LiquidTank
    v-if="displayMode === 'pedestal' && !showInfoPanel"
    :percent="100 - tokensPercent"
    :state="petState"
  />
</transition>
```

**Step 4: Remove old pedestal CSS**

Delete the entire `.pet-pedestal` CSS block (lines 802-865):

```css
/* DELETE: .pet-pedestal, .pet-pedestal.state-*, .pedestal-bar, .pedestal-fill,
   .pedestal-text, and all .state-* .pedestal-text rules */
```

Keep the `.pedestal-fade-enter-*` / `.pedestal-fade-leave-*` transition rules (lines 857-865) since the `<transition name="pedestal-fade">` still uses them.

**Step 5: Verify build**

Run: `npm run build 2>&1 | tail -5`
Expected: Build succeeds

**Step 6: Commit**

```bash
git add src/components/PetWidget.vue
git commit -m "refactor: replace pedestal bar with LiquidTank in PetWidget"
```

---

## Task 4: Update InfoPanel.vue

**Files:**
- Modify: `src/components/InfoPanel.vue`

**Step 1: Add import**

Add near the top imports (around line 5):

```typescript
import { getStatusColor } from '../utils/statusColor'
```

**Step 2: Remove local `getStatusColor()`**

Delete lines 18-23:

```typescript
// DELETE this block:
function getStatusColor(percent: number): string {
  if (percent >= 96) return '#6B7280'
  if (percent >= 81) return '#F97316'
  if (percent >= 61) return '#F59E0B'
  return '#3B82F6'
}
```

**Step 3: Verify build**

Run: `npm run build 2>&1 | tail -5`
Expected: Build succeeds

**Step 4: Commit**

```bash
git add src/components/InfoPanel.vue
git commit -m "refactor: use shared getStatusColor in InfoPanel"
```

---

## Task 5: Visual verification & polish

**Step 1: Run dev mode**

Run: `npm run tauri:dev`

**Step 2: Verify all 5 states**

Manually test by adjusting API response or mocking `tokensPercent` values:
- 15% → Fresh: green liquid, calm wave
- 50% → Flow: blue liquid
- 70% → Warning: yellow liquid, slightly calmer wave
- 90% → Panic: red liquid, agitated wave
- 98% → Dead: gray liquid, nearly still

**Step 3: Verify transitions**

- Confirm liquid level animates smoothly when percent changes
- Confirm color transitions blend between states
- Confirm wave speed changes between states

**Step 4: Verify info-bubble still works**

- Click pet → info-bubble opens
- Confirm `getStatusColor()` is being called correctly (colors match)

**Step 5: Final commit**

```bash
git add -A
git commit -m "feat: liquid tank pedestal with wave animation"
```

---

## Task Map

| Task | What | Files touched |
|------|------|--------------|
| 1 | Extract `getStatusColor()` | `src/utils/statusColor.ts` (new) |
| 2 | Create `LiquidTank.vue` | `src/components/LiquidTank.vue` (new) |
| 3 | Wire into PetWidget | `src/components/PetWidget.vue` |
| 4 | Deduplicate InfoPanel | `src/components/InfoPanel.vue` |
| 5 | Visual QA | no code changes |
