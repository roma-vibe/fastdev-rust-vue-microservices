<script setup lang="ts">
/** Small pill. tone: neutral | accent | success | warning | danger | info. */
withDefaults(
  defineProps<{ tone?: 'neutral' | 'accent' | 'success' | 'warning' | 'danger' | 'info' }>(),
  {
    tone: 'neutral',
  },
);
const tones = {
  neutral: 'bg-stone-500/10 text-stone-700 ring-stone-500/20 dark:text-stone-300',
  accent: 'bg-accent-500/15 text-accent-700 ring-accent-500/30 dark:text-accent-300',
  success: 'bg-emerald-500/15 text-emerald-700 ring-emerald-500/30 dark:text-emerald-300',
  warning: 'bg-amber-500/15 text-amber-700 ring-amber-500/30 dark:text-amber-300',
  danger: 'bg-red-500/15 text-red-700 ring-red-500/30 dark:text-red-300',
  info: 'bg-sky-500/15 text-sky-700 ring-sky-500/30 dark:text-sky-300',
};
</script>

<template>
  <span
    class="inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-xs font-medium ring-1 ring-inset"
    :class="tones[tone]"
  >
    <slot />
  </span>
</template>
