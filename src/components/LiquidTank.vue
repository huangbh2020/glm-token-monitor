<script setup lang="ts">
import { computed } from 'vue'
import type { PetState } from '../composables/useUsageState'

const props = defineProps<{
  percent: number   // 剩余百分比 0-100
  state: PetState
}>()

// 电池尺寸
const batteryW = 60
const batteryH = 16
const nubW = 2
const nubH = 6
const borderR = 3
const innerPad = 1.5

// 百分比
const pct = computed(() => Math.round(Math.max(0, Math.min(100, props.percent))))

// 填充宽度
const fillW = computed(() => {
  const maxW = batteryW - innerPad * 2
  return (pct.value / 100) * maxW
})

// 填充颜色
const fillColor = computed(() => {
  if (pct.value <= 20) return '#FF3B30'
  return '#34C759'
})

// 边框颜色
const borderColor = computed(() => {
  if (pct.value <= 10) return 'rgba(255,59,48,0.5)'
  return 'rgba(255,255,255,0.22)'
})

// 文字颜色（深色字在绿底上更清晰）
const textColor = computed(() => {
  if (pct.value <= 20) return '#FFFFFF'
  return 'rgba(0, 0, 0, 0.75)'
})

// 文字阴影（让文字在任何背景上都可读）
const textShadow = computed(() => {
  if (pct.value <= 20) return '0 0.5px 1px rgba(0,0,0,0.5)'
  return '0 0.5px 0 rgba(255,255,255,0.4)'
})
</script>

<template>
  <div class="battery-wrap">
    <div class="battery-container">
      <!-- 电池主体 SVG -->
      <svg
        :width="batteryW + nubW + 2"
        :height="batteryH"
        :viewBox="`0 0 ${batteryW + nubW + 2} ${batteryH}`"
        class="battery-svg"
      >
        <!-- 正极凸起 -->
        <rect
          :x="batteryW + 1"
          :y="(batteryH - nubH) / 2"
          :width="nubW"
          :height="nubH"
          :rx="1"
          fill="rgba(255,255,255,0.18)"
        />

        <!-- 外壳边框 -->
        <rect
          x="0" y="0"
          :width="batteryW"
          :height="batteryH"
          :rx="borderR"
          fill="none"
          :stroke="borderColor"
          stroke-width="1"
          class="battery-border"
        />

        <!-- 内部深色背景 -->
        <rect
          :x="innerPad"
          :y="innerPad"
          :width="batteryW - innerPad * 2"
          :height="batteryH - innerPad * 2"
          :rx="1.5"
          fill="rgba(255,255,255,0.06)"
        />

        <!-- 电量填充 -->
        <rect
          :x="innerPad"
          :y="innerPad"
          :width="fillW"
          :height="batteryH - innerPad * 2"
          :rx="1.5"
          :fill="fillColor"
          class="battery-fill"
        />
      </svg>

      <!-- 百分比文字（居中覆盖在电池上） -->
      <span
        class="battery-pct"
        :style="{ color: textColor, textShadow: textShadow }"
      >
        {{ pct }}%
      </span>
    </div>
  </div>
</template>

<style scoped>
.battery-wrap {
  display: flex;
  justify-content: center;
  margin-top: 2px;
  z-index: 10;
}

.battery-container {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

.battery-svg {
  display: block;
  flex-shrink: 0;
}

.battery-border {
  transition: stroke 0.4s ease;
}

.battery-fill {
  transition: width 0.5s ease, fill 0.4s ease;
}

.battery-pct {
  position: absolute;
  font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', 'Segoe UI', sans-serif;
  font-size: 10px;
  font-weight: 800;
  letter-spacing: 0.2px;
  pointer-events: none;
  z-index: 5;
  transition: color 0.4s ease, text-shadow 0.4s ease;
}
</style>
