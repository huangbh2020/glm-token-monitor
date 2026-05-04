<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'

// 导入所有精灵帧
const spriteModules = import.meta.glob('../../assets/pets/dog_frames/*.png', { eager: true, as: 'url' })

// 按动作分组帧
const actionFrames: Record<string, string[]> = {
  'sit_right': [],
  'sit_left': [],
  'beg': [],
  'bark': [],
}

for (const path in spriteModules) {
  const filename = path.split('/').pop()!
  const action = filename.replace(/_\d+\.png$/, '')
  if (actionFrames[action]) {
    actionFrames[action].push(spriteModules[path] as string)
  }
}

// 按编号排序帧
for (const action in actionFrames) {
  actionFrames[action].sort((a, b) => {
    const numA = parseInt(a.match(/_(\d+)\.png$/)?.[1] || '0')
    const numB = parseInt(b.match(/_(\d+)\.png$/)?.[1] || '0')
    return numA - numB
  })
}

// 动作映射：将 usePetAction 的动作名映射到精灵动作
const ACTION_MAP: Record<string, string> = {
  'dog-sit': 'sit_right',
  'dog-bark': 'bark',
  'dog-walk': 'bark',
  'dog-beg': 'beg',
}

const props = withDefaults(defineProps<{
  action?: string
  fps?: number
  width?: number
  height?: number
}>(), {
  action: 'dog-sit',
  fps: 10,
  width: 80,
  height: 80,
})

const currentFrameIndex = ref(0)
let animTimer: number | null = null

const spriteAction = computed(() => ACTION_MAP[props.action] || 'sit_right')
const frames = computed(() => actionFrames[spriteAction.value] || [])
const currentFrameSrc = computed(() => frames.value[currentFrameIndex.value] || '')

function startAnimation() {
  stopAnimation()
  if (frames.value.length <= 1) return
  animTimer = window.setInterval(() => {
    currentFrameIndex.value = (currentFrameIndex.value + 1) % frames.value.length
  }, 1000 / props.fps)
}

function stopAnimation() {
  if (animTimer) {
    clearInterval(animTimer)
    animTimer = null
  }
}

watch(() => props.action, () => {
  currentFrameIndex.value = 0
  startAnimation()
})

onMounted(startAnimation)
onUnmounted(stopAnimation)
</script>

<template>
  <div class="sprite-dog" :style="{ width: width + 'px', height: height + 'px' }">
    <img
      v-if="currentFrameSrc"
      :src="currentFrameSrc"
      :width="width"
      :height="height"
      draggable="false"
    />
  </div>
</template>

<style scoped>
.sprite-dog {
  display: flex;
  align-items: center;
  justify-content: center;
  pointer-events: none;
}

.sprite-dog img {
  image-rendering: pixelated;
  object-fit: contain;
}
</style>
