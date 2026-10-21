<script setup lang="ts">
/** Public top bar: "Sign in", or "Open the app" for a signed-in user. */
import { useI18n } from 'vue-i18n';
import { useAuthStore } from '@/shared/auth.ts';
import { UiButton } from '@/shared/ui/index.ts';

const { t } = useI18n();
const auth = useAuthStore();
</script>

<template>
  <UiButton size="sm" :to="auth.signedIn ? '/app' : '/login'">
    {{ auth.signedIn ? t('auth.openApp') : t('auth.signIn') }}
  </UiButton>
</template>
