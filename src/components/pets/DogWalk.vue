<script setup lang="ts">
// 狗狗踱步动作 - 左右走动
</script>

<template>
  <svg class="pet-svg" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
    <!-- 狗身体 -->
    <rect class="dog-body" x="10" y="42" width="26" height="24" rx="10" fill="#D97706"/>
    <!-- 狗头 -->
    <ellipse cx="23" cy="24" rx="16" ry="14" fill="#D97706"/>
    <!-- 耳朵（垂耳） -->
    <ellipse cx="10" cy="28" rx="6" ry="10" fill="#B45309"/>
    <ellipse cx="36" cy="28" rx="6" ry="10" fill="#B45309"/>
    <!-- 眼睛 -->
    <circle cx="17" cy="22" r="3" fill="white"/>
    <circle cx="29" cy="22" r="3" fill="white"/>
    <circle cx="17" cy="22" r="1.5" fill="#1E293B"/>
    <circle cx="29" cy="22" r="1.5" fill="#1E293B"/>
    <circle cx="17.5" cy="21" r="0.6" fill="white"/>
    <circle cx="29.5" cy="21" r="0.6" fill="white"/>
    <!-- 鼻子 -->
    <ellipse cx="23" cy="30" rx="4" ry="3" fill="#1E293B"/>
    <!-- 嘴巴 -->
    <path d="M17 33 Q23 35 29 33" stroke="#1E293B" stroke-width="1.5" fill="none" stroke-linecap="round"/>
    <!-- 前爪（交替移动） -->
    <ellipse class="paw-fl" cx="16" cy="66" rx="5" ry="4" fill="#B45309"/>
    <ellipse class="paw-fr" cx="30" cy="66" rx="5" ry="4" fill="#B45309"/>
    <!-- 尾巴摆动 -->
    <path class="tail-wag" d="M36 50 Q48 44 44 32 Q40 24 36 30" stroke="#D97706" stroke-width="6" fill="none" stroke-linecap="round"/>
  </svg>
</template>

<style scoped>
.pet-svg {
  width: 80px;
  height: 80px;
  overflow: visible;
}

/* 身体轻微晃动 */
.dog-body {
  animation: walk-bob 0.6s ease-in-out infinite;
}

@keyframes walk-bob {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-2px); }
}

/* 爪子交替 */
.paw-fl {
  animation: walk-paw 0.6s ease-in-out infinite;
}
.paw-fr {
  animation: walk-paw 0.6s ease-in-out 0.3s infinite;
}

@keyframes walk-paw {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-3px); }
}

/* 尾巴摆动 */
.tail-wag {
  animation: wag 0.3s ease-in-out infinite alternate;
  transform-origin: 36px 50px;
}

@keyframes wag {
  from { transform: rotate(-5deg); }
  to { transform: rotate(5deg); }
}
</style>
