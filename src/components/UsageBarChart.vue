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

const barHeight = 80
const barWidth = 10
const gap = 2

const bars = computed(() =>
  hours.value.map((val, i) => ({
    hour: i,
    value: val,
    height: Math.max(2, (val / maxVal.value) * barHeight),
    color: val === 0 ? 'var(--bar-empty)' : 'var(--bar-fill)',
  }))
)
</script>

<template>
  <div class="bar-chart">
    <svg :width="24 * (barWidth + gap)" :height="barHeight + 20" class="bar-svg">
      <!-- 柱子 -->
      <rect
        v-for="b in bars"
        :key="b.hour"
        :x="b.hour * (barWidth + gap)"
        :y="barHeight - b.height"
        :width="barWidth"
        :height="b.height"
        :fill="b.color"
        rx="2"
      >
        <title>{{ b.hour }}:00 - {{ formatTokens(b.value) }} tokens</title>
      </rect>
      <!-- 小时标签 -->
      <text
        v-for="h in [0, 3, 6, 9, 12, 15, 18, 21]"
        :key="h"
        :x="h * (barWidth + gap) + barWidth / 2"
        :y="barHeight + 14"
        text-anchor="middle"
        class="hour-text"
      >{{ h }}</text>
    </svg>
  </div>
</template>

<style scoped>
.bar-chart {
  overflow-x: auto;
  padding: 4px 0;
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

.hour-text {
  font-size: 8px;
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
