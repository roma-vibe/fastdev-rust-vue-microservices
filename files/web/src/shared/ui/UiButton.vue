<script setup lang="ts">
/** Button. Variants: primary (orange), secondary (glass), ghost, danger. Renders a RouterLink when `to` is set. */
import { RouterLink, type RouteLocationRaw } from 'vue-router';
import UiSpinner from './UiSpinner.vue';

const props = withDefaults(
  defineProps<{
    variant?: 'primary' | 'secondary' | 'ghost' | 'danger';
    size?: 'sm' | 'md' | 'lg';
    type?: 'button' | 'submit';
    loading?: boolean;
    disabled?: boolean;
    to?: RouteLocationRaw;
    href?: string;
    block?: boolean;
  }>(),
  { variant: 'primary', size: 'md', type: 'button' },
);

const variants = {
  primary:
    'bg-gradient-to-b from-accent-400 to-accent-600 text-white shadow-lg shadow-accent-600/25 ring-1 ring-inset ring-white/25 hover:from-accent-500 hover:to-accent-700',
  secondary:
    'glass !rounded-xl text-stone-800 hover:bg-white/70 dark:text-stone-100 dark:hover:bg-white/10',
  ghost: 'text-stone-700 hover:bg-stone-900/5 dark:text-stone-300 dark:hover:bg-white/10',
  danger: 'bg-red-600/90 text-white shadow-lg shadow-red-600/20 hover:bg-red-600',
};
const sizes = {
  sm: 'h-8 px-3 text-sm gap-1.5',
  md: 'h-10 px-4 text-sm gap-2',
  lg: 'h-12 px-6 text-base gap-2',
};
</script>

<template>
  <component
    :is="props.to ? RouterLink : props.href ? 'a' : 'button'"
    :to="props.to"
    :href="props.href"
    :type="props.to || props.href ? undefined : props.type"
    :disabled="props.to || props.href ? undefined : props.disabled || props.loading"
    class="inline-flex items-center justify-center rounded-xl font-medium whitespace-nowrap transition select-none active:scale-[0.98] disabled:pointer-events-none disabled:opacity-50"
    :class="[variants[props.variant], sizes[props.size], props.block && 'w-full']"
  >
    <UiSpinner v-if="props.loading" size="sm" />
    <slot />
  </component>
</template>
