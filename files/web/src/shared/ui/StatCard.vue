<script setup lang="ts">
/** KPI tile: label, big value, optional hint. */
import type { Component } from 'vue';
import GlassCard from './GlassCard.vue';

defineProps<{ label: string; value: string | number; hint?: string; icon?: Component }>();
</script>

<template>
  <GlassCard>
    <div class="flex items-start justify-between gap-3">
      <p class="text-sm text-stone-600 dark:text-stone-400">{{ label }}</p>
      <component :is="icon" v-if="icon" class="size-5 text-accent-500" />
    </div>
    <p class="mt-2 text-3xl font-semibold tracking-tight tabular-nums">{{ value }}</p>
    <p v-if="hint" class="mt-1 text-xs text-stone-500 dark:text-stone-400">{{ hint }}</p>
  </GlassCard>
</template>
