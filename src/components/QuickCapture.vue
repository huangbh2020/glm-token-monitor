<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'

const visible = ref(false)
const inputText = ref('')
const inputRef = ref<HTMLInputElement | null>(null)

async function handleSubmit() {
  const text = inputText.value.trim()
  if (!text) return

  const isTodo = text.startsWith('~')
  const cleanedText = isTodo ? text.slice(1).trim() : text
  if (!cleanedText) return

  try {
    await invoke('todo_add_item', {
      text: cleanedText,
      isDone: !isTodo,
    })
  } catch (e) {
    console.error('Quick capture add failed:', e)
  }

  inputText.value = ''
  visible.value = false
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    visible.value = false
    inputText.value = ''
  }
}

let unlisten: (() => void) | undefined

onMounted(async () => {
  unlisten = await listen('show-quick-capture', () => {
    visible.value = true
    setTimeout(() => inputRef.value?.focus(), 50)
  })
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  unlisten?.()
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
  <Teleport to="body">
    <Transition name="capture">
      <div v-if="visible" class="quick-capture-overlay" @click.self="visible = false">
        <div class="quick-capture">
          <div class="capture-icon">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="10"/>
              <path d="M12 8v8M8 12h8"/>
            </svg>
          </div>
          <input
            ref="inputRef"
            v-model="inputText"
            class="capture-input"
            placeholder="记录完成的事项（~ 开头为待办）"
            @keydown.enter="handleSubmit"
            @keydown.escape="visible = false"
          />
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.quick-capture-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding-top: 25vh;
  z-index: 9999;
  background: rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(4px);
}

.quick-capture {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 16px;
  background: #1e1e23;
  border: 1px solid rgba(255, 255, 255, 0.25);
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
  width: 320px;
}

.capture-icon {
  color: #60a5fa;
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

.capture-input {
  flex: 1;
  background: transparent;
  border: none;
  color: #ffffff;
  font-size: 13px;
  outline: none;
}

.capture-input::placeholder {
  color: #888888;
}

/* Transition */
.capture-enter-active,
.capture-leave-active {
  transition: all 0.2s ease;
}

.capture-enter-from,
.capture-leave-to {
  opacity: 0;
}

.capture-enter-from .quick-capture,
.capture-leave-to .quick-capture {
  transform: scale(0.95) translateY(-10px);
}
</style>
