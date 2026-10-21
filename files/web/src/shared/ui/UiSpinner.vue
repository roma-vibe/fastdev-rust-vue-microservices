<script setup lang="ts">
/** Spinning ring. Always sized by `size` (an unsized SVG would stretch to the container width). */
const props = withDefaults(defineProps<{ size?: 'sm' | 'md' | 'lg' }>(), { size: 'md' });
const sizes = { sm: 'size-4', md: 'size-5', lg: 'size-7' };
</script>

<template>
  <svg
    class="shrink-0 animate-spin"
    :class="sizes[props.size]"
    viewBox="0 0 24 24"
    fill="none"
    aria-hidden="true"
  >
    <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-opacity="0.25" stroke-width="3" />
    <path
      d="M22 12a10 10 0 0 0-10-10"
      stroke="currentColor"
      stroke-width="3"
      stroke-linecap="round"
    />
  </svg>
</template>
