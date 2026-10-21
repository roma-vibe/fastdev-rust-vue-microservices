<script setup lang="ts">
/** Change the password; other sessions of the user are signed out by the server. */
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { errorMessage } from '@/shared/api/http.ts';
import { toast } from '@/shared/toast.ts';
import { GlassCard, UiButton, UiInput } from '@/shared/ui/index.ts';
import { profileApi } from '../api.ts';

const { t } = useI18n();
const current = ref('');
const next = ref('');
const busy = ref(false);

async function save() {
  busy.value = true;
  try {
    await profileApi.changePassword(current.value, next.value);
    current.value = '';
    next.value = '';
    toast.success(t('profile.passwordChanged'));
  } catch (e) {
    toast.error(errorMessage(e));
  } finally {
    busy.value = false;
  }
}
</script>

<template>
  <GlassCard>
    <h2 class="text-lg font-semibold">{{ t('profile.password') }}</h2>
    <form class="mt-4 space-y-4" @submit.prevent="save">
      <UiInput
        v-model="current"
        type="password"
        :label="t('profile.currentPassword')"
        autocomplete="current-password"
        required
      />
      <UiInput
        v-model="next"
        type="password"
        :label="t('profile.newPassword')"
        :hint="t('profile.passwordHint')"
        autocomplete="new-password"
        required
      />
      <UiButton type="submit" :loading="busy">{{ t('profile.changePassword') }}</UiButton>
    </form>
  </GlassCard>
</template>
