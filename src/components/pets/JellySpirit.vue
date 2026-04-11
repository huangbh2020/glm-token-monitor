<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  state: 'Fresh' | 'Flow' | 'Warning' | 'Panic' | 'Dead'
  width?: number
  height?: number
}

const props = withDefaults(defineProps<Props>(), {
  width: 100,
  height: 100
})

// 颜色配置
const colors = {
  Fresh:  { fill: '#10B981', stroke: '#059669', eye: '#1F2937' },
  Flow:   { fill: '#3B82F6', stroke: '#2563EB', eye: '#1F2937' },
  Warning:{ fill: '#F59E0B', stroke: '#D97706', eye: '#1F2937' },
  Panic:  { fill: '#EF4444', stroke: '#DC2626', eye: '#1F2937' },
  Dead:   { fill: '#9CA3AF', stroke: '#6B7280', eye: '#6B7280' }
}

const currentColors = computed(() => colors[props.state])
</script>

<template>
  <div class="spirit-container" :style="{ width: `${width}px`, height: `${height}px` }">
    <svg
      :width="width"
      :height="height"
      viewBox="0 0 64 64"
      xmlns="http://www.w3.org/2000/svg"
      class="spirit-svg"
    >
      <!-- 状态动画组 -->
      <g>
        <!-- Flow: 摇摆动画 -->
        <animateTransform
          v-if="state === 'Flow'"
          attributeName="transform"
          type="rotate"
          values="-3 32 36; 3 32 36; -3 32 36"
          dur="2s"
          repeatCount="indefinite"
          calcMode="spline"
          keyTimes="0;0.5;1"
          keySplines="0.45 0 0.55 1; 0.45 0 0.55 1"
        />
        <!-- Warning: 抖动动画 -->
        <animateTransform
          v-if="state === 'Warning'"
          attributeName="transform"
          type="translate"
          values="0,0; 1,0; 0,0; -1,0; 0,0"
          dur="0.3s"
          repeatCount="indefinite"
        />
        <!-- Panic: 快速弹跳 -->
        <animateTransform
          v-if="state === 'Panic'"
          attributeName="transform"
          type="translate"
          values="0,0; 0,-4; 0,0"
          dur="0.25s"
          repeatCount="indefinite"
          calcMode="spline"
          keyTimes="0;0.5;1"
          keySplines="0.5 0 0.5 1; 0.5 0 0.5 1"
        />
        <!-- 身体 -->
        <ellipse cx="32" cy="36" rx="24" ry="26" :fill="currentColors.fill" :stroke="currentColors.stroke" stroke-width="2">
          <!-- Fresh: 呼吸动画 -->
          <animateTransform
            v-if="state === 'Fresh'"
            attributeName="transform"
            type="scale"
            values="1,1; 1,1.05; 1,1"
            dur="3s"
            repeatCount="indefinite"
            calcMode="spline"
            keyTimes="0;0.5;1"
            keySplines="0.4 0 0.6 1; 0.4 0 0.6 1"
          />
        </ellipse>
        <!-- 高光 -->
        <ellipse cx="26" cy="20" rx="6" ry="4" fill="white" opacity="0.3"/>
        <!-- 左眼 -->
        <circle cx="26" cy="32" r="3" :fill="currentColors.eye">
          <!-- Fresh: 慢眨眼 -->
          <animate
            v-if="state === 'Fresh'"
            attributeName="r"
            values="3;0.3;3"
            dur="4s"
            repeatCount="indefinite"
          />
          <!-- Flow: 正常眨眼 -->
          <animate
            v-if="state === 'Flow'"
            attributeName="r"
            values="3;0.3;3"
            dur="3s"
            repeatCount="indefinite"
          />
          <!-- Warning: 眼睛放大 -->
          <animate
            v-if="state === 'Warning'"
            attributeName="r"
            values="4;4;4"
            dur="1s"
          />
          <!-- Panic: 眼睛瞪大 -->
          <animate
            v-if="state === 'Panic'"
            attributeName="r"
            values="5;5;5"
            dur="1s"
          />
        </circle>
        <!-- 右眼 -->
        <circle cx="38" cy="32" r="3" :fill="currentColors.eye">
          <!-- Fresh: 慢眨眼（稍延迟） -->
          <animate
            v-if="state === 'Fresh'"
            attributeName="r"
            values="3;0.3;3"
            dur="4s"
            begin="0.1s"
            repeatCount="indefinite"
          />
          <!-- Flow: 正常眨眼（稍延迟） -->
          <animate
            v-if="state === 'Flow'"
            attributeName="r"
            values="3;0.3;3"
            dur="3s"
            begin="0.1s"
            repeatCount="indefinite"
          />
          <!-- Warning: 眼睛放大 -->
          <animate
            v-if="state === 'Warning'"
            attributeName="r"
            values="4;4;4"
            dur="1s"
          />
          <!-- Panic: 眼睛瞪大 -->
          <animate
            v-if="state === 'Panic'"
            attributeName="r"
            values="5;5;5"
            dur="1s"
          />
        </circle>
      </g>
    </svg>
  </div>
</template>

<style scoped>
.spirit-container {
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  z-index: 10;
  padding-bottom: 9px;
}

.spirit-svg {
  display: block;
}
</style>
