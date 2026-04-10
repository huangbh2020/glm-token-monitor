export type DisplayMode =
  | 'holo-bubble'      // 全息像素气泡 (Holo-Bubble)
  | 'cyber-ring'       // 赛博能量环 (Cyber-Ring)
  | 'aura-field'       // 灵能力场波纹 (Aura-Field)
  | 'energy-core'      // 格子能量核心 (Energy-Core)
  | 'status-floater'   // 悬浮状态标 (Status-Floater)

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
export type PetType = 'cat' | 'dog'

// 猫咪动作
export type CatAction = 'cat-sleep' | 'cat-play' | 'cat-stare' | 'cat-stretch'

// 狗狗动作
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
