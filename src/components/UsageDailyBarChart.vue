<script setup lang="ts">
import { computed } from 'vue'
import type { ModelUsageData } from '../composables/useModelUsage'

const props = defineProps<{
  data: ModelUsageData
}>()

interface DayData {
  label: string
  total: number
}

const dailyData = computed<DayData[]>(() => {
  const { x_time, tokensUsage } = props.data
  const dateMap = new Map<string, number>()

  x_time.forEach((time, i) => {
    const date = time.split(' ')[0]
    const current = dateMap.get(date) || 0
    dateMap.set(date, current + (tokensUsage[i] || 0))
  })

  const days: DayData[] = []
  dateMap.forEach((total, date) => {
    const d = new Date(date)
    days.push({
      label: `${d.getMonth() + 1}/${d.getDate()}`,
      total,
    })
  })
  return days
})

const maxVal = computed(() => Math.max(...dailyData.value.map(d => d.total), 1))

const barHeight = 100
const barWidth = 32
const gap = 8

const bars = computed(() =>
  dailyData.value.map((d, i) => ({
    ...d,
    x: i * (barWidth + gap),
    height: Math.max(2, (d.total / maxVal.value) * barHeight),
    y: barHeight - Math.max(2, (d.total / maxVal.value) * barHeight),
  }))
)

const svgWidth = computed(() => dailyData.value.length * (barWidth + gap) - gap)

function formatTokens(value: number): string {
  if (value === 0) return '0'
  if (value >= 1_000_000) return `${(value / 1_000_000).toFixed(1)}M`
  if (value >= 1_000) return `${(value / 1_000).toFixed(1)}K`
  return String(value)
}
</script>

<template>
  <div class="daily-bar-chart">
    <div v-if="dailyData.length === 0" class="empty-hint">暂无数据</div>
    <svg v-else :width="svgWidth" :height="barHeight + 24" class="bar-svg">
      <rect
        v-for="(b, i) in bars"
        :key="i"
        :x="b.x"
        :y="b.y"
        :width="barWidth"
        :height="b.height"
        :fill="b.total === 0 ? 'var(--bar-empty)' : 'var(--bar-fill)'"
        rx="3"
      >
        <title>{{ b.label }} - {{ formatTokens(b.total) }} tokens</title>
      </rect>
      <!-- 数值标签 -->
      <text
        v-for="(b, i) in bars"
        :key="'val-' + i"
        :x="b.x + barWidth / 2"
        :y="b.y - 4"
        text-anchor="middle"
        class="val-text"
      >{{ formatTokens(b.total) }}</text>
      <!-- 日期标签 -->
      <text
        v-for="(b, i) in bars"
        :key="'label-' + i"
        :x="b.x + barWidth / 2"
        :y="barHeight + 14"
        text-anchor="middle"
        class="day-text"
      >{{ b.label }}</text>
    </svg>
  </div>
</template>

<style scoped>
.daily-bar-chart {
  overflow-x: auto;
  padding: 4px 0;
}

.empty-hint {
  text-align: center;
  padding: 20px;
  color: #71717a;
  font-size: 11px;
}

.bar-svg {
  display: block;
}

rect {
  cursor: pointer;
  transition: opacity 0.15s;
}

rect:hover {
  opacity: 0.8;
}

.val-text {
  font-size: 8px;
  fill: var(--text-muted, #a1a1aa);
  font-family: ui-monospace, monospace;
}

.day-text {
  font-size: 9px;
  fill: var(--text-muted, #71717a);
  font-family: ui-monospace, monospace;
}

:root {
  --bar-fill: #39d353;
  --bar-empty: #161b22;
}

[data-theme="light"] {
  --bar-fill: #216e39;
  --bar-empty: #ebedf0;
}
</style>
