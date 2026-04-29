<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import lottie from 'lottie-web'
import bicycleData from '../../assets/pets/Bicycle.json'

type PetState = 'Fresh' | 'Flow' | 'Warning' | 'Panic' | 'Dead'

interface Props {
  width?: number
  height?: number
  state?: PetState
}

const props = withDefaults(defineProps<Props>(), {
  width: 96,
  height: 96,
  state: 'Fresh'
})

const containerRef = ref<HTMLDivElement | null>(null)
let animation: any = null

onMounted(() => {
  if (containerRef.value) {
    try {
      animation = lottie.loadAnimation({
        container: containerRef.value,
        renderer: 'svg',
        loop: true,
        autoplay: true,
        animationData: bicycleData
      })
    } catch (e) {
      console.error('[LottieBicycle] loadAnimation error:', e)
    }
  }
})

onUnmounted(() => {
  if (animation) {
    animation.destroy()
  }
})

watch(() => props.state, (newState) => {
  if (animation) {
    switch (newState) {
      case 'Panic':
        animation.setSpeed(1.5)
        break
      case 'Warning':
        animation.setSpeed(1.2)
        break
      case 'Dead':
        animation.setSpeed(0.5)
        break
      default:
        animation.setSpeed(1)
    }
  }
}, { immediate: true })
</script>

<template>
  <div
    ref="containerRef"
    class="lottie-bicycle-container"
    :style="{ width: `${props.width}px`, height: `${props.height}px` }"
  ></div>
</template>

<style scoped>
.lottie-bicycle-container {
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  z-index: 10;
}

.lottie-bicycle-container :deep(svg) {
  width: 100%;
  height: 100%;
}
</style>
