<script setup lang="ts">
import PetWidget from './components/PetWidget.vue'
</script>

<template>
  <div class="app-container">
    <PetWidget />
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
}
</style>
