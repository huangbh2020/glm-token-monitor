<script setup lang="ts">
import { computed } from 'vue'
import type { PetState } from '../composables/useUsageState'

const props = defineProps<{
  percent: number
  state: PetState
}>()

const tankW = 136
const tankH = 20
const pad = 2
const innerW = tankW - pad * 2   // 132
const innerH = tankH - pad * 2   // 16

// 容器边框发光色（由状态驱动）
const borderGlows: Record<PetState, string> = {
  Fresh:   'rgba(52,211,153,0.4)',
  Flow:    'rgba(96,165,250,0.4)',
  Warning: 'rgba(251,191,36,0.4)',
  Panic:   'rgba(248,113,113,0.5)',
  Dead:    'rgba(107,114,128,0.2)',
}

// 波浪振幅（由状态驱动）
const stateAmplitudes: Record<PetState, number> = {
  Fresh: 2, Flow: 2, Warning: 1.5, Panic: 3, Dead: 0.5,
}

// 动画速度（由状态驱动）
const stateDurations: Record<PetState, string> = {
  Fresh: '3s', Flow: '2.5s', Warning: '2s', Panic: '1s', Dead: '6s',
}

const glow      = computed(() => borderGlows[props.state])
const amplitude = computed(() => stateAmplitudes[props.state])
const duration  = computed(() => stateDurations[props.state])

// ─── 液面颜色：根据剩余百分比连续渐变 ───
// 100% → 绿色(hue 140)，50% → 黄色(hue 50)，0% → 红色(hue 0)
const liquidHue = computed(() => {
  const p = Math.max(0, Math.min(100, props.percent))
  // 非线性映射：低百分比时更快变红
  const t = p / 100
  return t * t * 100 + t * 40  // 0→0, 50→45, 100→140
})

const liquidColorTop    = computed(() => `hsl(${liquidHue.value}, 80%, 55%)`)
const liquidColorBottom = computed(() => `hsl(${liquidHue.value}, 70%, 40%)`)
const liquidColorMid    = computed(() => `hsl(${liquidHue.value}, 75%, 48%)`)

// 百分比显示
const percentDisplay = computed(() => Math.round(props.percent))

// 液面高度
const fillHeight = computed(() => {
  const h = (Math.max(0, Math.min(100, props.percent)) / 100) * innerH
  return Math.max(h, 0)
})

// 波浪基准 Y
const waveY = computed(() => tankH - pad - fillHeight.value)

// 生成波浪 SVG path（加宽覆盖范围避免边缘缝隙）
function wavePath(y: number, amp: number): string {
  const a = Math.min(amp, fillHeight.value / 2)
  return [
    `M -6 ${y}`,
    `C 10 ${y - a}, 30 ${y + a}, 50 ${y}`,
    `C 70 ${y - a}, 90 ${y + a}, 110 ${y}`,
    `C 124 ${y - a}, 134 ${y + a}, 142 ${y}`,
    `L 142 ${tankH} L -6 ${tankH} Z`,
  ].join(' ')
}

const waveFrontPath = computed(() => wavePath(waveY.value, amplitude.value))
const waveBackPath  = computed(() => wavePath(waveY.value, amplitude.value * 0.6))

// 文字颜色：液面亮时用深色字，液面暗时用亮色字
const textColor = computed(() => {
  const p = props.percent
  if (p > 60) return 'rgba(255,255,255,0.95)'
  if (p > 30) return 'rgba(255,255,255,0.9)'
  return 'rgba(255,255,255,0.85)'
})
</script>

<template>
  <div class="liquid-tank-wrap">
    <div class="tank-container">
      <svg
        :width="tankW"
        :height="tankH"
        :viewBox="`0 0 ${tankW} ${tankH}`"
        xmlns="http://www.w3.org/2000/svg"
        class="liquid-tank"
      >
        <defs>
          <clipPath id="tank-clip">
            <rect :x="pad" :y="pad" :width="innerW" :height="innerH" :rx="(innerH / 2)" />
          </clipPath>

          <!-- 液面渐变：颜色随 percent 连续变化 -->
          <linearGradient id="liquid-grad" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%"   :stop-color="liquidColorTop" stop-opacity="0.85" />
            <stop offset="50%"  :stop-color="liquidColorMid" />
            <stop offset="100%" :stop-color="liquidColorBottom" />
          </linearGradient>

          <!-- 液面高光 -->
          <linearGradient id="highlight" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%"   stop-color="white" stop-opacity="0.3" />
            <stop offset="100%" stop-color="white" stop-opacity="0" />
          </linearGradient>
        </defs>

        <!-- 容器背景 -->
        <rect
          :x="pad" :y="pad" :width="innerW" :height="innerH"
          :rx="(innerH / 2)"
          fill="rgba(10,10,15,0.9)"
        />

        <!-- 液面（裁剪到胶囊内） -->
        <g clip-path="url(#tank-clip)">
          <path
            :d="waveBackPath"
            fill="url(#liquid-grad)"
            opacity="0.35"
            class="wave wave-back"
            :style="{ animationDuration: `calc(${duration} * 1.5)` }"
          />
          <path
            :d="waveFrontPath"
            fill="url(#liquid-grad)"
            class="wave wave-front"
            :style="{ animationDuration: duration }"
          />
          <!-- 高光条 -->
          <rect
            v-if="fillHeight > 2"
            :x="pad + 6"
            :y="waveY + 1"
            :width="innerW - 12"
            height="1.5"
            rx="0.75"
            fill="url(#highlight)"
            class="wave-highlight"
          />
        </g>

        <!-- 边框 + 发光 -->
        <rect
          :x="pad" :y="pad" :width="innerW" :height="innerH"
          :rx="(innerH / 2)"
          fill="none"
          stroke="rgba(255,255,255,0.12)"
          stroke-width="0.8"
          class="tank-stroke"
          :style="{ filter: `drop-shadow(0 0 3px ${glow})` }"
        />
      </svg>

      <!-- 百分比数字 -->
      <span class="tank-percent" :style="{ color: textColor }">
        {{ percentDisplay }}%
      </span>
    </div>
  </div>
</template>

<style scoped>
.liquid-tank-wrap {
  display: flex;
  justify-content: center;
  margin-top: 2px;
  z-index: 10;
}

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

.tank-stroke {
  transition: filter 0.6s ease;
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
.wave-highlight {
  will-change: transform;
  animation: waveShift 2.5s linear infinite;
}

@keyframes waveShift {
  from { transform: translateX(0); }
  to   { transform: translateX(-56px); }
}

.tank-percent {
  position: absolute;
  font-family: 'SF Mono', 'Cascadia Code', ui-monospace, monospace;
  font-size: 10px;
  font-weight: 800;
  letter-spacing: 0.3px;
  text-shadow: 0 1px 3px rgba(0,0,0,0.9), 0 0 6px rgba(0,0,0,0.6);
  pointer-events: none;
  z-index: 5;
  transition: color 0.8s ease;
}
</style>
