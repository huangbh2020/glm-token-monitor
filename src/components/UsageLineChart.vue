<script setup lang="ts">
import { computed } from 'vue'
import type { ModelUsageData } from '../composables/useModelUsage'

const props = defineProps<{
  data: ModelUsageData
}>()

// 按天聚合
const dailyTotals = computed(() => {
  const { x_time, tokensUsage } = props.data
  const dateMap = new Map<string, number>()
  x_time.forEach((time, i) => {
    const date = time.split(' ')[0]
    dateMap.set(date, (dateMap.get(date) || 0) + (tokensUsage[i] || 0))
  })
  const result: { date: string; label: string; value: number }[] = []
  dateMap.forEach((value, date) => {
    const d = new Date(date)
    result.push({
      date,
      label: `${d.getMonth() + 1}/${d.getDate()}`,
      value,
    })
  })
  return result
})

const maxVal = computed(() => Math.max(...dailyTotals.value.map(d => d.value), 1))

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
  const n = dailyTotals.value.length
  if (n === 0) return ''
  const step = n > 1 ? chartW / (n - 1) : 0
  return dailyTotals.value
    .map((d, i) => {
      const x = padX + i * step
      const y = padY + chartH - (d.value / maxVal.value) * chartH
      return `${x},${y}`
    })
    .join(' ')
})

const areaPath = computed(() => {
  const n = dailyTotals.value.length
  if (n === 0) return ''
  const step = n > 1 ? chartW / (n - 1) : 0
  const linePart = dailyTotals.value
    .map((d, i) => {
      const x = padX + i * step
      const y = padY + chartH - (d.value / maxVal.value) * chartH
      return `${i === 0 ? 'M' : 'L'}${x},${y}`
    })
    .join(' ')
  const lastX = padX + (n - 1) * step
  return `${linePart} L${lastX},${padY + chartH} L${padX},${padY + chartH} Z`
})

const dotPositions = computed(() => {
  const n = dailyTotals.value.length
  if (n === 0) return []
  const step = n > 1 ? chartW / (n - 1) : 0
  return dailyTotals.value.map((d, i) => ({
    x: padX + i * step,
    y: padY + chartH - (d.value / maxVal.value) * chartH,
    label: d.label,
    value: d.value,
  }))
})

// X 轴标签（每隔几个显示一个）
const xLabels = computed(() => {
  const n = dailyTotals.value.length
  if (n <= 10) return dotPositions.value
  const step = Math.ceil(n / 8)
  return dotPositions.value.filter((_, i) => i % step === 0 || i === n - 1)
})

// Y 轴标签
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
  <div class="line-chart">
    <svg :width="svgWidth" :height="svgHeight" class="line-svg">
      <!-- Y 轴标签 -->
      <text
        v-for="(yl, i) in yLabels"
        :key="'y' + i"
        :x="padX - 4"
        :y="yl.y + 3"
        text-anchor="end"
        class="axis-text"
      >{{ yl.label }}</text>

      <!-- 网格线 -->
      <line
        v-for="(yl, i) in yLabels"
        :key="'g' + i"
        :x1="padX"
        :y1="yl.y"
        :x2="padX + chartW"
        :y2="yl.y"
        class="grid-line"
      />

      <!-- 面积填充 -->
      <path :d="areaPath" class="area-fill" />

      <!-- 折线 -->
      <polyline :points="points" class="line-stroke" />

      <!-- 数据点 -->
      <circle
        v-for="(dot, i) in dotPositions"
        :key="'d' + i"
        :cx="dot.x"
        :cy="dot.y"
        r="3"
        class="dot"
      >
        <title>{{ dot.label }} - {{ formatTokens(dot.value) }} tokens</title>
      </circle>

      <!-- X 轴标签 -->
      <text
        v-for="(xl, i) in xLabels"
        :key="'x' + i"
        :x="xl.x"
        :y="padY + chartH + 14"
        text-anchor="middle"
        class="axis-text"
      >{{ xl.label }}</text>
    </svg>
  </div>
</template>

<style scoped>
.line-chart {
  overflow-x: auto;
  padding: 4px 0;
}

.line-svg {
  display: block;
}

.grid-line {
  stroke: var(--grid-color, rgba(255, 255, 255, 0.06));
  stroke-width: 1;
  stroke-dasharray: 2 2;
}

.area-fill {
  fill: var(--area-color, rgba(57, 211, 83, 0.15));
}

.line-stroke {
  fill: none;
  stroke: var(--line-color, #39d353);
  stroke-width: 2;
  stroke-linecap: round;
  stroke-linejoin: round;
}

.dot {
  fill: var(--line-color, #39d353);
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
  --line-color: #39d353;
  --area-color: rgba(57, 211, 83, 0.15);
  --grid-color: rgba(255, 255, 255, 0.06);
}

[data-theme="light"] {
  --line-color: #216e39;
  --area-color: rgba(33, 110, 57, 0.12);
  --grid-color: rgba(0, 0, 0, 0.06);
}
</style>
