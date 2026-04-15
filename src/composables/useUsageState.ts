import { computed, type Ref } from 'vue'

export type PetState = 'Fresh' | 'Flow' | 'Warning' | 'Panic' | 'Dead'

export interface UsageData {
  used: number
  total: number
  time_percent: number       // 月MCP额度 %
  tokens_percent: number     // 5h Token额度 %
  weekly_tokens_percent: number // 周Token额度 %
  time_remaining?: number    // 月MCP剩余次数（可能为null）
  tokens_reset_time?: number  // 5h Token下次重置时间（时间戳毫秒）
  weekly_tokens_reset_time?: number // 周Token下次重置时间（时间戳毫秒）
  time_reset_time?: number    // 月度额度下次重置时间（时间戳毫秒）
  level?: string             // 会员等级
  usage_details?: UsageDetail[] // 工具使用详情
}

export interface UsageDetail {
  model_code: string
  usage: number
}

export const COLORS: Record<PetState, string> = {
  Fresh: '#10B981',
  Flow: '#3B82F6',
  Warning: '#F59E0B',
  Panic: '#F97316',
  Dead: '#6B7280'
}

export function useUsageState(used: Ref<number>, total: Ref<number>) {
  const usagePercent = computed(() => {
    if (total.value === 0) return 0
    return (used.value / total.value) * 100
  })

  const petState = computed<PetState>(() => {
    const p = usagePercent.value
    if (p <= 30) return 'Fresh'
    if (p <= 60) return 'Flow'
    if (p <= 80) return 'Warning'
    if (p <= 95) return 'Panic'
    return 'Dead'
  })

  const stateColor = computed(() => COLORS[petState.value])

  const gradientColor = computed<string>(() => {
    const p = usagePercent.value
    // Dead 状态：100%及以上使用灰色
    if (p >= 100) return '#6B7280'
    // 0-100% 映射到 HSL 色相 120°(绿) → 0°(红)
    const hue = 120 * (1 - p / 100)
    return `hsl(${hue}, 75%, 45%)`
  })

  const gradientStrokeColor = computed<string>(() => {
    const p = usagePercent.value
    if (p >= 100) return '#4B5563'
    const hue = 120 * (1 - p / 100)
    return `hsl(${hue}, 80%, 35%)`
  })

  return {
    usagePercent,
    petState,
    stateColor,
    gradientColor,
    gradientStrokeColor
  }
}
