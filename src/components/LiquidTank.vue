<script setup lang="ts">
import { computed } from 'vue'
import type { PetState } from '../composables/useUsageState'

const props = defineProps<{
  percent: number   // 剩余百分比 0-100
  state: PetState
}>()

// 电池尺寸（Apple 风格）
const batteryW = 52
const batteryH = 12
const nubW = 2
const nubH = 5
const borderR = 2.5
const innerPad = 1.5  // 内部填充间距

// 百分比
const pct = computed(() => Math.round(Math.max(0, Math.min(100, props.percent))))

// 填充宽度
const fillW = computed(() => {
  const maxW = batteryW - innerPad * 2
  return (pct.value / 100) * maxW
})

// Apple 风格颜色：
// >=20% → 绿色, <20% → 红色
const fillColor = computed(() => {
  if (pct.value <= 20) return '#FF3B30'   // Apple 红
  if (pct.value <= 50) return '#34C759'   // Apple 绿（低电量也是绿，但你可以改黄）
  return '#34C759'                         // Apple 绿
})

// 低电量时背景变红框
const borderColor = computed(() => {
  if (pct.value <= 10) return 'rgba(255,59,48,0.6)'
  return 'rgba(255,255,255,0.25)'
})
</script>

<template>
  <div class="battery-wrap">
    <!-- 电池主体 -->
    <svg
      :width="batteryW + nubW + 2"
      :height="batteryH"
      :viewBox="`0 0 ${batteryW + nubW + 2} ${batteryH}`"
      class="battery-svg"
    >
      <!-- 电池正极凸起（右侧小块） -->
      <rect
        :x="batteryW + 1"
        :y="(batteryH - nubH) / 2"
        :width="nubW"
        :height="nubH"
        :rx="1"
        fill="rgba(255,255,255,0.2)"
      />

      <!-- 电池外壳 -->
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

      <!-- 电池内部背景（深色底） -->
      <rect
        :x="innerPad"
        :y="innerPad"
        :width="batteryW - innerPad * 2"
        :height="batteryH - innerPad * 2"
        :rx="1"
        fill="rgba(255,255,255,0.08)"
      />

      <!-- 电量填充 -->
      <rect
        :x="innerPad"
        :y="innerPad"
        :width="fillW"
        :height="batteryH - innerPad * 2"
        :rx="1"
        :fill="fillColor"
        class="battery-fill"
      />
    </svg>

    <!-- 百分比文字（电池右侧） -->
    <span class="battery-pct" :class="{ low: pct <= 20 }">{{ pct }}%</span>
  </div>
</template>

<style scoped>
.battery-wrap {
  display: flex;
  align-items: center;
  gap: 4px;
  justify-content: center;
  margin-top: 2px;
  z-index: 10;
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
  font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', 'Segoe UI', sans-serif;
  font-size: 10px;
  font-weight: 600;
  color: rgba(255, 255, 255, 0.7);
  letter-spacing: -0.1px;
  transition: color 0.4s ease;
  white-space: nowrap;
}

.battery-pct.low {
  color: #FF3B30;
}
</style>
