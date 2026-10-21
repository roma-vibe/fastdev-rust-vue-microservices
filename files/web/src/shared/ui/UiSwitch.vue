<script setup lang="ts">
defineProps<{ label?: string; disabled?: boolean }>();
const model = defineModel<boolean>({ default: false });
</script>

<template>
  <label
    class="inline-flex cursor-pointer items-center gap-3 select-none"
    :class="disabled && 'pointer-events-none opacity-50'"
  >
    <button
      type="button"
      role="switch"
      :aria-checked="model"
      class="relative h-6 w-11 shrink-0 rounded-full transition"
      :class="model ? 'bg-accent-500 shadow-inner shadow-accent-800/30' : 'glass-inset'"
      @click="model = !model"
    >
      <span
        class="absolute top-0.5 left-0.5 size-5 rounded-full bg-white shadow-md transition-transform"
        :class="model && 'translate-x-5'"
      />
    </button>
    <span v-if="label" class="text-sm text-stone-700 dark:text-stone-300">{{ label }}</span>
  </label>
</template>
