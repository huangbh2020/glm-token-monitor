<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useUsageState } from '../composables/useUsageState'
import { useTauriEvents } from '../composables/useTauriEvents'

const { usageData, setupEventListener } = useTauriEvents()
const { petState, stateColor } = useUsageState(
  computed(() => usageData.value.used),
  computed(() => usageData.value.total)
)

onMounted(async () => {
  await setupEventListener()
})
</script>

<template>
  <div class="pet-widget">
    <div
      class="pet-container"
      :style="{ backgroundColor: stateColor }"
      data-tauri-drag-region
    >
      <div class="pet-face" :class="`state-${petState.toLowerCase()}`">
        <div class="eye left"></div>
        <div class="eye right"></div>
        <div class="mouth"></div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.pet-widget {
  width: 96px;
  height: 96px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent !important;
}

.pet-container {
  width: 64px;
  height: 64px;
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.4s ease;
  user-select: none;
  -webkit-app-region: drag;
  app-region: drag;
  cursor: grab;
}

.pet-container:active {
  cursor: grabbing;
}

.pet-face {
  position: relative;
  width: 40px;
  height: 32px;
  user-select: none;
}

.eye {
  position: absolute;
  width: 6px;
  height: 6px;
  background: white;
  border-radius: 50%;
  top: 8px;
  border: none;
}

.eye.left { left: 6px; }
.eye.right { right: 6px; }

.mouth {
  position: absolute;
  bottom: 6px;
  left: 50%;
  transform: translateX(-50%);
  width: 12px;
  height: 6px;
  border: 2px solid white;
  border-top: none;
  border-radius: 0 0 12px 12px;
}

/* 确保所有元素无边框 */
* {
  border: none !important;
  outline: none !important;
}

/* State colors */
.state-fresh .pet-container {
  background-color: #10B981;
}

.state-flow .pet-container {
  background-color: #3B82F6;
}

.state-warning .pet-container {
  background-color: #F59E0B;
}

.state-panic .pet-container {
  background-color: #F97316;
}

.state-dead .pet-container {
  background-color: #6B7280;
}

/* State animations */
.state-fresh {
  animation: breathe 2.5s ease-in-out infinite;
}

@keyframes breathe {
  0%, 100% { transform: scale(1); opacity: 0.85; }
  50% { transform: scale(1.06); opacity: 1; }
}

.state-flow {
  animation: type 1s ease-in-out infinite;
}

@keyframes type {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-1.5px); }
}

.state-warning {
  animation: shake 0.6s ease-in-out infinite;
}

@keyframes shake {
  0%, 100% { transform: translateX(0); }
  25% { transform: translateX(-1.5px); }
  75% { transform: translateX(1.5px); }
}

.state-panic {
  animation: sweat 0.25s ease-in-out infinite;
}

@keyframes sweat {
  0%, 100% { transform: scale(1) rotate(0deg); }
  25% { transform: scale(1.03) rotate(-0.5deg); }
  75% { transform: scale(0.97) rotate(0.5deg); }
}

.state-dead {
  animation: ghost 2.5s ease-in-out infinite;
}

@keyframes ghost {
  0% { transform: translateY(0); opacity: 1; }
  50% { transform: translateY(-8px); opacity: 0.4; }
  100% { transform: translateY(0); opacity: 1; }
}
</style>
