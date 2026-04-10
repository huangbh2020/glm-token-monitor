<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useUsageState } from '../composables/useUsageState'
import { useTauriEvents } from '../composables/useTauriEvents'
import { useTheme } from '../composables/useTheme'

const { usageData, setupEventListener } = useTauriEvents()
const { petState } = useUsageState(
  computed(() => usageData.value.used),
  computed(() => usageData.value.total)
)
const { currentTheme, initTheme } = useTheme()

// 双指标数据
const timePercent = computed(() => usageData.value.time_percent ?? 0)
const tokensPercent = computed(() => usageData.value.tokens_percent ?? 0)
const timeRemaining = computed(() => usageData.value.time_remaining)

// 会员等级
const membershipLevel = computed(() => {
  const level = usageData.value.level || 'lite'
  const levelMap: Record<string, string> = {
    lite: 'LITE',
    standard: 'STD',
    premium: 'PRO',
    enterprise: 'ENT'
  }
  return levelMap[level] || level.toUpperCase()
})

// 格式化重置时间
function formatResetTime(timestamp?: number): string {
  if (!timestamp) return '--'
  const date = new Date(timestamp)
  return date.toLocaleString('zh-CN', { month: 'numeric', day: 'numeric', hour: '2-digit', minute: '2-digit' })
}

const timeResetTime = computed(() => formatResetTime(usageData.value.time_reset_time))
const tokensResetTime = computed(() => formatResetTime(usageData.value.tokens_reset_time))

// 工具使用详情
const usageDetails = computed(() => usageData.value.usage_details || [])

// 状态
const isRefreshing = ref(false)
const lastUpdateTime = ref<string>('')
const fetchError = ref<string>('')

// Event listener cleanup
let cleanup: (() => void) | undefined

// 手动刷新数据
async function refreshUsageData() {
  try {
    isRefreshing.value = true
    const data = await invoke<typeof usageData.value>('get_current_usage')
    usageData.value = data
    const now = new Date()
    lastUpdateTime.value = `${now.getHours().toString().padStart(2,'0')}:${now.getMinutes().toString().padStart(2,'0')}`
    fetchError.value = ''
  } catch (err) {
    fetchError.value = String(err)
    console.error('Refresh failed:', err)
  } finally {
    isRefreshing.value = false
  }
}

// 关闭窗口
async function closeWindow() {
  try {
    await invoke('close_info_panel')
  } catch (err) {
    console.error('Close window failed:', err)
  }
}

onMounted(async () => {
  await initTheme()
  refreshUsageData()
  setupEventListener().then((cleanupFn) => {
    cleanup = cleanupFn
  }).catch((err) => {
    console.error('Setup event listener failed:', err)
  })
})

onUnmounted(() => {
  cleanup?.()
})
</script>

<template>
  <div class="info-panel" :data-theme="currentTheme">
    <!-- 顶部栏 -->
    <header class="panel-header">
      <div class="header-left">
        <span class="member-badge" :class="`member-${usageData.level || 'lite'}`">{{ membershipLevel }}</span>
        <span class="panel-title">USAGE</span>
      </div>
      <div class="header-right">
        <span class="update-time">{{ lastUpdateTime || '--:--' }}</span>
        <button class="icon-btn" @click="refreshUsageData" :disabled="isRefreshing">
          <svg :class="{ spinning: isRefreshing }" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M23 4v6h-6M1 20v-6h6"/>
            <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/>
          </svg>
        </button>
        <button class="icon-btn close" @click="closeWindow">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 6L6 18M6 6l12 12"/>
          </svg>
        </button>
      </div>
    </header>

    <!-- 主内容区 -->
    <main class="panel-content">
      <!-- 状态指示器 -->
      <div class="status-bar" :class="`status-${petState.toLowerCase()}`">
        <div class="status-dot"></div>
        <span class="status-text">{{ petState.toUpperCase() }}</span>
      </div>

      <!-- 核心指标网格 -->
      <div class="metrics-grid">
        <!-- 5H Token -->
        <div class="metric-card">
          <div class="metric-header">
            <span class="metric-label">5H TOKEN</span>
            <span class="metric-value">{{ 100 - tokensPercent }}%</span>
          </div>
          <div class="metric-bar-container">
            <div class="metric-bar" :class="`bar-${petState.toLowerCase()}`" :style="{ width: tokensPercent + '%' }"></div>
          </div>
          <div class="metric-footer">
            <span class="metric-used">{{ tokensPercent }}% used</span>
            <span class="metric-reset">{{ tokensResetTime }}</span>
          </div>
        </div>

        <!-- 月度额度 -->
        <div class="metric-card">
          <div class="metric-header">
            <span class="metric-label">MONTHLY</span>
            <span class="metric-value">{{ timeRemaining }}/{{ usageData.total }}</span>
          </div>
          <div class="metric-bar-container">
            <div class="metric-bar" :class="`bar-${petState.toLowerCase()}`" :style="{ width: timePercent + '%' }"></div>
          </div>
          <div class="metric-footer">
            <span class="metric-used">{{ timePercent }}% used</span>
            <span class="metric-reset">{{ timeResetTime }}</span>
          </div>
        </div>
      </div>

      <!-- 工具详情 -->
      <div class="tools-section" v-if="usageDetails.length > 0">
        <div class="section-header">
          <span class="section-title">DETAILS</span>
          <span class="section-count">{{ usageDetails.length }} tools</span>
        </div>
        <div class="tools-list">
          <div v-for="detail in usageDetails" :key="detail.model_code" class="tool-row">
            <span class="tool-name">{{ detail.model_code }}</span>
            <span class="tool-usage">{{ detail.usage }}</span>
          </div>
        </div>
      </div>

      <!-- 错误提示 -->
      <div v-if="fetchError" class="error-bar">
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"/>
          <path d="M12 8v4M12 16h.01"/>
        </svg>
        <span>{{ fetchError }}</span>
      </div>
    </main>
  </div>
</template>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600;700&display=swap');

/* ── 基础容器 ── */
.info-panel {
  width: 100vw;
  height: 100vh;
  font-family: 'JetBrains Mono', 'SF Mono', 'Consolas', monospace;
  background: #0a0a0b;
  color: #e4e4e7;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* ── 浅色主题 ── */
.info-panel[data-theme="light"] {
  background: #f5f5f4;
  color: #1c1c1e;
}

.info-panel[data-theme="light"] .panel-header {
  background: #ffffff;
  border-bottom-color: #e4e4e7;
}

.info-panel[data-theme="light"] .panel-title {
  color: #737373;
}

.info-panel[data-theme="light"] .update-time {
  color: #a3a3a3;
}

.info-panel[data-theme="light"] .icon-btn {
  color: #737373;
}

.info-panel[data-theme="light"] .icon-btn:hover {
  background: #f5f5f4;
  color: #52525b;
}

.info-panel[data-theme="light"] .icon-btn.close:hover {
  background: #fef2f2;
  color: #ef4444;
}

.info-panel[data-theme="light"] .panel-content::-webkit-scrollbar-thumb {
  background: #d4d4d8;
}

.info-panel[data-theme="light"] .status-bar {
  background: #ffffff;
  border-color: #e4e4e7;
}

.info-panel[data-theme="light"] .metric-card,
.info-panel[data-theme="light"] .tools-section {
  background: #ffffff;
  border-color: #e4e4e7;
}

.info-panel[data-theme="light"] .metric-label,
.info-panel[data-theme="light"] .section-title {
  color: #a3a3a3;
}

.info-panel[data-theme="light"] .metric-value,
.info-panel[data-theme="light"] .tool-usage {
  color: #1c1c1e;
}

.info-panel[data-theme="light"] .metric-bar-container {
  background: #f5f5f4;
}

.info-panel[data-theme="light"] .metric-used {
  color: #737373;
}

.info-panel[data-theme="light"] .metric-reset {
  color: #a3a3a3;
}

.info-panel[data-theme="light"] .section-count {
  color: #a3a3a3;
}

.info-panel[data-theme="light"] .tool-row {
  background: #fafaf9;
  border-color: #e4e4e7;
}

.info-panel[data-theme="light"] .tool-name {
  color: #52525b;
}

.info-panel[data-theme="light"] .error-bar {
  background: #fef2f2;
  border-color: #fecaca;
  color: #dc2626;
}

/* ── 顶部栏 ── */
.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background: #111113;
  border-bottom: 1px solid #1c1c1e;
  flex-shrink: 0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.member-badge {
  font-size: 9px;
  font-weight: 700;
  padding: 3px 6px;
  border-radius: 3px;
  letter-spacing: 0.5px;
}

.member-lite { background: #27272a; color: #a1a1aa; }
.member-standard { background: #1e3a5f; color: #60a5fa; }
.member-premium { background: #3f3f46; color: #f4f4f5; }
.member-enterprise { background: #422006; color: #fbbf24; }

.info-panel[data-theme="light"] .member-lite { background: #f5f5f4; color: #737373; }
.info-panel[data-theme="light"] .member-standard { background: #eff6ff; color: #3b82f6; }
.info-panel[data-theme="light"] .member-premium { background: #fafaf9; color: #1c1c1e; }
.info-panel[data-theme="light"] .member-enterprise { background: #fef3c7; color: #d97706; }

.panel-title {
  font-size: 11px;
  font-weight: 600;
  color: #71717a;
  letter-spacing: 1px;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.update-time {
  font-size: 10px;
  color: #52525b;
}

.icon-btn {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: #71717a;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.15s;
}

.icon-btn:hover {
  background: #18181b;
  color: #a1a1aa;
}

.icon-btn.close:hover {
  background: #450a0a;
  color: #f87171;
}

.icon-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.spinning {
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* ── 内容区 ── */
.panel-content {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.panel-content::-webkit-scrollbar {
  width: 4px;
}

.panel-content::-webkit-scrollbar-track {
  background: transparent;
}

.panel-content::-webkit-scrollbar-thumb {
  background: #27272a;
  border-radius: 2px;
}

/* ── 状态栏 ── */
.status-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 14px;
  background: #111113;
  border: 1px solid #1c1c1e;
  border-radius: 6px;
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  animation: pulse 2s ease-in-out infinite;
}

.status-fresh .status-dot { background: #22c55e; box-shadow: 0 0 8px #22c55e; }
.status-flow .status-dot { background: #3b82f6; box-shadow: 0 0 8px #3b82f6; }
.status-warning .status-dot { background: #f59e0b; box-shadow: 0 0 8px #f59e0b; }
.status-panic .status-dot { background: #ef4444; box-shadow: 0 0 12px #ef4444; animation: pulse-fast 0.5s ease-in-out infinite; }
.status-dead .status-dot { background: #71717a; }

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

@keyframes pulse-fast {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.3; }
}

.status-text {
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 1px;
}

.status-fresh .status-text { color: #22c55e; }
.status-flow .status-text { color: #3b82f6; }
.status-warning .status-text { color: #f59e0b; }
.status-panic .status-text { color: #ef4444; }
.status-dead .status-text { color: #71717a; }

/* ── 指标网格 ── */
.metrics-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
}

.metric-card {
  background: #111113;
  border: 1px solid #1c1c1e;
  border-radius: 8px;
  padding: 14px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.metric-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.metric-label {
  font-size: 9px;
  font-weight: 600;
  color: #71717a;
  letter-spacing: 0.5px;
}

.metric-value {
  font-size: 18px;
  font-weight: 700;
  color: #e4e4e7;
  letter-spacing: -0.5px;
}

.metric-bar-container {
  height: 4px;
  background: #1c1c1e;
  border-radius: 2px;
  overflow: hidden;
}

.metric-bar {
  height: 100%;
  border-radius: 2px;
  transition: width 0.4s ease, background 0.4s ease;
}

.bar-fresh { background: #22c55e; }
.bar-flow { background: #3b82f6; }
.bar-warning { background: #f59e0b; }
.bar-panic { background: #ef4444; }
.bar-dead { background: #71717a; }

.metric-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.metric-used {
  font-size: 9px;
  color: #52525b;
}

.metric-reset {
  font-size: 9px;
  color: #3f3f46;
}

/* ── 工具详情 ── */
.tools-section {
  background: #111113;
  border: 1px solid #1c1c1e;
  border-radius: 8px;
  padding: 14px;
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 10px;
}

.section-title {
  font-size: 9px;
  font-weight: 600;
  color: #71717a;
  letter-spacing: 0.5px;
}

.section-count {
  font-size: 9px;
  color: #52525b;
}

.tools-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.tool-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 10px;
  background: #0a0a0b;
  border-radius: 4px;
  border: 1px solid #18181b;
}

.tool-name {
  font-size: 10px;
  font-weight: 500;
  color: #a1a1aa;
}

.tool-usage {
  font-size: 10px;
  font-weight: 600;
  color: #e4e4e7;
}

/* ── 错误提示 ── */
.error-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 14px;
  background: #450a0a;
  border: 1px solid #7f1d1d;
  border-radius: 6px;
  color: #fca5a5;
  font-size: 10px;
}

.error-bar svg {
  flex-shrink: 0;
}
</style>
