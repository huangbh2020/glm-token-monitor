import { ref, computed, watch } from 'vue'
import { useSettings } from './useSettings'

const THEME_KEY = 'planguard_theme'

// 全局主题状态
const currentTheme = ref<'dark' | 'light'>('dark')

// 同步到配置和 localStorage
export function useTheme() {
  const { basicConfig, loadConfig, saveConfig } = useSettings()

  // 从 localStorage 读取初始主题
  function getStoredTheme(): 'dark' | 'light' {
    const stored = localStorage.getItem(THEME_KEY)
    if (stored === 'dark' || stored === 'light') {
      return stored
    }
    return 'dark'
  }

  // 初始化主题
  async function initTheme() {
    await loadConfig()

    // 优先级：配置 > localStorage > 默认值
    const configTheme = basicConfig.value?.theme
    if (configTheme === 'dark' || configTheme === 'light') {
      currentTheme.value = configTheme
    } else {
      currentTheme.value = getStoredTheme()
    }

    applyTheme()
  }

  // 应用主题到 document
  function applyTheme() {
    document.documentElement.setAttribute('data-theme', currentTheme.value)
    localStorage.setItem(THEME_KEY, currentTheme.value)
  }

  // 切换主题
  async function setTheme(theme: 'dark' | 'light') {
    if (currentTheme.value === theme) return

    currentTheme.value = theme
    applyTheme()

    // 同步到配置
    if (basicConfig.value) {
      basicConfig.value.theme = theme
      try {
        await saveConfig()
      } catch (err) {
        console.error('Failed to save theme config:', err)
      }
    }
  }

  // 切换主题（dark <-> light）
  async function toggleTheme() {
    const newTheme = currentTheme.value === 'dark' ? 'light' : 'dark'
    await setTheme(newTheme)
  }

  // 监听配置变化同步主题
  watch(() => basicConfig.value?.theme, (newTheme) => {
    if (newTheme === 'dark' || newTheme === 'light') {
      if (newTheme !== currentTheme.value) {
        currentTheme.value = newTheme
        applyTheme()
      }
    }
  })

  return {
    currentTheme,
    isDark: computed(() => currentTheme.value === 'dark'),
    isLight: computed(() => currentTheme.value === 'light'),
    setTheme,
    toggleTheme,
    initTheme
  }
}
