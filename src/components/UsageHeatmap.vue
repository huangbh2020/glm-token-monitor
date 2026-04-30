<script setup lang="ts">
import { computed } from 'vue'
import type { ModelUsageData, TimeRange } from '../composables/useModelUsage'

const props = defineProps<{
  data: ModelUsageData
  timeRange: TimeRange
}>()

// 将数据按天分组
const dailyData = computed(() => {
  const { x_time, tokensUsage: tokens_usage } = props.data
  console.log('[Heatmap] timeRange:', props.timeRange, 'x_time:', x_time?.length, 'tokensUsage:', tokens_usage?.length)
  console.log('[Heatmap] x_time[0]:', x_time?.[0], 'tokensUsage[0]:', tokens_usage?.[0])
  const days: { label: string; hours: number[] }[] = []

  if (props.timeRange === 'today') {
    const today = new Date()
    const label = `${today.getMonth() + 1}/${today.getDate()}`
    days.push({ label, hours: tokens_usage.slice(0, 24) })
  } else {
    const dateMap = new Map<string, number[]>()
    x_time.forEach((time, i) => {
      const date = time.split(' ')[0]
      if (!dateMap.has(date)) {
        dateMap.set(date, new Array(24).fill(0))
      }
      const hour = parseInt(time.split(' ')[1]?.split(':')[0] || '0')
      dateMap.get(date)![hour] = tokens_usage[i] || 0
    })

    dateMap.forEach((hours, date) => {
      const d = new Date(date)
      const label = `${d.getMonth() + 1}/${d.getDate()}`
      days.push({ label, hours })
    })
  }

  return days
})

const maxUsage = computed(() => {
  let max = 0
  dailyData.value.forEach(day => {
    day.hours.forEach(v => { if (v > max) max = v })
  })
  return max || 1
})

function getColor(value: number): string {
  if (value === 0) return 'var(--heatmap-empty)'
  const ratio = value / maxUsage.value
  if (ratio < 0.25) return 'var(--heatmap-low)'
  if (ratio < 0.5) return 'var(--heatmap-mid)'
  if (ratio < 0.75) return 'var(--heatmap-high)'
  return 'var(--heatmap-max)'
}

function formatTokens(value: number): string {
  if (value === 0) return '0'
  if (value >= 1_000_000) return `${(value / 1_000_000).toFixed(1)}M`
  if (value >= 1_000) return `${(value / 1_000).toFixed(1)}K`
  return String(value)
}

const hourLabels = ['0', '', '', '3', '', '', '6', '', '', '9', '', '', '12', '', '', '15', '', '', '18', '', '', '21', '', '']
</script>

<template>
  <div class="heatmap-container">
    <div v-if="dailyData.length === 0" class="empty-hint">暂无数据</div>
    <template v-else>
      <div class="hour-labels">
        <span class="day-label-spacer"></span>
        <span v-for="(h, i) in hourLabels" :key="i" class="hour-label">{{ h }}</span>
      </div>
      <div class="heatmap-grid" :class="`range-${timeRange}`">
        <div v-for="(day, di) in dailyData" :key="di" class="heatmap-row">
          <span class="day-label">{{ day.label }}</span>
          <span
            v-for="(val, hi) in day.hours"
            :key="hi"
            class="heatmap-cell"
            :style="{ background: getColor(val) }"
            :title="`${day.label} ${hi}:00 - ${formatTokens(val)} tokens`"
          ></span>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.heatmap-container {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.empty-hint {
  text-align: center;
  padding: 20px;
  color: #71717a;
  font-size: 11px;
}

.hour-labels {
  display: flex;
  align-items: center;
  gap: 2px;
  margin-bottom: 2px;
}

.day-label-spacer {
  width: 32px;
  flex-shrink: 0;
}

.hour-label {
  width: 8px;
  font-size: 8px;
  color: var(--text-muted, #71717a);
  text-align: center;
  line-height: 1;
}

.heatmap-grid {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.heatmap-grid.range-30days {
  max-height: 200px;
  overflow-y: auto;
  padding-right: 4px;
}

.heatmap-row {
  display: flex;
  align-items: center;
  gap: 2px;
}

.day-label {
  width: 32px;
  font-size: 9px;
  color: var(--text-muted, #71717a);
  text-align: right;
  padding-right: 4px;
  flex-shrink: 0;
  font-family: ui-monospace, monospace;
}

.heatmap-cell {
  width: 8px;
  height: 8px;
  border-radius: 2px;
  cursor: pointer;
  transition: transform 0.1s ease;
  flex-shrink: 0;
}

.heatmap-cell:hover {
  transform: scale(1.4);
  outline: 1px solid rgba(255, 255, 255, 0.3);
}

:root {
  --heatmap-empty: #161b22;
  --heatmap-low: #0e4429;
  --heatmap-mid: #006d32;
  --heatmap-high: #26a641;
  --heatmap-max: #39d353;
}

[data-theme="light"] {
  --heatmap-empty: #ebedf0;
  --heatmap-low: #9be9a8;
  --heatmap-mid: #40c463;
  --heatmap-high: #30a14e;
  --heatmap-max: #216e39;
}
</style>
