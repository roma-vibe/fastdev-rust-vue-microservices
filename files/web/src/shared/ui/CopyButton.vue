<script setup lang="ts">
/** Copies `value` to the clipboard and shows a check for a moment. */
import { Check, Copy } from '@lucide/vue';
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

const props = defineProps<{ value: string; label?: boolean }>();
const { t } = useI18n();
const done = ref(false);

async function copy() {
  await navigator.clipboard.writeText(props.value);
  done.value = true;
  setTimeout(() => (done.value = false), 1500);
}
</script>

<template>
  <button
    type="button"
    class="inline-flex items-center gap-1.5 rounded-lg px-2 py-1 text-xs text-stone-600 transition hover:bg-stone-900/5 hover:text-accent-600 dark:text-stone-400 dark:hover:bg-white/10"
    :title="t('common.copy')"
    @click="copy"
  >
    <Check v-if="done" class="size-4 text-emerald-500" />
    <Copy v-else class="size-4" />
    <span v-if="label">{{ done ? t('common.copied') : t('common.copy') }}</span>
  </button>
</template>
