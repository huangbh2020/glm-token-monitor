<script setup lang="ts">
defineProps<{
  currentDate: string
}>()

defineEmits<{
  change: [date: string]
}>()

function shiftDate(dateStr: string, days: number): string {
  const d = new Date(dateStr)
  d.setDate(d.getDate() + days)
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`
}

function getTodayString(): string {
  const now = new Date()
  return `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-${String(now.getDate()).padStart(2, '0')}`
}

function formatDisplay(dateStr: string): string {
  const d = new Date(dateStr)
  const today = getTodayString()
  const weekDays = ['日', '一', '二', '三', '四', '五', '六']
  const dayLabel = dateStr === today ? '今天' : `周${weekDays[d.getDay()]}`
  return `${d.getMonth() + 1}/${d.getDate()} ${dayLabel}`
}
</script>

<template>
  <div class="day-switcher">
    <button class="nav-btn" @click="$emit('change', shiftDate(currentDate, -1))">
      <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
        <polyline points="15 18 9 12 15 6"></polyline>
      </svg>
    </button>
    <span class="date-display">{{ formatDisplay(currentDate) }}</span>
    <button class="nav-btn" @click="$emit('change', shiftDate(currentDate, 1))">
      <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
        <polyline points="9 6 15 12 9 18"></polyline>
      </svg>
    </button>
    <button v-if="currentDate !== getTodayString()" class="today-btn" @click="$emit('change', getTodayString())">
      今天
    </button>
  </div>
</template>

<style scoped>
.day-switcher {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 10px;
}

.nav-btn {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.05);
  border: none;
  color: #a1a1aa;
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.15s ease;
}

.nav-btn:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #e4e4e7;
}

.date-display {
  flex: 1;
  text-align: center;
  font-size: 12px;
  font-weight: 600;
  color: #d4d4d8;
}

.today-btn {
  padding: 2px 8px;
  font-size: 10px;
  background: rgba(59, 130, 246, 0.15);
  border: 1px solid rgba(59, 130, 246, 0.25);
  border-radius: 10px;
  color: #60a5fa;
  cursor: pointer;
  transition: all 0.15s ease;
}

.today-btn:hover {
  background: rgba(59, 130, 246, 0.25);
}
</style>
