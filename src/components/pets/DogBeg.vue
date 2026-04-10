<script setup lang="ts">
// 狗狗讨食动作 - 前爪抬起，期待表情
</script>

<template>
  <svg class="pet-svg" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
    <!-- 狗身体 -->
    <rect x="10" y="48" width="26" height="20" rx="8" fill="#D97706"/>
    <!-- 狗头 -->
    <ellipse cx="23" cy="24" rx="16" ry="14" fill="#D97706"/>
    <!-- 耳朵（垂耳，更下垂） -->
    <ellipse cx="8" cy="30" rx="6" ry="12" fill="#B45309"/>
    <ellipse cx="38" cy="30" rx="6" ry="12" fill="#B45309"/>
    <!-- 大眼睛（期待） -->
    <circle cx="17" cy="22" r="4" fill="white"/>
    <circle cx="29" cy="22" r="4" fill="white"/>
    <circle cx="17" cy="22" r="2.5" fill="#1E293B"/>
    <circle cx="29" cy="22" r="2.5" fill="#1E293B"/>
    <circle cx="18" cy="21" r="1" fill="white"/>
    <circle cx="30" cy="21" r="1" fill="white"/>
    <!-- 鼻子 -->
    <ellipse cx="23" cy="30" rx="4" ry="3" fill="#1E293B"/>
    <!-- 嘴巴（微笑） -->
    <path d="M17 34 Q23 37 29 34" stroke="#1E293B" stroke-width="1.5" fill="none" stroke-linecap="round"/>
    <!-- 抬起的前爪 -->
    <ellipse class="beg-paw-l" cx="14" cy="45" rx="5" ry="4" fill="#B45309"/>
    <ellipse class="beg-paw-r" cx="32" cy="45" rx="5" ry="4" fill="#B45309"/>
    <!-- 后爪 -->
    <ellipse cx="16" cy="68" rx="5" ry="4" fill="#B45309"/>
    <ellipse cx="30" cy="68" rx="5" ry="4" fill="#B45309"/>
    <!-- 尾巴摇摆 -->
    <path class="tail-wag-fast" d="M36 52 Q48 44 44 32 Q40 24 36 30" stroke="#D97706" stroke-width="6" fill="none" stroke-linecap="round"/>
  </svg>
</template>

<style scoped>
.pet-svg {
  width: 80px;
  height: 80px;
  overflow: visible;
}

/* 抬起的爪子抖动 */
.beg-paw-l {
  animation: beg-shake 0.5s ease-in-out infinite;
}
.beg-paw-r {
  animation: beg-shake 0.5s ease-in-out 0.1s infinite;
}

@keyframes beg-shake {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-2px); }
}

/* 快速摇摆尾巴 */
.tail-wag-fast {
  animation: wag-fast 0.2s ease-in-out infinite alternate;
  transform-origin: 36px 52px;
}

@keyframes wag-fast {
  from { transform: rotate(-10deg); }
  to { transform: rotate(10deg); }
}
</style>
