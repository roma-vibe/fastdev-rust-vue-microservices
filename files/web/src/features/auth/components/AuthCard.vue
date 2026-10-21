<script setup lang="ts">
/** Centered glass card of the sign-in and sign-up pages. */
import { GlassCard } from '@/shared/ui/index.ts';

defineProps<{ title: string; text?: string }>();
</script>

<template>
  <GlassCard class="mx-auto mt-10 w-full max-w-md">
    <h1 class="text-2xl font-semibold tracking-tight">{{ title }}</h1>
    <p v-if="text" class="mt-1 text-sm text-stone-600 dark:text-stone-400">{{ text }}</p>
    <div class="mt-6"><slot /></div>
  </GlassCard>
</template>
