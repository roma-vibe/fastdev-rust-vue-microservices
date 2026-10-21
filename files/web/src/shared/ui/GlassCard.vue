<script setup lang="ts">
/** Liquid-glass surface. `padded` (default) adds inner spacing; `interactive` adds hover lift. */
withDefaults(defineProps<{ padded?: boolean; interactive?: boolean; as?: string }>(), {
  padded: true,
  interactive: false,
  as: 'section',
});
</script>

<template>
  <component
    :is="as"
    class="glass relative"
    :class="[
      padded && 'p-5 sm:p-6',
      interactive &&
        'transition duration-200 hover:-translate-y-0.5 hover:shadow-xl hover:shadow-accent-500/10',
    ]"
  >
    <slot />
  </component>
</template>
