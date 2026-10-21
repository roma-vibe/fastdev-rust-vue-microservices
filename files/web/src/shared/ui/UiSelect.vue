<script setup lang="ts">
/** Native select. options: [{value, label}]. */
import { useId } from 'vue';

defineProps<{
  label?: string;
  options: { value: string | number; label: string }[];
  placeholder?: string;
  required?: boolean;
}>();
const model = defineModel<string | number | null>();
const id = useId();
</script>

<template>
  <div class="flex flex-col gap-1.5">
    <label v-if="label" :for="id" class="text-sm font-medium text-stone-700 dark:text-stone-300">{{
      label
    }}</label>
    <select
      :id="id"
      v-model="model"
      :required="required"
      class="glass-inset h-10 w-full rounded-xl px-3 text-sm text-stone-900 focus:border-accent-500 focus:ring-2 focus:ring-accent-500/30 focus:outline-none dark:text-stone-100 [&>option]:bg-white dark:[&>option]:bg-stone-900"
    >
      <option v-if="placeholder" value="" disabled>{{ placeholder }}</option>
      <option v-for="option in options" :key="option.value" :value="option.value">
        {{ option.label }}
      </option>
    </select>
  </div>
</template>
