<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useUsageState } from '../composables/useUsageState'
import { useTauriEvents } from '../composables/useTauriEvents'

const { usageData, setupEventListener } = useTauriEvents()
const { petState } = useUsageState(
  computed(() => usageData.value.used),
  computed(() => usageData.value.total)
)

// 双指标数据
const timePercent = computed(() => usageData.value.time_percent ?? 0)
const tokensPercent = computed(() => usageData.value.tokens_percent ?? 0)
const timeRemaining = computed(() => usageData.value.time_remaining)

// 会员等级
const membershipLevel = computed(() => {
  const level = usageData.value.level || 'lite'
  const levelMap: Record<string, string> = {
    lite: '轻量版',
    standard: '标准版',
    premium: '高级版',
    enterprise: '企业版'
  }
  return levelMap[level] || level
})

// 格式化重置时间
function formatResetTime(timestamp?: number): string {
  if (!timestamp) return '--'
  const date = new Date(timestamp)
  const now = new Date()
  const diff = timestamp - now.getTime()

  // 如果是今天，显示时间
  // 如果是明天或更晚，显示日期和时间
  if (diff > 0 && diff < 86400000) {
    return date.toLocaleString('zh-CN', { month: 'numeric', day: 'numeric', hour: '2-digit', minute: '2-digit' })
  } else {
    return date.toLocaleString('zh-CN', { month: 'numeric', day: 'numeric', hour: '2-digit', minute: '2-digit' })
  }
}

const timeResetTime = computed(() => formatResetTime(usageData.value.time_reset_time))
const tokensResetTime = computed(() => formatResetTime(usageData.value.tokens_reset_time))

// 工具使用详情
const usageDetails = computed(() => usageData.value.usage_details || [])

// 心语映射
const heartMessages: Record<string, string> = {
  Fresh:   '新的一天，能量满格！冲冲冲！',
  Flow:    '代码写得正顺手，不要打扰我~',
  Warning: '用量有点多了，要省着点...',
  Panic:   '要炸了！谁在疯狂 Call API？！',
  Dead:    '系统崩溃... 请充值续命...',
}
const heartMsg = computed(() => heartMessages[petState.value])

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
    lastUpdateTime.value = `${now.getHours().toString().padStart(2,'0')}:${now.getMinutes().toString().padStart(2,'0')}:${now.getSeconds().toString().padStart(2,'0')}`
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
  cleanup = await setupEventListener()
  refreshUsageData()
})

onUnmounted(() => {
  cleanup?.()
})
</script>

<template>
  <div class="info-panel rpg-panel" :class="`panel-${petState.toLowerCase()}`">
    <!-- RPG 风格边框装饰 -->
    <div class="rpg-border-tl"></div>
    <div class="rpg-border-tr"></div>
    <div class="rpg-border-bl"></div>
    <div class="rpg-border-br"></div>

    <!-- 标题栏 -->
    <div class="rpg-header">
      <span class="rpg-title">STATUS MENU</span>
      <button class="rpg-close" @click="closeWindow">[X]</button>
    </div>

    <div class="rpg-content">
      <!-- 玩家等级卡片 -->
      <div class="rpg-card level-card">
        <div class="rpg-label">MEMBER</div>
        <div class="rpg-level">{{ membershipLevel }}</div>
      </div>

      <!-- 状态消息卡片 -->
      <div class="rpg-card status-card">
        <div class="rpg-label">MESSAGE</div>
        <div class="rpg-message">{{ heartMsg }}</div>
      </div>

      <!-- 5小时额度卡片 -->
      <div class="rpg-card">
        <div class="rpg-label">5H TOKENS</div>
        <div class="rpg-stats">
          <span class="rpg-value">{{ tokensPercent }}%</span>
          <span class="rpg-remain">REM: {{ 100 - tokensPercent }}%</span>
        </div>
        <div class="rpg-bar-container">
          <div class="rpg-bar">
            <div class="rpg-bar-fill" :style="{ width: tokensPercent + '%' }"></div>
          </div>
        </div>
        <div class="rpg-reset">RESET: {{ tokensResetTime }}</div>
      </div>

      <!-- 月度额度卡片 -->
      <div class="rpg-card">
        <div class="rpg-label">MONTHLY</div>
        <div class="rpg-stats">
          <span class="rpg-value">{{ timePercent }}%</span>
          <span class="rpg-remain">REM: {{ timeRemaining }}</span>
        </div>
        <div class="rpg-bar-container">
          <div class="rpg-bar">
            <div class="rpg-bar-fill" :class="`fill-${petState.toLowerCase()}`" :style="{ width: timePercent + '%' }"></div>
          </div>
        </div>
        <div class="rpg-reset">RESET: {{ timeResetTime }}</div>
      </div>

      <!-- 工具详情卡片 -->
      <div class="rpg-card" v-if="usageDetails.length > 0">
        <div class="rpg-label">TOOLS</div>
        <div class="rpg-items">
          <div v-for="detail in usageDetails" :key="detail.model_code" class="rpg-item">
            <span class="item-icon">►</span>
            <span class="item-name">{{ detail.model_code }}</span>
            <span class="item-count">x{{ detail.usage }}</span>
          </div>
        </div>
      </div>

      <!-- 刷新按钮 -->
      <button class="rpg-button" @click="refreshUsageData" :disabled="isRefreshing">
        <span v-if="isRefreshing">...</span>
        <span v-else>[REFRESH]</span>
      </button>

      <!-- 最后更新 -->
      <div class="rpg-footer">UPDATED: {{ lastUpdateTime || 'LOADING...' }}</div>

      <!-- 错误提示 -->
      <div v-if="fetchError" class="rpg-error">
        ! ERROR: {{ fetchError }}
      </div>
    </div>
  </div>
</template>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap');

/* ── RPG 面板容器 ── */
.info-panel.rpg-panel {
  width: 100vw;
  height: 100vh;
  background: #1a1a2e;
  background-image:
    repeating-linear-gradient(0deg, transparent, transparent 2px, rgba(0, 0, 0, 0.1) 2px, rgba(0, 0, 0, 0.1) 4px),
    repeating-linear-gradient(90deg, transparent, transparent 2px, rgba(0, 0, 0, 0.1) 2px, rgba(0, 0, 0, 0.1) 4px);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  font-family: 'Press Start 2P', monospace;
  position: relative;
  color: #e0e0e0;
}

/* ── 金色边框装饰 ── */
.rpg-border-tl,
.rpg-border-tr,
.rpg-border-bl,
.rpg-border-br {
  position: absolute;
  width: 16px;
  height: 16px;
  border: 2px solid #ffd700;
  pointer-events: none;
  z-index: 10;
}

.rpg-border-tl { top: 6px; left: 6px; border-right: none; border-bottom: none; }
.rpg-border-tr { top: 6px; right: 6px; border-left: none; border-bottom: none; }
.rpg-border-bl { bottom: 6px; left: 6px; border-right: none; border-top: none; }
.rpg-border-br { bottom: 6px; right: 6px; border-left: none; border-top: none; }

/* ── 标题栏 ── */
.rpg-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 16px 8px;
  background: linear-gradient(180deg, #16213e 0%, #1a1a2e 100%);
  border-bottom: 2px solid #ffd700;
  margin: 0 6px;
}

.rpg-title {
  font-size: 11px;
  color: #ffd700;
  text-shadow: 1px 1px 0 #000;
  letter-spacing: 1px;
}

.rpg-close {
  font-family: 'Press Start 2P', monospace;
  font-size: 10px;
  color: #ff6b6b;
  background: transparent;
  border: 2px solid #ff6b6b;
  padding: 3px 6px;
  cursor: pointer;
  transition: all 0.1s;
}

.rpg-close:hover {
  background: #ff6b6b;
  color: #fff;
}

/* ── 内容区 ── */
.rpg-content {
  flex: 1;
  padding: 10px 12px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

/* ── 卡片样式 ── */
.rpg-card {
  background: #0f0f1a;
  border: 2px solid #4a4e69;
  padding: 8px 10px;
  position: relative;
}

.rpg-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(90deg, #ffd700, #ffed4a, #ffd700);
}

/* ── 标签 ── */
.rpg-label {
  font-size: 8px;
  color: #948a77;
  margin-bottom: 4px;
  letter-spacing: 1px;
}

/* ── 等级卡片 ── */
.level-card .rpg-level {
  font-size: 12px;
  color: #ffd700;
  text-shadow: 1px 1px 0 #000;
  text-align: center;
  padding: 4px;
  background: rgba(255, 215, 0, 0.1);
  border: 2px solid #ffd700;
  margin-top: 2px;
}

/* ── 状态消息 ── */
.status-card .rpg-message {
  font-size: 8px;
  line-height: 1.4;
  color: #7dd3fc;
  font-style: italic;
}

/* ── 统计数据 ── */
.rpg-stats {
  display: flex;
  justify-content: space-between;
  margin-bottom: 6px;
}

.rpg-value {
  font-size: 16px;
  color: #ffd700;
  text-shadow: 1px 1px 0 #000;
}

.rpg-remain {
  font-size: 8px;
  color: #7dd3fc;
}

/* ── 像素进度条 ── */
.rpg-bar-container {
  margin: 6px 0 4px;
}

.rpg-bar {
  height: 12px;
  background: #000;
  border: 2px solid #4a4e69;
  position: relative;
  box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.5);
}

.rpg-bar-fill {
  height: 100%;
  background: repeating-linear-gradient(
    90deg,
    #ffd700 0px,
    #ffd700 3px,
    #b8860b 3px,
    #b8860b 6px
  );
  background-size: 6px 100%;
  transition: width 0.3s steps(1);
}

.fill-fresh .rpg-bar-fill {
  background: repeating-linear-gradient(90deg, #10b981 0px, #10b981 3px, #059669 3px, #059669 6px);
  background-size: 6px 100%;
}

.fill-flow .rpg-bar-fill {
  background: repeating-linear-gradient(90deg, #3b82f6 0px, #3b82f6 3px, #2563eb 3px, #2563eb 6px);
  background-size: 6px 100%;
}

.fill-warning .rpg-bar-fill {
  background: repeating-linear-gradient(90deg, #f59e0b 0px, #f59e0b 3px, #d97706 3px, #d97706 6px);
  background-size: 6px 100%;
}

.fill-panic .rpg-bar-fill {
  background: repeating-linear-gradient(90deg, #ef4444 0px, #ef4444 3px, #dc2626 3px, #dc2626 6px);
  background-size: 6px 100%;
}

.fill-dead .rpg-bar-fill {
  background: repeating-linear-gradient(90deg, #6b7280 0px, #6b7280 3px, #4b5563 3px, #4b5563 6px);
  background-size: 6px 100%;
}

/* ── 重置时间 ── */
.rpg-reset {
  font-size: 7px;
  color: #948a77;
  margin-top: 4px;
  text-align: center;
}

/* ── 工具列表 ── */
.rpg-items {
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.rpg-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 6px;
  background: rgba(255, 215, 0, 0.05);
  border: 1px solid #4a4e69;
  font-size: 8px;
}

.item-icon {
  color: #ffd700;
}

.item-name {
  color: #e0e0e0;
  flex: 1;
}

.item-count {
  color: #7dd3fc;
}

/* ── 刷新按钮 ── */
.rpg-button {
  font-family: 'Press Start 2P', monospace;
  font-size: 9px;
  color: #ffd700;
  background: #16213e;
  border: 2px solid #ffd700;
  padding: 8px;
  cursor: pointer;
  transition: all 0.1s;
  text-shadow: 1px 1px 0 #000;
  margin-top: 4px;
}

.rpg-button:hover:not(:disabled) {
  background: #ffd700;
  color: #000;
}

.rpg-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* ── 底部信息 ── */
.rpg-footer {
  font-size: 7px;
  color: #948a77;
  text-align: center;
  padding: 4px;
  border-top: 1px solid #4a4e69;
}

/* ── 错误提示 ── */
.rpg-error {
  font-size: 8px;
  color: #ff6b6b;
  background: rgba(255, 107, 107, 0.1);
  border: 2px solid #ff6b6b;
  padding: 6px;
  text-align: center;
}

/* ── 滚动条 ── */
.rpg-content::-webkit-scrollbar {
  width: 6px;
}

.rpg-content::-webkit-scrollbar-track {
  background: #0f0f1a;
}

.rpg-content::-webkit-scrollbar-thumb {
  background: #4a4e69;
  border: 1px solid #ffd700;
}

.rpg-content::-webkit-scrollbar-thumb:hover {
  background: #ffd700;
}
</style>
