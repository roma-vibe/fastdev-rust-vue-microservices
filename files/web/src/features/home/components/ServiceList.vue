<script setup lang="ts">
/** The backend services with their health, latency and version. */
import { useI18n } from 'vue-i18n';
import { formatMs } from '@/shared/lib/format.ts';
import { StatusDot } from '@/shared/ui/index.ts';
import type { ServiceHealth } from '../api.ts';

defineProps<{ services: ServiceHealth[] }>();
const { t } = useI18n();
</script>

<template>
  <ul class="divide-y divide-stone-900/5 dark:divide-white/10">
    <li v-for="service in services" :key="service.name" class="flex items-center gap-3 py-2.5">
      <StatusDot :status="service.ok ? 'up' : 'down'" />
      <span class="font-mono text-sm">{{ service.name }}</span>
      <span class="ml-auto text-xs text-stone-500">
        {{ service.ok ? formatMs(service.latencyMs) : t('home.down') }}
        <template v-if="service.version"> · v{{ service.version }}</template>
      </span>
    </li>
  </ul>
</template>
