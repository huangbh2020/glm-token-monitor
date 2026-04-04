<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useUsageState } from '../composables/useUsageState'
import { useTauriEvents } from '../composables/useTauriEvents'

const { usageData, setupEventListener } = useTauriEvents()
const { usagePercent, petState, stateColor } = useUsageState(
  computed(() => usageData.value.used),
  computed(() => usageData.value.total)
)

const isHovered = ref(false)

onMounted(async () => {
  await setupEventListener()
})
</script>

<template>
  <div
    class="pet-widget"
    data-tauri-drag-region
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
  >
    <div class="pet-container" :style="{ backgroundColor: stateColor }">
      <div class="pet-face" :class="`state-${petState.toLowerCase()}`">
        <div class="eye left"></div>
        <div class="eye right"></div>
        <div class="mouth"></div>
      </div>
    </div>

    <div class="status-bar" :class="{ expanded: isHovered }">
      <div class="progress-track">
        <div class="progress-fill" :style="{ width: `${usagePercent}%`, backgroundColor: stateColor }"></div>
      </div>
      <div class="percent-text">{{ Math.round(usagePercent) }}%</div>
    </div>
  </div>
</template>

<style scoped>
.pet-widget {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 16px;
}

.pet-container {
  width: 80px;
  height: 80px;
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.5s ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.pet-face {
  position: relative;
  width: 50px;
  height: 40px;
}

.eye {
  position: absolute;
  width: 8px;
  height: 8px;
  background: white;
  border-radius: 50%;
  top: 10px;
}

.eye.left { left: 8px; }
.eye.right { right: 8px; }

.mouth {
  position: absolute;
  bottom: 8px;
  left: 50%;
  transform: translateX(-50%);
  width: 16px;
  height: 8px;
  border: 2px solid white;
  border-top: none;
  border-radius: 0 0 16px 16px;
}

.status-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 0;
  opacity: 0;
  overflow: hidden;
  transition: all 0.3s ease;
}

.status-bar.expanded {
  width: 120px;
  opacity: 1;
}

.progress-track {
  flex: 1;
  height: 6px;
  background: rgba(0, 0, 0, 0.1);
  border-radius: 3px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  transition: width 0.5s ease, background-color 0.5s ease;
}

.percent-text {
  font-size: 12px;
  font-weight: 600;
  white-space: nowrap;
}

/* State animations */
/* Fresh 呼吸动画 */
.state-fresh {
  animation: breathe 2s ease-in-out infinite;
}

@keyframes breathe {
  0%, 100% { transform: scale(1); opacity: 0.9; }
  50% { transform: scale(1.05); opacity: 1; }
}

/* Flow 敲打动画 */
.state-flow {
  animation: type 0.8s ease-in-out infinite;
}

@keyframes type {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-2px); }
}

/* Warning 抖动动画 */
.state-warning {
  animation: shake 0.5s ease-in-out infinite;
}

@keyframes shake {
  0%, 100% { transform: translateX(0); }
  25% { transform: translateX(-2px); }
  75% { transform: translateX(2px); }
}

/* Panic 冒汗动画 */
.state-panic {
  animation: sweat 0.3s ease-in-out infinite;
}

@keyframes sweat {
  0%, 100% { transform: scale(1) rotate(0deg); }
  25% { transform: scale(1.02) rotate(-1deg); }
  75% { transform: scale(0.98) rotate(1deg); }
}

/* Dead 灵魂出窍动画 */
.state-dead {
  animation: ghost 2s ease-in-out infinite;
}

@keyframes ghost {
  0% { transform: translateY(0); opacity: 1; }
  50% { transform: translateY(-10px); opacity: 0.5; }
  100% { transform: translateY(0); opacity: 1; }
}
</style>
