<script setup lang="ts">
import { useId } from 'vue';

defineProps<{
  label?: string;
  placeholder?: string;
  rows?: number;
  hint?: string;
  mono?: boolean;
  required?: boolean;
}>();
const model = defineModel<string>();
const id = useId();
</script>

<template>
  <div class="flex flex-col gap-1.5">
    <label v-if="label" :for="id" class="text-sm font-medium text-stone-700 dark:text-stone-300">
      {{ label }}<span v-if="required" class="text-accent-600"> *</span>
    </label>
    <textarea
      :id="id"
      v-model="model"
      :rows="rows ?? 4"
      :placeholder="placeholder"
      :required="required"
      class="glass-inset w-full rounded-xl px-3 py-2 text-sm text-stone-900 placeholder:text-stone-400 focus:border-accent-500 focus:ring-2 focus:ring-accent-500/30 focus:outline-none dark:text-stone-100"
      :class="mono && 'font-mono'"
    />
    <p v-if="hint" class="text-xs text-stone-500 dark:text-stone-400">{{ hint }}</p>
  </div>
</template>
