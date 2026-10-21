<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { GlassCard, UiButton } from '@/shared/ui/index.ts';

const { t } = useI18n();
</script>

<template>
  <GlassCard class="mx-auto mt-16 max-w-md text-center">
    <p class="text-accent-gradient text-6xl font-bold">404</p>
    <h1 class="mt-4 text-xl font-semibold">{{ t('errors.notFoundTitle') }}</h1>
    <p class="mt-2 text-sm text-stone-600 dark:text-stone-400">{{ t('errors.notFoundText') }}</p>
    <UiButton class="mt-6" to="/">{{ t('common.back') }}</UiButton>
  </GlassCard>
</template>
