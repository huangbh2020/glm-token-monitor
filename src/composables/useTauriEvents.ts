import { ref, type Ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import type { UsageData } from './useUsageState'

export function useTauriEvents() {
  const usageData: Ref<UsageData> = ref({
    used: 0,
    total: 100,
    time_percent: 0,
    tokens_percent: 0,
    weekly_tokens_percent: 0,
    time_remaining: 0,
    tokens_reset_time: undefined,
    weekly_tokens_reset_time: undefined,
    time_reset_time: undefined,
    level: '',
    usage_details: []
  })
  const lastError: Ref<string | null> = ref(null)

  // 监听 Rust 端推送的 usage-update 事件
  const setupEventListener = async () => {
    const unlistenUsage = await listen<UsageData>('usage-update', (event) => {
      usageData.value = event.payload
      lastError.value = null // 清除错误状态
    })

    // 监听错误事件
    const unlistenError = await listen<string>('usage-error', (event) => {
      lastError.value = event.payload
      console.error('Usage error:', event.payload)
    })

    // 返回清理函数
    return () => {
      unlistenUsage()
      unlistenError()
    }
  }

  return {
    usageData,
    lastError,
    setupEventListener
  }
}
