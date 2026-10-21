<script setup lang="ts">
/** The user's name (email and role are read-only). */
import { ref, watchEffect } from 'vue';
import { useI18n } from 'vue-i18n';
import { errorMessage } from '@/shared/api/http.ts';
import { useAuthStore } from '@/shared/auth.ts';
import { toast } from '@/shared/toast.ts';
import { GlassCard, UiBadge, UiButton, UiInput } from '@/shared/ui/index.ts';

const { t } = useI18n();
const auth = useAuthStore();
const name = ref('');
const busy = ref(false);

watchEffect(() => (name.value = auth.user?.name ?? ''));

async function save() {
  busy.value = true;
  try {
    await auth.update({ name: name.value });
    toast.success(t('common.saved'));
  } catch (e) {
    toast.error(errorMessage(e));
  } finally {
    busy.value = false;
  }
}
</script>

<template>
  <GlassCard>
    <div class="flex items-center justify-between gap-3">
      <h2 class="text-lg font-semibold">{{ t('profile.account') }}</h2>
      <UiBadge :tone="auth.isAdmin ? 'accent' : 'neutral'">
        {{ t(auth.isAdmin ? 'profile.roleAdmin' : 'profile.roleUser') }}
      </UiBadge>
    </div>
    <form class="mt-4 space-y-4" @submit.prevent="save">
      <UiInput :model-value="auth.user?.email ?? ''" :label="t('profile.email')" disabled />
      <UiInput v-model="name" :label="t('profile.name')" autocomplete="name" />
      <UiButton type="submit" :loading="busy">{{ t('common.save') }}</UiButton>
    </form>
  </GlassCard>
</template>
