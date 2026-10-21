<script setup lang="ts">
/** One API key: name, prefix, dates and the revoke button. */
import { useI18n } from 'vue-i18n';
import { formatDateTime, formatRelative } from '@/shared/lib/format.ts';
import { UiBadge, UiButton } from '@/shared/ui/index.ts';
import type { ApiKey } from '../api.ts';

defineProps<{ apiKey: ApiKey }>();
defineEmits<{ revoke: [] }>();
const { t } = useI18n();
</script>

<template>
  <li class="flex flex-wrap items-center gap-4 py-3">
    <div class="min-w-0 flex-1">
      <p class="flex items-center gap-2 font-medium">
        {{ apiKey.name }}
        <UiBadge v-if="apiKey.revokedAt" tone="danger">{{ t('keys.revoked') }}</UiBadge>
      </p>
      <p class="mt-0.5 text-xs text-stone-500">
        <code class="font-mono">{{ apiKey.prefix }}…</code>
        · {{ t('keys.created', { date: formatDateTime(apiKey.createdAt) }) }} ·
        {{ t('keys.lastUsed', { when: formatRelative(apiKey.lastUsedAt) }) }}
      </p>
    </div>
    <UiButton v-if="!apiKey.revokedAt" variant="ghost" size="sm" @click="$emit('revoke')">
      {{ t('keys.revoke') }}
    </UiButton>
  </li>
</template>
