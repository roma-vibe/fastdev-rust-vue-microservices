<script setup lang="ts">
/** Text input with label, hint and error. `v-model` binds the value. */
import { useId } from 'vue';

defineProps<{
  label?: string;
  type?: string;
  placeholder?: string;
  hint?: string;
  error?: string;
  required?: boolean;
  disabled?: boolean;
  autocomplete?: string;
  mono?: boolean;
}>();
const model = defineModel<string | number | null>();
const id = useId();
</script>

<template>
  <div class="flex flex-col gap-1.5">
    <label v-if="label" :for="id" class="text-sm font-medium text-stone-700 dark:text-stone-300">
      {{ label }}<span v-if="required" class="text-accent-600"> *</span>
    </label>
    <div class="relative flex items-center">
      <slot name="prefix" />
      <input
        :id="id"
        v-model="model"
        :type="type ?? 'text'"
        :placeholder="placeholder"
        :required="required"
        :disabled="disabled"
        :autocomplete="autocomplete"
        class="glass-inset h-10 w-full rounded-xl px-3 text-sm text-stone-900 placeholder:text-stone-400 focus:border-accent-500 focus:ring-2 focus:ring-accent-500/30 focus:outline-none disabled:opacity-60 dark:text-stone-100 dark:placeholder:text-stone-500"
        :class="[mono && 'font-mono', error && '!border-red-500']"
      />
      <slot name="suffix" />
    </div>
    <p v-if="error" class="text-xs text-red-600 dark:text-red-400">{{ error }}</p>
    <p v-else-if="hint" class="text-xs text-stone-500 dark:text-stone-400">{{ hint }}</p>
  </div>
</template>
