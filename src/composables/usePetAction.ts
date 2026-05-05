import { ref, computed, onMounted, onUnmounted } from 'vue'
import type { PetType } from '../types/config'

// 默认配置
const DEFAULT_ACTION_INTERVAL = 25 // 秒

export function usePetAction(initialPet: PetType = 'fixing', interval: number = DEFAULT_ACTION_INTERVAL) {
  const petType = ref<PetType>(initialPet)
  const currentAction = ref<string>('fixing')

  // 获取当前宠物的所有动作
  const availableActions = computed(() => {
    return ['idle']
  })

  // 随机切换动作
  function switchAction() {
    const actions = availableActions.value
    const randomIndex = Math.floor(Math.random() * actions.length)
    currentAction.value = actions[randomIndex]
    console.log(`[PetAction] Switched to: ${currentAction.value}`)
  }

  // 切换宠物类型
  function setPetType(type: PetType) {
    if (petType.value !== type) {
      petType.value = type
      currentAction.value = type
    }
  }

  // 定时器引用
  let actionTimer: number | null = null

  // 设置定时切换
  function setupActionTimer() {
    if (actionTimer) clearInterval(actionTimer)
    actionTimer = window.setInterval(() => {
      switchAction()
    }, interval * 1000)
  }

  // 清理定时器
  function cleanupActionTimer() {
    if (actionTimer) {
      clearInterval(actionTimer)
      actionTimer = null
    }
  }

  // 组件挂载时启动
  onMounted(() => {
    setupActionTimer()
    // 初始动作
    currentAction.value = availableActions.value[0]
  })

  // 组件卸载时清理
  onUnmounted(() => {
    cleanupActionTimer()
  })

  return {
    petType,
    currentAction,
    availableActions,
    switchAction,
    setPetType,
    setupActionTimer,
    cleanupActionTimer
  }
}
