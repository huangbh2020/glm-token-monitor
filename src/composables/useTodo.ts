import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export interface TodoItem {
  id: string
  text: string
  done: boolean
  done_at: number | null
  created_at: number
}

export interface TodoDay {
  date: string
  items: TodoItem[]
}

export function useTodo() {
  const todayItems = ref<TodoItem[]>([])
  const currentDate = ref(getTodayString())
  const pendingCount = ref(0)
  const loading = ref(false)

  function getTodayString(): string {
    const now = new Date()
    return `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-${String(now.getDate()).padStart(2, '0')}`
  }

  async function loadDay(date?: string) {
    loading.value = true
    try {
      const day = await invoke<TodoDay>('todo_get_day', { date: date || currentDate.value })
      todayItems.value = day.items
      currentDate.value = day.date
    } catch (e) {
      console.error('Failed to load todo day:', e)
    } finally {
      loading.value = false
    }
  }

  async function addItem(text: string, isDone = true, date?: string) {
    try {
      const day = await invoke<TodoDay>('todo_add_item', {
        text,
        isDone,
        date: date || currentDate.value,
      })
      todayItems.value = day.items
      await refreshPendingCount()
    } catch (e) {
      console.error('Failed to add todo item:', e)
    }
  }

  async function toggleItem(id: string, date?: string) {
    try {
      const day = await invoke<TodoDay>('todo_toggle_item', {
        id,
        date: date || currentDate.value,
      })
      todayItems.value = day.items
      await refreshPendingCount()
    } catch (e) {
      console.error('Failed to toggle todo item:', e)
    }
  }

  async function deleteItem(id: string, date?: string) {
    try {
      const day = await invoke<TodoDay>('todo_delete_item', {
        id,
        date: date || currentDate.value,
      })
      todayItems.value = day.items
      await refreshPendingCount()
    } catch (e) {
      console.error('Failed to delete todo item:', e)
    }
  }

  async function copyDayText(date?: string): Promise<string> {
    try {
      return await invoke<string>('todo_copy_day', { date: date || currentDate.value })
    } catch (e) {
      console.error('Failed to copy day text:', e)
      return ''
    }
  }

  async function copyWeekText(): Promise<string> {
    try {
      return await invoke<string>('todo_copy_week', {})
    } catch (e) {
      console.error('Failed to copy week text:', e)
      return ''
    }
  }

  async function refreshPendingCount() {
    try {
      pendingCount.value = await invoke<number>('todo_get_pending_count')
    } catch (e) {
      console.error('Failed to get pending count:', e)
    }
  }

  async function switchDate(date: string) {
    currentDate.value = date
    await loadDay(date)
  }

  async function setupListener() {
    const unlisten = await listen('todo-list-changed', () => {
      loadDay()
      refreshPendingCount()
    })
    return unlisten
  }

  // Init
  loadDay()
  refreshPendingCount()

  return {
    todayItems,
    currentDate,
    pendingCount,
    loading,
    loadDay,
    addItem,
    toggleItem,
    deleteItem,
    copyDayText,
    copyWeekText,
    refreshPendingCount,
    switchDate,
    setupListener,
  }
}
