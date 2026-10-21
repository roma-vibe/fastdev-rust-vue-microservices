<script setup lang="ts">
/** Personal API keys: list, create (secret shown once), revoke. */
import { KeyRound, Plus } from '@lucide/vue';
import { onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { errorMessage } from '@/shared/api/http.ts';
import { toast } from '@/shared/toast.ts';
import {
  ConfirmDialog,
  EmptyState,
  GlassCard,
  PageHeader,
  UiButton,
  UiLoader,
} from '@/shared/ui/index.ts';
import type { ApiKey } from '../api.ts';
import KeyRow from '../components/KeyRow.vue';
import NewKeyDialog from '../components/NewKeyDialog.vue';
import { useKeysStore } from '../store.ts';

const { t } = useI18n();
const store = useKeysStore();
const createOpen = ref(false);
const revoking = ref<ApiKey | null>(null);
const confirmOpen = ref(false);

onMounted(() => store.load());

function askRevoke(key: ApiKey) {
  revoking.value = key;
  confirmOpen.value = true;
}

async function confirmRevoke() {
  const key = revoking.value;
  confirmOpen.value = false;
  if (!key) return;
  try {
    await store.revoke(key.id);
    toast.success(t('keys.revokedToast'));
  } catch (e) {
    toast.error(errorMessage(e));
  }
}
</script>

<template>
  <div>
    <PageHeader :title="t('keys.title')" :description="t('keys.description')">
      <UiButton @click="createOpen = true"><Plus class="size-4" />{{ t('keys.new') }}</UiButton>
    </PageHeader>
    <p v-if="store.error" class="mb-4 text-sm text-red-600">{{ store.error }}</p>
    <UiLoader v-if="!store.loaded" />
    <GlassCard v-else-if="!store.items.length">
      <EmptyState :icon="KeyRound" :title="t('keys.emptyTitle')" :text="t('keys.emptyText')" />
    </GlassCard>
    <GlassCard v-else>
      <ul class="divide-y divide-stone-900/5 dark:divide-white/10">
        <KeyRow v-for="key in store.items" :key="key.id" :api-key="key" @revoke="askRevoke(key)" />
      </ul>
    </GlassCard>
    <NewKeyDialog v-model:open="createOpen" />
    <ConfirmDialog
      v-model:open="confirmOpen"
      :text="t('keys.revokeText', { name: revoking?.name ?? '' })"
      :confirm-label="t('keys.revoke')"
      danger
      @confirm="confirmRevoke"
    />
  </div>
</template>
