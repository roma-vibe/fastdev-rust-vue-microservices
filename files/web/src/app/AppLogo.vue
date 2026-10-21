<script setup lang="ts">
/** The app mark and name (APP_NAME from .env); links to `to` (the landing page by default). */
import { Sparkles } from '@lucide/vue';
import { RouterLink } from 'vue-router';
import { appName } from '@/config.ts';

const props = withDefaults(defineProps<{ to?: string }>(), { to: '/' });
</script>

<template>
  <RouterLink :to="props.to" class="flex min-w-0 items-center gap-2 font-semibold tracking-tight">
    <span
      class="grid size-8 shrink-0 place-items-center rounded-xl bg-gradient-to-br from-accent-400 to-accent-600 text-white shadow-lg shadow-accent-600/30"
    >
      <Sparkles class="size-4" />
    </span>
    <span class="truncate">{{ appName }}</span>
  </RouterLink>
</template>
