<script setup lang="ts">
import { computed } from 'vue'
import type { PetState } from '../composables/useUsageState'

const props = defineProps<{
  percent: number
  state: PetState
}>()

const tankW = 130
const tankH = 22
const pad = 2
const innerW = tankW - pad * 2   // 126
const innerH = tankH - pad * 2   // 18

const gradientColors: Record<PetState, [string, string]> = {
  Fresh:   ['#10B981', '#34D399'],
  Flow:    ['#3B82F6', '#60A5FA'],
  Warning: ['#F59E0B', '#FBBF24'],
  Panic:   ['#EF4444', '#F87171'],
  Dead:    ['#4B5563', '#6B7280'],
}

const borderGlows: Record<PetState, string> = {
  Fresh:   'rgba(52,211,153,0.3)',
  Flow:    'rgba(96,165,250,0.3)',
  Warning: 'rgba(251,191,36,0.3)',
  Panic:   'rgba(248,113,113,0.35)',
  Dead:    'rgba(107,114,128,0.2)',
}

const stateAmplitudes: Record<PetState, number> = {
  Fresh: 2, Flow: 2, Warning: 1.5, Panic: 3, Dead: 0.5,
}

const stateDurations: Record<PetState, string> = {
  Fresh: '3s', Flow: '2.5s', Warning: '2s', Panic: '1.2s', Dead: '5s',
}

const colors    = computed(() => gradientColors[props.state])
const glow      = computed(() => borderGlows[props.state])
const amplitude = computed(() => stateAmplitudes[props.state])
const duration  = computed(() => stateDurations[props.state])

const fillHeight = computed(() => {
  const h = (Math.max(0, Math.min(100, props.percent)) / 100) * innerH
  return Math.max(h, 0)
})

const waveY = computed(() => tankH - pad - fillHeight.value)

function wavePath(y: number, amp: number): string {
  const a = Math.min(amp, fillHeight.value / 2)
  return [
    `M -2 ${y}`,
    `C 15 ${y - a}, 35 ${y + a}, 52 ${y}`,
    `C 69 ${y - a}, 89 ${y + a}, 106 ${y}`,
    `C 115 ${y - a}, 126 ${y + a}, 132 ${y}`,
    `L 132 ${tankH} L -2 ${tankH} Z`,
  ].join(' ')
}

const waveFrontPath = computed(() => wavePath(waveY.value, amplitude.value))
const waveBackPath  = computed(() => wavePath(waveY.value, amplitude.value * 0.6))
</script>

<template>
  <div class="liquid-tank-wrap">
    <svg
      :width="tankW"
      :height="tankH"
      :viewBox="`0 0 ${tankW} ${tankH}`"
      xmlns="http://www.w3.org/2000/svg"
      class="liquid-tank"
    >
      <defs>
        <clipPath :id="`tank-clip-${state}`">
          <rect :x="pad" :y="pad" :width="innerW" :height="innerH" :rx="(innerH / 2)" />
        </clipPath>

        <linearGradient :id="`liquid-grad-${state}`" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%"   :stop-color="colors[0]" />
          <stop offset="100%" :stop-color="colors[1]" />
        </linearGradient>
      </defs>

      <!-- Container background -->
      <rect
        :x="pad" :y="pad" :width="innerW" :height="innerH"
        :rx="(innerH / 2)"
        fill="rgba(15,15,17,0.85)"
        class="tank-bg"
      />

      <!-- Liquid (clipped to capsule) -->
      <g :clip-path="`url(#tank-clip-${state})`">
        <path
          :d="waveBackPath"
          :fill="`url(#liquid-grad-${state})`"
          opacity="0.4"
          class="wave wave-back"
          :style="{ animationDuration: `calc(${duration} * 1.4)` }"
        />
        <path
          :d="waveFrontPath"
          :fill="`url(#liquid-grad-${state})`"
          class="wave wave-front"
          :style="{ animationDuration: duration }"
        />
      </g>

      <!-- Border + glow -->
      <rect
        :x="pad" :y="pad" :width="innerW" :height="innerH"
        :rx="(innerH / 2)"
        fill="none"
        stroke="rgba(255,255,255,0.12)"
        stroke-width="1"
        class="tank-stroke"
        :style="{ filter: `drop-shadow(0 0 3px ${glow})` }"
      />
    </svg>
  </div>
</template>

<style scoped>
.liquid-tank-wrap {
  display: flex;
  justify-content: center;
  margin-top: 2px;
  z-index: 10;
}

.liquid-tank {
  display: block;
  overflow: visible;
}

.tank-bg {
  transition: fill 0.6s ease;
}

.tank-stroke {
  transition: stroke 0.6s ease, filter 0.6s ease;
}

.wave {
  will-change: transform;
}
.wave-front {
  animation: waveShift 2.5s linear infinite;
}
.wave-back {
  animation: waveShift 3.5s linear infinite reverse;
}

@keyframes waveShift {
  from { transform: translateX(0); }
  to   { transform: translateX(-52px); }
}
</style>
