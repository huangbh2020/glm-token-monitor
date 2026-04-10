<script setup lang="ts">
import { computed } from 'vue'
import PetWidget from './components/PetWidget.vue'
import PopoverPanel from './components/PopoverPanel.vue'
import { usePlatform } from './composables/usePlatform'

const { isWindows, isMacOS } = usePlatform()

const currentComponent = computed(() => {
  if (isMacOS.value) {
    return PopoverPanel
  }
  return PetWidget
})
</script>

<template>
  <div class="app-container">
    <component :is="currentComponent" />
  </div>
</template>

<style scoped>
.app-container {
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
  overflow: visible;
  border: none !important;
  outline: none !important;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
