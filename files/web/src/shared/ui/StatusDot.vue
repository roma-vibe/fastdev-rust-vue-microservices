<script setup lang="ts">
/** Health dot: up | degraded | down | unknown (pulses when up). */
defineProps<{ status: 'up' | 'degraded' | 'down' | 'unknown' }>();
const colors = {
  up: 'bg-emerald-500',
  degraded: 'bg-amber-500',
  down: 'bg-red-500',
  unknown: 'bg-stone-400',
};
</script>

<template>
  <span class="relative inline-flex size-2.5">
    <span
      v-if="status === 'up'"
      class="absolute inline-flex size-full animate-ping rounded-full bg-emerald-400 opacity-60"
    />
    <span class="relative inline-flex size-2.5 rounded-full" :class="colors[status]" />
  </span>
</template>
