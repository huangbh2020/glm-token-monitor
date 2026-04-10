<script setup lang="ts">
// 狗狗吠叫动作 - 张嘴，音符飘出
</script>

<template>
  <svg class="pet-svg" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
    <!-- 音符 -->
    <text class="note-a" x="50" y="12" font-size="10" fill="#FCD34D" font-family="monospace">♪</text>
    <text class="note-b" x="58" y="18" font-size="8" fill="#FCD34D" font-family="monospace">♫</text>
    <!-- 狗身体 -->
    <rect x="10" y="42" width="26" height="24" rx="10" fill="#D97706"/>
    <!-- 狗头 -->
    <ellipse cx="23" cy="24" rx="16" ry="14" fill="#D97706"/>
    <!-- 耳朵（竖起） -->
    <polygon points="8,14 12,4 16,14" fill="#D97706"/>
    <polygon points="30,14 34,4 38,14" fill="#D97706"/>
    <!-- 眼睛（睁大） -->
    <circle cx="17" cy="22" r="3.5" fill="white"/>
    <circle cx="29" cy="22" r="3.5" fill="white"/>
    <circle cx="17" cy="22" r="2" fill="#1E293B"/>
    <circle cx="29" cy="22" r="2" fill="#1E293B"/>
    <!-- 张嘴 -->
    <ellipse class="mouth-open" cx="23" cy="32" rx="6" ry="5" fill="#1E293B"/>
    <!-- 舌头伸出 -->
    <ellipse class="tongue-out" cx="23" cy="35" rx="4" ry="3" fill="#F472B6"/>
    <!-- 前爪 -->
    <ellipse cx="16" cy="66" rx="5" ry="4" fill="#B45309"/>
    <ellipse cx="30" cy="66" rx="5" ry="4" fill="#B45309"/>
    <!-- 尾巴竖直 -->
    <path d="M36 50 Q46 38 42 26 Q38 18 34 24" stroke="#D97706" stroke-width="6" fill="none" stroke-linecap="round"/>
  </svg>
</template>

<style scoped>
.pet-svg {
  width: 80px;
  height: 80px;
  overflow: visible;
}

/* 嘴巴开合 */
.mouth-open {
  animation: bark-mouth 0.3s ease-in-out infinite;
}

@keyframes bark-mouth {
  0%, 100% { ry: 5; }
  50% { ry: 7; }
}

/* 舌头伸出 */
.tongue-out {
  animation: bark-tongue 0.3s ease-in-out infinite;
}

@keyframes bark-tongue {
  0%, 100% { cy: 35; }
  50% { cy: 37; }
}

/* 音符飘出 */
.note-a { animation: note-float 0.8s ease-out infinite; }
.note-b { animation: note-float 0.8s ease-out 0.2s infinite; }

@keyframes note-float {
  0% { opacity: 1; transform: translateY(0) scale(1); }
  100% { opacity: 0; transform: translateY(-10px) scale(0.5); }
}
</style>
