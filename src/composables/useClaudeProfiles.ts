import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { ClaudeApiProfile, ProfileStore } from '../types/config'

const globalStore = ref<ProfileStore>({
  profiles: [],
  active_profile_id: null,
  claude_config_path: null,
})

const globalIsLoading = ref(false)
const globalError = ref<string | null>(null)

export function useClaudeProfiles() {
  const store = globalStore
  const isLoading = globalIsLoading
  const error = globalError

  const activeProfile = ref<ClaudeApiProfile | null>(null)

  async function loadProfiles() {
    try {
      isLoading.value = true
      error.value = null
      const result = await invoke<ProfileStore>('get_profiles')
      store.value = result
      syncActiveProfile()
    } catch (err) {
      error.value = String(err)
    } finally {
      isLoading.value = false
    }
  }

  function syncActiveProfile() {
    activeProfile.value = store.value.profiles.find(
      p => p.id === store.value.active_profile_id
    ) || null
  }

  async function saveProfile(profile: ClaudeApiProfile) {
    try {
      error.value = null
      const result = await invoke<ProfileStore>('save_profile_handler', { profile })
      store.value = result
      syncActiveProfile()
    } catch (err) {
      error.value = String(err)
      throw err
    }
  }

  async function deleteProfile(id: string) {
    try {
      error.value = null
      const result = await invoke<ProfileStore>('delete_profile_handler', { id })
      store.value = result
      syncActiveProfile()
    } catch (err) {
      error.value = String(err)
      throw err
    }
  }

  async function switchProfile(id: string) {
    try {
      error.value = null
      await invoke('switch_profile_handler', { id })
      store.value.active_profile_id = id
      syncActiveProfile()
    } catch (err) {
      error.value = String(err)
      throw err
    }
  }

  async function getClaudeConfigPath(): Promise<string> {
    return await invoke<string>('get_claude_config_path_cmd')
  }

  async function setClaudeConfigPath(path: string) {
    await invoke('set_claude_config_path_handler', { path })
    store.value.claude_config_path = path
  }

  async function setupProfileListener() {
    return await listen<ProfileStore>('claude-profile-changed', (event) => {
      if (event.payload) {
        store.value = event.payload
        syncActiveProfile()
      }
    })
  }

  function createEmptyProfile(): ClaudeApiProfile {
    return {
      id: crypto.randomUUID(),
      name: '',
      inference_gateway_base_url: '',
      inference_gateway_api_key: '',
      inference_models: [],
    }
  }

  return {
    store,
    isLoading,
    error,
    activeProfile,
    loadProfiles,
    saveProfile,
    deleteProfile,
    switchProfile,
    getClaudeConfigPath,
    setClaudeConfigPath,
    setupProfileListener,
    createEmptyProfile,
  }
}
