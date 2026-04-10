<script setup lang="ts">
// 狗狗坐立动作 - 端坐，尾巴摇摆
</script>

<template>
  <svg class="pet-svg" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
    <!-- 狗身体（坐姿） -->
    <rect x="10" y="42" width="26" height="24" rx="10" fill="#D97706"/>
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
    <path d="M17 33 Q23 36 29 33" stroke="#1E293B" stroke-width="1.5" fill="none" stroke-linecap="round"/>
    <!-- 舌头 -->
    <ellipse class="tongue-pant" cx="23" cy="36" rx="3" ry="4" fill="#F472B6"/>
    <!-- 前爪 -->
    <ellipse cx="16" cy="66" rx="5" ry="4" fill="#B45309"/>
    <ellipse cx="30" cy="66" rx="5" ry="4" fill="#B45309"/>
    <!-- 摇摆的尾巴 -->
    <path class="tail-wag" d="M36 50 Q50 42 48 30 Q44 22 40 28" stroke="#D97706" stroke-width="6" fill="none" stroke-linecap="round"/>
  </svg>
</template>

<style scoped>
.pet-svg {
  width: 80px;
  height: 80px;
  overflow: visible;
}

/* 舌头喘气 */
.tongue-pant {
  animation: pant 1s ease-in-out infinite;
}

@keyframes pant {
  0%, 100% { ry: 4; }
  50% { ry: 5; }
}

/* 尾巴摇摆 */
.tail-wag {
  animation: wag 0.4s ease-in-out infinite alternate;
  transform-origin: 36px 50px;
}

@keyframes wag {
  from { transform: rotate(-8deg); }
  to { transform: rotate(8deg); }
}
</style>
