<script setup lang="ts">
/** Yes/no confirmation. `v-model:open`, emits `confirm`. */
import { useI18n } from 'vue-i18n';
import UiButton from './UiButton.vue';
import UiModal from './UiModal.vue';

defineProps<{
  title?: string;
  text?: string;
  confirmLabel?: string;
  danger?: boolean;
  loading?: boolean;
}>();
const emit = defineEmits<{ confirm: [] }>();
const open = defineModel<boolean>('open', { default: false });
const { t } = useI18n();
</script>

<template>
  <UiModal v-model:open="open" :title="title ?? t('common.areYouSure')" size="sm">
    <p class="text-sm text-stone-600 dark:text-stone-400">{{ text }}</p>
    <template #footer>
      <UiButton variant="ghost" @click="open = false">{{ t('common.cancel') }}</UiButton>
      <UiButton
        :variant="danger ? 'danger' : 'primary'"
        :loading="loading"
        @click="emit('confirm')"
      >
        {{ confirmLabel ?? t('common.confirm') }}
      </UiButton>
    </template>
  </UiModal>
</template>
