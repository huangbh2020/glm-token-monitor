<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useUsageState } from '../composables/useUsageState'
import { useTauriEvents } from '../composables/useTauriEvents'

// 窗口尺寸常量
const WINDOW_SIZE_SMALL = 96
const WINDOW_SIZE_LARGE = 246
const TRANSITION_DURATION_MS = 350
const REFRESH_DEBOUNCE_MS = 500

const { usageData, setupEventListener } = useTauriEvents()
const { petState } = useUsageState(
  computed(() => usageData.value.used),
  computed(() => usageData.value.total)
)

// 双指标数据
const timePercent = computed(() => usageData.value.time_percent ?? 0)
const tokensPercent = computed(() => usageData.value.tokens_percent ?? 0)
const timeRemaining = computed(() => usageData.value.time_remaining)

const heartMessages: Record<string, string> = {
  Fresh:   '新的一天，能量满格！冲冲冲！',
  Flow:    '代码写得正顺手，不要打扰我~',
  Warning: '用量有点多了，要省着点...',
  Panic:   '要炸了！谁在疯狂 Call API？！',
  Dead:    '系统崩溃... 请充值续命...',
}
const heartMsg = computed(() => heartMessages[petState.value])

// 刺新状态
const isRefreshing = ref(false)
const lastUpdateTime = ref<string>('')
const fetchError = ref<string>('')

// 窗口扩展状态
const isHovered = ref(false)
const isExpanded = ref(false)
const refreshDebounce = ref<number | null>(null)
const isTransitioning = ref(false)

// 鼠标悬停时扩展窗口并刷新接口
async function onHoverRefresh() {
  // 防抖检查
  if (refreshDebounce.value) return

  refreshDebounce.value = window.setTimeout(() => {
    refreshDebounce.value = null
  }, REFRESH_DEBOUNCE_MS)

  // 扩展窗口
  if (!isExpanded.value) {
    await expandWindow()
  }

  // 静默刷新接口
  await refreshUsageData()
}

const startDrag = async (event: MouseEvent) => {
  event.preventDefault()
  try {
    const { Window } = await import('@tauri-apps/api/window')
    const win = Window.getCurrent()
    await win.startDragging()
  } catch (error) {
    console.error('Failed to start dragging:', error)
  }
}

// 窗口扩展
async function expandWindow() {
  if (isTransitioning.value) return
  isTransitioning.value = true

  try {
    const { Window, LogicalSize } = await import('@tauri-apps/api/window')
    const win = Window.getCurrent()
    await win.setSize(new LogicalSize(WINDOW_SIZE_LARGE, WINDOW_SIZE_LARGE))
    isExpanded.value = true
  } catch (error) {
    console.error('Failed to expand window:', error)
  } finally {
    setTimeout(() => {
      isTransitioning.value = false
    }, TRANSITION_DURATION_MS)
  }
}

// 窗口收缩
async function collapseWindow() {
  try {
    const { Window, LogicalSize } = await import('@tauri-apps/api/window')
    const win = Window.getCurrent()
    await win.setSize(new LogicalSize(WINDOW_SIZE_SMALL, WINDOW_SIZE_SMALL))
    isExpanded.value = false
  } catch (error) {
    console.error('Failed to collapse window:', error)
    // 强制重置状态
    isExpanded.value = false
  }
}

// 静默刷新数据（不显示加载提示）
async function refreshUsageData() {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const data = await invoke<typeof usageData.value>('get_current_usage')
    usageData.value = data
    const now = new Date()
    lastUpdateTime.value = `${now.getHours().toString().padStart(2,'0')}:${now.getMinutes().toString().padStart(2,'0')}:${now.getSeconds().toString().padStart(2,'0')}`
    fetchError.value = ''
  } catch (err) {
    fetchError.value = String(err)
    console.error('Silent refresh failed:', err)
  }
}

// 鼠标离开时收缩窗口
async function onHoverLeave() {
  // 清除防抖定时器
  if (refreshDebounce.value) {
    clearTimeout(refreshDebounce.value)
    refreshDebounce.value = null
  }

  // 收缩窗口
  await collapseWindow()
}

onMounted(async () => {
  await setupEventListener()

  // 组件卸载时清理
  return () => {
    if (refreshDebounce.value) {
      clearTimeout(refreshDebounce.value)
    }
  }
})
</script>

<template>
  <div class="pet-widget" :class="[`pet-${petState.toLowerCase()}`, { expanded: isExpanded }]"
    data-tauri-drag-region
    @mousedown="startDrag"
    @mouseenter="onHoverRefresh"
    @mouseleave="onHoverLeave"
  >
    <!-- 光晕层 -->
    <!-- <div class="glow-backdrop"></div> -->

    <!-- ===== FRESH: 睡猫 + 咖啡 ===== -->
    <svg v-if="petState === 'Fresh'" class="pet-svg" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
      <!-- ZZZ 浮动 -->
      <text class="zzz-a" x="44" y="26" font-size="7" fill="#6EE7B7" font-family="monospace" font-weight="bold">z</text>
      <text class="zzz-b" x="50" y="19" font-size="9" fill="#6EE7B7" font-family="monospace" font-weight="bold">z</text>
      <text class="zzz-c" x="57" y="12" font-size="11" fill="#6EE7B7" font-family="monospace" font-weight="bold">Z</text>
      <!-- 猫身体（蜷缩） -->
      <ellipse class="cat-breathe" cx="22" cy="55" rx="18" ry="11" fill="#F4A460"/>
      <!-- 猫头 -->
      <rect x="6" y="30" width="22" height="18" rx="5" fill="#F4A460"/>
      <!-- 耳朵左 -->
      <polygon points="8,31 12,22 16,31" fill="#F4A460"/>
      <polygon points="9,30 12,24 15,30" fill="#FFB6C1"/>
      <!-- 耳朵右 -->
      <polygon points="18,31 22,22 26,31" fill="#F4A460"/>
      <polygon points="19,30 22,24 25,30" fill="#FFB6C1"/>
      <!-- 闭眼（睡觉弧线） -->
      <path d="M9 38 Q13 35.5 17 38" stroke="#6B4226" stroke-width="1.5" fill="none" stroke-linecap="round"/>
      <path d="M19 38 Q23 35.5 27 38" stroke="#6B4226" stroke-width="1.5" fill="none" stroke-linecap="round"/>
      <!-- 鼻子 -->
      <polygon points="17,42 19,43 21,42 19,44" fill="#FF9AA2"/>
      <!-- 胡须左 -->
      <line x1="3" y1="40" x2="13" y2="42" stroke="#BBB" stroke-width="0.8"/>
      <line x1="3" y1="43" x2="13" y2="43" stroke="#BBB" stroke-width="0.8"/>
      <!-- 胡须右 -->
      <line x1="24" y1="42" x2="34" y2="40" stroke="#BBB" stroke-width="0.8"/>
      <line x1="24" y1="43" x2="34" y2="43" stroke="#BBB" stroke-width="0.8"/>
      <!-- 尾巴 -->
      <path d="M40 54 Q52 48 48 38 Q44 30 40 36" stroke="#F4A460" stroke-width="5" fill="none" stroke-linecap="round"/>
      <!-- 咖啡杯 -->
      <rect x="53" y="46" width="20" height="22" rx="3" fill="#6B3A2A"/>
      <rect x="55" y="48" width="16" height="18" rx="2" fill="#3D1A0A"/>
      <!-- 杯把 -->
      <path d="M73 50 Q80 54 73 62" stroke="#6B3A2A" stroke-width="3.5" fill="none" stroke-linecap="round"/>
      <!-- 咖啡液面 -->
      <ellipse cx="63" cy="48" rx="8" ry="2.5" fill="#5C2D0E"/>
      <!-- 热气 -->
      <path class="steam-a" d="M58 46 Q56 41 58 36" stroke="#A7F3D0" stroke-width="1.5" fill="none" stroke-linecap="round"/>
      <path class="steam-b" d="M63 44 Q61 39 63 34" stroke="#A7F3D0" stroke-width="1.5" fill="none" stroke-linecap="round"/>
      <path class="steam-c" d="M68 46 Q66 41 68 36" stroke="#A7F3D0" stroke-width="1.5" fill="none" stroke-linecap="round"/>
    </svg>

    <!-- ===== FLOW: 戴眼镜猫 + 电脑疯狂敲键盘 ===== -->
    <svg v-else-if="petState === 'Flow'" class="pet-svg" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
      <!-- TODO 气泡 -->
      <g class="todo-bubble">
        <rect x="30" y="4" width="34" height="13" rx="6" fill="#1D4ED8"/>
        <polygon points="34,17 38,17 36,21" fill="#1D4ED8"/>
        <text x="33" y="14" font-size="5.5" fill="white" font-family="monospace">// TODO</text>
      </g>
      <!-- 显示器 -->
      <rect x="40" y="22" width="34" height="26" rx="3" fill="#1E293B"/>
      <rect x="42" y="24" width="30" height="22" rx="2" fill="#0F172A"/>
      <!-- 代码行 -->
      <rect x="44" y="27" width="14" height="2" rx="0.5" fill="#60A5FA"/>
      <rect x="44" y="31" width="20" height="2" rx="0.5" fill="#34D399"/>
      <rect x="44" y="35" width="12" height="2" rx="0.5" fill="#A78BFA"/>
      <rect x="44" y="39" width="18" height="2" rx="0.5" fill="#60A5FA"/>
      <rect x="44" y="43" width="10" height="2" rx="0.5" fill="#F9A8D4"/>
      <!-- 光标闪烁 -->
      <rect class="cursor-blink" x="56" y="43" width="2" height="2.5" fill="#60A5FA"/>
      <!-- 显示器支架 -->
      <rect x="54" y="48" width="4" height="5" rx="1" fill="#334155"/>
      <rect x="49" y="53" width="14" height="2.5" rx="1" fill="#334155"/>
      <!-- 键盘 -->
      <rect x="30" y="60" width="44" height="14" rx="2.5" fill="#1E293B"/>
      <rect x="32" y="62" width="5" height="3.5" rx="0.5" fill="#334155"/>
      <rect x="38" y="62" width="5" height="3.5" rx="0.5" fill="#334155"/>
      <rect x="44" y="62" width="5" height="3.5" rx="0.5" fill="#334155"/>
      <rect x="50" y="62" width="5" height="3.5" rx="0.5" fill="#334155"/>
      <rect x="56" y="62" width="5" height="3.5" rx="0.5" fill="#334155"/>
      <rect x="62" y="62" width="6" height="3.5" rx="0.5" fill="#334155"/>
      <rect x="34" y="67" width="28" height="4" rx="0.5" fill="#334155"/>
      <!-- 猫身体 -->
      <rect x="4" y="38" width="28" height="24" rx="6" fill="#F4A460"/>
      <!-- 猫头 -->
      <rect x="5" y="18" width="24" height="22" rx="5" fill="#F4A460"/>
      <!-- 耳朵左 -->
      <polygon points="7,20 11,10 15,20" fill="#F4A460"/>
      <polygon points="8,19 11,12 14,19" fill="#FFB6C1"/>
      <!-- 耳朵右 -->
      <polygon points="19,20 23,10 27,20" fill="#F4A460"/>
      <polygon points="20,19 23,12 26,19" fill="#FFB6C1"/>
      <!-- 像素眼镜框 -->
      <rect x="6" y="26" width="8" height="6" rx="1.5" fill="none" stroke="#374151" stroke-width="1.5"/>
      <rect x="16" y="26" width="8" height="6" rx="1.5" fill="none" stroke="#374151" stroke-width="1.5"/>
      <line x1="14" y1="29" x2="16" y2="29" stroke="#374151" stroke-width="1.5"/>
      <line x1="6" y1="29" x2="4" y2="29" stroke="#374151" stroke-width="1.5"/>
      <line x1="24" y1="29" x2="26" y2="29" stroke="#374151" stroke-width="1.5"/>
      <!-- 专注的眼睛 -->
      <circle cx="10" cy="29" r="2" fill="#1E293B"/>
      <circle cx="20" cy="29" r="2" fill="#1E293B"/>
      <circle cx="10.8" cy="28.2" r="0.7" fill="white"/>
      <circle cx="20.8" cy="28.2" r="0.7" fill="white"/>
      <!-- 专注眉毛（平直） -->
      <line x1="6" y1="25" x2="14" y2="25" stroke="#6B4226" stroke-width="1.2"/>
      <line x1="16" y1="25" x2="24" y2="25" stroke="#6B4226" stroke-width="1.2"/>
      <!-- 鼻子 -->
      <polygon points="15,35 17,36 19,35 17,37" fill="#FF9AA2"/>
      <!-- 手臂/爪子打字 -->
      <rect class="arm-l" x="4" y="54" width="11" height="8" rx="4" fill="#E8944A"/>
      <rect class="arm-r" x="19" y="54" width="11" height="8" rx="4" fill="#E8944A"/>
    </svg>

    <!-- ===== WARNING: 暴躁猫 + 加速敲击 ===== -->
    <svg v-else-if="petState === 'Warning'" class="pet-svg warn-svg" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
      <!-- 警告符号 -->
      <text class="warn-flash" x="62" y="22" font-size="14" fill="#F59E0B" font-family="monospace" font-weight="bold">!</text>
      <!-- 侧面蒸汽 -->
      <path class="swarn-a" d="M48 50 Q52 44 48 38" stroke="#FDE68A" stroke-width="2" fill="none" stroke-linecap="round" opacity="0.85"/>
      <path class="swarn-b" d="M55 48 Q59 42 55 36" stroke="#FDE68A" stroke-width="2" fill="none" stroke-linecap="round" opacity="0.85"/>
      <path class="swarn-c" d="M62 50 Q66 44 62 38" stroke="#FDE68A" stroke-width="2" fill="none" stroke-linecap="round" opacity="0.85"/>
      <!-- 猫身 -->
      <rect x="5" y="38" width="30" height="24" rx="6" fill="#F4A460"/>
      <!-- 猫头 -->
      <rect x="5" y="18" width="26" height="22" rx="5" fill="#F4A460"/>
      <!-- 耳朵（略贴后=愤怒） -->
      <polygon points="7,20 10,12 14,20" fill="#F4A460"/>
      <polygon points="8,19 10,14 13,19" fill="#FFB6C1"/>
      <polygon points="22,20 26,12 30,20" fill="#F4A460"/>
      <polygon points="23,19 26,14 29,19" fill="#FFB6C1"/>
      <!-- V形愤怒眉 -->
      <path d="M7 25 L14 28" stroke="#333" stroke-width="2.2" stroke-linecap="round"/>
      <path d="M22 28 L29 25" stroke="#333" stroke-width="2.2" stroke-linecap="round"/>
      <!-- 半眯愤怒眼 -->
      <rect x="8" y="29" width="7" height="4" rx="2" fill="#1E293B"/>
      <rect x="22" y="29" width="7" height="4" rx="2" fill="#1E293B"/>
      <circle cx="11.5" cy="31" r="1.2" fill="#EF4444"/>
      <circle cx="25.5" cy="31" r="1.2" fill="#EF4444"/>
      <!-- 鼻子 -->
      <polygon points="16,35 18,36 20,35 18,37" fill="#FF9AA2"/>
      <!-- 不爽嘴巴（下撇） -->
      <path d="M11 40 Q18 37.5 25 40" stroke="#6B4226" stroke-width="1.5" fill="none" stroke-linecap="round"/>
      <!-- 速线手臂 -->
      <rect class="warnarm-l" x="3" y="54" width="13" height="8" rx="4" fill="#E8944A"/>
      <rect class="warnarm-r" x="22" y="54" width="13" height="8" rx="4" fill="#E8944A"/>
      <!-- 速度线 -->
      <line x1="0" y1="52" x2="5" y2="55" stroke="#FDE68A" stroke-width="1.3" opacity="0.8"/>
      <line x1="0" y1="57" x2="4" y2="58" stroke="#FDE68A" stroke-width="1.3" opacity="0.8"/>
      <line x1="35" y1="52" x2="40" y2="55" stroke="#FDE68A" stroke-width="1.3" opacity="0.8"/>
      <line x1="35" y1="57" x2="40" y2="58" stroke="#FDE68A" stroke-width="1.3" opacity="0.8"/>
      <!-- 键盘（橙色按键=疯狂） -->
      <rect x="6" y="64" width="44" height="13" rx="2" fill="#292524"/>
      <rect x="8" y="66" width="5" height="3.5" rx="0.5" fill="#F59E0B"/>
      <rect x="15" y="66" width="5" height="3.5" rx="0.5" fill="#F59E0B"/>
      <rect x="22" y="66" width="5" height="3.5" rx="0.5" fill="#F59E0B"/>
      <rect x="29" y="66" width="5" height="3.5" rx="0.5" fill="#F59E0B"/>
      <rect x="36" y="66" width="5" height="3.5" rx="0.5" fill="#F59E0B"/>
      <rect x="11" y="71" width="26" height="3.5" rx="0.5" fill="#F59E0B"/>
    </svg>

    <!-- ===== PANIC: 满头大汗 + 电脑冒烟 + Error ===== -->
    <svg v-else-if="petState === 'Panic'" class="pet-svg panic-svg" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
      <!-- 浮动Error符号 -->
      <text class="err-a" x="36" y="14" font-size="8" fill="#F97316" font-family="monospace" font-weight="bold">!</text>
      <text class="err-b" x="46" y="10" font-size="7" fill="#EF4444" font-family="monospace" font-weight="bold">×</text>
      <!-- 电脑冒烟 -->
      <circle class="smoke-a" cx="56" cy="24" r="5" fill="#9CA3AF"/>
      <circle class="smoke-b" cx="63" cy="20" r="4" fill="#9CA3AF"/>
      <circle class="smoke-c" cx="69" cy="24" r="5" fill="#9CA3AF"/>
      <!-- ERROR显示器 -->
      <rect x="44" y="28" width="30" height="24" rx="3" fill="#1E293B"/>
      <rect x="46" y="30" width="26" height="20" rx="2" fill="#2D0000"/>
      <!-- X标志 -->
      <line x1="50" y1="34" x2="60" y2="42" stroke="#EF4444" stroke-width="2.5" stroke-linecap="round"/>
      <line x1="60" y1="34" x2="50" y2="42" stroke="#EF4444" stroke-width="2.5" stroke-linecap="round"/>
      <!-- ERROR文字 -->
      <text x="48" y="46" font-size="5.5" fill="#EF4444" font-family="monospace" font-weight="bold">ERROR!</text>
      <!-- 支架 -->
      <rect x="56" y="52" width="4" height="5" rx="1" fill="#374151"/>
      <rect x="51" y="57" width="14" height="2.5" rx="1" fill="#374151"/>
      <!-- 汗水 -->
      <ellipse class="sweat-a" cx="30" cy="16" rx="1.5" ry="2.5" fill="#93C5FD"/>
      <ellipse class="sweat-b" cx="4" cy="30" rx="1.5" ry="2.5" fill="#93C5FD"/>
      <!-- 猫耳朵 -->
      <polygon points="8,18 12,8 16,18" fill="#F4A460"/>
      <polygon points="9,17 12,10 15,17" fill="#FFB6C1"/>
      <polygon points="20,18 24,8 28,18" fill="#F4A460"/>
      <polygon points="21,17 24,10 27,17" fill="#FFB6C1"/>
      <!-- 猫头 -->
      <rect x="4" y="16" width="28" height="24" rx="6" fill="#F4A460"/>
      <!-- 惊恐大眼 -->
      <circle cx="13" cy="26" r="5" fill="white"/>
      <circle cx="25" cy="26" r="5" fill="white"/>
      <circle cx="13" cy="26" r="3" fill="#1E293B"/>
      <circle cx="25" cy="26" r="3" fill="#1E293B"/>
      <circle cx="14.5" cy="24.5" r="1.2" fill="white"/>
      <circle cx="26.5" cy="24.5" r="1.2" fill="white"/>
      <!-- 惊恐眉（高高扬起） -->
      <path d="M8 21 Q13 18 18 21" stroke="#6B4226" stroke-width="1.5" fill="none"/>
      <path d="M20 21 Q25 18 30 21" stroke="#6B4226" stroke-width="1.5" fill="none"/>
      <!-- 大张的嘴（惊叫O） -->
      <ellipse cx="18" cy="36" rx="4.5" ry="3.5" fill="#1E293B"/>
      <ellipse cx="18" cy="36" rx="3" ry="2.2" fill="#EF4444"/>
      <!-- 猫身 -->
      <rect x="4" y="38" width="28" height="24" rx="6" fill="#F4A460"/>
      <!-- 举起的手臂 -->
      <rect class="parm-l" x="0" y="42" width="8" height="12" rx="4" fill="#E8944A" transform="rotate(-35 4 48)"/>
      <rect class="parm-r" x="30" y="42" width="8" height="12" rx="4" fill="#E8944A" transform="rotate(35 34 48)"/>
    </svg>

    <!-- ===== DEAD: 像素幽灵 + 404 ===== -->
    <svg v-else class="pet-svg" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
      <!-- 404显示器 -->
      <rect x="36" y="28" width="36" height="28" rx="3" fill="#374151"/>
      <rect x="38" y="30" width="32" height="24" rx="2" fill="#111827"/>
      <text x="41" y="46" font-size="12" fill="#4B5563" font-family="monospace" font-weight="bold">404</text>
      <text x="40" y="52" font-size="5" fill="#374151" font-family="monospace">Not Found</text>
      <!-- 显示器支架 -->
      <rect x="51" y="56" width="4" height="5" rx="1" fill="#374151"/>
      <rect x="46" y="61" width="14" height="2.5" rx="1" fill="#374151"/>
      <!-- 尘埃颗粒 -->
      <circle class="dust-a" cx="6" cy="20" r="2" fill="#9CA3AF"/>
      <circle class="dust-b" cx="33" cy="16" r="1.5" fill="#9CA3AF"/>
      <circle class="dust-c" cx="4" cy="55" r="1.2" fill="#9CA3AF"/>
      <circle class="dust-d" cx="34" cy="62" r="2" fill="#9CA3AF"/>
      <!-- 幽灵身体 -->
      <g class="ghost-body">
        <!-- 幽灵主体 -->
        <path d="M7 66 L7 34 Q7 18 18 18 Q29 18 29 34 L29 66 Q26 61 23 66 Q20 61 18 66 Q16 61 13 66 Q10 61 7 66 Z" fill="#E5E7EB" opacity="0.9"/>
        <!-- 幽灵大眼睛（空洞） -->
        <ellipse cx="14" cy="36" rx="3.5" ry="4" fill="#374151"/>
        <ellipse cx="22" cy="36" rx="3.5" ry="4" fill="#374151"/>
        <!-- 悲伤小嘴 -->
        <path d="M13 46 Q18 43 23 46" stroke="#9CA3AF" stroke-width="1.5" fill="none" stroke-linecap="round"/>
        <!-- 幽灵光泽 -->
        <ellipse cx="12" cy="26" rx="4" ry="2.5" fill="white" opacity="0.3"/>
      </g>
      <!-- 省略号 -->
      <text x="8" y="76" font-size="8" fill="#4B5563" font-family="monospace">. . .</text>
    </svg>

    <!-- 心语 + 双指标信息面板 -->
    <div class="heart-msg">
      <!-- 加载中覆盖层 -->
      <div v-if="isRefreshing" class="hm-loading">
        <span class="hm-spinner"></span>
        <span style="font-size:8px;color:#94A3B8">刷新中...</span>
      </div>
      <template v-else>
        <!-- 情感心语 -->
        <div class="hm-quote">{{ heartMsg }}</div>
        <!-- 错误提示 -->
        <div v-if="fetchError" class="hm-error">⚠ {{ fetchError.slice(0, 30) }}</div>
        <!-- 月 MCP 额度 -->
        <div class="hm-metric">
          <span class="hm-label">🗓 月额</span>
          <div class="hm-bar-wrap">
            <div class="hm-bar hm-bar-time" :style="{ width: timePercent + '%' }"></div>
          </div>
          <span class="hm-val">{{ timePercent }}%</span>
        </div>
        <!-- 5h Token 额度 -->
        <div class="hm-metric">
          <span class="hm-label">⏱ 5h额</span>
          <div class="hm-bar-wrap">
            <div class="hm-bar hm-bar-tok" :style="{ width: tokensPercent + '%' }"></div>
          </div>
          <span class="hm-val">{{ tokensPercent }}%</span>
        </div>
        <!-- 剩余次数 -->
        <div v-if="timeRemaining !== undefined" class="hm-remaining">
          剩余 {{ timeRemaining }} 次
        </div>
        <!-- 最后更新时间 -->
        <div v-if="lastUpdateTime" class="hm-time">⟳ {{ lastUpdateTime }}</div>
      </template>
    </div>
  </div>
</template>

<style scoped>
/* ── 基础容器 ── */
.pet-widget {
  width: 96px;
  height: 96px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent !important;
  position: relative;
  cursor: pointer;
  user-select: none;
  pointer-events: auto;
  -webkit-app-region: drag;
  app-region: drag;
}
.pet-widget:active { cursor: pointer; }

/* ── 光晕层 ── */
.glow-backdrop {
  position: absolute;
  inset: 0;
  border-radius: 50%;
  pointer-events: none;
  -webkit-app-region: no-drag;
}

.pet-fresh .glow-backdrop {
  background: radial-gradient(circle, rgba(16,185,129,0.22) 0%, transparent 68%);
  box-shadow: 0 0 22px rgba(16,185,129,0.45), 0 0 44px rgba(16,185,129,0.15);
  animation: glow-green 2.8s ease-in-out infinite;
}
.pet-flow .glow-backdrop {
  background: radial-gradient(circle, rgba(59,130,246,0.22) 0%, transparent 68%);
  box-shadow: 0 0 20px rgba(59,130,246,0.4), 0 0 40px rgba(59,130,246,0.15);
  animation: glow-blue 1.8s ease-in-out infinite;
}
.pet-warning .glow-backdrop {
  background: radial-gradient(circle, rgba(245,158,11,0.28) 0%, transparent 68%);
  box-shadow: 0 0 22px rgba(245,158,11,0.55), 0 0 44px rgba(245,158,11,0.2);
  animation: glow-yellow 0.7s ease-in-out infinite;
}
.pet-panic .glow-backdrop {
  background: radial-gradient(circle, rgba(239,68,68,0.3) 0%, transparent 68%);
  box-shadow: 0 0 24px rgba(239,68,68,0.65), 0 0 50px rgba(249,115,22,0.3);
  animation: glow-panic 0.28s ease-in-out infinite;
}
.pet-dead .glow-backdrop {
  background: radial-gradient(circle, rgba(107,114,128,0.18) 0%, transparent 68%);
  box-shadow: 0 0 16px rgba(107,114,128,0.3);
  animation: glow-dead 3.5s ease-in-out infinite;
}

@keyframes glow-green {
  0%,100% { opacity: 0.7; transform: scale(1); }
  50% { opacity: 1; transform: scale(1.06); }
}
@keyframes glow-blue {
  0%,100% { opacity: 0.75; }
  50% { opacity: 1; }
}
@keyframes glow-yellow {
  0%,100% { opacity: 0.65; box-shadow: 0 0 16px rgba(245,158,11,0.4); }
  50% { opacity: 1; box-shadow: 0 0 30px rgba(245,158,11,0.8); }
}
@keyframes glow-panic {
  0%,100% { opacity: 0.7; box-shadow: 0 0 20px rgba(239,68,68,0.55); }
  50% { opacity: 1; box-shadow: 0 0 40px rgba(239,68,68,1), 0 0 60px rgba(249,115,22,0.4); }
}
@keyframes glow-dead {
  0%,100% { opacity: 0.8; }
  50% { opacity: 0.4; }
}

/* ── SVG 通用 ── */
.pet-svg {
  position: relative;
  z-index: 2;
  width: 80px;
  height: 80px;
  overflow: visible;
  pointer-events: none;
}

/* ───── FRESH 动画 ───── */
.cat-breathe {
  animation: breathe 2.8s ease-in-out infinite;
  transform-origin: 22px 55px;
}
@keyframes breathe {
  0%,100% { transform: scaleY(1); }
  50% { transform: scaleY(1.1); }
}

.zzz-a { animation: zzz 2.2s ease-in-out infinite; }
.zzz-b { animation: zzz 2.2s ease-in-out 0.35s infinite; }
.zzz-c { animation: zzz 2.2s ease-in-out 0.7s infinite; }
@keyframes zzz {
  0%   { opacity: 0; transform: translate(0, 4px); }
  25%  { opacity: 1; }
  80%  { opacity: 0.8; }
  100% { opacity: 0; transform: translate(2px, -10px); }
}

.steam-a { animation: steam 2.2s ease-in-out infinite; }
.steam-b { animation: steam 2.2s ease-in-out 0.45s infinite; }
.steam-c { animation: steam 2.2s ease-in-out 0.9s infinite; }
@keyframes steam {
  0%   { opacity: 0; transform: translateY(4px) scaleX(1); }
  30%  { opacity: 0.85; transform: translateY(0) scaleX(1.3); }
  70%  { opacity: 0.4; }
  100% { opacity: 0; transform: translateY(-7px) scaleX(0.7); }
}

/* ───── FLOW 动画 ───── */
.cursor-blink { animation: blink 1s step-end infinite; }
@keyframes blink {
  0%,100% { opacity: 1; }
  50% { opacity: 0; }
}

.arm-l {
  animation: type-l 0.28s ease-in-out infinite alternate;
  transform-origin: 9px 58px;
}
.arm-r {
  animation: type-r 0.28s ease-in-out 0.14s infinite alternate;
  transform-origin: 24px 58px;
}
@keyframes type-l {
  from { transform: translateY(0); }
  to   { transform: translateY(4px); }
}
@keyframes type-r {
  from { transform: translateY(0); }
  to   { transform: translateY(4px); }
}

.todo-bubble {
  animation: bubble 3.2s ease-in-out infinite;
}
@keyframes bubble {
  0%,100% { transform: translateY(0); opacity: 0.9; }
  50%      { transform: translateY(-4px); opacity: 1; }
}

/* ───── WARNING 动画 ───── */
.warn-svg { animation: warn-shake 0.45s ease-in-out infinite; }
@keyframes warn-shake {
  0%,100% { transform: translateX(0); }
  25%     { transform: translateX(-2px); }
  75%     { transform: translateX(2px); }
}

.warnarm-l {
  animation: wtype 0.18s ease-in-out infinite alternate;
}
.warnarm-r {
  animation: wtype 0.18s ease-in-out 0.09s infinite alternate;
}
@keyframes wtype {
  from { transform: translateY(0); }
  to   { transform: translateY(5px); }
}

.swarn-a { animation: steam-warn 1.4s ease-in-out infinite; }
.swarn-b { animation: steam-warn 1.4s ease-in-out 0.28s infinite; }
.swarn-c { animation: steam-warn 1.4s ease-in-out 0.56s infinite; }
@keyframes steam-warn {
  0%   { opacity: 0; transform: translateY(5px); }
  30%  { opacity: 0.9; }
  100% { opacity: 0; transform: translateY(-10px); }
}

.warn-flash { animation: flash-warn 0.6s ease-in-out infinite alternate; }
@keyframes flash-warn {
  from { opacity: 0.5; transform: scale(0.9); }
  to   { opacity: 1; transform: scale(1.1); }
}

/* ───── PANIC 动画 ───── */
.panic-svg { animation: panic-shake 0.12s ease-in-out infinite; }
@keyframes panic-shake {
  0%,100% { transform: translate(0, 0); }
  25%     { transform: translate(-2px, 1px); }
  50%     { transform: translate(1px, -1px); }
  75%     { transform: translate(2px, 1px); }
}

.smoke-a { animation: smoke 1.8s ease-out infinite; }
.smoke-b { animation: smoke 1.8s ease-out 0.4s infinite; }
.smoke-c { animation: smoke 1.8s ease-out 0.8s infinite; }
@keyframes smoke {
  0%   { transform: translateY(0) scale(1); opacity: 0.6; }
  100% { transform: translateY(-18px) scale(1.8); opacity: 0; }
}

.sweat-a { animation: sweat-fly 1.6s ease-in-out infinite; }
.sweat-b { animation: sweat-fly 1.6s ease-in-out 0.6s infinite; }
@keyframes sweat-fly {
  0%   { opacity: 0; transform: translate(0, 0); }
  30%  { opacity: 1; }
  100% { opacity: 0; transform: translate(8px, -14px); }
}

.err-a { animation: err-flash 0.4s ease-in-out infinite alternate; }
.err-b { animation: err-flash 0.4s ease-in-out 0.2s infinite alternate; }
@keyframes err-flash {
  from { opacity: 0.3; transform: scale(0.85); }
  to   { opacity: 1; transform: scale(1.15); }
}

.parm-l { animation: parm 0.12s ease-in-out infinite alternate; }
.parm-r { animation: parm 0.12s ease-in-out 0.06s infinite alternate; }
@keyframes parm {
  from { transform: rotate(-35deg) translateY(0); }
  to   { transform: rotate(-35deg) translateY(-3px); }
}

/* ───── DEAD 动画 ───── */
.ghost-body {
  animation: ghost-float 3.2s ease-in-out infinite;
}
@keyframes ghost-float {
  0%,100% { transform: translateY(0); opacity: 0.9; }
  50%      { transform: translateY(-9px); opacity: 0.55; }
}

.dust-a { animation: dust 3.5s ease-in-out infinite; }
.dust-b { animation: dust 3.5s ease-in-out 0.8s infinite; }
.dust-c { animation: dust 3.5s ease-in-out 1.6s infinite; }
.dust-d { animation: dust 3.5s ease-in-out 2.4s infinite; }
@keyframes dust {
  0%,100% { opacity: 0; transform: translateY(0); }
  50%      { opacity: 0.5; transform: translateY(-6px); }
}

/* ── 心语 + 信息面板 ── */
.heart-msg {
  position: absolute;
  inset: 3px;
  border-radius: 50%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 3px;
  background: rgba(8, 14, 28, 0.91);
  color: #E2E8F0;
  pointer-events: none;
  opacity: 0;
  transition: opacity 0.22s ease;
  z-index: 10;
  font-family: 'PingFang SC', 'Microsoft YaHei', 'Noto Sans SC', sans-serif;
  border: 1px solid rgba(255,255,255,0.1);
  overflow: hidden;
  padding: 6px 9px;
}
.pet-widget:hover .heart-msg { opacity: 1; }

/* 情感心语文字 */
.hm-quote {
  font-size: 7.5px;
  line-height: 1.4;
  text-align: center;
  color: #CBD5E1;
  margin-bottom: 2px;
}

/* 单行指标：图标 + 进度条 + 百分比 */
.hm-metric {
  display: flex;
  align-items: center;
  gap: 3px;
  width: 100%;
}
.hm-label {
  font-size: 7px;
  color: #94A3B8;
  flex-shrink: 0;
  width: 26px;
}
.hm-bar-wrap {
  flex: 1;
  height: 4px;
  background: rgba(255,255,255,0.1);
  border-radius: 2px;
  overflow: hidden;
}
.hm-bar {
  height: 100%;
  border-radius: 2px;
  transition: width 0.4s ease;
  max-width: 100%;
}
/* 月额度进度条颜色跟随宠物状态 */
.pet-fresh  .hm-bar-time { background: #10B981; }
.pet-flow   .hm-bar-time { background: #3B82F6; }
.pet-warning .hm-bar-time { background: #F59E0B; }
.pet-panic  .hm-bar-time { background: #EF4444; }
.pet-dead   .hm-bar-time { background: #6B7280; }
/* 5h Token 进度条固定蓝色 */
.hm-bar-tok { background: #60A5FA; }

.hm-val {
  font-size: 7px;
  color: #94A3B8;
  flex-shrink: 0;
  width: 18px;
  text-align: right;
}
/* 剩余次数提示 */
.hm-remaining {
  font-size: 7px;
  color: #64748B;
  margin-top: 1px;
}
/* 最后更新时间 */
.hm-time {
  font-size: 6.5px;
  color: #475569;
  margin-top: 2px;
}
/* 错误提示 */
.hm-error {
  font-size: 7px;
  color: #F87171;
  text-align: center;
  line-height: 1.3;
}
/* 加载中状态 */
.hm-loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}
.hm-spinner {
  display: inline-block;
  width: 14px;
  height: 14px;
  border: 2px solid rgba(148,163,184,0.2);
  border-top-color: #60A5FA;
  border-radius: 50%;
  animation: spin 0.7s linear infinite;
}
@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
