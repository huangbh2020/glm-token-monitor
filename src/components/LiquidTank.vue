<script setup lang="ts">
import { computed } from 'vue'
import type { PetState } from '../composables/useUsageState'

const props = defineProps<{
  percent: number
  state: PetState
}>()

const tankW = 140
const tankH = 30
const pad = 2
const innerW = tankW - pad * 2
const innerH = tankH - pad * 2

// 状态对应的表情图标
const stateEmoji: Record<PetState, string> = {
  Fresh:   '😊',
  Flow:    '💪',
  Warning: '😰',
  Panic:   '😱',
  Dead:    '💀',
}

// 液面颜色 - 更鲜艳活泼
const gradientColors: Record<PetState, [string, string, string]> = {
  Fresh:   ['#10B981', '#34D399', '#6EE7B7'],
  Flow:    ['#3B82F6', '#60A5FA', '#93C5FD'],
  Warning: ['#F59E0B', '#FBBF24', '#FCD34D'],
  Panic:   ['#EF4444', '#F87171', '#FCA5A5'],
  Dead:    ['#4B5563', '#6B7280', '#9CA3AF'],
}

// 容器边框发光色
const borderGlows: Record<PetState, string> = {
  Fresh:   'rgba(52,211,153,0.5)',
  Flow:    'rgba(96,165,250,0.5)',
  Warning: 'rgba(251,191,36,0.5)',
  Panic:   'rgba(248,113,113,0.6)',
  Dead:    'rgba(107,114,128,0.3)',
}

// 波浪振幅
const stateAmplitudes: Record<PetState, number> = {
  Fresh: 2.5, Flow: 2.5, Warning: 2, Panic: 4, Dead: 0.8,
}

// 动画速度
const stateDurations: Record<PetState, string> = {
  Fresh: '3s', Flow: '2.5s', Warning: '2s', Panic: '1s', Dead: '6s',
}

const colors    = computed(() => gradientColors[props.state])
const glow      = computed(() => borderGlows[props.state])
const emoji     = computed(() => stateEmoji[props.state])
const amplitude = computed(() => stateAmplitudes[props.state])
const duration  = computed(() => stateDurations[props.state])

// 液面百分比（剩余量）
const percentDisplay = computed(() => Math.round(props.percent))

// 液面高度
const fillHeight = computed(() => {
  const h = (Math.max(0, Math.min(100, props.percent)) / 100) * innerH
  return Math.max(h, 0)
})

// 波浪基准 Y 坐标（从底部算）
const waveY = computed(() => tankH - pad - fillHeight.value)

// 生成正弦波 SVG path
function wavePath(y: number, amp: number): string {
  const a = Math.min(amp, fillHeight.value / 2)
  return [
    `M -4 ${y}`,
    `C 12 ${y - a}, 32 ${y + a}, 52 ${y}`,
    `C 72 ${y - a}, 92 ${y + a}, 112 ${y}`,
    `C 125 ${y - a}, 136 ${y + a}, 144 ${y}`,
    `L 144 ${tankH} L -4 ${tankH} Z`,
  ].join(' ')
}

const waveFrontPath = computed(() => wavePath(waveY.value, amplitude.value))
const waveBackPath  = computed(() => wavePath(waveY.value, amplitude.value * 0.6))
</script>

<template>
  <div class="liquid-tank-wrap" :class="`state-${state.toLowerCase()}`">
    <!-- 左侧表情图标 -->
    <span class="tank-emoji">{{ emoji }}</span>

    <!-- 液面容器 -->
    <div class="tank-container">
      <svg
        :width="tankW"
        :height="tankH"
        :viewBox="`0 0 ${tankW} ${tankH}`"
        xmlns="http://www.w3.org/2000/svg"
        class="liquid-tank"
      >
        <defs>
          <!-- 胶囊裁剪区域 -->
          <clipPath :id="`tank-clip-${state}`">
            <rect :x="pad" :y="pad" :width="innerW" :height="innerH" :rx="(innerH / 2)" />
          </clipPath>

          <!-- 液面渐变（3色，更有层次） -->
          <linearGradient :id="`liquid-grad-${state}`" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%"   :stop-color="colors[2]" stop-opacity="0.9" />
            <stop offset="50%"  :stop-color="colors[0]" />
            <stop offset="100%" :stop-color="colors[1]" />
          </linearGradient>

          <!-- 液面高光渐变 -->
          <linearGradient :id="`highlight-${state}`" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%"   stop-color="white" stop-opacity="0.35" />
            <stop offset="100%" stop-color="white" stop-opacity="0" />
          </linearGradient>
        </defs>

        <!-- 容器背景 -->
        <rect
          :x="pad" :y="pad" :width="innerW" :height="innerH"
          :rx="(innerH / 2)"
          fill="rgba(10,10,15,0.9)"
          class="tank-bg"
        />

        <!-- 液面组（裁剪到胶囊内） -->
        <g :clip-path="`url(#tank-clip-${state})`">
          <!-- 背景波（低透明度，反向） -->
          <path
            :d="waveBackPath"
            :fill="`url(#liquid-grad-${state})`"
            opacity="0.35"
            class="wave wave-back"
            :style="{ animationDuration: `calc(${duration} * 1.5)` }"
          />
          <!-- 前景波 -->
          <path
            :d="waveFrontPath"
            :fill="`url(#liquid-grad-${state})`"
            class="wave wave-front"
            :style="{ animationDuration: duration }"
          />
          <!-- 液面高光条 -->
          <rect
            v-if="fillHeight > 2"
            :x="pad + 4"
            :y="waveY + 1"
            :width="innerW - 8"
            height="2"
            rx="1"
            :fill="`url(#highlight-${state})`"
            class="wave-highlight"
          />
        </g>

        <!-- 容器边框 + 发光 -->
        <rect
          :x="pad" :y="pad" :width="innerW" :height="innerH"
          :rx="(innerH / 2)"
          fill="none"
          stroke="rgba(255,255,255,0.15)"
          stroke-width="1"
          class="tank-stroke"
          :style="{ filter: `drop-shadow(0 0 4px ${glow})` }"
        />
      </svg>

      <!-- 百分比数字（覆盖在 SVG 上方） -->
      <span class="tank-percent" :class="`pct-${state.toLowerCase()}`">
        {{ percentDisplay }}%
      </span>
    </div>
  </div>
</template>

<style scoped>
.liquid-tank-wrap {
  display: flex;
  align-items: center;
  gap: 4px;
  margin-top: 2px;
  z-index: 10;
  padding: 0 4px;
}

/* 表情图标 */
.tank-emoji {
  font-size: 14px;
  line-height: 1;
  filter: drop-shadow(0 1px 2px rgba(0,0,0,0.5));
  animation: emojiBob 2s ease-in-out infinite;
  flex-shrink: 0;
}

@keyframes emojiBob {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-2px); }
}

/* 容器 */
.tank-container {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

.liquid-tank {
  display: block;
  overflow: visible;
}

/* 背景过渡 */
.tank-bg {
  transition: fill 0.6s ease;
}

/* 边框发光过渡 */
.tank-stroke {
  transition: stroke 0.6s ease, filter 0.6s ease;
}

/* 波浪动画 */
.wave {
  will-change: transform;
}
.wave-front {
  animation: waveShift 2.5s linear infinite;
}
.wave-back {
  animation: waveShift 3.5s linear infinite reverse;
}

/* 高光跟随波浪 */
.wave-highlight {
  will-change: transform;
  animation: waveShift 2.5s linear infinite;
}

@keyframes waveShift {
  from { transform: translateX(0); }
  to   { transform: translateX(-56px); }
}

/* 百分比数字 */
.tank-percent {
  position: absolute;
  font-family: 'SF Mono', 'Cascadia Code', ui-monospace, monospace;
  font-size: 11px;
  font-weight: 800;
  letter-spacing: 0.5px;
  text-shadow: 0 1px 3px rgba(0,0,0,0.8), 0 0 6px rgba(0,0,0,0.5);
  pointer-events: none;
  z-index: 5;
  transition: color 0.5s ease;
}

/* 按状态给数字加颜色倾向 */
.pct-fresh   { color: #ECFDF5; }
.pct-flow    { color: #EFF6FF; }
.pct-warning { color: #FFFBEB; }
.pct-panic   { color: #FEF2F2; }
.pct-dead    { color: #D1D5DB; }
</style>
