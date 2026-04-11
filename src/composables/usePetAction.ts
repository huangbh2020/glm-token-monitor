import { ref, computed, onMounted, onUnmounted } from 'vue'
import type { PetType, CatAction, DogAction } from '../types/config'

// 动作定义
const CAT_ACTIONS: CatAction[] = ['cat-sleep', 'cat-play', 'cat-stare', 'cat-stretch']
const DOG_ACTIONS: DogAction[] = ['dog-sit', 'dog-bark', 'dog-walk', 'dog-beg']
const SPIRIT_ACTIONS = ['spirit-idle'] // Spirit 只有一个 idle 状态，由 petState 驱动

// 默认配置
const DEFAULT_ACTION_INTERVAL = 25 // 秒

export function usePetAction(initialPet: PetType = 'spirit', interval: number = DEFAULT_ACTION_INTERVAL) {
  const petType = ref<PetType>(initialPet)
  const currentAction = ref<string>('spirit-idle')

  // 获取当前宠物的所有动作
  const availableActions = computed(() => {
    if (petType.value === 'cat') return CAT_ACTIONS
    if (petType.value === 'dog') return DOG_ACTIONS
    return SPIRIT_ACTIONS
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
      // 切换宠物时立即更新动作
      let actions: string[]
      if (type === 'cat') actions = CAT_ACTIONS
      else if (type === 'dog') actions = DOG_ACTIONS
      else actions = SPIRIT_ACTIONS
      currentAction.value = actions[0]
      console.log(`[PetAction] Pet changed to: ${type}`)
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
