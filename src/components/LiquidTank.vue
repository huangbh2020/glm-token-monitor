<script setup lang="ts">
import { computed } from 'vue'
import type { PetState } from '../composables/useUsageState'

const props = defineProps<{
  percent: number   // 剩余百分比 0-100
  state: PetState
}>()

const tankW = 96
const tankH = 10
const pad = 1
const innerW = tankW - pad * 2   // 94
const innerH = tankH - pad * 2   // 8
const rx = innerH / 2            // 4 → 胶囊圆角

// 容器边框发光（由状态驱动，更柔和）
const borderGlows: Record<PetState, string> = {
  Fresh:   'rgba(52,211,153,0.2)',
  Flow:    'rgba(96,165,250,0.2)',
  Warning: 'rgba(251,191,36,0.2)',
  Panic:   'rgba(248,113,113,0.3)',
  Dead:    'rgba(107,114,128,0.15)',
}
const glow = computed(() => borderGlows[props.state])

// 百分比（剩余）
const percentDisplay = computed(() => Math.round(Math.max(0, Math.min(100, props.percent))))

// 剩余区域宽度
const remainW = computed(() => (percentDisplay.value / 100) * innerW)

// 已用量区域宽度
const usedW = computed(() => innerW - remainW.value)

// ─── 颜色：剩余量颜色随百分比连续变化 ───
// 100% → 绿(hue 140)，50% → 黄(hue 50)，0% → 红(hue 0)
const hue = computed(() => {
  const t = percentDisplay.value / 100
  return t * t * 100 + t * 40
})

const remainColor = computed(() => `hsl(${hue.value}, 72%, 50%)`)
const remainColorLight = computed(() => `hsl(${hue.value}, 65%, 62%)`)
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
          <!-- 胶囊裁剪 -->
          <clipPath id="tank-clip">
            <rect :x="pad" :y="pad" :width="innerW" :height="innerH" :rx="rx" />
          </clipPath>

          <!-- 剩余区域渐变 -->
          <linearGradient id="remain-grad" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%"   :stop-color="remainColorLight" />
            <stop offset="100%" :stop-color="remainColor" />
          </linearGradient>
        </defs>

        <!-- 容器背景（深色底 = 已用量视觉） -->
        <rect
          :x="pad" :y="pad" :width="innerW" :height="innerH"
          :rx="rx"
          fill="rgba(60,60,70,0.85)"
        />

        <!-- 已用量区域（较暗） -->
        <g clip-path="url(#tank-clip)">
          <rect
            :x="pad + remainW"
            :y="pad"
            :width="usedW"
            :height="innerH"
            fill="rgba(30,30,38,0.9)"
          />
        </g>

        <!-- 剩余量区域（亮色渐变，从左侧填充） -->
        <g clip-path="url(#tank-clip)">
          <rect
            :x="pad"
            :y="pad"
            :width="remainW"
            :height="innerH"
            fill="url(#remain-grad)"
            class="remain-bar"
          />
          <!-- 顶部高光条 -->
          <rect
            v-if="remainW > 4"
            :x="pad + 2"
            :y="pad + 1"
            :width="remainW - 4"
            height="1"
            rx="0.5"
            fill="rgba(255,255,255,0.18)"
          />
        </g>

        <!-- 分割线（剩余/已用交界处） -->
        <line
          v-if="remainW > 2 && remainW < innerW - 2"
          :x1="pad + remainW"
          :y1="pad + 1"
          :x2="pad + remainW"
          :y2="pad + innerH - 1"
          stroke="rgba(255,255,255,0.2)"
          stroke-width="0.8"
        />

        <!-- 边框（极简） -->
        <rect
          :x="pad" :y="pad" :width="innerW" :height="innerH"
          :rx="rx"
          fill="none"
          stroke="rgba(255,255,255,0.08)"
          stroke-width="0.5"
          class="tank-stroke"
          :style="{ filter: `drop-shadow(0 0 2px ${glow})` }"
        />
      </svg>

      <!-- 百分比文字（居中） -->
      <span class="tank-percent">{{ percentDisplay }}%</span>
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

/* 剩余量颜色过渡 */
.remain-bar {
  transition: width 0.6s ease, fill 0.6s ease;
}

.tank-stroke {
  transition: filter 0.6s ease;
}

.tank-percent {
  position: absolute;
  font-family: 'SF Mono', 'Cascadia Code', ui-monospace, monospace;
  font-size: 8px;
  font-weight: 700;
  letter-spacing: 0.2px;
  color: rgba(255, 255, 255, 0.85);
  text-shadow: 0 0.5px 1px rgba(0, 0, 0, 0.9);
  pointer-events: none;
  z-index: 5;
}
</style>
