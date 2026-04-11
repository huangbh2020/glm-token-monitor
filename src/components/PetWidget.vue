<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import { useUsageState } from '../composables/useUsageState'
import { useTauriEvents } from '../composables/useTauriEvents'
import { useDisplayMode } from '../composables/useDisplayMode'
import { useSettings } from '../composables/useSettings'
import { usePetAction } from '../composables/usePetAction'
import type { PetType } from '../types/config'
import CatGifViewer from './pets/CatGifViewer.vue'
import DogSit from './pets/DogSit.vue'
import DogBark from './pets/DogBark.vue'
import DogWalk from './pets/DogWalk.vue'
import DogBeg from './pets/DogBeg.vue'
import JellySpirit from './pets/JellySpirit.vue'
import PixelGhost from './pets/PixelGhost.vue'

const { displayMode } = useDisplayMode()
const { loadConfig, setupConfigListener, config, basicConfig } = useSettings()
const { usageData, setupEventListener } = useTauriEvents()

// 计算是否显示光晕层
const showGlowEffect = computed(() => basicConfig.value?.enable_glow ?? true)
const { usagePercent, petState, gradientColor, gradientStrokeColor } = useUsageState(
  computed(() => usageData.value.used),
  computed(() => usageData.value.total)
)

// 宠物动作系统
const { petType, currentAction, setPetType } = usePetAction()

// 监听配置变化更新宠物
watch(() => config.value?.pet_config?.selected_pet, (newPet) => {
  if (newPet && newPet !== petType.value) {
    console.log(`[PetWidget] Pet config changed to: ${newPet}`)
    setPetType(newPet as PetType)
  }
})

// 宠物组件映射
const petComponents = {
  'dog-sit': DogSit,
  'dog-bark': DogBark,
  'dog-walk': DogWalk,
  'dog-beg': DogBeg
} as const

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

// 悬浮与拖拽相关状态
const isExpanded = ref(false)
const isDragging = ref(false)

// 拖动和点击处理
let dragStartTime = 0
let dragStartPos = { x: 0, y: 0 }

const startDrag = async (event: MouseEvent) => {
  console.log('[Drag] mousedown triggered, target:', (event.target as HTMLElement).tagName, 'button:', event.button)
  event.preventDefault()
  isDragging.value = true

  // 记录拖动开始时间和位置
  dragStartTime = Date.now()
  dragStartPos = { x: event.clientX, y: event.clientY }

  try {
    const { Window } = await import('@tauri-apps/api/window')
    const win = Window.getCurrent()
    console.log('[Drag] calling startDragging...')
    await win.startDragging()
    console.log('[Drag] startDragging completed')
  } catch (error) {
    console.error('[Drag] startDragging failed:', error)
  } finally {
    // 延迟重置，避免 mouseleave 立即触发收缩
    setTimeout(() => {
      isDragging.value = false
      console.log('[Drag] isDragging reset to false')
    }, 200)
  }
}

// 点击处理（区分拖动和点击）
const handleClick = async (event: MouseEvent) => {
  const dragDuration = Date.now() - dragStartTime
  const dragDistance = Math.sqrt(
    Math.pow(event.clientX - dragStartPos.x, 2) +
    Math.pow(event.clientY - dragStartPos.y, 2)
  )

  // 如果移动距离小于5px且持续时间小于300ms，认为是点击而非拖动
  if (dragDistance < 5 && dragDuration < 300) {
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      await invoke('open_info_panel')
    } catch (err) {
      console.error('Open info panel failed:', err)
    }
  }
}

// 双击处理 - 阻止全屏
const handleDblClick = (event: MouseEvent) => {
  console.log('[DblClick] Preventing default behavior')
  event.preventDefault()
  event.stopPropagation()
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

// 定时刷新数据（1分钟间隔）
const DATA_REFRESH_INTERVAL = 60000 // 1分钟
let dataRefreshTimer: number | null = null

// 定时随机展现心语对话气泡
const showQuoteBubble = ref(false)
let quoteTimer: number | null = null

function setupQuoteTimer() {
  // const triggerQuote = () => {
  //   if (!isExpanded.value && !isDragging.value) {
  //     showQuoteBubble.value = true
  //     setTimeout(() => {
  //       showQuoteBubble.value = false
  //     }, 5000)
  //   }
  // }
  
  // setTimeout(triggerQuote, 2500) // 开场2.5秒展示一次
  // quoteTimer = window.setInterval(triggerQuote, 20000) // 20秒轮询
}

// 设置定时刷新数据（每1分钟）
function setupDataRefreshTimer() {
  // 立即执行一次刷新
  refreshUsageData()

  // 设置定时器，每1分钟刷新一次
  dataRefreshTimer = window.setInterval(() => {
    refreshUsageData()
  }, DATA_REFRESH_INTERVAL)
}

onMounted(async () => {
  // 【重要修复】：把启动定时器放到最顶部！防止由于 Tauri 或其他 await 函数执行超时阻塞定时器注册。
  setupQuoteTimer()
  setupDataRefreshTimer()

  try {
    await setupEventListener()
  } catch (err) {
    console.error('setupEventListener failed:', err)
  }

  // Load configuration and listen to settings change
  try {
    console.log('[DEBUG PetWidget] Initializing config...')
    await loadConfig()
    await setupConfigListener()
    console.log('[DEBUG PetWidget] Config initialized.')
  } catch (err) {
    console.error('Config initialized failed:', err)
  }

  // Debug watcher for displayMode changes
  const { watch } = await import('vue')
  watch(displayMode, (newVal) => {
    console.log('[DEBUG PetWidget] Watcher triggered! displayMode is now:', newVal)
  }, { immediate: true })

  // 强制设置窗口始终置顶，防止失去焦点后被其他窗口遮挡
  try {
    const { Window } = await import('@tauri-apps/api/window')
    const win = Window.getCurrent()
    await win.setAlwaysOnTop(true)
  } catch (err) {
    console.error('Enforce always on top failed:', err)
  }
})

onUnmounted(() => {
  if (quoteTimer) clearInterval(quoteTimer)
  if (dataRefreshTimer) clearInterval(dataRefreshTimer)
})
</script>

<template>
  <div class="pet-widget" :class="[`pet-${petState.toLowerCase()}`, { expanded: isExpanded }]"
    data-tauri-drag-region
    @mousedown="startDrag"
    @click="handleClick"
    @dblclick.prevent="handleDblClick"
  >
    <!-- 光晕层 -->
    <div v-if="showGlowEffect" class="glow-backdrop"></div>

    <!-- 动态宠物动作组件 -->
    <JellySpirit v-if="petType === 'spirit'" :color="gradientColor" :stroke-color="gradientStrokeColor" :state="petState" :width="100" :height="100" />
    <PixelGhost v-else-if="petType === 'ghost'" :color="gradientColor" :stroke-color="gradientStrokeColor" :state="petState" :width="100" :height="100" />
    <CatGifViewer v-else-if="currentAction.startsWith('cat-')" :action="currentAction" :width="100" :height="100" />
    <component v-else :is="petComponents[currentAction as keyof typeof petComponents]" :key="currentAction" />

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

    <!-- 独立的定时心语对话框（置于底部并指向上方） -->
    <transition name="bubble-fade">
      <div v-if="showQuoteBubble && !isExpanded" class="pixel-bubble quote-bubble" :class="`bubble-${petState.toLowerCase()}`"
        @mousedown.stop
        @click.stop
        @dblclick.stop>
        <span class="bubble-val quote-text">{{ heartMsg }}</span>
      </div>
    </transition>

    <!-- 5 种展示模式多态呈现 -->
    <!-- 0. none - 不显示用量 -->
    <!-- 1. holo-bubble -->
    <div v-if="displayMode === 'holo-bubble'" class="holo-bubble token-mode" :class="`state-${petState.toLowerCase()}`">
      <div class="scanlines"></div>
      5h: <span class="holo-val">{{ 100 - tokensPercent }}%</span>
    </div>

    <!-- 2. cyber-ring -->
    <div v-else-if="displayMode === 'cyber-ring'" class="cyber-ring token-mode" :class="`state-${petState.toLowerCase()}`">
      <svg viewBox="0 0 100 100" class="cr-svg">
        <circle class="cr-bg-dashed" cx="50" cy="50" r="46"/>
        <circle class="cr-progress" cx="50" cy="50" r="42"
          stroke-dasharray="264"
          :stroke-dashoffset="264 * (tokensPercent / 100)"
        />
      </svg>
      <div class="cr-center-val">{{ 100 - tokensPercent }}%</div>
    </div>

    <!-- 3. aura-field -->
    <div v-else-if="displayMode === 'aura-field'" class="aura-field token-mode" :class="`state-${petState.toLowerCase()}`">
      <div class="aura-ripple r1"></div>
      <div class="aura-ripple r2"></div>
      <div class="aura-ripple r3"></div>
      <div class="aura-val">{{ 100 - tokensPercent }}%</div>
    </div>

    <!-- 4. energy-core -->
    <div v-else-if="displayMode === 'energy-core'" class="energy-core token-mode" :class="`state-${petState.toLowerCase()}`">
      <div class="ec-grid">
        <div v-for="i in 16" :key="i" class="ec-pixel" :class="{ on: (100 - tokensPercent) >= (i * 6.25 - 3.125) }"></div>
      </div>
      <div class="ec-val">{{ 100 - tokensPercent }}%</div>
    </div>

    <!-- 5. status-floater -->
    <div v-else-if="displayMode === 'status-floater'" class="status-floater token-mode" :class="`state-${petState.toLowerCase()}`">
      <div class="sf-bar-container">
        <div class="sf-bar-fill" :style="{ height: (100 - tokensPercent) + '%' }"></div>
      </div>
      <div class="sf-text">{{ 100 - tokensPercent }}%</div>
    </div>
  </div>
</template>

<style scoped>
/* ── 基础容器 ── */
.pet-widget {
  width: 120px;
  height: 120px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent !important;
  position: relative;
  cursor: pointer;
  user-select: none;
  pointer-events: auto;
  border-radius: 8px;
  overflow: hidden;
  -webkit-app-region: drag;
}
.pet-widget:active { cursor: pointer; }

/* 扩展状态下的窗口大小 */
.pet-widget.expanded {
  width: 246px;
  height: 246px;
  overflow: hidden;
}

/* ── 光晕层 ── */
.glow-backdrop {
  position: absolute;
  inset: 8px;
  border-radius: 8px;
  pointer-events: none;
}

.pet-fresh .glow-backdrop {
  background: radial-gradient(circle, rgba(16,185,129,0.14) 0%, transparent 68%);
  box-shadow: 0 0 14px rgba(16,185,129,0.26), 0 0 21px rgba(16,185,129,0.06);
  animation: glow-green 5s ease-in-out infinite;
}
.pet-flow .glow-backdrop {
  background: radial-gradient(circle, rgba(59,130,246,0.14) 0%, transparent 68%);
  box-shadow: 0 0 12px rgba(59,130,246,0.22), 0 0 20px rgba(59,130,246,0.06);
  animation: glow-blue 3s ease-in-out infinite;
}
.pet-warning .glow-backdrop {
  background: radial-gradient(circle, rgba(245,158,11,0.17) 0%, transparent 68%);
  box-shadow: 0 0 14px rgba(245,158,11,0.3), 0 0 21px rgba(245,158,11,0.08);
  animation: glow-yellow 1.2s ease-in-out infinite;
}
.pet-panic .glow-backdrop {
  background: radial-gradient(circle, rgba(239,68,68,0.17) 0%, transparent 68%);
  box-shadow: 0 0 15px rgba(239,68,68,0.34), 0 0 23px rgba(249,115,22,0.09);
  animation: glow-panic 0.5s ease-in-out infinite;
}
.pet-dead .glow-backdrop {
  background: radial-gradient(circle, rgba(107,114,128,0.11) 0%, transparent 68%);
  box-shadow: 0 0 9px rgba(107,114,128,0.15);
  animation: glow-dead 5s ease-in-out infinite;
}

@keyframes glow-green {
  0%,100% { opacity: 0.75; transform: scale(1); }
  50% { opacity: 0.95; transform: scale(1.03); }
}
@keyframes glow-blue {
  0%,100% { opacity: 0.78; }
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
  border-radius: 8px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 3px;
  background: rgba(8, 14, 28, 0.91);
  color: #E2E8F0;
  pointer-events: none;
  opacity: 0;
  transition: all 350ms ease-out;
  z-index: 10;
  font-family: 'PingFang SC', 'Microsoft YaHei', 'Noto Sans SC', sans-serif;
  border: 1px solid rgba(255,255,255,0.1);
  overflow: hidden;
  padding: 6px 9px;
}
/* 鼠标悬停不再触发面板显示 */
.pet-widget.expanded .heart-msg {
  opacity: 1;
}

/* 扩展状态下的内容放大样式 */
.pet-widget.expanded .heart-msg {
  transform: scale(1.5);
}

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

/* ── Token 剩余容量像素气泡 (Retro Bubble) ── */
.pixel-bubble {
  position: absolute;
  top: 10px;
  right: 6px;
  background: #0F172A; /* 深色背景以提升对比度 */
  border: 2px solid #334155;
  /* 经典的像素风硬阴影 */
  box-shadow: 2px 2px 0 rgba(0, 0, 0, 0.8);
  padding: 3px 6px; /* 内边距 */
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace; /* 无衬线等宽字体 */
  font-size: 10px; /* 增大字号 */
  font-weight: 700;
  letter-spacing: 0.5px; /* 字间距 */
  color: #94A3B8; /* 前缀 'Tk:' 的颜色 */
  z-index: 15;
  pointer-events: none;
  display: flex;
  align-items: center;
  gap: 2px;
  border-radius: 3px;
  /* 悬浮微动效 */
  animation: float-bubble 2s ease-in-out infinite alternate;
  transition: opacity 0.3s ease, transform 0.3s ease;
}

/* 气泡对话框的尾巴 (外侧边缘) */
.pixel-bubble::after {
  content: '';
  position: absolute;
  bottom: -6px;
  left: 6px;
  width: 0;
  height: 0;
  border-left: 6px solid #334155;
  border-bottom: 6px solid transparent;
}

/* 气泡对话框的尾巴 (内侧留白) */
.pixel-bubble::before {
  content: '';
  position: absolute;
  bottom: -2.5px;
  left: 8.5px;
  width: 0;
  height: 0;
  border-left: 3.5px solid #0F172A;
  border-bottom: 3.5px solid transparent;
  z-index: 1;
}

/* 悬浮面板展开时的样式变化 */
.pet-widget.expanded .pixel-bubble {
  opacity: 0;
  transform: translateY(-8px) scale(0.8);
}

/* 长文本的心语模式特定样式扩展（置于底部中央，避免遮挡宠物和token） */
.quote-bubble {
  left: 50%;
  right: auto;
  top: auto;
  bottom: -4px;
  transform: translateX(-50%);
  max-width: 160px; /* 两倍宽度，更好展示心语内容 */
  white-space: normal;
  text-align: center; /* 居中对齐 */
  line-height: 1.25;
  padding: 3px 5px;
  border-radius: 4px;
  justify-content: center;
  overflow-wrap: break-word;
  z-index: 50;
}
/* 尾巴指向下方（朝向宠物） */
.quote-bubble::after {
  top: auto;
  bottom: -6px;
  left: 50%;
  transform: translateX(-50%);
  border-top: 6px solid #334155;
  border-left: 6px solid transparent;
  border-right: 6px solid transparent;
  border-bottom: none;
}
.quote-bubble::before {
  top: auto;
  bottom: -2.5px;
  left: 50%;
  transform: translateX(-50%);
  border-top: 3.5px solid #0F172A;
  border-left: 3.5px solid transparent;
  border-right: 3.5px solid transparent;
  border-bottom: none;
}

.quote-text {
  font-family: 'PingFang SC', 'Microsoft YaHei', sans-serif;
  font-size: 7.5px; /* 极限压缩字号，允许四行中文安全展示 */
  font-weight: 500; /* 反加粗，减少水平体积 */
  letter-spacing: normal; 
}

/* Vue 依靠基础的 transition 就能平滑处理，只需定义从/去状态 */
.bubble-fade-enter-from,
.bubble-fade-leave-to {
  opacity: 0;
  transform: scale(0.7) translateY(6px);
}

@keyframes float-bubble {
  from { transform: translateY(0); }
  to { transform: translateY(-4px); }
}

/* 高对比度的亮色系带点微发光 */
.bubble-fresh .bubble-val { color: #34D399; text-shadow: 0 0 3px rgba(52,211,153,0.3); }
.bubble-flow .bubble-val { color: #60A5FA; text-shadow: 0 0 3px rgba(96,165,250,0.3); }
.bubble-warning .bubble-val { color: #FBBF24; text-shadow: 0 0 3px rgba(251,191,36,0.3); }
.bubble-panic .bubble-val { color: #F87171; text-shadow: 0 0 3px rgba(248,113,113,0.4); }
.bubble-panic {
  /* 恐慌状态抖动 */
  animation: float-bubble 0.4s ease-in-out infinite alternate, shake-bubble 0.2s infinite;
}
.bubble-dead .bubble-val { color: #9CA3AF; }

@keyframes shake-bubble {
  0% { transform: translateX(0); }
  25% { transform: translateX(1px); }
  75% { transform: translateX(-1px); }
  100% { transform: translateX(0); }
}

/* ── Display Modes Base ── */
.token-mode { z-index: 20; pointer-events: none; transition: opacity 0.3s ease, transform 0.3s ease; }
.pet-widget.expanded .token-mode { opacity: 0; transform: scale(0.8); }

/* 1. Holo Bubble */
.holo-bubble {
  position: absolute; top: 15px; right: 4px;
  background: rgba(15, 23, 42, 0.85); border: 1px solid #475569;
  box-shadow: 0 0 6px rgba(0,0,0,0.5), inset 0 0 8px rgba(96,165,250,0.1);
  padding: 1px 4px; font-family: ui-monospace, SFMono-Regular, monospace;
  font-size: 8px; font-weight: 700; color: #94A3B8;
  border-radius: 3px; overflow: hidden;
  animation: holo-float 2.5s ease-in-out infinite alternate;
  display: flex; align-items: center; gap: 2px;
  z-index: 25;
}
.holo-bubble .scanlines {
  position: absolute; inset: 0; pointer-events: none;
  background: linear-gradient(to bottom, rgba(255,255,255,0), rgba(255,255,255,0) 50%, rgba(0,0,0,0.2) 50%, rgba(0,0,0,0.2));
  background-size: 100% 4px; opacity: 0.3;
}
.holo-bubble .holo-val {
  text-shadow: 0 0 4px currentColor; font-family: 'Press Start 2P', monospace; font-size: 10px;
  position: relative; z-index: 2;
}
.holo-bubble.state-fresh .holo-val { color: #34D399; }
.holo-bubble.state-flow .holo-val { color: #60A5FA; }
.holo-bubble.state-warning .holo-val { color: #FBBF24; }
.holo-bubble.state-panic .holo-val { color: #F87171; animation: glitch 0.3s infinite; }
.holo-bubble.state-dead .holo-val { color: #9CA3AF; }

@keyframes holo-float { 
  from { transform: translateY(0); box-shadow: 0 0 5px rgba(96,165,250,0.2); }
  to { transform: translateY(-3px); box-shadow: 0 0 12px rgba(96,165,250,0.4); } 
}

/* 2. Cyber Ring */
.cyber-ring {
  position: absolute; inset: 4px; pointer-events: none; z-index: 1;
}
.cr-svg {
  width: 100%; height: 100%; transform: rotate(-90deg); filter: drop-shadow(0 0 3px rgba(0,0,0,0.5));
}
.cr-bg-dashed {
  fill: none; stroke: #334155; stroke-width: 1.5; stroke-dasharray: 3 4;
  transform-origin: 50px 50px; animation: cr-spin 20s linear infinite;
}
.cr-progress {
  fill: none; stroke-width: 2; stroke-linecap: butt;
  transition: stroke-dashoffset 0.5s ease, stroke 0.5s ease;
}
.state-fresh .cr-progress { stroke: #34D399; filter: drop-shadow(0 0 4px #34D399); }
.state-flow .cr-progress { stroke: #60A5FA; filter: drop-shadow(0 0 4px #60A5FA); }
.state-warning .cr-progress { stroke: #FBBF24; filter: drop-shadow(0 0 4px #FBBF24); }
.state-panic .cr-progress { stroke: #F87171; filter: drop-shadow(0 0 6px #F87171); animation: cr-alarm 1s ease infinite alternate; }
.state-dead .cr-progress { stroke: #9CA3AF; }

.cr-center-val {
  position: absolute; bottom: 8px; right: 0px; font-family: 'Press Start 2P', monospace; font-size: 10px; font-weight: bold;
  background: rgba(15,23,42,0.9); padding: 2px 3px; border-radius: 3px; border: 1px solid #1E293B;
  z-index: 25;
}
.state-fresh .cr-center-val { color: #34D399; }
.state-flow .cr-center-val { color: #60A5FA; }
.state-warning .cr-center-val { color: #FBBF24; }
.state-panic .cr-center-val { color: #F87171; }
.state-dead .cr-center-val { color: #9CA3AF; }

@keyframes cr-spin { to { transform: rotate(360deg); } }
@keyframes cr-alarm { from { opacity: 0.6; } to { opacity: 1; stroke-width: 4; } }

/* 3. Aura Field */
.aura-field {
  position: absolute; inset: 4px; pointer-events: none; z-index: 0;
  display: flex; justify-content: center; align-items: center;
}
.aura-ripple {
  position: absolute; border-radius: 50%; opacity: 0;
  border: 1.5px solid; animation: aura-pulse 3s cubic-bezier(0.2, 0.8, 0.2, 1) infinite;
}
.aura-field .r1 { animation-delay: 0s; }
.aura-field .r2 { animation-delay: 1s; }
.aura-field .r3 { animation-delay: 2s; }
.state-fresh .aura-ripple { border-color: #34D399; }
.state-flow .aura-ripple { border-color: #60A5FA; }
.state-warning .aura-ripple { border-color: #FBBF24; animation-duration: 1.5s; }
.state-panic .aura-ripple { border-color: #F87171; animation-duration: 0.8s; border-width: 2px; }
.state-dead .aura-ripple { border-color: #6B7280; animation: none; opacity: 0.2; width: 40px; height: 40px; }

.aura-val {
  position: absolute; bottom: 8px; right: 0px; font-family: 'Press Start 2P', monospace; font-size: 10px;
  background: rgba(0,0,0,0.8); padding: 2px 3px; border-radius: 2px; border: 1px dashed;
  z-index: 25;
}
.state-fresh .aura-val { color: #34D399; border-color: #34D399; }
.state-flow .aura-val { color: #60A5FA; border-color: #60A5FA; }
.state-warning .aura-val { color: #FBBF24; border-color: #FBBF24; }
.state-panic .aura-val { color: #F87171; border-color: #F87171; }
.state-dead .aura-val { color: #9CA3AF; border-color: #9CA3AF; }

@keyframes aura-pulse {
  0% { width: 40px; height: 40px; opacity: 0.8; }
  100% { width: 100px; height: 100px; opacity: 0; }
}

/* 4. Energy Core - 像素格网格 */
.energy-core {
  position: absolute; bottom: 2px; right: 2px; pointer-events: none; z-index: 15;
  background: rgba(15,23,42,0.9); padding: 3px; border: 1px solid #334155; border-radius: 2px;
}
.ec-grid {
  display: grid; grid-template-columns: repeat(4, 1fr); gap: 1px;
}
.ec-pixel {
  width: 4px; height: 4px; background: #1E293B; transition: all 0.3s;
}
.state-fresh .ec-pixel.on { background: #34D399; box-shadow: 0 0 3px #34D399; }
.state-flow .ec-pixel.on { background: #60A5FA; box-shadow: 0 0 3px #60A5FA; }
.state-warning .ec-pixel.on { background: #FBBF24; box-shadow: 0 0 3px #FBBF24; }
.state-panic .ec-pixel.on { background: #F87171; box-shadow: 0 0 4px #F87171; animation: ec-flash 0.5s infinite alternate; }
.state-dead .ec-pixel.on { background: #6B7280; box-shadow: none; }

.ec-val {
  position: absolute; top: -20px; right: 0px; font-family: 'Press Start 2P', monospace; font-size: 10px;
  background: rgba(15,23,42,0.9); padding: 1px 3px; border-radius: 2px; border: 1px solid #1E293B;
}
.state-fresh .ec-val { color: #34D399; }
.state-flow .ec-val { color: #60A5FA; }
.state-warning .ec-val { color: #FBBF24; }
.state-panic .ec-val { color: #F87171; }
.state-dead .ec-val { color: #9CA3AF; }

@keyframes ec-flash { from { opacity: 0.5; } to { opacity: 1; } }

/* 5. Status Floater - 侧边进度条 */
.status-floater {
  position: absolute; left: 2px; top: 50%; transform: translateY(-50%); pointer-events: none; z-index: 15;
  display: flex; flex-direction: column; align-items: center; gap: 2px;
}
.sf-bar-container {
  width: 6px; height: 50px; background: rgba(30,41,59,0.8); border: 1px solid #475569; border-radius: 2px;
  overflow: hidden; position: relative;
}
.sf-bar-fill {
  position: absolute; bottom: 0; left: 0; right: 0;
  transition: height 0.5s ease, background 0.5s ease;
}
.state-fresh .sf-bar-fill { background: linear-gradient(to top, #34D399, #6EE7B7); }
.state-flow .sf-bar-fill { background: linear-gradient(to top, #3B82F6, #60A5FA); }
.state-warning .sf-bar-fill { background: linear-gradient(to top, #F59E0B, #FBBF24); }
.state-panic .sf-bar-fill { background: linear-gradient(to top, #EF4444, #F87171); animation: sf-flash 0.8s infinite alternate; }
.state-dead .sf-bar-fill { background: linear-gradient(to top, #4B5563, #6B7280); }

.sf-text {
  font-family: 'Press Start 2P', monospace; font-size: 10px; margin-top: 2px;
  background: rgba(15,23,42,0.9); padding: 1px 2px; border-radius: 2px; border: 1px solid #1E293B;
  white-space: nowrap;
}
.state-fresh .sf-text { color: #34D399; }
.state-flow .sf-text { color: #60A5FA; }
.state-warning .sf-text { color: #FBBF24; }
.state-panic .sf-text { color: #F87171; }
.state-dead .sf-text { color: #9CA3AF; }

@keyframes sf-flash { from { opacity: 0.7; } to { opacity: 1; } }
</style>
