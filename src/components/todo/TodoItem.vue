<script setup lang="ts">
import { ref } from 'vue'
import type { TodoItem } from '../../composables/useTodo'

defineProps<{
  item: TodoItem
}>()

defineEmits<{
  toggle: [id: string]
  delete: [id: string]
}>()

const hovered = ref(false)

function formatTime(timestamp: number | null): string {
  if (!timestamp) return ''
  const d = new Date(timestamp * 1000)
  return `${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}`
}
</script>

<template>
  <div class="todo-item" :class="{ done: item.done }" @mouseenter="hovered = true" @mouseleave="hovered = false">
    <button class="checkbox" :class="{ checked: item.done }" @click="$emit('toggle', item.id)">
      <svg v-if="item.done" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
        <polyline points="20 6 9 17 4 12"></polyline>
      </svg>
    </button>
    <span class="item-text">{{ item.text }}</span>
    <span v-if="item.done && item.done_at" class="item-time">{{ formatTime(item.done_at) }}</span>
    <button class="delete-btn" @click="$emit('delete', item.id)">
      <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
        <path d="M18 6L6 18M6 6l12 12"/>
      </svg>
    </button>
  </div>
</template>

<style scoped>
.todo-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 10px;
  border-radius: 8px;
  transition: background 0.15s ease;
  min-height: 32px;
}

.todo-item:hover {
  background: rgba(255, 255, 255, 0.06);
}

.todo-item.done .item-text {
  color: #555555;
  text-decoration: line-through;
}

.checkbox {
  width: 18px;
  height: 18px;
  border-radius: 50%;
  border: 1.5px solid #000000;
  background: transparent;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  flex-shrink: 0;
  transition: all 0.15s ease;
  color: transparent;
}

.checkbox:hover {
  border-color: #333333;
}

.checkbox.checked {
  border-color: #10B981;
  background: rgba(16, 185, 129, 0.15);
  color: #10B981;
}

.item-text {
  flex: 1;
  font-size: 13px;
  color: #000000;
  line-height: 1.5;
  word-break: break-word;
}

.item-time {
  font-size: 10px;
  color: #555555;
  flex-shrink: 0;
}

.delete-btn {
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: #52525b;
  cursor: pointer;
  border-radius: 4px;
  flex-shrink: 0;
  opacity: 0;
  transition: all 0.15s ease;
}

.todo-item:hover .delete-btn {
  opacity: 1;
}

.delete-btn:hover {
  background: rgba(239, 68, 68, 0.15);
  color: #f87171;
}
</style>
