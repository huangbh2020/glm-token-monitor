<script setup lang="ts">
import { ref } from 'vue'
import TodoItem from './TodoItem.vue'
import type { TodoItem as TodoItemType } from '../../composables/useTodo'

const props = defineProps<{
  items: TodoItemType[]
}>()

const emit = defineEmits<{
  toggle: [id: string]
  delete: [id: string]
  add: [text: string, isDone: boolean]
}>()

const newTodoText = ref('')

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter' && !e.shiftKey && newTodoText.value.trim()) {
    e.preventDefault()
    const text = newTodoText.value.trim()
    const isTodo = text.startsWith('~')
    const cleanedText = isTodo ? text.slice(1).trim() : text
    if (!cleanedText) return
    emit('add', cleanedText, !isTodo)
    newTodoText.value = ''
  }
}
</script>

<template>
  <div class="todo-list">
    <div class="items-container">
      <TransitionGroup name="list" tag="div" class="items">
        <TodoItem
          v-for="item in items"
          :key="item.id"
          :item="item"
          @toggle="(id: string) => emit('toggle', id)"
          @delete="(id: string) => emit('delete', id)"
        />
      </TransitionGroup>
      <div v-if="items.length === 0" class="empty-state">
        <span>暂无记录</span>
      </div>
    </div>
    <div class="input-area">
      <textarea
        v-model="newTodoText"
        class="todo-input"
        placeholder="输入内容，回车添加（Shift+回车换行，~ 开头为待办）"
        rows="2"
        @keydown="handleKeydown"
      />
    </div>
  </div>
</template>

<style scoped>
.todo-list {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.items-container {
  flex: 1;
  overflow-y: auto;
  padding: 0 10px;
}

.items {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.empty-state {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 32px 0;
  color: #52525b;
  font-size: 11px;
}

.input-area {
  padding: 8px 10px;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.todo-input {
  width: 100%;
  padding: 8px 12px;
  background: rgba(255, 255, 255, 0.9);
  border: 1px solid #000000;
  border-radius: 8px;
  color: #000000;
  font-size: 12px;
  line-height: 1.5;
  outline: none;
  resize: none;
  transition: border-color 0.15s ease;
  box-sizing: border-box;
  font-family: inherit;
}

.todo-input:focus {
  border-color: #3b82f6;
  background: #ffffff;
}

.todo-input::placeholder {
  color: #666666;
}

.list-enter-active,
.list-leave-active {
  transition: all 0.2s ease;
}

.list-enter-from {
  opacity: 0;
  transform: translateY(-8px);
}

.list-leave-to {
  opacity: 0;
  transform: translateX(20px);
}
</style>
