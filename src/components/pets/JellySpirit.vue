<script setup lang="ts">
interface Props {
  color: string          // 主色
  strokeColor: string    // 描边色
  eyeColor?: string      // 眼睛颜色（可选，默认深色）
  width?: number
  height?: number
  state?: 'Fresh' | 'Flow' | 'Warning' | 'Panic' | 'Dead'  // 仅用于动画
}

const props = withDefaults(defineProps<Props>(), {
  eyeColor: '#1F2937',
  width: 100,
  height: 100
})
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
        <!-- 身体 - 完整的椭圆形状 -->
        <ellipse
          cx="32"
          cy="36"
          rx="24"
          ry="26"
          :fill="props.color"
          :stroke="props.strokeColor"
          stroke-width="2"
        >
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
        <circle cx="26" cy="32" r="3" :fill="props.eyeColor">
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
        <circle cx="38" cy="32" r="3" :fill="props.eyeColor">
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
