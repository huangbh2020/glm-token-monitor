<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useTauriEvents } from '../composables/useTauriEvents'
import { useTheme } from '../composables/useTheme'

const { usageData, setupEventListener } = useTauriEvents()
const { currentTheme, initTheme } = useTheme()

// 双指标数据
const timePercent = computed(() => usageData.value.time_percent ?? 0)
const tokensPercent = computed(() => usageData.value.tokens_percent ?? 0)

// 根据百分比获取进度条颜色类
function getBarClass(percent: number): string {
  if (percent >= 96) return 'bar-dead'
  if (percent >= 81) return 'bar-panic'
  if (percent >= 61) return 'bar-warning'
  return 'bar-flow'
}

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
const isMonthlyExpanded = ref(false)

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
        <span class="panel-title">使用量</span>
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
      <!-- <div class="status-bar" :class="`status-${petState.toLowerCase()}`">
        <div class="status-dot"></div>
        <span class="status-text">{{ petState.toUpperCase() }}</span>
      </div> -->

      <!-- 核心指标网格 -->
      <div class="metrics-grid">
        <!-- 5小时Token -->
        <div class="metric-card">
          <div class="metric-header">
            <span class="metric-label">5小时Token</span>
            <span class="metric-value">{{ 100 - tokensPercent }}%</span>
          </div>
          <div class="metric-bar-container">
            <div class="metric-bar" :class="getBarClass(tokensPercent)" :style="{ width: tokensPercent + '%' }"></div>
          </div>
          <div class="metric-footer">
            <span class="metric-used">已用 {{ tokensPercent }}%</span>
            <span class="metric-reset">下次刷新：{{ tokensResetTime }}</span>
          </div>
        </div>

        <!-- 月度额度 -->
        <div class="metric-card clickable" :class="{ expanded: isMonthlyExpanded }" @click="isMonthlyExpanded = !isMonthlyExpanded">
          <div class="metric-header">
            <div class="metric-label-row">
              <span class="metric-label">MCP额度</span>
              <svg class="expand-icon" :class="{ rotated: isMonthlyExpanded }" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="6 9 12 15 18 9"></polyline>
              </svg>
            </div>
            <span class="metric-value">{{ timePercent }}%</span>
          </div>
          <div class="metric-bar-container">
            <div class="metric-bar" :class="getBarClass(timePercent)" :style="{ width: timePercent + '%' }"></div>
          </div>
          <div class="metric-footer">
            <span class="metric-used">已用 {{ timePercent }}%</span>
            <span class="metric-reset">下次刷新：{{ timeResetTime }}</span>
          </div>
          <!-- 展开的详情 -->
          <div class="metric-details" v-if="isMonthlyExpanded && usageDetails.length > 0">
            <div class="details-list">
              <div v-for="detail in usageDetails" :key="detail.model_code" class="detail-row">
                <span class="detail-name">{{ detail.model_code }}</span>
                <span class="detail-usage">{{ detail.usage }}</span>
              </div>
            </div>
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
  background: linear-gradient(180deg, #0d0d0f 0%, #0a0a0b 100%);
  color: #e4e4e7;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* ── 浅色主题 ── */
.info-panel[data-theme="light"] {
  background: linear-gradient(180deg, #fafaf9 0%, #f5f5f4 100%);
  color: #1c1c1e;
}

.info-panel[data-theme="light"] .panel-header {
  background: #ffffff;
  border-bottom-color: #e5e5e5;
}

.info-panel[data-theme="light"] .panel-title {
  color: #52525b;
}

.info-panel[data-theme="light"] .update-time {
  color: #a3a3a3;
}

.info-panel[data-theme="light"] .icon-btn {
  color: #737373;
}

.info-panel[data-theme="light"] .icon-btn:hover {
  background: #f5f5f4;
  color: #404040;
}

.info-panel[data-theme="light"] .icon-btn.close:hover {
  background: #fef2f2;
  color: #ef4444;
}

.info-panel[data-theme="light"] .panel-content::-webkit-scrollbar-thumb {
  background: #d4d4d8;
}

.info-panel[data-theme="light"] .metric-card {
  background: #ffffff;
  border-color: #e5e5e5;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.info-panel[data-theme="light"] .metric-card.clickable:hover {
  border-color: #d4d4d8;
}

.info-panel[data-theme="light"] .expand-icon {
  color: #a3a3a3;
}

.info-panel[data-theme="light"] .metric-details {
  border-top-color: #f0f0f0;
}

.info-panel[data-theme="light"] .detail-row {
  background: #fafaf9;
  border-color: #e5e5e5;
}

.info-panel[data-theme="light"] .detail-name {
  color: #52525b;
}

.info-panel[data-theme="light"] .detail-usage {
  color: #1c1c1e;
}

.info-panel[data-theme="light"] .metric-label {
  color: #737373;
}

.info-panel[data-theme="light"] .metric-value {
  color: #171717;
}

.info-panel[data-theme="light"] .metric-bar-container {
  background: #f0f0f0;
}

.info-panel[data-theme="light"] .metric-used {
  color: #52525b;
}

.info-panel[data-theme="light"] .metric-reset {
  color: #a3a3a3;
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
  padding: 14px 16px;
  background: rgba(17, 17, 19, 0.8);
  backdrop-filter: blur(10px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  flex-shrink: 0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 10px;
}

.member-badge {
  font-size: 8px;
  font-weight: 700;
  padding: 4px 8px;
  border-radius: 4px;
  letter-spacing: 0.8px;
  text-transform: uppercase;
}

.member-lite {
  background: linear-gradient(135deg, #27272a 0%, #1f1f22 100%);
  color: #a1a1aa;
  border: 1px solid #3f3f46;
}
.member-standard {
  background: linear-gradient(135deg, #1e3a5f 0%, #1a3454 100%);
  color: #60a5fa;
  border: 1px solid #2563eb;
}
.member-premium {
  background: linear-gradient(135deg, #3f3f46 0%, #333338 100%);
  color: #f4f4f5;
  border: 1px solid #52525b;
}
.member-enterprise {
  background: linear-gradient(135deg, #422006 0%, #361a05 100%);
  color: #fbbf24;
  border: 1px solid #92400e;
}

.info-panel[data-theme="light"] .member-lite {
  background: #f5f5f4;
  color: #737373;
  border-color: #d4d4d4;
}
.info-panel[data-theme="light"] .member-standard {
  background: #eff6ff;
  color: #3b82f6;
  border-color: #bfdbfe;
}
.info-panel[data-theme="light"] .member-premium {
  background: #fafaf9;
  color: #1c1c1e;
  border-color: #e5e5e5;
}
.info-panel[data-theme="light"] .member-enterprise {
  background: #fef3c7;
  color: #d97706;
  border-color: #fcd34d;
}

.panel-title {
  font-size: 12px;
  font-weight: 600;
  color: #8a8a8e;
  letter-spacing: 0.5px;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.update-time {
  font-size: 10px;
  color: #52525b;
  font-weight: 500;
}

.icon-btn {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: #71717a;
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.2s ease;
}

.icon-btn:hover {
  background: rgba(255, 255, 255, 0.08);
  color: #d4d4d8;
}

.icon-btn.close:hover {
  background: rgba(239, 68, 68, 0.15);
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
  padding: 12px 0;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.panel-content::-webkit-scrollbar {
  width: 3px;
}

.panel-content::-webkit-scrollbar-track {
  background: transparent;
}

.panel-content::-webkit-scrollbar-thumb {
  background: #27272a;
  border-radius: 3px;
}

/* ── 指标网格 ── */
.metrics-grid {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 0 12px;
}

.metric-card {
  background: rgba(24, 24, 27, 0.6);
  border: 1px solid rgba(63, 63, 70, 0.3);
  border-radius: 12px;
  padding: 14px 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  transition: all 0.25s ease;
}

.metric-card.clickable {
  cursor: pointer;
}

.metric-card.clickable:hover {
  border-color: rgba(63, 63, 70, 0.5);
}

.metric-card.clickable.expanded {
  border-color: rgba(63, 63, 70, 0.5);
}

.metric-label-row {
  display: flex;
  align-items: center;
  gap: 6px;
}

.expand-icon {
  color: #52525b;
  transition: transform 0.25s ease;
}

.expand-icon.rotated {
  transform: rotate(180deg);
}

.metric-details {
  margin-top: 4px;
  padding-top: 12px;
  border-top: 1px solid rgba(63, 63, 70, 0.3);
}

.details-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.detail-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  background: rgba(10, 10, 11, 0.5);
  border-radius: 6px;
  border: 1px solid rgba(24, 24, 27, 0.8);
  transition: background 0.2s;
}

.detail-row:hover {
  background: rgba(24, 24, 27, 0.5);
}

.detail-name {
  font-size: 10px;
  font-weight: 500;
  color: #a1a1aa;
}

.detail-usage {
  font-size: 10px;
  font-weight: 600;
  color: #e4e4e7;
}

.metric-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.metric-label {
  font-size: 10px;
  font-weight: 600;
  color: #71717a;
  letter-spacing: 0.3px;
  text-transform: uppercase;
}

.metric-value {
  font-size: 20px;
  font-weight: 700;
  color: #f4f4f5;
  letter-spacing: -0.5px;
}

.metric-bar-container {
  height: 6px;
  background: rgba(39, 39, 42, 0.8);
  border-radius: 3px;
  overflow: hidden;
}

.metric-bar {
  height: 100%;
  border-radius: 3px;
  transition: width 0.5s cubic-bezier(0.4, 0, 0.2, 1), background 0.4s ease;
}

.bar-flow {
  background: linear-gradient(90deg, #3b82f6 0%, #60a5fa 100%);
  box-shadow: 0 0 8px rgba(59, 130, 246, 0.4);
}
.bar-warning {
  background: linear-gradient(90deg, #f59e0b 0%, #fbbf24 100%);
  box-shadow: 0 0 8px rgba(245, 158, 11, 0.4);
}
.bar-panic {
  background: linear-gradient(90deg, #ef4444 0%, #f87171 100%);
  box-shadow: 0 0 12px rgba(239, 68, 68, 0.5);
}
.bar-dead {
  background: linear-gradient(90deg, #52525b 0%, #71717a 100%);
}

.metric-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.metric-used {
  font-size: 9px;
  color: #71717a;
  font-weight: 500;
}

.metric-reset {
  font-size: 9px;
  color: #52525b;
}

/* ── 错误提示 ── */
.error-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 14px;
  margin: 0 12px;
  background: rgba(127, 29, 29, 0.3);
  border: 1px solid rgba(239, 68, 68, 0.3);
  border-radius: 8px;
  color: #fca5a5;
  font-size: 10px;
}

.error-bar svg {
  flex-shrink: 0;
}
</style>
