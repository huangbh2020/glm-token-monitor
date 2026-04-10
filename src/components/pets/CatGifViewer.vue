<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  action: string
  width?: number
  height?: number
}

const props = withDefaults(defineProps<Props>(), {
  width: 100,
  height: 100
})

const anim = computed(() => {
  switch (props.action) {
    case 'cat-sleep':
      return { eyes: 'closed', tail: 'slow', bounce: false, ears: 'relaxed' }
    case 'cat-play':
      return { eyes: 'happy', tail: 'fast', bounce: true, ears: 'alert' }
    case 'cat-stare':
      return { eyes: 'wide', tail: 'still', bounce: false, ears: 'alert' }
    case 'cat-stretch':
      return { eyes: 'happy', tail: 'medium', bounce: true, ears: 'relaxed' }
    case 'cat-walk':
      return { eyes: 'open', tail: 'fast', bounce: true, ears: 'alert' }
    case 'cat-sit':
      return { eyes: 'happy', tail: 'pendulum', bounce: false, ears: 'relaxed' }
    default:
      return { eyes: 'open', tail: 'medium', bounce: false, ears: 'relaxed' }
  }
})

const isSit = computed(() => props.action === 'cat-sit')
</script>

<template>
  <div class="cat-container" :style="{ width: `${width}px`, height: `${height}px` }">
    <svg
      :width="width"
      :height="height"
      viewBox="0 0 128 128"
      xmlns="http://www.w3.org/2000/svg"
      class="cat-svg"
    >
      <defs>
        <!-- 橘猫主色渐变 -->
        <linearGradient id="furMain" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stop-color="#F5A623"/>
          <stop offset="100%" stop-color="#E08A1E"/>
        </linearGradient>
        <!-- 浅色腹部 -->
        <linearGradient id="furBelly" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stop-color="#FDEBD0"/>
          <stop offset="100%" stop-color="#FAE0B8"/>
        </linearGradient>
        <!-- 虎斑深色 -->
        <linearGradient id="tabbyDark" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stop-color="#C47A17"/>
          <stop offset="100%" stop-color="#A86B10"/>
        </linearGradient>
        <!-- 鼻子粉色 -->
        <radialGradient id="nosePink" cx="50%" cy="40%">
          <stop offset="0%" stop-color="#F5A0A0"/>
          <stop offset="100%" stop-color="#E08080"/>
        </radialGradient>
        <!-- 耳内粉色 -->
        <radialGradient id="earPink" cx="50%" cy="60%">
          <stop offset="0%" stop-color="#F8D0C0"/>
          <stop offset="100%" stop-color="#F0B8A8"/>
        </radialGradient>
        <!-- 眼睛虹膜 -->
        <radialGradient id="irisGold" cx="45%" cy="40%">
          <stop offset="0%" stop-color="#F5D060"/>
          <stop offset="50%" stop-color="#D4A030"/>
          <stop offset="100%" stop-color="#B08020"/>
        </radialGradient>
        <radialGradient id="irisGreen" cx="45%" cy="40%">
          <stop offset="0%" stop-color="#90C860"/>
          <stop offset="50%" stop-color="#6CA040"/>
          <stop offset="100%" stop-color="#508030"/>
        </radialGradient>
      </defs>

      <!-- ============================================================ -->
      <!--  尾巴 g#tail                                                  -->
      <!-- ============================================================ -->
      <g id="tail">
        <animateTransform
          v-if="anim.tail === 'pendulum'"
          attributeName="transform" type="rotate"
          values="-6 88 92; 8 88 92; -6 88 92"
          dur="3s" repeatCount="indefinite"
          calcMode="spline" keyTimes="0;0.5;1"
          keySplines="0.45 0 0.55 1; 0.45 0 0.55 1"
        />
        <animateTransform
          v-if="anim.tail === 'fast'"
          attributeName="transform" type="rotate"
          values="-12 88 92; 16 88 92; -12 88 92"
          dur="0.6s" repeatCount="indefinite"
        />
        <animateTransform
          v-if="anim.tail === 'medium'"
          attributeName="transform" type="rotate"
          values="-8 88 92; 10 88 92; -8 88 92"
          dur="1.4s" repeatCount="indefinite"
          calcMode="spline" keyTimes="0;0.5;1"
          keySplines="0.45 0 0.55 1; 0.45 0 0.55 1"
        />
        <animateTransform
          v-if="anim.tail === 'slow'"
          attributeName="transform" type="rotate"
          values="-4 88 92; 6 88 92; -4 88 92"
          dur="2.5s" repeatCount="indefinite"
          calcMode="spline" keyTimes="0;0.5;1"
          keySplines="0.45 0 0.55 1; 0.45 0 0.55 1"
        />
        <!-- 尾巴轮廓 -->
        <path d="M88 92 C98 84, 114 72, 116 56 C118 46, 112 42, 108 48 C104 54, 106 66, 96 78 C92 82, 88 88, 88 90"
          fill="url(#furMain)" stroke="#92400E" stroke-width="1.2" stroke-linejoin="round"/>
        <!-- 尾巴虎斑环纹 -->
        <path d="M98 82 C100 78, 102 74, 101 70" fill="none" stroke="#B87A18" stroke-width="3" stroke-linecap="round" opacity="0.5"/>
        <path d="M104 70 C106 64, 108 58, 106 52" fill="none" stroke="#B87A18" stroke-width="3" stroke-linecap="round" opacity="0.5"/>
        <path d="M112 54 C113 50, 112 46, 110 44" fill="none" stroke="#B87A18" stroke-width="2.5" stroke-linecap="round" opacity="0.45"/>
        <!-- 尾尖深色 -->
        <path d="M110 48 C112 42, 116 42, 116 48 C116 44, 113 44, 112 46" fill="#A86B10" opacity="0.4"/>
      </g>

      <!-- ============================================================ -->
      <!--  后腿                                                         -->
      <!-- ============================================================ -->
      <g id="back-legs">
        <!-- 左后腿 -->
        <path d="M34 96 C30 100, 28 108, 30 114 C32 118, 38 120, 42 118 C46 116, 46 110, 44 106 C42 102, 38 98, 36 96"
          fill="url(#furMain)" stroke="#92400E" stroke-width="1.2" stroke-linejoin="round"/>
        <ellipse cx="36" cy="116" rx="8" ry="5" fill="url(#furBelly)" stroke="#92400E" stroke-width="0.8"/>
        <!-- 肉垫 -->
        <ellipse cx="36" cy="118" rx="4" ry="2.5" fill="#F0B8A8" opacity="0.6"/>
        <circle cx="32" cy="116" r="1.5" fill="#F0B8A8" opacity="0.4"/>
        <circle cx="40" cy="116" r="1.5" fill="#F0B8A8" opacity="0.4"/>

        <!-- 右后腿 -->
        <path d="M94 96 C98 100, 100 108, 98 114 C96 118, 90 120, 86 118 C82 116, 82 110, 84 106 C86 102, 90 98, 92 96"
          fill="url(#furMain)" stroke="#92400E" stroke-width="1.2" stroke-linejoin="round"/>
        <ellipse cx="92" cy="116" rx="8" ry="5" fill="url(#furBelly)" stroke="#92400E" stroke-width="0.8"/>
        <ellipse cx="92" cy="118" rx="4" ry="2.5" fill="#F0B8A8" opacity="0.6"/>
        <circle cx="88" cy="116" r="1.5" fill="#F0B8A8" opacity="0.4"/>
        <circle cx="96" cy="116" r="1.5" fill="#F0B8A8" opacity="0.4"/>
      </g>

      <!-- ============================================================ -->
      <!--  主躯干 g#body                                                -->
      <!-- ============================================================ -->
      <g id="body">
        <animateTransform
          v-if="isSit"
          attributeName="transform" type="scale"
          values="1,1; 1,1.03; 1,1"
          dur="3s" repeatCount="indefinite"
          calcMode="spline" keyTimes="0;0.5;1"
          keySplines="0.45 0 0.55 1; 0.45 0 0.55 1"
          additive="sum"
        />
        <animateTransform
          v-if="anim.bounce"
          attributeName="transform" type="translate"
          values="0,0; 0,-5; 0,0"
          dur="0.5s" repeatCount="indefinite"
          calcMode="spline" keyTimes="0;0.4;1"
          keySplines="0.3 0 0.7 1; 0.3 0 0.7 1"
        />

        <!-- 身体轮廓 — 坐姿梨形 -->
        <path d="M34 70 C28 78, 26 90, 30 100 C34 108, 44 114, 64 114 C84 114, 94 108, 98 100 C102 90, 100 78, 94 70 C88 62, 76 58, 64 58 C52 58, 40 62, 34 70 Z"
          fill="url(#furMain)" stroke="#92400E" stroke-width="1.5" stroke-linejoin="round"/>

        <!-- 胸腹部浅色区域 -->
        <path d="M42 72 C38 80, 38 92, 42 102 C46 108, 54 112, 64 112 C74 112, 82 108, 86 102 C90 92, 90 80, 86 72 C80 66, 72 64, 64 64 C56 64, 48 66, 42 72 Z"
          fill="url(#furBelly)" opacity="0.7"/>

        <!-- 身体虎斑条纹 -->
        <path d="M36 76 C40 80, 38 86, 36 90" fill="none" stroke="#B87A18" stroke-width="2.5" stroke-linecap="round" opacity="0.4"/>
        <path d="M44 72 C46 78, 44 84, 42 88" fill="none" stroke="#B87A18" stroke-width="2" stroke-linecap="round" opacity="0.35"/>
        <path d="M92 76 C88 80, 90 86, 92 90" fill="none" stroke="#B87A18" stroke-width="2.5" stroke-linecap="round" opacity="0.4"/>
        <path d="M84 72 C82 78, 84 84, 86 88" fill="none" stroke="#B87A18" stroke-width="2" stroke-linecap="round" opacity="0.35"/>

        <!-- ============================================================ -->
        <!--  前腿                                                         -->
        <!-- ============================================================ -->
        <g id="front-legs">
          <!-- 左前腿 -->
          <path d="M40 88 C38 94, 36 102, 36 108 C36 112, 40 116, 44 116 C48 116, 50 112, 50 108 C50 102, 48 96, 46 90"
            fill="url(#furMain)" stroke="#92400E" stroke-width="1.2" stroke-linejoin="round"/>
          <!-- 左前爪 -->
          <ellipse cx="42" cy="115" rx="7" ry="4.5" fill="url(#furBelly)" stroke="#92400E" stroke-width="0.8"/>
          <ellipse cx="42" cy="117" rx="3.5" ry="2" fill="#F0B8A8" opacity="0.5"/>
          <circle cx="38" cy="115" r="1.2" fill="#F0B8A8" opacity="0.35"/>
          <circle cx="46" cy="115" r="1.2" fill="#F0B8A8" opacity="0.35"/>

          <!-- 右前腿 -->
          <path d="M88 88 C90 94, 92 102, 92 108 C92 112, 88 116, 84 116 C80 116, 78 112, 78 108 C78 102, 80 96, 82 90"
            fill="url(#furMain)" stroke="#92400E" stroke-width="1.2" stroke-linejoin="round"/>
          <!-- 右前爪 -->
          <ellipse cx="86" cy="115" rx="7" ry="4.5" fill="url(#furBelly)" stroke="#92400E" stroke-width="0.8"/>
          <ellipse cx="86" cy="117" rx="3.5" ry="2" fill="#F0B8A8" opacity="0.5"/>
          <circle cx="82" cy="115" r="1.2" fill="#F0B8A8" opacity="0.35"/>
          <circle cx="90" cy="115" r="1.2" fill="#F0B8A8" opacity="0.35"/>
        </g>

        <!-- ============================================================ -->
        <!--  头部 g#head                                                  -->
        <!-- ============================================================ -->
        <g id="head">
          <animateTransform
            v-if="isSit"
            attributeName="transform" type="rotate"
            values="0 64 38; 1.5 64 38; 0 64 38; -0.8 64 38; 0 64 38"
            dur="3s" repeatCount="indefinite"
            calcMode="spline" keyTimes="0;0.25;0.5;0.75;1"
            keySplines="0.45 0 0.55 1; 0.45 0 0.55 1; 0.45 0 0.55 1; 0.45 0 0.55 1"
          />
          <animateTransform
            v-if="!isSit"
            attributeName="transform" type="translate"
            values="0,0; 0,-1; 0,0"
            dur="2s" repeatCount="indefinite"
          />

          <!-- 左耳 — 三角形，微圆角 -->
          <path
            d="M36 30 C34 20, 38 8, 42 6 C46 4, 50 8, 52 14 C54 20, 54 28, 52 32"
            fill="url(#furMain)" stroke="#92400E" stroke-width="1.5" stroke-linejoin="round"
          >
            <animate v-if="anim.ears === 'alert'"
              attributeName="d"
              values="M36 30 C34 20, 38 8, 42 6 C46 4, 50 8, 52 14 C54 20, 54 28, 52 32;
                      M35 28 C33 18, 37 6, 41 4 C45 2, 49 6, 51 12 C53 18, 53 26, 51 30;
                      M36 30 C34 20, 38 8, 42 6 C46 4, 50 8, 52 14 C54 20, 54 28, 52 32"
              dur="0.8s" repeatCount="indefinite"/>
          </path>
          <!-- 左耳内 -->
          <path d="M40 26 C40 18, 42 12, 44 10 C46 10, 48 14, 49 18 C50 22, 49 26, 48 28"
            fill="url(#earPink)" opacity="0.7"/>

          <!-- 右耳 -->
          <path
            d="M92 30 C94 20, 90 8, 86 6 C82 4, 78 8, 76 14 C74 20, 74 28, 76 32"
            fill="url(#furMain)" stroke="#92400E" stroke-width="1.5" stroke-linejoin="round"
          >
            <animate v-if="anim.ears === 'alert'"
              attributeName="d"
              values="M92 30 C94 20, 90 8, 86 6 C82 4, 78 8, 76 14 C74 20, 74 28, 76 32;
                      M93 28 C95 18, 91 6, 87 4 C83 2, 79 6, 77 12 C75 18, 75 26, 77 30;
                      M92 30 C94 20, 90 8, 86 6 C82 4, 78 8, 76 14 C74 20, 74 28, 76 32"
              dur="0.8s" repeatCount="indefinite"/>
          </path>
          <!-- 右耳内 -->
          <path d="M88 26 C88 18, 86 12, 84 10 C82 10, 80 14, 79 18 C78 22, 79 26, 80 28"
            fill="url(#earPink)" opacity="0.7"/>

          <!-- 头部轮廓 — 更像猫的楔形头 -->
          <path d="M36 36 C32 28, 34 18, 42 12 C48 8, 56 6, 64 6 C72 6, 80 8, 86 12 C94 18, 96 28, 92 36 C88 44, 82 50, 76 52 C70 54, 58 54, 52 52 C46 50, 40 44, 36 36 Z"
            fill="url(#furMain)" stroke="#92400E" stroke-width="1.5" stroke-linejoin="round"/>

          <!-- 面部浅色（嘴部区域） -->
          <path d="M44 38 C42 42, 42 48, 46 52 C50 56, 58 58, 64 58 C70 58, 78 56, 82 52 C86 48, 86 42, 84 38 C80 34, 72 32, 64 32 C56 32, 48 34, 44 38 Z"
            fill="url(#furBelly)" opacity="0.5"/>

          <!-- 额头 M 型虎斑 -->
          <path d="M42 24 C46 18, 50 22, 54 18 C58 14, 62 18, 64 16 C66 18, 70 14, 74 18 C78 22, 82 18, 86 24"
            fill="none" stroke="#B87A18" stroke-width="2" stroke-linecap="round" opacity="0.45"/>
          <!-- 面部虎斑 -->
          <path d="M38 30 C40 34, 38 38, 36 40" fill="none" stroke="#B87A18" stroke-width="1.8" stroke-linecap="round" opacity="0.3"/>
          <path d="M90 30 C88 34, 90 38, 92 40" fill="none" stroke="#B87A18" stroke-width="1.8" stroke-linecap="round" opacity="0.3"/>

          <!-- ======================== -->
          <!--  眼睛                     -->
          <!-- ======================== -->

          <!-- 开心笑眼 -->
          <g v-if="anim.eyes === 'happy'">
            <path d="M44 38 Q50 32 56 38" fill="none" stroke="#92400E" stroke-width="2" stroke-linecap="round">
              <animate attributeName="d"
                values="M44 38 Q50 32 56 38; M44 38 Q50 30 56 38; M44 38 Q50 32 56 38"
                dur="1.2s" repeatCount="indefinite"/>
            </path>
            <path d="M72 38 Q78 32 84 38" fill="none" stroke="#92400E" stroke-width="2" stroke-linecap="round">
              <animate attributeName="d"
                values="M72 38 Q78 32 84 38; M72 38 Q78 30 84 38; M72 38 Q78 32 84 38"
                dur="1.2s" repeatCount="indefinite"/>
            </path>
          </g>

          <!-- 闭眼睡觉 -->
          <g v-if="anim.eyes === 'closed'">
            <path d="M44 38 Q50 44 56 38" fill="none" stroke="#92400E" stroke-width="1.8" stroke-linecap="round">
              <animate attributeName="d"
                values="M44 38 Q50 44 56 38; M44 38 Q50 46 56 38; M44 38 Q50 44 56 38"
                dur="3s" repeatCount="indefinite"/>
            </path>
            <path d="M72 38 Q78 44 84 38" fill="none" stroke="#92400E" stroke-width="1.8" stroke-linecap="round">
              <animate attributeName="d"
                values="M72 38 Q78 44 84 38; M72 38 Q78 46 84 38; M72 38 Q78 44 84 38"
                dur="3s" repeatCount="indefinite"/>
            </path>
            <g fill="#C47A17" font-family="Georgia, serif" font-style="italic">
              <text x="92" y="18" font-size="9" opacity="0">
                Z
                <animate attributeName="opacity" values="0;0.6;0" dur="3s" repeatCount="indefinite"/>
                <animate attributeName="y" values="18;10" dur="3s" repeatCount="indefinite"/>
              </text>
              <text x="98" y="14" font-size="7" opacity="0">
                z
                <animate attributeName="opacity" values="0;0.4;0" dur="3s" begin="0.8s" repeatCount="indefinite"/>
                <animate attributeName="y" values="14;6" dur="3s" begin="0.8s" repeatCount="indefinite"/>
              </text>
            </g>
          </g>

          <!-- 睁大眼睛 (受惊/专注) -->
          <g v-if="anim.eyes === 'wide'">
            <!-- 眼白 -->
            <ellipse cx="50" cy="38" rx="7" ry="8" fill="white" stroke="#92400E" stroke-width="1"/>
            <ellipse cx="78" cy="38" rx="7" ry="8" fill="white" stroke="#92400E" stroke-width="1"/>
            <!-- 虹膜 -->
            <circle cx="50" cy="38" r="5" fill="url(#irisGold)"/>
            <circle cx="78" cy="38" r="5" fill="url(#irisGold)"/>
            <!-- 竖瞳 -->
            <ellipse cx="50" cy="38" rx="1.5" ry="4" fill="#1a1a1a"/>
            <ellipse cx="78" cy="38" rx="1.5" ry="4" fill="#1a1a1a"/>
            <!-- 高光 -->
            <circle cx="48" cy="36" r="1.8" fill="white" opacity="0.85"/>
            <circle cx="76" cy="36" r="1.8" fill="white" opacity="0.85"/>
            <circle cx="52" cy="40" r="0.8" fill="white" opacity="0.5"/>
            <circle cx="80" cy="40" r="0.8" fill="white" opacity="0.5"/>
            <!-- 眼睛微颤 -->
            <animateTransform
              attributeName="transform" type="translate"
              values="0,0; 0.6,0; 0,0; -0.6,0; 0,0"
              dur="0.2s" repeatCount="indefinite"
            />
          </g>

          <!-- 正常睁眼 — 杏仁形、竖瞳 -->
          <g v-if="anim.eyes === 'open'">
            <!-- 眼白 — 杏仁形 -->
            <path d="M43 38 Q50 30 57 38 Q50 46 43 38 Z" fill="white" stroke="#92400E" stroke-width="1">
              <animate attributeName="d"
                values="M43 38 Q50 30 57 38 Q50 46 43 38 Z; M43 38 Q50 37 57 38 Q50 39 43 38 Z; M43 38 Q50 30 57 38 Q50 46 43 38 Z"
                dur="4s" repeatCount="indefinite"/>
            </path>
            <path d="M71 38 Q78 30 85 38 Q78 46 71 38 Z" fill="white" stroke="#92400E" stroke-width="1">
              <animate attributeName="d"
                values="M71 38 Q78 30 85 38 Q78 46 71 38 Z; M71 38 Q78 37 85 38 Q78 39 71 38 Z; M71 38 Q78 30 85 38 Q78 46 71 38 Z"
                dur="4s" begin="0.06s" repeatCount="indefinite"/>
            </path>
            <!-- 虹膜 -->
            <circle cx="50" cy="38" r="4.5" fill="url(#irisGreen)">
              <animate attributeName="r" values="4.5;0.5;4.5" dur="4s" repeatCount="indefinite"/>
            </circle>
            <circle cx="78" cy="38" r="4.5" fill="url(#irisGreen)">
              <animate attributeName="r" values="4.5;0.5;4.5" dur="4s" begin="0.06s" repeatCount="indefinite"/>
            </circle>
            <!-- 竖瞳 -->
            <ellipse cx="50" cy="38" rx="1.2" ry="3.5" fill="#1a1a1a">
              <animate attributeName="ry" values="3.5;0.3;3.5" dur="4s" repeatCount="indefinite"/>
            </ellipse>
            <ellipse cx="78" cy="38" rx="1.2" ry="3.5" fill="#1a1a1a">
              <animate attributeName="ry" values="3.5;0.3;3.5" dur="4s" begin="0.06s" repeatCount="indefinite"/>
            </ellipse>
            <!-- 高光 -->
            <circle cx="48" cy="36" r="1.5" fill="white" opacity="0.8">
              <animate attributeName="opacity" values="0.8;0;0.8" dur="4s" repeatCount="indefinite"/>
            </circle>
            <circle cx="76" cy="36" r="1.5" fill="white" opacity="0.8">
              <animate attributeName="opacity" values="0.8;0;0.8" dur="4s" begin="0.06s" repeatCount="indefinite"/>
            </circle>
          </g>

          <!-- ======================== -->
          <!--  鼻子                     -->
          <!-- ======================== -->
          <path d="M62 46 L66 46 L64 49 Z" fill="url(#nosePink)" stroke="#92400E" stroke-width="0.8" stroke-linejoin="round"/>
          <!-- 鼻梁 -->
          <path d="M64 44 L64 46" stroke="#92400E" stroke-width="0.6" opacity="0.3"/>

          <!-- ======================== -->
          <!--  嘴巴                     -->
          <!-- ======================== -->
          <path d="M60 51 Q62 54 64 51 Q66 54 68 51" fill="none" stroke="#92400E" stroke-width="1.2" stroke-linecap="round"/>
          <!-- 嘴角连接到鼻子的线 -->
          <path d="M64 49 L64 51" stroke="#92400E" stroke-width="0.8" opacity="0.4"/>

          <!-- ======================== -->
          <!--  胡须                     -->
          <!-- ======================== -->
          <g stroke="#92400E" stroke-linecap="round" opacity="0.45">
            <!-- 左 -->
            <line x1="24" y1="44" x2="40" y2="48" stroke-width="0.9">
              <animate attributeName="x2" values="40;41;40;39;40" dur="0.7s" repeatCount="indefinite"/>
            </line>
            <line x1="22" y1="49" x2="40" y2="49" stroke-width="0.9">
              <animate attributeName="x2" values="40;41;40;39;40" dur="0.8s" repeatCount="indefinite"/>
            </line>
            <line x1="24" y1="54" x2="40" y2="50" stroke-width="0.8">
              <animate attributeName="x2" values="40;41;40;39;40" dur="0.75s" repeatCount="indefinite"/>
            </line>
            <!-- 右 -->
            <line x1="104" y1="44" x2="88" y2="48" stroke-width="0.9">
              <animate attributeName="x2" values="88;87;88;89;88" dur="0.7s" repeatCount="indefinite"/>
            </line>
            <line x1="106" y1="49" x2="88" y2="49" stroke-width="0.9">
              <animate attributeName="x2" values="88;87;88;89;88" dur="0.8s" repeatCount="indefinite"/>
            </line>
            <line x1="104" y1="54" x2="88" y2="50" stroke-width="0.8">
              <animate attributeName="x2" values="88;87;88;89;88" dur="0.75s" repeatCount="indefinite"/>
            </line>
          </g>
        </g>
      </g>
    </svg>
  </div>
</template>

<style scoped>
.cat-container {
  display: flex;
  align-items: center;
  justify-content: center;
}

.cat-svg {
  display: block;
}
</style>
