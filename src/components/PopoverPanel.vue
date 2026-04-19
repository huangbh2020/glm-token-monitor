<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import { useUsageState } from '../composables/useUsageState'
import { useTauriEvents } from '../composables/useTauriEvents'
import { useDisplayMode } from '../composables/useDisplayMode'
import { useSettings } from '../composables/useSettings'
import { usePetAction } from '../composables/usePetAction'
import { useTheme } from '../composables/useTheme'
import type { PetType } from '../types/config'
import CatGifViewer from './pets/CatGifViewer.vue'
import DogSit from './pets/DogSit.vue'
import DogBark from './pets/DogBark.vue'
import DogWalk from './pets/DogWalk.vue'
import DogBeg from './pets/DogBeg.vue'
import JellySpirit from './pets/JellySpirit.vue'
import PixelGhost from './pets/PixelGhost.vue'

const { displayMode } = useDisplayMode()
const { loadConfig, setupConfigListener, config, basicConfig, hasApiKey } = useSettings()
const { usageData, setupEventListener } = useTauriEvents()
const { currentTheme } = useTheme()

const showGlowEffect = computed(() => basicConfig.value?.enable_glow ?? true)
const { usagePercent, petState, gradientColor, gradientStrokeColor } = useUsageState(
  computed(() => usageData.value.used),
  computed(() => usageData.value.total)
)

const { petType, currentAction, setPetType } = usePetAction()

watch(() => config.value?.pet_config?.selected_pet, (newPet) => {
  if (newPet && newPet !== petType.value) {
    setPetType(newPet as PetType)
  }
})

const petComponents = {
  'dog-sit': DogSit,
  'dog-bark': DogBark,
  'dog-walk': DogWalk,
  'dog-beg': DogBeg
} as const

const timePercent = computed(() => usageData.value.time_percent ?? 0)
const tokensPercent = computed(() => usageData.value.tokens_percent ?? 0)
const weeklyTokensPercent = computed(() => usageData.value.weekly_tokens_percent ?? 0)

const hasWeeklyLimit = computed(() => {
  return usageData.value.weekly_tokens_reset_time !== undefined && usageData.value.weekly_tokens_reset_time !== null
})

function formatResetTime(timestamp?: number): string {
  if (!timestamp) return '--'
  const date = new Date(timestamp)
  const now = new Date()
  const isToday = date.getDate() === now.getDate() && date.getMonth() === now.getMonth()
  if (isToday) {
    return `今天 ${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}`
  }
  return `${date.getMonth() + 1}/${date.getDate()} ${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}`
}

const tokensResetTime = computed(() => formatResetTime(usageData.value.tokens_reset_time))
const weeklyTokensResetTime = computed(() => formatResetTime(usageData.value.weekly_tokens_reset_time))
const timeResetTime = computed(() => formatResetTime(usageData.value.time_reset_time))

const isRefreshing = ref(false)
const lastUpdateTime = ref<string>('')
const fetchError = ref<string>('')
const nextRefreshTime = ref<string>('')
const nextRefreshCountdown = ref(60)

function updateCountdown() {
  if (nextRefreshCountdown.value > 0) {
    nextRefreshCountdown.value--
  }
  if (nextRefreshCountdown.value <= 0) {
    nextRefreshCountdown.value = 60
  }
  nextRefreshTime.value = `${nextRefreshCountdown.value}s`
}

const showInfoPanel = ref(false)

// 拖动相关状态
const isDragging = ref(false)
let dragStartTime = 0
let dragStartPos = { x: 0, y: 0 }

const startDrag = async (event: MouseEvent) => {
  event.preventDefault()
  isDragging.value = true
  dragStartTime = Date.now()
  dragStartPos = { x: event.clientX, y: event.clientY }

  try {
    const { Window } = await import('@tauri-apps/api/window')
    const win = Window.getCurrent()
    await win.startDragging()
  } catch (error) {
    console.error('[Drag] startDragging failed:', error)
  } finally {
    setTimeout(() => {
      isDragging.value = false
    }, 200)
  }
}

const handleClick = async (event: MouseEvent) => {
  const dragDuration = Date.now() - dragStartTime
  const dragDistance = Math.sqrt(
    Math.pow(event.clientX - dragStartPos.x, 2) +
    Math.pow(event.clientY - dragStartPos.y, 2)
  )

  if (dragDistance < 5 && dragDuration < 300) {
    if (!hasApiKey.value) {
      try {
        const { invoke } = await import('@tauri-apps/api/core')
        await invoke('open_settings_panel')
      } catch (err) {
        console.error('Open settings failed:', err)
      }
    } else {
      showInfoPanel.value = !showInfoPanel.value
    }
  }
}

const handleDblClick = (event: MouseEvent) => {
  event.preventDefault()
  event.stopPropagation()
}

function togglePanel() {
  if (!hasApiKey.value) {
    openSettings()
    return
  }
  showInfoPanel.value = !showInfoPanel.value
}

function closeInfoPanel() {
  showInfoPanel.value = false
}

async function openSettings() {
  showInfoPanel.value = false
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('open_settings_panel')
  } catch (err) {
    console.error('Open settings failed:', err)
  }
}

function getStatusColor(percent: number): string {
  if (percent >= 96) return '#6B7280'
  if (percent >= 81) return '#F97316'
  if (percent >= 61) return '#F59E0B'
  return '#3B82F6'
}

async function refreshUsageData() {
  try {
    isRefreshing.value = true
    const { invoke } = await import('@tauri-apps/api/core')
    const data = await invoke<typeof usageData.value>('get_current_usage')
    usageData.value = data
    const now = new Date()
    lastUpdateTime.value = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}`
    fetchError.value = ''
    nextRefreshCountdown.value = 60
  } catch (err) {
    fetchError.value = String(err)
    console.error('Silent refresh failed:', err)
  } finally {
    isRefreshing.value = false
  }
}

const DATA_REFRESH_INTERVAL = 60000
let dataRefreshTimer: number | null = null
let countdownTimer: number | null = null

function setupDataRefreshTimer() {
  refreshUsageData()
  dataRefreshTimer = window.setInterval(() => {
    refreshUsageData()
  }, DATA_REFRESH_INTERVAL)
  countdownTimer = window.setInterval(() => {
    updateCountdown()
  }, 1000)
}

// 监听状态变化，动态调整窗口大小（与 PetWidget 一致）
watch([hasApiKey, showInfoPanel, displayMode, hasWeeklyLimit], async ([hasKey, showPanel, mode, hasWeekly]) => {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    if (!hasKey) {
      await invoke('resize_main_window', { width: 160, height: 180 })
    } else if (showPanel) {
      const panelHeight = hasWeeklyLimit.value ? 220 : 180
      await invoke('resize_main_window', { width: 154, height: panelHeight })
    } else if (mode && mode !== 'none') {
      await invoke('resize_main_window', { width: 120, height: 120 })
    } else {
      await invoke('resize_main_window', { width: 120, height: 120 })
    }
  } catch (err) {
    console.error('Failed to resize window:', err)
  }
}, { immediate: true })

onMounted(async () => {
  setupDataRefreshTimer()

  try {
    await setupEventListener()
  } catch (err) {
    console.error('setupEventListener failed:', err)
  }

  try {
    await loadConfig()
    await setupConfigListener()
  } catch (err) {
    console.error('Config initialized failed:', err)
  }

  try {
    const { Window } = await import('@tauri-apps/api/window')
    const win = Window.getCurrent()
    await win.setAlwaysOnTop(true)
  } catch (err) {
    console.error('Enforce always on top failed:', err)
  }
})

onUnmounted(() => {
  if (dataRefreshTimer) clearInterval(dataRefreshTimer)
  if (countdownTimer) clearInterval(countdownTimer)
})
</script>

<template>
  <div class="popover-widget" :class="[`pet-${petState.toLowerCase()}`, { 'show-panel': showInfoPanel }]"
    @mousedown="startDrag"
    @click="handleClick"
    @dblclick.prevent="handleDblClick">
    <!-- 宠物区域 -->
    <div class="pet-container" :class="{ hidden: showInfoPanel && hasApiKey }">
      <JellySpirit v-if="petType === 'spirit'" :color="gradientColor" :stroke-color="gradientStrokeColor" :state="petState" :width="80" :height="80" />
      <PixelGhost v-else-if="petType === 'ghost'" :color="gradientColor" :stroke-color="gradientStrokeColor" :state="petState" :width="80" :height="80" />
      <CatGifViewer v-else-if="currentAction.startsWith('cat-')" :action="currentAction" :width="80" :height="80" />
      <component v-else :is="petComponents[currentAction as keyof typeof petComponents]" :key="currentAction" />

      <!-- 5 种展示模式 -->
      <div v-if="displayMode === 'holo-bubble'" class="holo-bubble token-mode" :class="`state-${petState.toLowerCase()}`">
        <span class="holo-val">{{ 100 - tokensPercent }}%</span>
      </div>
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
      <div v-else-if="displayMode === 'aura-field'" class="aura-field token-mode" :class="`state-${petState.toLowerCase()}`">
        <div class="aura-ripple r1"></div>
        <div class="aura-ripple r2"></div>
        <div class="aura-ripple r3"></div>
        <div class="aura-val">{{ 100 - tokensPercent }}%</div>
      </div>
      <div v-else-if="displayMode === 'energy-core'" class="energy-core token-mode" :class="`state-${petState.toLowerCase()}`">
        <div class="ec-grid">
          <div v-for="i in 16" :key="i" class="ec-pixel" :class="{ on: (100 - tokensPercent) >= (i * 6.25 - 3.125) }"></div>
        </div>
        <div class="ec-val">{{ 100 - tokensPercent }}%</div>
      </div>
      <div v-else-if="displayMode === 'status-floater'" class="status-floater token-mode" :class="`state-${petState.toLowerCase()}`">
        <div class="sf-bar-container">
          <div class="sf-bar-fill" :style="{ height: (100 - tokensPercent) + '%' }"></div>
        </div>
        <div class="sf-text">{{ 100 - tokensPercent }}%</div>
      </div>
    </div>

    <!-- API 配置提示气泡（未配置时显示） -->
    <transition name="bubble-fade">
      <div v-if="!hasApiKey" class="api-config-bubble"
        @mousedown.stop
        @click.stop="openSettings"
        @dblclick.stop>
        <span class="bubble-icon">🔑</span>
        <span class="bubble-text">配置 API Key</span>
        <span class="bubble-arrow">→</span>
      </div>
    </transition>

    <!-- 信息面板气泡 -->
    <transition name="panel-slide">
      <div v-if="showInfoPanel && hasApiKey" class="info-bubble" :data-theme="currentTheme"
        @click.stop>
        <!-- 顶部栏 -->
        <div class="info-header">
          <div class="info-header-left">
            <span class="info-time">{{ lastUpdateTime || '--:--' }}</span>
            <span class="info-countdown">{{ nextRefreshTime }}后刷新</span>
          </div>
          <div class="info-actions">
            <button class="info-btn" @mousedown.stop @click="refreshUsageData" :disabled="isRefreshing" title="刷新">
              <svg :class="{ spinning: isRefreshing }" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                <path d="M23 4v6h-6M1 20v-6h6"/>
                <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/>
              </svg>
            </button>
            <button class="info-btn" @mousedown.stop @click="openSettings" title="设置">
              <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="3"/>
                <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
              </svg>
            </button>
            <button class="info-btn close" @mousedown.stop @click="closeInfoPanel" title="关闭">
              <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                <path d="M18 6L6 18M6 6l12 12"/>
              </svg>
            </button>
          </div>
        </div>

        <!-- 错误提示 -->
        <div v-if="fetchError" class="info-error">
          <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <path d="M12 8v4M12 16h.01"/>
          </svg>
          <span>{{ fetchError.slice(0, 15) }}</span>
        </div>

        <!-- 数据区域 -->
        <div class="info-data">
          <!-- 5h Token -->
          <div class="info-row">
            <div class="info-row-header">
              <span class="info-label">5h Token</span>
              <span class="info-val" :style="{ color: getStatusColor(tokensPercent) }">{{ 100 - tokensPercent }}%</span>
            </div>
            <div class="info-bar">
              <div class="bar-fill" :style="{ width: tokensPercent + '%', background: getStatusColor(tokensPercent) }"></div>
            </div>
            <span class="info-reset">刷新: {{ tokensResetTime }}</span>
          </div>
          <!-- 周限制 -->
          <div v-if="hasWeeklyLimit" class="info-row">
            <div class="info-row-header">
              <span class="info-label">周限制</span>
              <span class="info-val" :style="{ color: getStatusColor(weeklyTokensPercent) }">{{ 100 - weeklyTokensPercent }}%</span>
            </div>
            <div class="info-bar">
              <div class="bar-fill" :style="{ width: weeklyTokensPercent + '%', background: getStatusColor(weeklyTokensPercent) }"></div>
            </div>
            <span class="info-reset">刷新: {{ weeklyTokensResetTime }}</span>
          </div>
          <!-- MCP 额度 -->
          <div class="info-row">
            <div class="info-row-header">
              <span class="info-label">MCP额度</span>
              <span class="info-val" :style="{ color: getStatusColor(timePercent) }">{{ 100 - timePercent }}%</span>
            </div>
            <div class="info-bar">
              <div class="bar-fill" :style="{ width: timePercent + '%', background: getStatusColor(timePercent) }"></div>
            </div>
            <span class="info-reset">刷新: {{ timeResetTime }}</span>
          </div>
        </div>
      </div>
    </transition>
  </div>
</template>

<style scoped>
/* ── 基础容器 ── */
.popover-widget {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  background: transparent !important;
  position: relative;
  cursor: pointer;
  user-select: none;
  pointer-events: auto;
  border-radius: 8px;
  overflow: visible;
  padding-top: 8px;
}

.popover-widget.show-panel {
  padding-top: 0;
  align-items: center;
  justify-content: center;
}

.pet-container {
  position: relative;
  z-index: 2;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: opacity 0.25s ease, transform 0.25s ease;
  width: 80px;
  height: 80px;
  cursor: pointer;
}

.pet-container.hidden {
  opacity: 0;
  transform: scale(0.8);
  pointer-events: none;
  position: absolute;
  width: 0;
  height: 0;
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

/* ── API 配置提示气泡 ── */
.api-config-bubble {
  position: absolute;
  left: 50%;
  bottom: 4px;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  cursor: pointer;
  background: rgba(15, 23, 42, 0.95);
  border: 1px solid rgba(59, 130, 246, 0.4);
  border-radius: 16px;
  z-index: 1000;
  animation: bubbleIn 0.4s cubic-bezier(0.34, 1.56, 0.64, 1), bubbleFloat 3s ease-in-out infinite;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4), 0 0 8px rgba(59, 130, 246, 0.2);
  backdrop-filter: blur(8px);
  pointer-events: auto;
  white-space: nowrap;
}

.api-config-bubble::before {
  content: '';
  position: absolute;
  top: -5px;
  left: 50%;
  transform: translateX(-50%);
  width: 0;
  height: 0;
  border-left: 5px solid transparent;
  border-right: 5px solid transparent;
  border-bottom: 5px solid rgba(59, 130, 246, 0.4);
}

.api-config-bubble::after {
  content: '';
  position: absolute;
  top: -3px;
  left: 50%;
  transform: translateX(-50%);
  width: 0;
  height: 0;
  border-left: 4px solid transparent;
  border-right: 4px solid transparent;
  border-bottom: 4px solid rgba(15, 23, 42, 0.95);
}

.api-config-bubble:hover {
  background: rgba(30, 41, 59, 0.98);
  border-color: rgba(59, 130, 246, 0.6);
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.5), 0 0 16px rgba(59, 130, 246, 0.3);
  transform: translateX(-50%) translateY(-2px);
}

.api-config-bubble .bubble-icon {
  font-size: 14px;
  animation: keyWiggle 1s ease-in-out infinite;
}

.api-config-bubble .bubble-text {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  font-size: 11px;
  font-weight: 600;
  color: #60a5fa;
}

.api-config-bubble .bubble-arrow {
  font-size: 12px;
  color: #60a5fa;
  transition: transform 0.2s ease;
}

.api-config-bubble:hover .bubble-arrow {
  transform: translateX(3px);
}

@keyframes bubbleIn {
  from {
    opacity: 0;
    transform: translateX(-50%) translateY(8px) scale(0.9);
  }
  to {
    opacity: 1;
    transform: translateX(-50%) translateY(0) scale(1);
  }
}

@keyframes bubbleFloat {
  0%, 100% { transform: translateX(-50%) translateY(0); }
  50% { transform: translateX(-50%) translateY(-3px); }
}

@keyframes keyWiggle {
  0%, 100% { transform: rotate(0deg); }
  25% { transform: rotate(-8deg); }
  75% { transform: rotate(8deg); }
}

/* ── Display Modes Base ── */
.token-mode { z-index: 20; pointer-events: none; transition: opacity 0.25s ease, transform 0.25s ease; position: absolute; }

/* 1. Holo Bubble */
.holo-bubble {
  top: 0; right: -10px;
  background: rgba(15, 23, 42, 0.85); border: 1px solid #475569;
  box-shadow: 0 0 6px rgba(0,0,0,0.5), inset 0 0 8px rgba(96,165,250,0.1);
  padding: 1px 4px; font-family: ui-monospace, SFMono-Regular, monospace;
  font-size: 8px; font-weight: 700; color: #94A3B8;
  border-radius: 3px; overflow: hidden;
  animation: holo-float 2.5s ease-in-out infinite alternate;
  display: flex; align-items: center; gap: 2px;
  z-index: 25;
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
  inset: -10px; pointer-events: none; z-index: 1;
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
  position: absolute; bottom: -8px; right: -12px; font-family: 'Press Start 2P', monospace; font-size: 10px; font-weight: bold;
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
  inset: -10px; pointer-events: none; z-index: 0;
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
  position: absolute; bottom: -8px; right: -12px; font-family: 'Press Start 2P', monospace; font-size: 10px;
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

/* 4. Energy Core */
.energy-core {
  bottom: -10px; right: -10px; pointer-events: none; z-index: 15;
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
  position: absolute; top: -16px; right: 0px; font-family: 'Press Start 2P', monospace; font-size: 10px;
  background: rgba(15,23,42,0.9); padding: 1px 3px; border-radius: 2px; border: 1px solid #1E293B;
}
.state-fresh .ec-val { color: #34D399; }
.state-flow .ec-val { color: #60A5FA; }
.state-warning .ec-val { color: #FBBF24; }
.state-panic .ec-val { color: #F87171; }
.state-dead .ec-val { color: #9CA3AF; }

@keyframes ec-flash { from { opacity: 0.5; } to { opacity: 1; } }

/* 5. Status Floater */
.status-floater {
  left: -12px; top: 50%; transform: translateY(-50%); pointer-events: none; z-index: 15;
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

/* ── 信息面板气泡 ── */
.info-bubble {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  width: 150px;
  background: rgba(15, 15, 17, 0.96);
  border: 1px solid rgba(59, 130, 246, 0.3);
  border-radius: 12px;
  padding: 10px 12px;
  z-index: 100;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5), 0 0 12px rgba(59, 130, 246, 0.15);
  backdrop-filter: blur(12px);
  pointer-events: auto;
  max-height: calc(100% - 10px);
  overflow: hidden;
}

.info-bubble[data-theme="light"] {
  background: rgba(255, 255, 255, 0.98);
  border-color: rgba(59, 130, 246, 0.25);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.12), 0 0 12px rgba(59, 130, 246, 0.1);
}

.info-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
}

.info-header-left {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.info-time {
  font-size: 11px;
  font-weight: 500;
  color: #52525b;
  font-family: ui-monospace, monospace;
}

.info-bubble[data-theme="light"] .info-time {
  color: #737373;
}

.info-countdown {
  font-size: 10px;
  color: #3b82f6;
  font-weight: 500;
}

.info-bubble[data-theme="light"] .info-countdown {
  color: #2563eb;
}

.info-actions {
  display: flex;
  gap: 4px;
}

.info-btn {
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: #71717a;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.15s ease;
}

.info-bubble[data-theme="light"] .info-btn {
  color: #6b7280;
}

.info-btn:hover {
  background: rgba(255, 255, 255, 0.08);
  color: #d4d4d8;
}

.info-bubble[data-theme="light"] .info-btn:hover {
  background: rgba(0, 0, 0, 0.05);
  color: #374151;
}

.info-btn.close:hover {
  background: rgba(239, 68, 68, 0.15);
  color: #f87171;
}

.info-bubble[data-theme="light"] .info-btn.close:hover {
  background: rgba(239, 68, 68, 0.1);
}

.info-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.spinning {
  animation: spin 0.7s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.info-error {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
  padding: 6px;
  margin-bottom: 8px;
  background: rgba(239, 68, 68, 0.1);
  border-radius: 6px;
  color: #fca5a5;
  font-size: 11px;
}

.info-bubble[data-theme="light"] .info-error {
  background: rgba(239, 68, 68, 0.08);
  color: #ef4444;
}

.info-data {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.info-row {
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.info-row-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.info-label {
  font-size: 11px;
  font-weight: 500;
  color: #71717a;
}

.info-bubble[data-theme="light"] .info-label {
  color: #6b7280;
}

.info-reset {
  font-size: 10px;
  color: #52525b;
}

.info-bubble[data-theme="light"] .info-reset {
  color: #9ca3af;
}

.info-val {
  font-size: 16px;
  font-weight: 700;
}

.info-bar {
  height: 3px;
  background: rgba(255, 255, 255, 0.08);
  border-radius: 2px;
  overflow: hidden;
}

.info-bubble[data-theme="light"] .info-bar {
  background: rgba(0, 0, 0, 0.06);
}

.bar-fill {
  height: 100%;
  border-radius: 2px;
  transition: width 0.4s ease;
}

/* ── 面板滑入动画 ── */
.panel-slide-enter-active,
.panel-slide-leave-active {
  transition: all 0.25s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.panel-slide-enter-from,
.panel-slide-leave-to {
  opacity: 0;
  transform: translate(-50%, -50%) scale(0.9);
}

/* ── 气泡淡入淡出 ── */
.bubble-fade-enter-active,
.bubble-fade-leave-active {
  transition: all 0.3s ease;
}

.bubble-fade-enter-from,
.bubble-fade-leave-to {
  opacity: 0;
  transform: translateX(-50%) scale(0.9);
}
</style>
