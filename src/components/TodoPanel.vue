<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useTodo } from '../composables/useTodo'
import { useTheme } from '../composables/useTheme'
import DaySwitcher from './todo/DaySwitcher.vue'
import TodoList from './todo/TodoList.vue'

const { currentTheme, initTheme } = useTheme()
const {
  todayItems,
  currentDate,
  loading,
  addItem,
  toggleItem,
  deleteItem,
  copyDayText,
  copyWeekText,
  switchDate,
  setupListener,
} = useTodo()

async function handleCopyDay() {
  const text = await copyDayText()
  if (text) {
    await navigator.clipboard.writeText(text)
  }
}

async function handleCopyWeek() {
  const text = await copyWeekText()
  if (text) {
    await navigator.clipboard.writeText(text)
  }
}

async function closeWindow() {
  try {
    await invoke('close_todo_panel')
  } catch (err) {
    console.error('Close todo panel failed:', err)
  }
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    closeWindow()
  }
}

let unlisten: (() => void) | undefined

onMounted(async () => {
  await initTheme()
  setupListener().then((fn) => { unlisten = fn })
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  unlisten?.()
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
  <div class="todo-panel" :data-theme="currentTheme">
    <!-- 顶部操作栏 -->
    <div class="header">
      <span class="title">工作日志</span>
      <div class="actions">
        <button class="action-btn close" @click="closeWindow">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
            <path d="M18 6L6 18M6 6l12 12"/>
          </svg>
        </button>
      </div>
    </div>

    <!-- 日期切换 -->
    <DaySwitcher :current-date="currentDate" @change="switchDate" />

    <!-- 任务列表 -->
    <TodoList
      :items="todayItems"
      @toggle="toggleItem"
      @delete="deleteItem"
      @add="(text: string, isDone: boolean) => addItem(text, isDone)"
    />

    <!-- 底部操作栏 -->
    <div class="footer">
      <button class="copy-btn" @click="handleCopyDay">复制今日</button>
      <button class="copy-btn" @click="handleCopyWeek">复制本周</button>
    </div>
  </div>
</template>

<style scoped>
.todo-panel {
  width: 100vw;
  height: 100vh;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: rgba(15, 15, 17, 0.92);
  color: #e4e4e7;
  display: flex;
  flex-direction: column;
  border-radius: 12px;
  overflow: hidden;
  backdrop-filter: blur(12px);
  border: 1px solid rgba(255, 255, 255, 0.06);
}

.todo-panel[data-theme="light"] {
  background: rgba(255, 255, 255, 0.95);
  color: #1c1c1e;
  border-color: rgba(0, 0, 0, 0.06);
}

.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 14px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.todo-panel[data-theme="light"] .header {
  border-bottom-color: rgba(0, 0, 0, 0.05);
}

.title {
  font-size: 13px;
  font-weight: 600;
  color: #d4d4d8;
}

.todo-panel[data-theme="light"] .title {
  color: #1c1c1e;
}

.actions {
  display: flex;
  gap: 6px;
}

.action-btn {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: #71717a;
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.15s ease;
}

.action-btn:hover {
  background: rgba(255, 255, 255, 0.08);
  color: #d4d4d8;
}

.todo-panel[data-theme="light"] .action-btn:hover {
  background: rgba(0, 0, 0, 0.04);
  color: #404040;
}

.action-btn.close:hover {
  background: rgba(239, 68, 68, 0.15);
  color: #f87171;
}

.footer {
  display: flex;
  gap: 8px;
  padding: 8px 10px;
  border-top: 1px solid rgba(255, 255, 255, 0.06);
}

.todo-panel[data-theme="light"] .footer {
  border-top-color: rgba(0, 0, 0, 0.06);
}

.copy-btn {
  flex: 1;
  padding: 7px 0;
  background: rgba(59, 130, 246, 0.12);
  border: 1px solid rgba(59, 130, 246, 0.2);
  border-radius: 8px;
  color: #60a5fa;
  font-size: 11px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s ease;
}

.todo-panel[data-theme="light"] .copy-btn {
  background: rgba(59, 130, 246, 0.08);
  color: #3b82f6;
}

.copy-btn:hover {
  background: rgba(59, 130, 246, 0.2);
}

.copy-btn:active {
  transform: scale(0.97);
}
</style>
