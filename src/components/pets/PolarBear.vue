<script setup lang="ts">
import sleepingCatGif from '../../assets/pets/Ssssttt!!, Shut up! The cat is sleeping.gif'

type PetState = 'Fresh' | 'Flow' | 'Warning' | 'Panic' | 'Dead'

interface Props {
  width?: number
  height?: number
  state?: PetState
}

const props = withDefaults(defineProps<Props>(), {
  width: 100,
  height: 100,
  state: 'Fresh'
})
</script>

<template>
  <div class="polar-container" :style="{ width: `${props.width}px`, height: `${props.height}px` }">
    <img
      :src="sleepingCatGif"
      :style="{
        width: `${props.width}px`,
        height: `${props.height}px`
      }"
      class="pet-gif"
      alt="Sleeping Cat"
    />
  </div>
</template>

<style scoped>
.polar-container {
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  z-index: 10;
}

.pet-gif {
  object-fit: contain;
  border-radius: 8px;
}
</style>
