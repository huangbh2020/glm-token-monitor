<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
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

// 手动刷新数据
async function refreshUsageData() {
  try {
    isRefreshing.value = true
    const { invoke } = await import('@tauri-apps/api/core')
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
    const { invoke } = await import('@tauri-apps/api/core')
    await invoke('close_info_panel')
  } catch (err) {
    console.error('Close window failed:', err)
  }
}

onMounted(async () => {
  await setupEventListener()
  refreshUsageData()
})
</script>

<template>
  <div class="info-panel" :class="`panel-${petState.toLowerCase()}`">
    <div class="panel-header">
      <span class="panel-title">📊 用量详情</span>
      <button class="panel-close" @click="closeWindow">×</button>
    </div>

    <div class="panel-content">
      <!-- 状态心语 -->
      <div class="panel-section">
        <div class="section-label">💭 状态</div>
        <div class="heart-text-large">{{ heartMsg }}</div>
      </div>

      <!-- 月度额度 -->
      <div class="panel-section">
        <div class="section-label">🗓️ 月度额度</div>
        <div class="metric-row">
          <div class="metric-info">
            <span class="metric-percent">{{ timePercent }}%</span>
            <span class="metric-remaining">剩余 {{ timeRemaining }} 次</span>
          </div>
          <div class="progress-bar">
            <div class="progress-fill" :class="`progress-${petState.toLowerCase()}`" :style="{ width: timePercent + '%' }"></div>
          </div>
        </div>
      </div>

      <!-- 5小时额度 -->
      <div class="panel-section">
        <div class="section-label">⏱️ 5小时额度</div>
        <div class="metric-row">
          <div class="metric-info">
            <span class="metric-percent">{{ tokensPercent }}%</span>
            <span class="metric-remaining">剩余 {{ 100 - tokensPercent }}%</span>
          </div>
          <div class="progress-bar">
            <div class="progress-fill progress-tokens" :style="{ width: tokensPercent + '%' }"></div>
          </div>
        </div>
      </div>

      <!-- 最后更新时间 -->
      <div class="panel-section">
        <div class="section-label">🔄 最后更新</div>
        <div class="update-time">{{ lastUpdateTime || '加载中...' }}</div>
      </div>

      <!-- 错误提示 -->
      <div v-if="fetchError" class="panel-section">
        <div class="error-message">⚠ {{ fetchError }}</div>
      </div>

      <!-- 刷新按钮 -->
      <button class="refresh-btn" @click="refreshUsageData" :disabled="isRefreshing">
        {{ isRefreshing ? '刷新中...' : '🔄 刷新' }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.info-panel {
  width: 100vw;
  height: 100vh;
  background: rgba(15, 23, 42, 0.95);
  backdrop-filter: blur(10px);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  font-family: 'PingFang SC', 'Microsoft YaHei', sans-serif;
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 14px;
  background: rgba(30, 41, 59, 0.8);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.panel-title {
  font-size: 12px;
  font-weight: 600;
  color: #E2E8F0;
}

.panel-close {
  width: 24px;
  height: 24px;
  border: none;
  background: rgba(239, 68, 68, 0.2);
  color: #F87171;
  border-radius: 4px;
  cursor: pointer;
  font-size: 18px;
  line-height: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.panel-close:hover {
  background: rgba(239, 68, 68, 0.4);
  transform: scale(1.1);
}

.panel-content {
  flex: 1;
  padding: 14px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.panel-section {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.section-label {
  font-size: 10px;
  color: #94A3B8;
  font-weight: 500;
}

.heart-text-large {
  font-size: 12px;
  color: #CBD5E1;
  line-height: 1.5;
}

.metric-row {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.metric-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.metric-percent {
  font-size: 16px;
  font-weight: 700;
  font-family: ui-monospace, monospace;
}

.metric-remaining {
  font-size: 10px;
  color: #94A3B8;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  border-radius: 4px;
  transition: width 0.4s ease;
}

.progress-fresh { background: linear-gradient(90deg, #10B981, #34D399); }
.progress-flow { background: linear-gradient(90deg, #3B82F6, #60A5FA); }
.progress-warning { background: linear-gradient(90deg, #F59E0B, #FBBF24); }
.progress-panic { background: linear-gradient(90deg, #EF4444, #F87171); }
.progress-dead { background: linear-gradient(90deg, #6B7280, #9CA3AF); }
.progress-tokens { background: linear-gradient(90deg, #3B82F6, #60A5FA); }

.update-time {
  font-size: 10px;
  color: #64748B;
  font-family: ui-monospace, monospace;
}

.error-message {
  font-size: 10px;
  color: #F87171;
  background: rgba(239, 68, 68, 0.1);
  padding: 6px 8px;
  border-radius: 4px;
  line-height: 1.4;
}

.refresh-btn {
  margin-top: auto;
  padding: 8px 12px;
  background: rgba(59, 130, 246, 0.2);
  color: #60A5FA;
  border: 1px solid rgba(59, 130, 246, 0.3);
  border-radius: 6px;
  font-size: 11px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.refresh-btn:hover:not(:disabled) {
  background: rgba(59, 130, 246, 0.3);
}

.refresh-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>
