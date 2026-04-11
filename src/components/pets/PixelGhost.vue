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
  <div class="ghost-container" :style="{ width: `${width}px`, height: `${height}px` }">
    <svg
      :width="width"
      :height="height"
      viewBox="0 0 64 64"
      xmlns="http://www.w3.org/2000/svg"
      class="ghost-svg"
    >
      <g>
        <!-- 整体漂浮动画 -->
        <animateTransform
          attributeName="transform"
          type="translate"
          values="0,0; 0,-3; 0,0"
          dur="2s"
          repeatCount="indefinite"
          calcMode="spline"
          keyTimes="0;0.5;1"
          keySplines="0.45 0 0.55 1; 0.45 0 0.55 1"
        />

        <!-- Flow: 左右摇摆 -->
        <animateTransform
          v-if="state === 'Flow'"
          attributeName="transform"
          type="rotate"
          values="-2 32 32; 2 32 32; -2 32 32"
          dur="1.5s"
          repeatCount="indefinite"
          additive="sum"
        />

        <!-- Warning: 抖动 -->
        <animateTransform
          v-if="state === 'Warning'"
          attributeName="transform"
          type="translate"
          values="0,0; 1.5,0; -1.5,0; 0,0"
          dur="0.2s"
          repeatCount="indefinite"
          additive="sum"
        />

        <!-- Panic: 快速抖动 -->
        <animateTransform
          v-if="state === 'Panic'"
          attributeName="transform"
          type="translate"
          values="0,0; 2,1; -2,-1; 0,0"
          dur="0.1s"
          repeatCount="indefinite"
          additive="sum"
        />

        <!-- 身体 - 完整的幽灵形状 -->
        <path
          :fill="props.color"
          :stroke="props.strokeColor"
          stroke-width="2"
          stroke-linejoin="round"
          stroke-linecap="round"
          d="M 18 14 Q 32 12 46 14 Q 50 14 50 18 L 50 42 Q 50 46 46 46 Q 44 48 42 46 Q 40 44 38 46 Q 36 48 34 46 Q 32 44 30 46 Q 28 48 26 46 Q 24 44 22 46 Q 20 48 18 46 Q 14 46 14 42 L 14 18 Q 14 14 18 14 Z"
        >
          <!-- Fresh: 呼吸缩放 -->
          <animateTransform
            v-if="state === 'Fresh'"
            attributeName="transform"
            type="scale"
            values="1,1; 1.02,1.02; 1,1"
            dur="2.5s"
            repeatCount="indefinite"
            calcMode="spline"
            keyTimes="0;0.5;1"
            keySplines="0.4 0 0.6 1; 0.4 0 0.6 1"
          />
          <!-- Panic: 快速脉动 -->
          <animateTransform
            v-if="state === 'Panic'"
            attributeName="transform"
            type="scale"
            values="1,1; 1.05,1.05; 1,1"
            dur="0.3s"
            repeatCount="indefinite"
            calcMode="spline"
            keyTimes="0;0.5;1"
            keySplines="0.5 0 0.5 1; 0.5 0 0.5 1"
          />
        </path>

        <!-- 高光 -->
        <ellipse cx="24" cy="20" rx="5" ry="3.5" fill="white" opacity="0.3"/>

        <!-- 腮红 -->
        <ellipse cx="19" cy="30" rx="4" ry="2.5" :fill="props.color" opacity="0.35">
          <animate
            attributeName="opacity"
            values="0.3;0.5;0.3"
            dur="2s"
            repeatCount="indefinite"
          />
        </ellipse>
        <ellipse cx="45" cy="30" rx="4" ry="2.5" :fill="props.color" opacity="0.35">
          <animate
            attributeName="opacity"
            values="0.3;0.5;0.3"
            dur="2s"
            begin="0.2s"
            repeatCount="indefinite"
          />
        </ellipse>

        <!-- 左眼 -->
        <g class="eye-group">
          <ellipse cx="24" cy="26" rx="6" ry="6.5" :fill="props.eyeColor">
            <!-- Fresh: 眨眼 -->
            <animate
              v-if="state === 'Fresh'"
              attributeName="ry"
              values="6.5;0.8;6.5"
              dur="6s"
              repeatCount="indefinite"
            />
            <!-- Flow: 眨眼 -->
            <animate
              v-if="state === 'Flow'"
              attributeName="ry"
              values="6.5;0.8;6.5"
              dur="5s"
              repeatCount="indefinite"
            />
            <!-- Warning: 眼睛放大 -->
            <animate
              v-if="state === 'Warning'"
              attributeName="rx"
              values="6;7;6"
              dur="2s"
              repeatCount="indefinite"
            />
            <!-- Panic: 眼睛瞪大且快速眨眼 -->
            <animate
              v-if="state === 'Panic'"
              attributeName="ry"
              values="7;1;7"
              dur="0.8s"
              repeatCount="indefinite"
            />
          </ellipse>
          <!-- 眼神高光 -->
          <circle cx="22" cy="24" r="2" fill="white" opacity="0.85">
            <animate
              v-if="state === 'Panic'"
              attributeName="opacity"
              values="0.85;0.3;0.85"
              dur="0.4s"
              repeatCount="indefinite"
            />
          </circle>
        </g>

        <!-- 右眼 -->
        <g class="eye-group">
          <ellipse cx="40" cy="26" rx="6" ry="6.5" :fill="props.eyeColor">
            <!-- Fresh: 眨眼（延迟） -->
            <animate
              v-if="state === 'Fresh'"
              attributeName="ry"
              values="6.5;0.8;6.5"
              dur="6s"
              begin="0.15s"
              repeatCount="indefinite"
            />
            <!-- Flow: 眨眼（延迟） -->
            <animate
              v-if="state === 'Flow'"
              attributeName="ry"
              values="6.5;0.8;6.5"
              dur="5s"
              begin="0.15s"
              repeatCount="indefinite"
            />
            <!-- Warning: 眼睛放大 -->
            <animate
              v-if="state === 'Warning'"
              attributeName="rx"
              values="6;7;6"
              dur="2s"
              repeatCount="indefinite"
            />
            <!-- Panic: 眼睛瞪大且快速眨眼 -->
            <animate
              v-if="state === 'Panic'"
              attributeName="ry"
              values="7;1;7"
              dur="0.8s"
              begin="0.1s"
              repeatCount="indefinite"
            />
          </ellipse>
          <!-- 眼神高光 -->
          <circle cx="38" cy="24" r="2" fill="white" opacity="0.85">
            <animate
              v-if="state === 'Panic'"
              attributeName="opacity"
              values="0.85;0.3;0.85"
              dur="0.4s"
              begin="0.1s"
              repeatCount="indefinite"
            />
          </circle>
        </g>

        <!-- 小嘴巴 -->
        <path :stroke="props.strokeColor" stroke-width="1.5" fill="none" stroke-linecap="round"
          v-if="state === 'Fresh' || state === 'Flow'"
          d="M 29 34 Q 32 37 35 34"
        >
          <animate
            attributeName="d"
            values="M 29 34 Q 32 37 35 34; M 29 35 Q 32 38 35 35; M 29 34 Q 32 37 35 34"
            dur="2s"
            repeatCount="indefinite"
          />
        </path>

        <!-- Warning: 担心嘴型 -->
        <path :stroke="props.strokeColor" stroke-width="1.5" fill="none" stroke-linecap="round"
          v-if="state === 'Warning'"
          d="M 29 35 Q 32 33 35 35"
        />

        <!-- Panic: 惊恐嘴型 -->
        <ellipse :fill="props.strokeColor"
          v-if="state === 'Panic'"
          cx="32"
          cy="36"
          rx="3.5"
          ry="2.5"
        >
          <animate
            attributeName="ry"
            values="2.5;3.5;2.5"
            dur="0.2s"
            repeatCount="indefinite"
          />
        </ellipse>

        <!-- Dead: X_X 眼睛 -->
        <g v-if="state === 'Dead'">
          <path :stroke="props.strokeColor" stroke-width="2" stroke-linecap="round"
            d="M 20 23 L 28 31 M 28 23 L 20 31"
          />
          <path :stroke="props.strokeColor" stroke-width="2" stroke-linecap="round"
            d="M 36 23 L 44 31 M 44 23 L 36 31"
          />
        </g>
      </g>
    </svg>
  </div>
</template>

<style scoped>
.ghost-container {
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  z-index: 10;
  padding-bottom: 5px;
}

.ghost-svg {
  display: block;
}
</style>
