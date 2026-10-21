<script setup lang="ts">
/** Creates a key, then shows its secret once with a copy button. */
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { errorMessage } from '@/shared/api/http.ts';
import { CopyButton, UiButton, UiInput, UiModal } from '@/shared/ui/index.ts';
import { useKeysStore } from '../store.ts';

const open = defineModel<boolean>('open', { default: false });
const { t } = useI18n();
const store = useKeysStore();
const name = ref('');
const secret = ref('');
const error = ref('');
const busy = ref(false);

watch(open, (value) => {
  if (!value) return;
  name.value = '';
  secret.value = '';
  error.value = '';
});

async function create() {
  busy.value = true;
  error.value = '';
  try {
    secret.value = (await store.create(name.value)).secret;
  } catch (e) {
    error.value = errorMessage(e);
  } finally {
    busy.value = false;
  }
}
</script>

<template>
  <UiModal v-model:open="open" :title="t('keys.new')">
    <div v-if="secret" class="space-y-3">
      <p class="text-sm text-stone-600 dark:text-stone-400">{{ t('keys.secretOnce') }}</p>
      <div class="glass-inset flex items-center gap-2 rounded-xl p-2">
        <code class="min-w-0 flex-1 truncate font-mono text-sm">{{ secret }}</code>
        <CopyButton :value="secret" />
      </div>
      <UiButton block @click="open = false">{{ t('common.close') }}</UiButton>
    </div>
    <form v-else class="space-y-4" @submit.prevent="create">
      <UiInput
        v-model="name"
        :label="t('keys.name')"
        :placeholder="t('keys.namePlaceholder')"
        :error="error"
        required
      />
      <UiButton type="submit" block :loading="busy">{{ t('common.create') }}</UiButton>
    </form>
  </UiModal>
</template>
