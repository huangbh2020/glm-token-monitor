import { ref, type Ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface ModelSummary {
  modelName: string
  totalTokens: number
  sortOrder: number
}

export interface TotalUsage {
  totalModelCallCount: number
  totalTokensUsage: number
  modelSummaryList: ModelSummary[]
}

export interface ModelDataEntry {
  modelName: string
  sortOrder: number
  tokensUsage: number[]
  totalTokens: number
}

export interface ModelUsageData {
  x_time: string[]
  modelCallCount: number[]
  tokensUsage: number[]
  totalUsage: TotalUsage
  modelDataList: ModelDataEntry[]
  modelSummaryList: ModelSummary[]
  granularity: string
}

export type TimeRange = 'today' | '7days' | '30days'

export function useModelUsage() {
  const modelUsageData: Ref<ModelUsageData | null> = ref(null)
  const isLoading = ref(false)
  const error = ref<string>('')
  const activeTab: Ref<TimeRange> = ref('today')

  async function fetchModelUsage(timeRange: TimeRange) {
    isLoading.value = true
    error.value = ''
    try {
      const data = await invoke<ModelUsageData>('get_model_usage', {
        timeRange,
      })
      console.log('[useModelUsage] data received:', timeRange, 'x_time:', data?.x_time?.length, 'tokensUsage:', data?.tokensUsage?.length)
      console.log('[useModelUsage] totalUsage:', JSON.stringify(data?.totalUsage))
      modelUsageData.value = data
      activeTab.value = timeRange
    } catch (err) {
      error.value = String(err)
      console.error('Failed to fetch model usage:', err)
    } finally {
      isLoading.value = false
    }
  }

  return {
    modelUsageData,
    isLoading,
    error,
    activeTab,
    fetchModelUsage,
  }
}
