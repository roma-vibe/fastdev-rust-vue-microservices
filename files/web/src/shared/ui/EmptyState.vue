<script setup lang="ts">
import { Inbox } from '@lucide/vue';
import type { Component } from 'vue';

defineProps<{ title?: string; text?: string; icon?: Component }>();
</script>

<template>
  <div class="flex flex-col items-center justify-center gap-3 px-6 py-14 text-center">
    <div class="glass grid size-14 place-items-center !rounded-2xl text-accent-600">
      <component :is="icon ?? Inbox" class="size-6" />
    </div>
    <h3 v-if="title" class="font-semibold">{{ title }}</h3>
    <p v-if="text" class="max-w-sm text-sm text-stone-600 dark:text-stone-400">{{ text }}</p>
    <slot />
  </div>
</template>
