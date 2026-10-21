<script setup lang="ts">
/** RU / EN switch; the choice is remembered in this browser (shared/i18n.ts). */
import { LOCALES, locale, setLocale } from '@/shared/i18n.ts';
</script>

<template>
  <div class="glass-inset flex rounded-xl p-0.5 text-xs font-semibold">
    <button
      v-for="item in LOCALES"
      :key="item"
      type="button"
      class="rounded-[10px] px-2 py-1 uppercase transition"
      :class="
        locale === item
          ? 'bg-accent-500 text-white shadow'
          : 'text-stone-500 hover:text-stone-900 dark:hover:text-white'
      "
      @click="setLocale(item)"
    >
      {{ item }}
    </button>
  </div>
</template>
