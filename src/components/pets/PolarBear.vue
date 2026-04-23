<script setup lang="ts">
import { computed } from 'vue'

type PetState = 'Fresh' | 'Flow' | 'Warning' | 'Panic' | 'Dead'

interface Props {
  width?: number
  height?: number
  state?: PetState
}

const props = withDefaults(defineProps<Props>(), {
  width: 100,
  height: 100,
  state: 'Fresh'
})

const bobDuration = computed(() => {
  switch (props.state) {
    case 'Flow':
      return '2.2s'
    case 'Warning':
      return '1.2s'
    case 'Panic':
      return '0.62s'
    case 'Dead':
      return '2.8s'
    case 'Fresh':
    default:
      return '3.4s'
  }
})

const bobValues = computed(() => {
  switch (props.state) {
    case 'Flow':
      return '0,0; 0,-1.9; 0,0'
    case 'Warning':
      return '0,0; 0,-1.1; 0,0'
    case 'Panic':
      return '0,0; 0,-1.4; 0,0'
    case 'Dead':
      return '0,0; 0,-0.4; 0,0'
    case 'Fresh':
    default:
      return '0,0; 0,-1.3; 0,0'
  }
})

const bodySwayDuration = computed(() => {
  switch (props.state) {
    case 'Flow':
      return '1.15s'
    case 'Warning':
      return '0.58s'
    case 'Panic':
      return '0.32s'
    case 'Dead':
      return '2.4s'
    case 'Fresh':
    default:
      return '1.95s'
  }
})

const bodySwayAngle = computed(() => {
  switch (props.state) {
    case 'Flow':
      return 5
    case 'Warning':
      return 8
    case 'Panic':
      return 12
    case 'Dead':
      return 3
    case 'Fresh':
    default:
      return 4
  }
})

const bodySwayValues = computed(() => {
  const angle = bodySwayAngle.value
  return `-${angle} 40 49; ${angle} 40 49; -${angle} 40 49`
})

const jitterValues = computed(() => {
  switch (props.state) {
    case 'Warning':
      return '0,0; 0.9,0; -0.9,0; 0,0'
    case 'Panic':
      return '0,0; 1.8,0.6; -1.8,-0.6; 0,0'
    case 'Dead':
      return '0,0; 0,-0.35; 0,0'
    default:
      return '0,0; 0,0; 0,0'
  }
})

const jitterDuration = computed(() => {
  switch (props.state) {
    case 'Warning':
      return '0.26s'
    case 'Panic':
      return '0.12s'
    case 'Dead':
      return '2.7s'
    default:
      return '1s'
  }
})

const earWiggleDuration = computed(() => {
  switch (props.state) {
    case 'Flow':
      return '1.3s'
    case 'Warning':
      return '0.72s'
    case 'Panic':
      return '0.28s'
    case 'Dead':
      return '2.8s'
    case 'Fresh':
    default:
      return '2.3s'
  }
})

const earWiggleAngle = computed(() => {
  switch (props.state) {
    case 'Flow':
      return 4
    case 'Warning':
      return 7
    case 'Panic':
      return 12
    case 'Dead':
      return 2
    case 'Fresh':
    default:
      return 3
  }
})

const leftEarValues = computed(() => {
  const angle = earWiggleAngle.value
  return `-${angle} 28 18; ${angle} 28 18; -${angle} 28 18`
})

const rightEarValues = computed(() => {
  const angle = earWiggleAngle.value
  return `${angle} 52 18; -${angle} 52 18; ${angle} 52 18`
})

const blinkDuration = computed(() => {
  switch (props.state) {
    case 'Flow':
      return '3.3s'
    case 'Warning':
      return '2.5s'
    case 'Panic':
      return '0.9s'
    case 'Dead':
      return '1s'
    case 'Fresh':
    default:
      return '4.8s'
  }
})

const eyeRx = computed(() => {
  switch (props.state) {
    case 'Panic':
      return 2.35
    case 'Warning':
      return 2.1
    case 'Dead':
      return 2.6
    default:
      return 1.9
  }
})

const eyeRy = computed(() => {
  switch (props.state) {
    case 'Panic':
      return 2.7
    case 'Warning':
      return 2.35
    case 'Dead':
      return 0.9
    default:
      return 2.2
  }
})

const blushOpacity = computed(() => {
  switch (props.state) {
    case 'Warning':
      return 0.35
    case 'Panic':
      return 0.24
    case 'Dead':
      return 0.15
    default:
      return 0.42
  }
})

const pawSwingDuration = computed(() => {
  switch (props.state) {
    case 'Flow':
      return '1.05s'
    case 'Warning':
      return '0.65s'
    case 'Panic':
      return '0.34s'
    case 'Dead':
      return '2.8s'
    default:
      return '1.8s'
  }
})

const pawSwingAngle = computed(() => {
  switch (props.state) {
    case 'Flow':
      return 7
    case 'Warning':
      return 9
    case 'Panic':
      return 13
    case 'Dead':
      return 3
    default:
      return 5
  }
})

const leftPawValues = computed(() => {
  const angle = pawSwingAngle.value
  return `${angle} 29 49; -${angle} 29 49; ${angle} 29 49`
})

const rightPawValues = computed(() => {
  const angle = pawSwingAngle.value
  return `-${angle} 51 49; ${angle} 51 49; -${angle} 51 49`
})
</script>

<template>
  <div class="polar-container" :style="{ width: `${props.width}px`, height: `${props.height}px` }">
    <svg
      :width="props.width"
      :height="props.height"
      viewBox="0 0 80 80"
      xmlns="http://www.w3.org/2000/svg"
      class="polar-svg"
    >
      <defs>
        <linearGradient id="polarFur" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stop-color="#ffffff" />
          <stop offset="100%" stop-color="#e8f2fb" />
        </linearGradient>
        <linearGradient id="polarShade" x1="0" y1="0" x2="1" y2="1">
          <stop offset="0%" stop-color="#d7e6f4" />
          <stop offset="100%" stop-color="#bdd0e2" />
        </linearGradient>
        <radialGradient id="polarMuzzle" cx="0.5" cy="0.35" r="0.75">
          <stop offset="0%" stop-color="#ffffff" />
          <stop offset="100%" stop-color="#e9f3fb" />
        </radialGradient>
        <linearGradient id="polarNose" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stop-color="#4a5c6c" />
          <stop offset="100%" stop-color="#2f4152" />
        </linearGradient>
        <radialGradient id="polarEarInner" cx="0.4" cy="0.35" r="0.8">
          <stop offset="0%" stop-color="#ffd5df" />
          <stop offset="100%" stop-color="#f2b8c5" />
        </radialGradient>
      </defs>

      <ellipse cx="40" cy="73" rx="20" ry="4.6" fill="#8ea7be" opacity="0.24" />

      <g>
        <animateTransform
          v-if="props.state === 'Warning' || props.state === 'Panic' || props.state === 'Dead'"
          attributeName="transform"
          type="translate"
          :values="jitterValues"
          :dur="jitterDuration"
          repeatCount="indefinite"
        />

        <animateTransform
          attributeName="transform"
          type="translate"
          :values="bobValues"
          :dur="bobDuration"
          repeatCount="indefinite"
        />

        <animateTransform
          attributeName="transform"
          type="rotate"
          :values="bodySwayValues"
          :dur="bodySwayDuration"
          repeatCount="indefinite"
          additive="sum"
        />

        <animateTransform
          v-if="props.state === 'Dead'"
          attributeName="transform"
          type="rotate"
          values="3 40 49; 5 40 49; 3 40 49"
          dur="2.2s"
          repeatCount="indefinite"
          additive="sum"
        />

        <circle cx="56.2" cy="48.7" r="6.1" fill="url(#polarShade)" stroke="#4f6478" stroke-width="1.7" />
        <ellipse cx="40" cy="50" rx="18.2" ry="15.3" fill="url(#polarFur)" stroke="#4f6478" stroke-width="1.8" />
        <ellipse cx="40" cy="53.1" rx="10.5" ry="8.4" fill="#f8fcff" opacity="0.95" />

        <g>
          <animateTransform
            attributeName="transform"
            type="rotate"
            :values="leftPawValues"
            :dur="pawSwingDuration"
            repeatCount="indefinite"
          />
          <ellipse cx="29.4" cy="49" rx="6.1" ry="9.2" fill="url(#polarShade)" stroke="#4f6478" stroke-width="1.6" />
        </g>

        <g>
          <animateTransform
            attributeName="transform"
            type="rotate"
            :values="rightPawValues"
            :dur="pawSwingDuration"
            repeatCount="indefinite"
          />
          <ellipse cx="50.6" cy="49" rx="6.1" ry="9.2" fill="url(#polarShade)" stroke="#4f6478" stroke-width="1.6" />
        </g>

        <ellipse cx="32.8" cy="65.2" rx="6.8" ry="3.7" fill="url(#polarShade)" stroke="#4f6478" stroke-width="1.4" />
        <ellipse cx="47.2" cy="65.2" rx="6.8" ry="3.7" fill="url(#polarShade)" stroke="#4f6478" stroke-width="1.4" />
        <ellipse cx="32.8" cy="65.4" rx="3.5" ry="1.5" fill="#8ea9c0" />
        <ellipse cx="47.2" cy="65.4" rx="3.5" ry="1.5" fill="#8ea9c0" />

        <g>
          <animateTransform
            attributeName="transform"
            type="rotate"
            :values="leftEarValues"
            :dur="earWiggleDuration"
            repeatCount="indefinite"
          />
          <circle cx="28" cy="18" r="5.7" fill="url(#polarFur)" stroke="#4f6478" stroke-width="1.8" />
          <circle cx="28" cy="18.4" r="2.4" fill="url(#polarEarInner)" />
        </g>

        <g>
          <animateTransform
            attributeName="transform"
            type="rotate"
            :values="rightEarValues"
            :dur="earWiggleDuration"
            repeatCount="indefinite"
          />
          <circle cx="52" cy="18" r="5.7" fill="url(#polarFur)" stroke="#4f6478" stroke-width="1.8" />
          <circle cx="52" cy="18.4" r="2.4" fill="url(#polarEarInner)" />
        </g>

        <ellipse cx="40" cy="29.2" rx="18.8" ry="16.4" fill="url(#polarFur)" stroke="#4f6478" stroke-width="2" />
        <ellipse cx="40" cy="35.4" rx="10.8" ry="8.1" fill="url(#polarMuzzle)" stroke="#c7d9e7" stroke-width="0.9" />

        <ellipse cx="31.7" cy="34.9" rx="2.4" ry="1.5" fill="#f3b6c5" :opacity="blushOpacity" />
        <ellipse cx="48.3" cy="34.9" rx="2.4" ry="1.5" fill="#f3b6c5" :opacity="blushOpacity" />

        <g v-if="props.state !== 'Dead'">
          <ellipse cx="34.4" cy="28.4" :rx="eyeRx" :ry="eyeRy" fill="#223447">
            <animate
              attributeName="ry"
              :values="`${eyeRy};0.45;${eyeRy}`"
              :dur="blinkDuration"
              repeatCount="indefinite"
            />
          </ellipse>
          <ellipse cx="45.6" cy="28.4" :rx="eyeRx" :ry="eyeRy" fill="#223447">
            <animate
              attributeName="ry"
              :values="`${eyeRy};0.45;${eyeRy}`"
              :dur="blinkDuration"
              begin="0.12s"
              repeatCount="indefinite"
            />
          </ellipse>
          <circle cx="33.6" cy="27.5" r="0.7" fill="#ffffff" />
          <circle cx="44.8" cy="27.5" r="0.7" fill="#ffffff" />
        </g>

        <g v-else>
          <path d="M31.4 26.4 L37 30.4 M37 26.4 L31.4 30.4" stroke="#223447" stroke-width="1.7" stroke-linecap="round" />
          <path d="M43 26.4 L48.6 30.4 M48.6 26.4 L43 30.4" stroke="#223447" stroke-width="1.7" stroke-linecap="round" />
        </g>

        <path d="M36.7 33.4 L43.3 33.4 L41.9 36.6 L38.1 36.6 Z" fill="url(#polarNose)" />

        <path
          v-if="props.state === 'Fresh' || props.state === 'Flow'"
          d="M35.6 39.2 Q40 43.1 44.4 39.2"
          fill="none"
          stroke="#425668"
          stroke-width="1.4"
          stroke-linecap="round"
        />
        <path
          v-else-if="props.state === 'Warning'"
          d="M35.4 40.1 Q40 38.7 44.6 40.1"
          fill="none"
          stroke="#425668"
          stroke-width="1.4"
          stroke-linecap="round"
        />
        <ellipse
          v-else-if="props.state === 'Panic'"
          cx="40"
          cy="40.8"
          rx="2.6"
          ry="1.9"
          fill="#425668"
        >
          <animate
            attributeName="ry"
            values="1.9;3.1;1.9"
            dur="0.22s"
            repeatCount="indefinite"
          />
        </ellipse>
        <path
          v-else
          d="M36 40.5 L44 40.5"
          fill="none"
          stroke="#425668"
          stroke-width="1.4"
          stroke-linecap="round"
        />
      </g>
    </svg>
  </div>
</template>

<style scoped>
.polar-container {
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  z-index: 10;
  padding-bottom: 3px;
}

.polar-svg {
  display: block;
}
</style>
