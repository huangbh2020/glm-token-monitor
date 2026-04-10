import { computed, type Ref } from 'vue'
import { useSettings } from './useSettings'
import type { DisplayMode } from '../types/config'
import type { AppConfig } from '../types/config'

export function useDisplayMode(config?: Ref<AppConfig>) {
  const settings = config ? { config } : useSettings()

  const displayMode = computed<DisplayMode>(() => {
    const mode = settings.config.value.display_config?.display_mode || 'holo-bubble'
    console.log('[DEBUG useDisplayMode] computed evaluated to:', mode)
    return mode
  })

  return {
    displayMode
  }
}
