<script setup lang="ts">
/** Page title row: title, optional description, actions slot on the right. */
defineProps<{ title: string; description?: string }>();
</script>

<template>
  <header class="mb-6 flex flex-wrap items-end justify-between gap-4">
    <div>
      <h1 class="text-2xl font-semibold tracking-tight sm:text-3xl">{{ title }}</h1>
      <p v-if="description" class="mt-1 max-w-2xl text-sm text-stone-600 dark:text-stone-400">
        {{ description }}
      </p>
    </div>
    <div v-if="$slots.default" class="flex flex-wrap items-center gap-2"><slot /></div>
  </header>
</template>
