<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useSettings } from '../composables/useSettings'
import { useTheme } from '../composables/useTheme'
import type { DisplayMode } from '../types/config'

const {
  config,
  isLoading,
  error,
  testResult,
  activeModel,
  loadConfig,
  saveConfig,
  switchModel,
  updateModelConfig,
  testConnection,
  setupConfigListener,
  updatePetType,
  updateActionInterval,
  updateEnableGlow,
  updateAutoStart
} = useSettings()

const { currentTheme, setTheme } = useTheme()

// UI 状态
const activeTab = ref<'basic' | 'models' | 'pet'>('basic')

const displayModes: { value: DisplayMode; label: string }[] = [
  { value: 'holo-bubble', label: 'HOLO-BUBBLE' },
  { value: 'cyber-ring', label: 'CYBER-RING' },
  { value: 'aura-field', label: 'AURA-FIELD' },
  { value: 'energy-core', label: 'ENERGY-CORE' },
  { value: 'status-floater', label: 'STATUS-FLOATER' }
]

const isTesting = ref(false)

// 实时预览监听
watch(() => config.value?.display_config?.display_mode, async (newMode, oldMode) => {
  if (newMode && newMode !== oldMode) {
    try {
      await saveConfig()
    } catch(e) {
      console.error('saveConfig failed!', e)
    }
  }
})

watch(() => config.value?.pet_config?.selected_pet, async (newPet, oldPet) => {
  if (newPet && newPet !== oldPet) {
    try {
      await saveConfig()
    } catch(e) {
      console.error('saveConfig failed!', e)
    }
  }
})

watch(() => config.value?.pet_config?.action_interval, async (newInterval, oldInterval) => {
  if (newInterval !== undefined && newInterval !== oldInterval) {
    try {
      await saveConfig()
    } catch(e) {
      console.error('saveConfig failed!', e)
    }
  }
})

// 关闭窗口
async function closeWindow() {
  try {
    await invoke('close_settings_panel')
  } catch (err) {
    console.error('Close settings panel failed:', err)
  }
}

// 测试连接
async function handleTestConnection(model: any) {
  try {
    isTesting.value = true
    const domains: Record<string, string> = {
      'bigmodel': 'https://open.bigmodel.cn/',
      'zai': 'https://api.z.ai/'
    }
    const domain = domains[model.provider] || 'https://open.bigmodel.cn/'
    const apiUrl = domain + 'api/monitor/usage/quota/limit'
    await testConnection(apiUrl, model.api_key)
  } catch (err) {
    console.error('Test connection failed:', err)
  } finally {
    isTesting.value = false
  }
}

// 组件挂载
let cleanup: (() => void) | undefined

onMounted(async () => {
  await loadConfig()
  await setTheme(config.value.basic_config?.theme || 'dark')
  cleanup = await setupConfigListener()
})

onUnmounted(() => {
  cleanup?.()
})
</script>

<template>
  <div class="settings-panel" :data-theme="currentTheme">
    <!-- 顶部栏 -->
    <header class="panel-header">
      <div class="header-left">
        <span class="panel-title">SETTINGS</span>
      </div>
      <div class="header-right">
        <button class="icon-btn close" @click="closeWindow">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 6L6 6M6 6l12 12"/>
          </svg>
        </button>
      </div>
    </header>

    <!-- 标签导航 -->
    <nav class="tab-nav">
      <button
        v-for="tab in [
          { id: 'basic', label: 'BASIC' },
          { id: 'models', label: 'MODELS' },
          { id: 'pet', label: 'PET' }
        ]"
        :key="tab.id"
        class="tab-btn"
        :class="{ active: activeTab === tab.id }"
        @click="activeTab = tab.id as any"
      >
        {{ tab.label }}
      </button>
    </nav>

    <!-- 内容区 -->
    <main class="panel-content">
      <!-- 基础设置 -->
      <div v-if="activeTab === 'basic'" class="settings-section">
        <!-- 主题切换 -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">THEME</span>
            <span class="setting-desc">选择界面主题</span>
          </div>
          <div class="theme-selector">
            <button
              class="theme-btn"
              :class="{ active: currentTheme === 'dark' }"
              @click="setTheme('dark')"
            >DARK</button>
            <button
              class="theme-btn"
              :class="{ active: currentTheme === 'light' }"
              @click="setTheme('light')"
            >LIGHT</button>
          </div>
        </div>

        <!-- 光晕效果 -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">GLOW EFFECT</span>
            <span class="setting-desc">宠物周围彩色光晕</span>
          </div>
          <label class="toggle-switch">
            <input
              type="checkbox"
              :checked="config.basic_config?.enable_glow ?? true"
              @change="updateEnableGlow(($event.target as HTMLInputElement).checked)"
            />
            <span class="toggle-track"></span>
            <span class="toggle-thumb"></span>
          </label>
        </div>

        <!-- 开机自启 -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">AUTO START</span>
            <span class="setting-desc">系统启动时自动运行</span>
          </div>
          <label class="toggle-switch">
            <input
              type="checkbox"
              :checked="config.basic_config?.auto_start ?? false"
              @change="updateAutoStart(($event.target as HTMLInputElement).checked)"
            />
            <span class="toggle-track"></span>
            <span class="toggle-thumb"></span>
          </label>
        </div>
      </div>

      <!-- 模型配置 -->
      <div v-if="activeTab === 'models'" class="settings-section">
        <!-- 当前模型 -->
        <div class="setting-item full">
          <div class="setting-info">
            <span class="setting-label">ACTIVE MODEL</span>
          </div>
          <div class="model-selector">
            <select
              class="select-input"
              :value="config.api_config.current_model"
              @change="switchModel(($event.target as HTMLSelectElement).value)"
            >
              <option
                v-for="model in config.api_config.models.filter(m => m.enabled)"
                :key="model.provider"
                :value="model.provider"
              >
                {{ model.name }}
              </option>
            </select>
          </div>
        </div>

        <!-- 模型列表 -->
        <div class="models-list">
          <div
            v-for="model in config.api_config.models"
            :key="model.provider"
            class="model-item"
            :class="{ active: model.provider === config.api_config.current_model }"
          >
            <div class="model-info">
              <span class="model-name">{{ model.name }}</span>
              <span class="model-provider">{{ model.provider }}</span>
            </div>
            <div class="model-actions">
              <button
                class="action-btn test"
                :disabled="isTesting"
                @click="handleTestConnection(model)"
              >
                {{ isTesting ? '...' : 'TEST' }}
              </button>
              <label class="toggle-switch mini">
                <input
                  type="checkbox"
                  :checked="model.enabled"
                  @change="updateModelConfig(model.provider, { enabled: ($event.target as HTMLInputElement).checked })"
                />
                <span class="toggle-track"></span>
                <span class="toggle-thumb"></span>
              </label>
            </div>
          </div>
        </div>
      </div>

      <!-- 宠物设置 -->
      <div v-if="activeTab === 'pet'" class="settings-section">
        <!-- 宠物选择 -->
        <div class="setting-item full">
          <div class="setting-info">
            <span class="setting-label">PET TYPE</span>
            <span class="setting-desc">选择宠物类型</span>
          </div>
          <div class="pet-selector">
            <label
              v-for="pet in [
                { value: 'cat', label: 'CAT' },
                { value: 'dog', label: 'DOG' }
              ]"
              :key="pet.value"
              class="pet-option"
              :class="{ selected: config.pet_config?.selected_pet === pet.value }"
            >
              <input
                type="radio"
                name="petType"
                :value="pet.value"
                :checked="config.pet_config?.selected_pet === pet.value"
                @change="updatePetType(pet.value)"
              />
              <span>{{ pet.label }}</span>
            </label>
          </div>
        </div>

        <!-- 动作间隔 -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-label">ACTION INTERVAL</span>
            <span class="setting-desc">动作切换间隔（秒）</span>
          </div>
          <div class="input-group">
            <input
              type="number"
              class="number-input"
              :value="config.pet_config?.action_interval || 25"
              @input="updateActionInterval(parseInt(($event.target as HTMLInputElement).value) || 25)"
              min="10"
              max="60"
            />
          </div>
        </div>

        <!-- 展示模式 -->
        <div class="setting-item full">
          <div class="setting-info">
            <span class="setting-label">DISPLAY MODE</span>
            <span class="setting-desc">Token 展示模式</span>
          </div>
          <div class="display-grid">
            <label
              v-for="mode in displayModes"
              :key="mode.value"
              class="display-option"
              :class="{ selected: config.display_config?.display_mode === mode.value }"
            >
              <input
                type="radio"
                name="displayMode"
                :value="mode.value"
                v-model="config.display_config.display_mode"
              />
              <span>{{ mode.label }}</span>
            </label>
          </div>
        </div>
      </div>

      <!-- 错误提示 -->
      <div v-if="error" class="error-bar">
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="10"/>
          <path d="M12 8v4M12 16h.01"/>
        </svg>
        <span>{{ error }}</span>
      </div>
    </main>
  </div>
</template>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600;700&display=swap');

/* ── 基础容器 ── */
.settings-panel {
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
.settings-panel[data-theme="light"] {
  background: #f5f5f4;
  color: #1c1c1e;
}

.settings-panel[data-theme="light"] .panel-header {
  background: #ffffff;
  border-bottom-color: #e4e4e7;
}

.settings-panel[data-theme="light"] .panel-title {
  color: #1c1c1e;
}

.settings-panel[data-theme="light"] .tab-nav {
  background: #ffffff;
  border-bottom-color: #e4e4e7;
}

.settings-panel[data-theme="light"] .tab-btn {
  color: #737373;
}

.settings-panel[data-theme="light"] .tab-btn:hover {
  color: #52525b;
  background: #fafaf9;
}

.settings-panel[data-theme="light"] .tab-btn.active {
  color: #1c1c1e;
  border-bottom-color: #1c1c1e;
}

.settings-panel[data-theme="light"] .setting-item {
  background: #ffffff;
  border-color: #e4e4e7;
}

.settings-panel[data-theme="light"] .setting-label {
  color: #1c1c1e;
}

.settings-panel[data-theme="light"] .setting-desc,
.settings-panel[data-theme="light"] .model-provider {
  color: #737373;
}

.settings-panel[data-theme="light"] .model-item {
  background: #fafaf9;
  border-color: #e4e4e7;
}

.settings-panel[data-theme="light"] .model-item.active {
  background: #ffffff;
  border-color: #1c1c1e;
}

.settings-panel[data-theme="light"] .select-input {
  background: #fafaf9;
  border-color: #e4e4e7;
  color: #1c1c1e;
}

.settings-panel[data-theme="light"] .number-input {
  background: #fafaf9;
  border-color: #e4e4e7;
  color: #1c1c1e;
}

.settings-panel[data-theme="light"] .pet-option,
.settings-panel[data-theme="light"] .display-option {
  background: #fafaf9;
  border-color: #e4e4e7;
  color: #52525b;
}

.settings-panel[data-theme="light"] .pet-option.selected,
.settings-panel[data-theme="light"] .display-option.selected {
  background: #ffffff;
  border-color: #1c1c1e;
  color: #1c1c1e;
}

.settings-panel[data-theme="light"] .action-btn {
  background: #fafaf9;
  border-color: #e4e4e7;
  color: #52525b;
}

.settings-panel[data-theme="light"] .action-btn:hover {
  background: #ffffff;
  border-color: #1c1c1e;
}

.settings-panel[data-theme="light"] .theme-btn {
  background: #fafaf9;
  border-color: #e4e4e7;
  color: #52525b;
}

.settings-panel[data-theme="light"] .theme-btn.active {
  background: #1c1c1e;
  border-color: #1c1c1e;
  color: #ffffff;
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

.panel-title {
  font-size: 11px;
  font-weight: 700;
  color: #e4e4e7;
  letter-spacing: 1px;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 12px;
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

/* ── 标签导航 ── */
.tab-nav {
  display: flex;
  gap: 0;
  background: #111113;
  border-bottom: 1px solid #1c1c1e;
  flex-shrink: 0;
}

.tab-btn {
  padding: 12px 20px;
  background: transparent;
  border: none;
  border-bottom: 2px solid transparent;
  font-family: inherit;
  font-size: 10px;
  font-weight: 600;
  color: #71717a;
  letter-spacing: 0.5px;
  cursor: pointer;
  transition: all 0.15s;
}

.tab-btn:hover {
  color: #a1a1aa;
  background: rgba(255, 255, 255, 0.03);
}

.tab-btn.active {
  color: #e4e4e7;
  border-bottom-color: #e4e4e7;
}

/* ── 内容区 ── */
.panel-content {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
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

/* ── 设置项 ── */
.settings-section {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 16px;
  background: #111113;
  border: 1px solid #1c1c1e;
  border-radius: 8px;
  gap: 16px;
}

.setting-item.full {
  flex-direction: column;
  align-items: stretch;
  gap: 12px;
}

.setting-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
}

.setting-label {
  font-size: 10px;
  font-weight: 700;
  color: #e4e4e7;
  letter-spacing: 0.5px;
}

.setting-desc {
  font-size: 9px;
  color: #71717a;
}

/* ── 开关 ── */
.toggle-switch {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 22px;
  flex-shrink: 0;
}

.toggle-switch.mini {
  width: 36px;
  height: 18px;
}

.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-track {
  position: absolute;
  inset: 0;
  background: #1c1c1e;
  border: 1px solid #27272a;
  border-radius: 4px;
  transition: all 0.2s;
}

.toggle-thumb {
  position: absolute;
  width: 16px;
  height: 16px;
  top: 2px;
  left: 2px;
  background: #71717a;
  border-radius: 3px;
  transition: all 0.2s;
}

.toggle-switch.mini .toggle-thumb {
  width: 12px;
  height: 12px;
  top: 2px;
  left: 2px;
}

.toggle-switch input:checked + .toggle-track {
  border-color: #22c55e;
}

.toggle-switch input:checked ~ .toggle-thumb {
  transform: translateX(22px);
  background: #22c55e;
}

.toggle-switch.mini input:checked ~ .toggle-thumb {
  transform: translateX(18px);
}

/* ── 主题选择器 ── */
.theme-selector {
  display: flex;
  gap: 8px;
}

.theme-btn {
  padding: 8px 16px;
  background: #1c1c1e;
  border: 1px solid #27272a;
  border-radius: 4px;
  font-family: inherit;
  font-size: 9px;
  font-weight: 600;
  color: #a1a1aa;
  cursor: pointer;
  transition: all 0.15s;
}

.theme-btn:hover {
  border-color: #3f3f46;
  color: #e4e4e7;
}

.theme-btn.active {
  background: #e4e4e7;
  border-color: #e4e4e7;
  color: #0a0a0b;
}

/* ── 模型选择器 ── */
.model-selector {
  flex-shrink: 0;
}

.select-input {
  padding: 8px 12px;
  background: #1c1c1e;
  border: 1px solid #27272a;
  border-radius: 4px;
  font-family: inherit;
  font-size: 10px;
  color: #e4e4e7;
  cursor: pointer;
  min-width: 140px;
}

.select-input:focus {
  outline: none;
  border-color: #3f3f46;
}

/* ── 模型列表 ── */
.models-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.model-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 14px;
  background: #0a0a0b;
  border: 1px solid #1c1c1e;
  border-radius: 6px;
  gap: 12px;
}

.model-item.active {
  background: #111113;
  border-color: #27272a;
}

.model-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
}

.model-name {
  font-size: 11px;
  font-weight: 600;
  color: #e4e4e7;
}

.model-provider {
  font-size: 9px;
  color: #52525b;
}

.model-actions {
  display: flex;
  align-items: center;
  gap: 10px;
}

.action-btn {
  padding: 6px 12px;
  background: #1c1c1e;
  border: 1px solid #27272a;
  border-radius: 4px;
  font-family: inherit;
  font-size: 9px;
  font-weight: 600;
  color: #a1a1aa;
  cursor: pointer;
  transition: all 0.15s;
}

.action-btn:hover:not(:disabled) {
  background: #27272a;
  border-color: #3f3f46;
  color: #e4e4e7;
}

.action-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

/* ── 宠物选择器 ── */
.pet-selector {
  display: flex;
  gap: 8px;
}

.pet-option {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 10px 16px;
  background: #0a0a0b;
  border: 1px solid #1c1c1e;
  border-radius: 6px;
  font-size: 10px;
  font-weight: 600;
  color: #a1a1aa;
  cursor: pointer;
  transition: all 0.15s;
}

.pet-option input {
  display: none;
}

.pet-option:hover {
  border-color: #27272a;
  color: #e4e4e7;
}

.pet-option.selected {
  background: #111113;
  border-color: #3f3f46;
  color: #e4e4e7;
}

/* ── 输入框 ── */
.input-group {
  flex-shrink: 0;
}

.number-input {
  width: 70px;
  padding: 8px 12px;
  background: #1c1c1e;
  border: 1px solid #27272a;
  border-radius: 4px;
  font-family: inherit;
  font-size: 11px;
  color: #e4e4e7;
  text-align: center;
}

.number-input:focus {
  outline: none;
  border-color: #3f3f46;
}

/* ── 展示模式选择器 ── */
.display-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 8px;
}

.display-option {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 10px 12px;
  background: #0a0a0b;
  border: 1px solid #1c1c1e;
  border-radius: 6px;
  font-size: 9px;
  font-weight: 600;
  color: #a1a1aa;
  cursor: pointer;
  transition: all 0.15s;
}

.display-option input {
  display: none;
}

.display-option:hover {
  border-color: #27272a;
  color: #e4e4e7;
}

.display-option.selected {
  background: #111113;
  border-color: #3f3f46;
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
