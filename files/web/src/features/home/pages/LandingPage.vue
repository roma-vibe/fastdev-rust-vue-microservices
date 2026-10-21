<script setup lang="ts">
/** Public start page: what this skeleton is and what it contains. */
import { ArrowRight, Check } from '@lucide/vue';
import { useI18n } from 'vue-i18n';
import { appName } from '@/config.ts';
import { GlassCard, UiButton } from '@/shared/ui/index.ts';

const { t, tm } = useI18n();
const items = (tm('home.landing.inside.items') as string[]).map((_, index) => index);
</script>

<template>
  <div class="mx-auto max-w-3xl space-y-8 py-6 sm:py-12">
    <header class="space-y-4 text-center">
      <h1 class="text-accent-gradient text-4xl font-semibold tracking-tight sm:text-5xl">
        {{ appName }}
      </h1>
      <p class="text-lg text-stone-600 dark:text-stone-300">{{ t('home.landing.tagline') }}</p>
      <UiButton to="/app" size="lg">
        {{ t('home.landing.open') }}<ArrowRight class="size-4" />
      </UiButton>
    </header>

    <GlassCard>
      <h2 class="mb-2 text-lg font-semibold">{{ t('home.landing.what.title') }}</h2>
      <p class="text-stone-600 dark:text-stone-300">{{ t('home.landing.what.text') }}</p>
    </GlassCard>

    <GlassCard>
      <h2 class="mb-3 text-lg font-semibold">{{ t('home.landing.inside.title') }}</h2>
      <ul class="space-y-2 text-stone-600 dark:text-stone-300">
        <li v-for="index in items" :key="index" class="flex gap-3">
          <Check class="mt-0.5 size-4 shrink-0 text-accent-600" />
          <span>{{ t(`home.landing.inside.items.${index}`) }}</span>
        </li>
      </ul>
    </GlassCard>
  </div>
</template>
