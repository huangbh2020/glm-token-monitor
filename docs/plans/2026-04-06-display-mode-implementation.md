# Display Mode System Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add 5 display modes (bubble, ring, breathing, capsule, emoji) for showing 5h Token usage, allowing users to switch between modes in the settings panel.

**Architecture:** Simple enum-based rendering strategy - add `display_mode` field to config, use conditional rendering in PetWidget.vue based on the mode value. Each mode is implemented as a separate UI section with its own styling.

**Tech Stack:** Vue 3 (Composition API), TypeScript, Tauri 2.0, Rust, SVG, CSS animations

---

## Task 1: Add DisplayConfig to Rust Config Structure

**Files:**
- Modify: `src-tauri/src/config.rs`

**Step 1: Add DisplayConfig struct**

Add the following struct to `config.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    /// Display mode: bubble, ring, breathing, capsule, emoji
    #[serde(default = "default_display_mode")]
    pub display_mode: String,
}

fn default_display_mode() -> String {
    "bubble".to_string()
}
```

**Step 2: Update Config struct to include DisplayConfig**

Modify the existing `Config` struct to include the new field:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_config: ApiConfig,
    pub polling_config: PollingConfig,
    pub display_config: DisplayConfig,
}
```

**Step 3: Update default_config() function**

Update the default config function to include the new field:

```rust
pub fn default_config() -> Config {
    Config {
        api_config: default_api_config(),
        polling_config: default_polling_config(),
        display_config: DisplayConfig {
            display_mode: "bubble".to_string(),
        },
    }
}
```

**Step 4: Run cargo check to verify compilation**

Run: `cd src-tauri && cargo check`
Expected: No errors

**Step 5: Commit**

```bash
git add src-tauri/src/config.rs
git commit -m "feat: add DisplayConfig to Rust config structure"
```

---

## Task 2: Create TypeScript Type Definitions

**Files:**
- Create: `src/types/config.ts`

**Step 1: Create the types file**

Create `src/types/config.ts` with the following content:

```typescript
export type DisplayMode =
  | 'bubble'      // Pixel bubble (current default)
  | 'ring'        // Ring progress
  | 'breathing'   // Breathing glow effect
  | 'capsule'     // Capsule indicator
  | 'emoji'       // Emoji + number overhead

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

export interface AppConfig {
  api_config: ApiConfig
  polling_config: PollingConfig
  display_config: DisplayConfig
}
```

**Step 2: Commit**

```bash
git add src/types/config.ts
git commit -m "feat: add TypeScript type definitions for config"
```

---

## Task 3: Create useDisplayMode Composable

**Files:**
- Create: `src/composables/useDisplayMode.ts`

**Step 1: Create the composable**

Create `src/composables/useDisplayMode.ts`:

```typescript
import { computed } from 'vue'
import { useSettings } from './useSettings'
import type { DisplayMode } from '../types/config'

export function useDisplayMode() {
  const { config } = useSettings()

  const displayMode = computed<DisplayMode>(() => {
    return config.value.display_config?.display_mode || 'bubble'
  })

  return {
    displayMode
  }
}
```

**Step 2: Commit**

```bash
git add src/composables/useDisplayMode.ts
git commit -m "feat: create useDisplayMode composable"
```

---

## Task 4: Update useSettings to Handle DisplayConfig

**Files:**
- Modify: `src/composables/useSettings.ts`

**Step 1: Add DisplayConfig import**

Add to imports:
```typescript
import type { DisplayConfig } from '../types/config'
```

**Step 2: Update config type**

Change the config type to include display_config:
```typescript
const config = ref<AppConfig>({
  api_config: {
    current_model: '',
    models: []
  },
  polling_config: {
    interval_minutes: 5
  },
  display_config: {
    display_mode: 'bubble'
  }
})
```

**Step 3: Update loadConfig function**

Update loadConfig to handle display_config:
```typescript
async function loadConfig() {
  // ... existing code ...
  // Add display_config parsing
  if (data.display_config) {
    config.value.display_config = data.display_config
  }
}
```

**Step 4: Commit**

```bash
git add src/composables/useSettings.ts
git commit -m "feat: update useSettings to handle DisplayConfig"
```

---

## Task 5: Implement Ring Progress Display Mode

**Files:**
- Modify: `src/components/PetWidget.vue`

**Step 1: Add ring display template**

Add after the existing pixel bubble (around line 458):

```vue
<!-- Ring Progress Display -->
<div v-else-if="displayMode === 'ring'" class="ring-display" :class="`ring-${petState.toLowerCase()}`">
  <svg class="ring-svg" viewBox="0 0 96 96" xmlns="http://www.w3.org/2000/svg">
    <!-- Background circle -->
    <circle cx="48" cy="48" r="42" fill="none" stroke="#1E293B" stroke-width="3"/>
    <!-- Progress circle -->
    <circle
      cx="48" cy="48" r="42"
      fill="none"
      :stroke="stateColor"
      stroke-width="3"
      stroke-linecap="round"
      :stroke-dasharray="circumference"
      :stroke-dashoffset="circumference * (1 - tokensPercent / 100)"
      transform="rotate(-90 48 48)"
      class="ring-progress"
    />
    <!-- Center text -->
    <text x="48" y="52" text-anchor="middle" :fill="stateColor" font-size="11" font-weight="bold" font-family="monospace">
      {{ tokensPercent }}%
    </text>
  </svg>
</div>
```

**Step 2: Add computed property for circumference**

Add to script setup:
```typescript
const circumference = 2 * Math.PI * 42 // 2πr where r=42
```

**Step 3: Add ring display styles**

Add to `<style scoped>`:

```css
/* Ring Display */
.ring-display {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  pointer-events: none;
  z-index: 5;
}

.ring-svg {
  width: 96px;
  height: 96px;
  overflow: visible;
}

.ring-progress {
  transition: stroke-dashoffset 0.4s ease;
}

.ring-fresh .ring-progress { filter: drop-shadow(0 0 2px rgba(16,185,129,0.3)); }
.ring-flow .ring-progress { filter: drop-shadow(0 0 2px rgba(59,130,246,0.3)); }
.ring-warning .ring-progress { filter: drop-shadow(0 0 2px rgba(245,158,11,0.3)); }
.ring-panic .ring-progress { filter: drop-shadow(0 0 3px rgba(239,68,68,0.4)); }
.ring-dead .ring-progress { filter: drop-shadow(0 0 1px rgba(107,114,128,0.2)); }
```

**Step 4: Commit**

```bash
git add src/components/PetWidget.vue
git commit -m "feat: implement ring progress display mode"
```

---

## Task 6: Implement Breathing Glow Display Mode

**Files:**
- Modify: `src/components/PetWidget.vue`

**Step 1: Add breathing glow template**

Add after the ring display:

```vue
<!-- Breathing Glow Display -->
<div v-else-if="displayMode === 'breathing'" class="breathing-display" :class="`breathing-${petState.toLowerCase()}`">
  <div class="breathing-glow" :style="breathingGlowStyle"></div>
</div>
```

**Step 2: Add computed property for breathing glow style**

Add to script setup:
```typescript
const breathingGlowStyle = computed(() => {
  const intensity = tokensPercent.value / 100
  return {
    '--glow-opacity': 0.2 + (intensity * 0.5),
    '--breath-duration': petState.value === 'Panic' ? '0.5s' : petState.value === 'Dead' ? '4s' : '2s'
  }
})
```

**Step 3: Add breathing glow styles**

Add to `<style scoped>`:

```css
/* Breathing Glow Display */
.breathing-display {
  position: absolute;
  inset: -8px;
  border-radius: 50%;
  pointer-events: none;
  z-index: 1;
}

.breathing-glow {
  width: 100%;
  height: 100%;
  border-radius: 50%;
  background: radial-gradient(circle, var(--glow-color) 0%, transparent 70%);
  opacity: var(--glow-opacity);
  animation: breathing var(--breath-duration, 2s) ease-in-out infinite;
}

.breathing-fresh .breathing-glow { --glow-color: rgba(16,185,129,0.6); }
.breathing-flow .breathing-glow { --glow-color: rgba(59,130,246,0.6); }
.breathing-warning .breathing-glow { --glow-color: rgba(245,158,11,0.7); }
.breathing-panic .breathing-glow {
  --glow-color: rgba(239,68,68,0.8);
  animation: breathing 0.3s ease-in-out infinite, panic-flash 0.2s infinite;
}
.breathing-dead .breathing-glow { --glow-color: rgba(107,114,128,0.3); }

@keyframes breathing {
  0%, 100% { opacity: calc(var(--glow-opacity) * 0.6); transform: scale(0.95); }
  50% { opacity: var(--glow-opacity); transform: scale(1.05); }
}

@keyframes panic-flash {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}
```

**Step 4: Commit**

```bash
git add src/components/PetWidget.vue
git commit -m "feat: implement breathing glow display mode"
```

---

## Task 7: Implement Capsule Indicator Display Mode

**Files:**
- Modify: `src/components/PetWidget.vue`

**Step 1: Add capsule indicator template**

Add after the breathing glow display:

```vue
<!-- Capsule Indicator Display -->
<div v-else-if="displayMode === 'capsule'" class="capsule-display">
  <div class="capsule-body" :class="`capsule-${petState.toLowerCase()}`">
    <div class="capsule-fill" :style="{ width: tokensPercent + '%' }"></div>
  </div>
  <span class="capsule-text">{{ tokensPercent }}%</span>
</div>
```

**Step 2: Add capsule indicator styles**

Add to `<style scoped>`:

```css
/* Capsule Indicator Display */
.capsule-display {
  position: absolute;
  bottom: 6px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: 4px;
  pointer-events: none;
  z-index: 15;
}

.capsule-body {
  width: 44px;
  height: 7px;
  background: #1E293B;
  border-radius: 4px;
  overflow: hidden;
  border: 1px solid #334155;
  box-shadow: 0 1px 2px rgba(0,0,0,0.3);
}

.capsule-fill {
  height: 100%;
  transition: width 0.4s ease, background 0.4s ease;
}

.capsule-fresh .capsule-fill { background: #10B981; }
.capsule-flow .capsule-fill { background: #3B82F6; }
.capsule-warning .capsule-fill { background: #F59E0B; }
.capsule-panic .capsule-fill { background: #EF4444; }
.capsule-dead .capsule-fill { background: #6B7280; }

.capsule-text {
  font-family: ui-monospace, monospace;
  font-size: 8px;
  font-weight: 700;
  color: #94A3B8;
}

/* Expanded state: hide capsule */
.pet-widget.expanded .capsule-display {
  opacity: 0;
}
```

**Step 3: Commit**

```bash
git add src/components/PetWidget.vue
git commit -m "feat: implement capsule indicator display mode"
```

---

## Task 8: Implement Emoji Display Mode

**Files:**
- Modify: `src/components/PetWidget.vue`

**Step 1: Add emoji display template**

Add after the capsule indicator:

```vue
<!-- Emoji Display -->
<div v-else-if="displayMode === 'emoji'" class="emoji-display">
  <span class="emoji-icon">{{ statusEmoji }}</span>
  <span class="emoji-percent">{{ tokensPercent }}%</span>
</div>
```

**Step 2: Add computed property for status emoji**

Add to script setup:
```typescript
const statusEmoji = computed(() => {
  const p = tokensPercent.value
  if (p <= 30) return '😊'
  if (p <= 60) return '😐'
  if (p <= 80) return '😟'
  if (p <= 95) return '😱'
  return '💀'
})
```

**Step 3: Add emoji display styles**

Add to `<style scoped>`:

```css
/* Emoji Display */
.emoji-display {
  position: absolute;
  top: 4px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1px;
  pointer-events: none;
  z-index: 15;
}

.emoji-icon {
  font-size: 11px;
  line-height: 1;
  animation: emoji-bounce 2s ease-in-out infinite;
}

.emoji-percent {
  font-family: ui-monospace, monospace;
  font-size: 8px;
  font-weight: 700;
  color: #94A3B8;
}

@keyframes emoji-bounce {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-2px); }
}

/* Expanded state: hide emoji */
.pet-widget.expanded .emoji-display {
  opacity: 0;
}
```

**Step 4: Commit**

```bash
git add src/components/PetWidget.vue
git commit -m "feat: implement emoji display mode"
```

---

## Task 9: Update PetWidget to Use DisplayMode Composable

**Files:**
- Modify: `src/components/PetWidget.vue`

**Step 1: Import useDisplayMode**

Add to imports:
```typescript
import { useDisplayMode } from '../composables/useDisplayMode'
```

**Step 2: Use displayMode in component**

Add to script setup:
```typescript
const { displayMode } = useDisplayMode()
```

**Step 3: Update pixel bubble condition**

Change the existing pixel bubble from `v-if` to `v-else-if`:
```vue
<!-- Find the existing pixel bubble div and change v-if to v-else-if -->
<div v-else-if="displayMode === 'bubble'" class="pixel-bubble" ...>
```

**Step 4: Commit**

```bash
git add src/components/PetWidget.vue
git commit -m "feat: integrate display mode composable into PetWidget"
```

---

## Task 10: Add Display Settings Tab to Settings Panel

**Files:**
- Modify: `src/components/SettingsPanel.vue`

**Step 1: Add 'display' to activeTab type**

Update the activeTab ref:
```typescript
const activeTab = ref<'models' | 'polling' | 'display'>('models')
```

**Step 2: Add display settings tab button**

Add to template in the tabs section:
```vue
<button
  class="rpg-tab"
  :class="{ active: activeTab === 'display' }"
  @click="activeTab = 'display'"
>
  [展示设置]
</button>
```

**Step 3: Add display settings tab content**

Add to template after polling tab:
```vue
<!-- Display Settings Tab -->
<div v-if="activeTab === 'display'" class="tab-content">
  <div class="rpg-card">
    <div class="rpg-label">展示模式</div>
    <div class="rpg-mode-options">
      <label
        v-for="mode in displayModes"
        :key="mode.value"
        class="rpg-mode-option"
        :class="{ active: config.display_config?.display_mode === mode.value }"
      >
        <input
          type="radio"
          :value="mode.value"
          :checked="config.display_config?.display_mode === mode.value"
          @change="updateDisplayMode(mode.value)"
        />
        <span class="rpg-mode-label">{{ mode.label }}</span>
        <button class="rpg-preview-btn" @click.stop="previewMode(mode.value)">
          [▶ 预览]
        </button>
      </label>
    </div>
  </div>

  <div class="rpg-card">
    <div class="rpg-label">实时预览</div>
    <div class="rpg-preview-area">
      <div class="preview-widget" :class="`preview-${previewDisplayMode}`">
        <!-- Mini preview of current mode -->
        <div v-if="previewDisplayMode === 'bubble'" class="preview-bubble">
          5h: <span>85%</span>
        </div>
        <div v-else-if="previewDisplayMode === 'ring'" class="preview-ring">
          <svg viewBox="0 0 48 48">
            <circle cx="24" cy="24" r="18" fill="none" stroke="#1E293B" stroke-width="2"/>
            <circle cx="24" cy="24" r="18" fill="none" stroke="#10B981" stroke-width="2"
              stroke-dasharray="113" stroke-dashoffset="17" stroke-linecap="round"
              transform="rotate(-90 24 24)"/>
            <text x="24" y="27" text-anchor="middle" fill="#10B981" font-size="6">85%</text>
          </svg>
        </div>
        <div v-else-if="previewDisplayMode === 'breathing'" class="preview-breathing">
          <div class="preview-glow"></div>
          <span>💚</span>
        </div>
        <div v-else-if="previewDisplayMode === 'capsule'" class="preview-capsule">
          <div class="preview-capsule-body">
            <div class="preview-capsule-fill" style="width: 85%"></div>
          </div>
          <span>85%</span>
        </div>
        <div v-else-if="previewDisplayMode === 'emoji'" class="preview-emoji">
          <span>😊</span>
          <span>85%</span>
        </div>
      </div>
    </div>
  </div>
</div>
```

**Step 4: Add display modes data and methods**

Add to script setup:
```typescript
import { ref, computed } from 'vue'

const displayModes = [
  { value: 'bubble' as const, label: '像素气泡' },
  { value: 'ring' as const, label: '圆环进度' },
  { value: 'breathing' as const, label: '呼吸灯效' },
  { value: 'capsule' as const, label: '胶囊指示器' },
  { value: 'emoji' as const, label: '头顶数字' }
]

const previewDisplayMode = computed(() => {
  return config.value.display_config?.display_mode || 'bubble'
})

function updateDisplayMode(mode: string) {
  if (config.value.display_config) {
    config.value.display_config.display_mode = mode as any
  }
}

function previewMode(mode: string) {
  // Emit event to main widget for temporary preview
  console.log('Preview mode:', mode)
  // TODO: Implement preview via Tauri event
}
```

**Step 5: Add display settings styles**

Add to `<style scoped>`:

```css
/* Mode Options */
.rpg-mode-options {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.rpg-mode-option {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 8px;
  background: #16213e;
  border: 2px solid #4a4e69;
  cursor: pointer;
  transition: all 0.1s;
}

.rpg-mode-option:hover {
  border-color: #7dd3fc;
}

.rpg-mode-option.active {
  border-color: #ffd700;
  background: rgba(255, 215, 0, 0.1);
}

.rpg-mode-option input[type="radio"] {
  display: none;
}

.rpg-mode-label {
  flex: 1;
  font-size: 8px;
  color: #e0e0e0;
}

.rpg-preview-btn {
  font-family: 'Press Start 2P', monospace;
  font-size: 6px;
  color: #7dd3fc;
  background: transparent;
  border: 1px solid #7dd3fc;
  padding: 2px 4px;
  cursor: pointer;
  transition: all 0.1s;
}

.rpg-preview-btn:hover {
  background: #7dd3fc;
  color: #000;
}

/* Preview Area */
.rpg-preview-area {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 12px;
  background: #0f0f1a;
  border: 1px dashed #4a4e69;
}

.preview-widget {
  width: 64px;
  height: 64px;
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* Preview: Bubble */
.preview-bubble {
  background: #0F172A;
  border: 2px solid #334155;
  padding: 2px 4px;
  font-family: monospace;
  font-size: 6px;
  font-weight: 700;
  color: #94A3B8;
}

/* Preview: Ring */
.preview-ring svg {
  width: 48px;
  height: 48px;
}

/* Preview: Breathing */
.preview-breathing {
  flex-direction: column;
  gap: 2px;
}

.preview-glow {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: radial-gradient(circle, rgba(16,185,129,0.5) 0%, transparent 70%);
  animation: preview-breathe 2s ease-in-out infinite;
}

@keyframes preview-breathe {
  0%, 100% { opacity: 0.5; transform: scale(0.9); }
  50% { opacity: 1; transform: scale(1.1); }
}

/* Preview: Capsule */
.preview-capsule {
  flex-direction: column;
  gap: 2px;
  font-size: 6px;
  color: #94A3B8;
}

.preview-capsule-body {
  width: 32px;
  height: 5px;
  background: #1E293B;
  border-radius: 3px;
  overflow: hidden;
  border: 1px solid #334155;
}

.preview-capsule-fill {
  height: 100%;
  background: #10B981;
}

/* Preview: Emoji */
.preview-emoji {
  flex-direction: column;
  gap: 1px;
  font-size: 8px;
}

.preview-emoji span:last-child {
  font-family: monospace;
  font-size: 6px;
  font-weight: 700;
  color: #94A3B8;
}
```

**Step 6: Commit**

```bash
git add src/components/SettingsPanel.vue
git commit -m "feat: add display settings tab to settings panel"
```

---

## Task 11: Test Display Mode Switching

**Files:**
- Test: Manual testing in dev mode

**Step 1: Start development server**

Run: `npm run tauri:dev`

**Step 2: Verify default mode**

Expected: PetWidget shows pixel bubble (current default)

**Step 3: Open settings panel**

Click on pet to open info panel, then navigate to settings

**Step 4: Switch to ring mode**

Select "圆环进度" in settings

**Step 5: Save settings**

Click [保存] button

**Step 6: Verify ring mode displays**

Expected: PetWidget shows ring progress around pet

**Step 7: Test all modes**

Repeat for breathing, capsule, and emoji modes

**Step 8: Verify persistence**

Restart the application
Expected: Selected mode is preserved

**Step 9: Note any issues**

Document any visual or functional issues found

**Step 10: Commit any fixes**

If issues were found and fixed:
```bash
git add -A
git commit -m "fix: address issues found during display mode testing"
```

---

## Task 12: Update Documentation

**Files:**
- Modify: `CLAUDE.md`

**Step 1: Add display modes section**

Add to CLAUDE.md after the state machine section:

```markdown
## Display Modes

The widget supports 5 display modes for showing 5h Token usage:

| Mode | Description | Visual |
|------|-------------|--------|
| bubble | Pixel bubble (default) |右上角显示 `5h: 85%` |
| ring | Ring progress | 围绕宠物的环形进度条 |
| breathing | Breathing glow | 宠物周围的光晕脉动效果 |
| capsule | Capsule indicator | 底部居中的进度条 |
| emoji | Emoji + number | 宠物头顶的表情+数字 |

Users can switch between modes in the settings panel under [展示设置].
```

**Step 2: Update config structure documentation**

Update the config structure section to include display_config:

```markdown
### Configuration Structure

```json
{
  "api_config": { ... },
  "polling_config": { ... },
  "display_config": {
    "display_mode": "bubble"
  }
}
```
```

**Step 3: Commit**

```bash
git add CLAUDE.md
git commit -m "docs: update documentation for display modes"
```

---

## Task 13: Final Integration Test

**Files:**
- All modified files

**Step 1: Run full build**

Run: `npm run tauri:build`

**Step 2: Test built application**

Run the built executable and verify:
- All display modes work correctly
- Settings panel functions properly
- Mode switching is smooth
- Configuration persists across restarts

**Step 3: Check for console errors**

Open DevTools and verify no errors

**Step 4: Test edge cases**

- Test with 0% usage
- Test with 100% usage
- Test rapid mode switching
- Test with invalid config (should default to bubble)

**Step 5: Create release notes**

Create a brief summary of the feature:

```markdown
## Display Mode System

Added 5 display modes for showing 5h Token usage:
- Pixel Bubble (default)
- Ring Progress
- Breathing Glow
- Capsule Indicator
- Emoji Display

Switch between modes in Settings > Display Settings.
```

**Step 6: Final commit**

```bash
git add -A
git commit -m "feat: complete display mode system implementation"
```

---

## Testing Checklist

- [ ] All 5 display modes render correctly
- [ ] Mode switching works in settings panel
- [ ] Preview buttons work (if implemented)
- [ ] Configuration persists across restarts
- [ ] Animations are smooth
- [ ] Colors match pet state
- [ ] No console errors
- [ ] Build succeeds without warnings
- [ ] Documentation is updated

---

**Implementation Plan Complete**
