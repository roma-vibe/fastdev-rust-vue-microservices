<script setup lang="ts">
import { Monitor, Moon, Sun } from '@lucide/vue';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { cycleTheme, themeMode } from '@/shared/theme.ts';

const { t } = useI18n();
const icon = computed(() => ({ light: Sun, dark: Moon, system: Monitor })[themeMode.value]);
</script>

<template>
  <button
    type="button"
    class="grid size-9 place-items-center rounded-xl text-stone-600 transition hover:bg-stone-900/5 hover:text-accent-600 dark:text-stone-300 dark:hover:bg-white/10"
    :title="t(`theme.${themeMode}`)"
    :aria-label="t(`theme.${themeMode}`)"
    @click="cycleTheme"
  >
    <component :is="icon" class="size-[18px]" />
  </button>
</template>
