<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useTauriEvents } from '../composables/useTauriEvents'
import { useTheme } from '../composables/useTheme'
import { useSettings } from '../composables/useSettings'

const { usageData, setupEventListener } = useTauriEvents()
const { currentTheme, initTheme } = useTheme()
const { hasApiKey } = useSettings()

// 双指标数据
const timePercent = computed(() => usageData.value.time_percent ?? 0)
const tokensPercent = computed(() => usageData.value.tokens_percent ?? 0)

// 根据百分比获取状态颜色
function getStatusColor(percent: number): string {
  if (percent >= 96) return '#6B7280'
  if (percent >= 81) return '#F97316'
  if (percent >= 61) return '#F59E0B'
  return '#3B82F6'
}

// 格式化重置时间（简化版）
function formatResetTime(timestamp?: number): string {
  if (!timestamp) return '--'
  const date = new Date(timestamp)
  return date.toLocaleString('zh-CN', { hour: '2-digit', minute: '2-digit' })
}

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

// 打开设置窗口
async function openSettings() {
  try {
    await invoke('open_settings_panel')
  } catch (err) {
    console.error('Open settings failed:', err)
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
  ;(window as any).__infoPanelRefresh = refreshUsageData
})

onUnmounted(() => {
  cleanup?.()
  delete (window as any).__infoPanelRefresh
})
</script>

<template>
  <div class="info-panel" :data-theme="currentTheme">
    <!-- 未配置 API - 简约提示 -->
    <div v-if="!hasApiKey" class="api-notice">
      <div class="notice-icon">🔑</div>
      <div class="notice-text">
        <div class="notice-title">配置 API Key</div>
        <div class="notice-desc">点击下方按钮开始配置</div>
      </div>
      <button class="notice-btn" @click="openSettings">
        去设置
      </button>
    </div>

    <!-- 已配置 API - 紧凑数据展示 -->
    <template v-else>
      <!-- 顶部操作栏 -->
      <div class="top-bar">
        <span class="time-badge">{{ lastUpdateTime || '--:--' }}</span>
        <div class="actions">
          <button class="action-btn" @click="refreshUsageData" :disabled="isRefreshing">
            <svg :class="{ spinning: isRefreshing }" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
              <path d="M23 4v6h-6M1 20v-6h6"/>
              <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/>
            </svg>
          </button>
          <button class="action-btn close" @click="closeWindow">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
              <path d="M18 6L6 18M6 6l12 12"/>
            </svg>
          </button>
        </div>
      </div>

      <!-- 错误提示 -->
      <div v-if="fetchError" class="error-tip">
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"/>
          <path d="M12 8v4M12 16h.01"/>
        </svg>
        <span>{{ fetchError.slice(0, 20) }}</span>
      </div>

      <!-- 数据区域 -->
      <div class="data-area">
        <!-- 5小时 Token -->
        <div class="stat-row">
          <div class="stat-info">
            <span class="stat-label">5h Token</span>
            <span class="stat-value" :style="{ color: getStatusColor(tokensPercent) }">{{ 100 - tokensPercent }}%</span>
          </div>
          <div class="stat-bar">
            <div class="bar-fill" :style="{ width: tokensPercent + '%', background: getStatusColor(tokensPercent) }"></div>
          </div>
          <span class="stat-hint">重置 {{ tokensResetTime }}</span>
        </div>

        <!-- MCP 额度 -->
        <div class="stat-row" :class="{ expanded: isMonthlyExpanded }" @click="isMonthlyExpanded = !isMonthlyExpanded">
          <div class="stat-info">
            <span class="stat-label">MCP额度</span>
            <span class="stat-value" :style="{ color: getStatusColor(timePercent) }">{{ 100 - timePercent }}%</span>
          </div>
          <div class="stat-bar">
            <div class="bar-fill" :style="{ width: timePercent + '%', background: getStatusColor(timePercent) }"></div>
          </div>
          <div class="stat-expand">
            <span class="stat-hint">已用 {{ timePercent }}%</span>
            <svg class="expand-arrow" :class="{ rotated: isMonthlyExpanded }" width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="6 9 12 15 18 9"></polyline>
            </svg>
          </div>
          <!-- 展开详情 -->
          <div v-if="isMonthlyExpanded && usageDetails.length > 0" class="details-list">
            <div v-for="detail in usageDetails" :key="detail.model_code" class="detail-item">
              <span class="detail-name">{{ detail.model_code }}</span>
              <span class="detail-val">{{ detail.usage }}</span>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
/* ── 基础容器 - 椭圆对话框风格 ── */
.info-panel {
  width: 100vw;
  height: 100vh;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background: rgba(15, 15, 17, 0.92);
  color: #e4e4e7;
  display: flex;
  flex-direction: column;
  border-radius: 16px;
  overflow: hidden;
  backdrop-filter: blur(12px);
  border: 1px solid rgba(255, 255, 255, 0.06);
}

/* ── 浅色主题 ── */
.info-panel[data-theme="light"] {
  background: rgba(255, 255, 255, 0.95);
  color: #1c1c1e;
  border-color: rgba(0, 0, 0, 0.06);
}

/* ── API 配置提示 ── */
.api-notice {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 24px 20px;
  text-align: center;
}

.notice-icon {
  font-size: 36px;
  animation: float 2s ease-in-out infinite;
}

@keyframes float {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-6px); }
}

.notice-text {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.notice-title {
  font-size: 14px;
  font-weight: 600;
  color: #60a5fa;
}

.info-panel[data-theme="light"] .notice-title {
  color: #3b82f6;
}

.notice-desc {
  font-size: 11px;
  color: #71717a;
}

.notice-btn {
  margin-top: 8px;
  padding: 10px 28px;
  background: rgba(59, 130, 246, 0.15);
  border: 1px solid rgba(59, 130, 246, 0.3);
  border-radius: 20px;
  color: #60a5fa;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.info-panel[data-theme="light"] .notice-btn {
  background: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
}

.notice-btn:hover {
  background: rgba(59, 130, 246, 0.25);
  transform: translateY(-1px);
}

/* ── 顶部操作栏 ── */
.top-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 14px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.info-panel[data-theme="light"] .top-bar {
  border-bottom-color: rgba(0, 0, 0, 0.05);
}

.time-badge {
  font-size: 10px;
  font-weight: 500;
  color: #52525b;
  padding: 3px 8px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 10px;
}

.info-panel[data-theme="light"] .time-badge {
  background: rgba(0, 0, 0, 0.04);
  color: #737373;
}

.actions {
  display: flex;
  gap: 6px;
}

.action-btn {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: #71717a;
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.15s ease;
}

.action-btn:hover {
  background: rgba(255, 255, 255, 0.08);
  color: #d4d4d8;
}

.info-panel[data-theme="light"] .action-btn:hover {
  background: rgba(0, 0, 0, 0.04);
  color: #404040;
}

.action-btn.close:hover {
  background: rgba(239, 68, 68, 0.15);
  color: #f87171;
}

.action-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.spinning {
  animation: spin 0.7s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

/* ── 错误提示 ── */
.error-tip {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 8px 14px;
  margin: 8px 14px;
  background: rgba(239, 68, 68, 0.12);
  border-radius: 10px;
  color: #fca5a5;
  font-size: 10px;
}

.info-panel[data-theme="light"] .error-tip {
  background: rgba(239, 68, 68, 0.08);
  color: #ef4444;
}

/* ── 数据区域 ── */
.data-area {
  flex: 1;
  padding: 10px 14px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.stat-row {
  background: rgba(255, 255, 255, 0.03);
  border-radius: 12px;
  padding: 12px 14px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  transition: all 0.2s ease;
}

.info-panel[data-theme="light"] .stat-row {
  background: rgba(0, 0, 0, 0.02);
}

.stat-row.expanded {
  cursor: pointer;
}

.stat-row.expanded:hover {
  background: rgba(255, 255, 255, 0.06);
}

.info-panel[data-theme="light"] .stat-row.expanded:hover {
  background: rgba(0, 0, 0, 0.04);
}

.stat-info {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.stat-label {
  font-size: 11px;
  font-weight: 500;
  color: #71717a;
  letter-spacing: 0.3px;
}

.info-panel[data-theme="light"] .stat-label {
  color: #6b7280;
}

.stat-value {
  font-size: 16px;
  font-weight: 700;
  letter-spacing: -0.3px;
}

.stat-bar {
  height: 4px;
  background: rgba(255, 255, 255, 0.08);
  border-radius: 2px;
  overflow: hidden;
}

.info-panel[data-theme="light"] .stat-bar {
  background: rgba(0, 0, 0, 0.06);
}

.bar-fill {
  height: 100%;
  border-radius: 2px;
  transition: width 0.4s ease;
}

.stat-hint {
  font-size: 9px;
  color: #52525b;
}

.info-panel[data-theme="light"] .stat-hint {
  color: #9ca3af;
}

.stat-expand {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.expand-arrow {
  color: #52525b;
  transition: transform 0.2s ease;
}

.expand-arrow.rotated {
  transform: rotate(180deg);
}

/* ── 展开详情 ── */
.details-list {
  margin-top: 8px;
  padding-top: 10px;
  border-top: 1px solid rgba(255, 255, 255, 0.06);
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.info-panel[data-theme="light"] .details-list {
  border-top-color: rgba(0, 0, 0, 0.06);
}

.detail-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 6px 10px;
  background: rgba(255, 255, 255, 0.03);
  border-radius: 6px;
}

.info-panel[data-theme="light"] .detail-item {
  background: rgba(0, 0, 0, 0.02);
}

.detail-name {
  font-size: 9px;
  font-weight: 500;
  color: #71717a;
}

.info-panel[data-theme="light"] .detail-name {
  color: #6b7280;
}

.detail-val {
  font-size: 9px;
  font-weight: 600;
  color: #e4e4e7;
}

.info-panel[data-theme="light"] .detail-val {
  color: #1c1c1e;
}
</style>
