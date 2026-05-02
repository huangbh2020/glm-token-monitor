<script setup lang="ts">
import { computed } from 'vue'
import type { ModelUsageData } from '../composables/useModelUsage'

const props = defineProps<{
  data: ModelUsageData
}>()

const hours = computed(() => props.data.tokensUsage.slice(0, 24))

const maxVal = computed(() => Math.max(...hours.value, 1))

function formatTokens(value: number): string {
  if (value === 0) return '0'
  if (value >= 1_000_000) return `${(value / 1_000_000).toFixed(1)}M`
  if (value >= 1_000) return `${(value / 1_000).toFixed(1)}K`
  return String(value)
}

const svgWidth = 290
const svgHeight = 100
const padX = 36
const padY = 8
const padBottom = 20
const chartW = svgWidth - padX - 8
const chartH = svgHeight - padY - padBottom

const points = computed(() => {
  const n = hours.value.length
  if (n === 0) return ''
  const step = chartW / (n - 1)
  return hours.value
    .map((val, i) => {
      const x = padX + i * step
      const y = padY + chartH - (val / maxVal.value) * chartH
      return `${x},${y}`
    })
    .join(' ')
})

const areaPath = computed(() => {
  const n = hours.value.length
  if (n === 0) return ''
  const step = chartW / (n - 1)
  const linePart = hours.value
    .map((val, i) => {
      const x = padX + i * step
      const y = padY + chartH - (val / maxVal.value) * chartH
      return `${i === 0 ? 'M' : 'L'}${x},${y}`
    })
    .join(' ')
  const lastX = padX + (n - 1) * step
  return `${linePart} L${lastX},${padY + chartH} L${padX},${padY + chartH} Z`
})

const dotPositions = computed(() => {
  const n = hours.value.length
  if (n === 0) return []
  const step = chartW / (n - 1)
  return hours.value.map((val, i) => ({
    x: padX + i * step,
    y: padY + chartH - (val / maxVal.value) * chartH,
    hour: i,
    value: val,
  }))
})

const xLabels = computed(() =>
  dotPositions.value.filter(d => d.hour % 3 === 0)
)

const yLabels = computed(() => {
  const count = 3
  return Array.from({ length: count + 1 }, (_, i) => {
    const val = (maxVal.value / count) * i
    return {
      y: padY + chartH - (val / maxVal.value) * chartH,
      label: formatTokens(val),
    }
  })
})
</script>

<template>
  <div class="area-chart">
    <svg :width="svgWidth" :height="svgHeight" class="area-svg">
      <text
        v-for="(yl, i) in yLabels"
        :key="'y' + i"
        :x="padX - 4"
        :y="yl.y + 3"
        text-anchor="end"
        class="axis-text"
      >{{ yl.label }}</text>

      <line
        v-for="(yl, i) in yLabels"
        :key="'g' + i"
        :x1="padX"
        :y1="yl.y"
        :x2="padX + chartW"
        :y2="yl.y"
        class="grid-line"
      />

      <path :d="areaPath" class="area-fill" />
      <polyline :points="points" class="line-stroke" />

      <circle
        v-for="(dot, i) in dotPositions"
        :key="'d' + i"
        :cx="dot.x"
        :cy="dot.y"
        r="3"
        class="dot"
      >
        <title>{{ dot.hour }}:00 - {{ formatTokens(dot.value) }} tokens</title>
      </circle>

      <text
        v-for="(xl, i) in xLabels"
        :key="'x' + i"
        :x="xl.x"
        :y="padY + chartH + 14"
        text-anchor="middle"
        class="axis-text"
      >{{ xl.hour }}</text>
    </svg>
  </div>
</template>

<style scoped>
.area-chart {
  overflow-x: auto;
  overflow-y: hidden;
  padding: 4px 0;
  scrollbar-width: none;
}

.area-chart::-webkit-scrollbar {
  display: none;
}

.area-svg {
  display: block;
}

.grid-line {
  stroke: var(--grid-color, rgba(255, 255, 255, 0.06));
  stroke-width: 1;
  stroke-dasharray: 2 2;
}

.area-fill {
  fill: var(--area-color, rgba(96, 165, 250, 0.15));
}

.line-stroke {
  fill: none;
  stroke: var(--line-color, #60a5fa);
  stroke-width: 2;
  stroke-linecap: round;
  stroke-linejoin: round;
}

.dot {
  fill: var(--line-color, #60a5fa);
  cursor: pointer;
  transition: r 0.15s;
}

.dot:hover {
  r: 5;
}

.axis-text {
  font-size: 8px;
  fill: var(--text-muted, #71717a);
  font-family: ui-monospace, monospace;
}

:root {
  --line-color: #60a5fa;
  --area-color: rgba(96, 165, 250, 0.15);
  --grid-color: rgba(255, 255, 255, 0.06);
}

[data-theme="light"] {
  --line-color: #2563eb;
  --area-color: rgba(37, 99, 235, 0.12);
  --grid-color: rgba(0, 0, 0, 0.06);
}
</style>
