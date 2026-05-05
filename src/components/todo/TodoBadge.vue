<script setup lang="ts">
defineProps<{
  count: number
}>()
</script>

<template>
  <Transition name="badge">
    <div v-if="count > 0" class="todo-badge">
      {{ count }}
    </div>
  </Transition>
</template>

<style scoped>
.todo-badge {
  position: absolute;
  top: -4px;
  right: -4px;
  min-width: 16px;
  height: 16px;
  padding: 0 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f97316;
  color: white;
  font-size: 9px;
  font-weight: 700;
  border-radius: 8px;
  line-height: 1;
  box-shadow: 0 1px 4px rgba(249, 115, 22, 0.4);
}

.badge-enter-active,
.badge-leave-active {
  transition: all 0.2s ease;
}

.badge-enter-from,
.badge-leave-to {
  opacity: 0;
  transform: scale(0.5);
}
</style>
