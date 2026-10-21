<script setup lang="ts">
/**
 * Loading placeholder of a page or card: a small centred spinner that appears only after `delay`
 * ms, so fast loads (and returning to a page with cached data) never flash a spinner.
 */
import { onBeforeUnmount, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import UiSpinner from './UiSpinner.vue';

const props = withDefaults(defineProps<{ delay?: number }>(), { delay: 300 });
const { t } = useI18n();
const visible = ref(props.delay <= 0);
let timer: ReturnType<typeof setTimeout> | undefined;
onMounted(() => {
  if (!visible.value) timer = setTimeout(() => (visible.value = true), props.delay);
});
onBeforeUnmount(() => clearTimeout(timer));
</script>

<template>
  <div class="flex min-h-24 items-center justify-center py-8" role="status" aria-live="polite">
    <template v-if="visible">
      <UiSpinner size="lg" class="text-accent-500" />
      <span class="sr-only">{{ t('common.loading') }}</span>
    </template>
  </div>
</template>
