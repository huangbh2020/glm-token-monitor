export type DisplayMode =
  | 'none'             // 无 - 不显示用量 (None)
  | 'pedestal'         // 底座展示 (Pedestal)

export interface DisplayConfig {
  display_mode: DisplayMode
}

export interface ApiConfig {
  current_model: string
  models: ModelConfig[]
}

export interface ModelConfig {
  provider: string
  name: string
  enabled: boolean
  api_key: string
}

export interface PollingConfig {
  interval_minutes: number
}

// 宠物类型
export type PetType = 'spirit' | 'ghost' | 'polar' | 'cat' | 'dog' | 'lottie-dog' | 'procrastination' | 'lottie-cat' | 'octoyaki' | 'fixing' | 'bicycle'

// 猫咪动作（已废弃，保留以兼容）
export type CatAction = 'cat-sleep' | 'cat-play' | 'cat-stare' | 'cat-stretch'

// 狗狗动作（已废弃，保留以兼容）
export type DogAction = 'dog-sit' | 'dog-bark' | 'dog-walk' | 'dog-beg'

// 宠物配置
export interface PetConfig {
  selected_pet: PetType
  action_interval: number  // 秒
}

// 基础设置配置
export interface BasicConfig {
  enable_glow: boolean     // 光晕层开关
  auto_start: boolean      // 开机自启动
  theme: 'dark' | 'light'  // 主题模式
}

export interface AppConfig {
  api_config: ApiConfig
  polling_config: PollingConfig
  display_config: DisplayConfig
  pet_config: PetConfig
  basic_config: BasicConfig
}

export interface ClaudeApiProfile {
  id: string
  name: string
  inference_gateway_base_url: string
  inference_gateway_api_key: string
  inference_models: string[]
}

export interface ProfileStore {
  profiles: ClaudeApiProfile[]
  active_profile_id: string | null
  claude_config_path: string | null
}
