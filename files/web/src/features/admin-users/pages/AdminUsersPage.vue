<script setup lang="ts">
/** Example administration page: everyone who has an account (read-only). */
import { RefreshCw, Users } from '@lucide/vue';
import { onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { formatDateTime } from '@/shared/lib/format.ts';
import {
  EmptyState,
  GlassCard,
  PageHeader,
  UiBadge,
  UiButton,
  UiLoader,
} from '@/shared/ui/index.ts';
import { useAdminUsersStore } from '../store.ts';

const { t } = useI18n();
const store = useAdminUsersStore();

onMounted(() => store.load());
</script>

<template>
  <div>
    <PageHeader :title="t('adminUsers.title')" :description="t('adminUsers.description')">
      <UiButton variant="secondary" :loading="store.loading" @click="store.load()">
        <RefreshCw v-if="!store.loading" class="size-4" />{{ t('common.retry') }}
      </UiButton>
    </PageHeader>
    <p v-if="store.error" class="mb-4 text-sm text-red-600">{{ store.error }}</p>
    <UiLoader v-if="!store.loaded" />
    <GlassCard v-else-if="!store.items.length" :padded="false">
      <EmptyState :icon="Users" :title="t('common.empty')" />
    </GlassCard>
    <GlassCard v-else :padded="false">
      <div class="overflow-x-auto">
        <table class="w-full text-sm">
          <thead>
            <tr
              class="border-b border-stone-900/5 text-left text-xs tracking-wide text-stone-500 uppercase dark:border-white/10 dark:text-stone-400"
            >
              <th class="px-5 py-3 font-medium">{{ t('adminUsers.user') }}</th>
              <th class="px-3 py-3 font-medium">{{ t('adminUsers.role') }}</th>
              <th class="px-3 py-3 text-right font-medium">{{ t('adminUsers.keys') }}</th>
              <th class="px-5 py-3 font-medium">{{ t('adminUsers.registered') }}</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-stone-900/5 dark:divide-white/10">
            <tr v-for="user in store.items" :key="user.id" :class="user.disabled && 'opacity-60'">
              <td class="px-5 py-3">
                <p class="font-medium">{{ user.name || user.email }}</p>
                <p v-if="user.name" class="text-xs text-stone-500">{{ user.email }}</p>
              </td>
              <td class="px-3 py-3">
                <UiBadge :tone="user.role === 'admin' ? 'accent' : 'neutral'">
                  {{ t(`adminUsers.roles.${user.role}`) }}
                </UiBadge>
                <UiBadge v-if="user.disabled" tone="danger" class="ml-1">
                  {{ t('adminUsers.disabled') }}
                </UiBadge>
              </td>
              <td class="px-3 py-3 text-right tabular-nums">{{ user.keyCount }}</td>
              <td class="px-5 py-3 whitespace-nowrap text-stone-600 dark:text-stone-400">
                {{ formatDateTime(user.createdAt) }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </GlassCard>
  </div>
</template>
