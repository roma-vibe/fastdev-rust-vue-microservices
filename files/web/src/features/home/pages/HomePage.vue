<script setup lang="ts">
/** Start page: a welcome and the health of every backend service. */
import { NotebookPen, RefreshCw } from '@lucide/vue';
import { onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { appName } from '@/config.ts';
import { GlassCard, PageHeader, UiBadge, UiButton, UiLoader } from '@/shared/ui/index.ts';
import ServiceList from '../components/ServiceList.vue';
import { useHomeStore } from '../store.ts';

const { t } = useI18n();
const store = useHomeStore();

// The last result stays on screen while it refreshes.
onMounted(() => store.load());
</script>

<template>
  <div class="space-y-6">
    <PageHeader :title="t('home.welcome', { name: appName })" :description="t('home.intro')">
      <UiButton to="/app/notes"><NotebookPen class="size-4" />{{ t('home.openNotes') }}</UiButton>
    </PageHeader>
    <GlassCard class="max-w-xl">
      <div class="mb-2 flex items-center gap-2">
        <h2 class="font-semibold">{{ t('home.services') }}</h2>
        <UiBadge v-if="store.health" :tone="store.health.status === 'ok' ? 'success' : 'warning'">
          {{ t(`home.status.${store.health.status}`) }}
        </UiBadge>
        <UiButton
          variant="ghost"
          size="sm"
          class="ml-auto"
          :loading="store.loading"
          @click="store.load()"
        >
          <RefreshCw class="size-4" />{{ t('common.retry') }}
        </UiButton>
      </div>
      <p v-if="store.error" class="text-sm text-red-600">{{ store.error }}</p>
      <ServiceList v-if="store.health" :services="store.health.services" />
      <UiLoader v-else-if="!store.error" />
    </GlassCard>
  </div>
</template>
