<script setup lang="ts">
/** Sign in with email and password; goes to `?next=` (or home) afterwards. */
import { onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { RouterLink, useRoute, useRouter } from 'vue-router';
import { errorMessage } from '@/shared/api/http.ts';
import { useAuthStore } from '@/shared/auth.ts';
import { UiButton, UiInput } from '@/shared/ui/index.ts';
import AuthCard from '../components/AuthCard.vue';
import { nextPath } from '../guard.ts';

const { t } = useI18n();
const auth = useAuthStore();
const route = useRoute();
const router = useRouter();
const email = ref('');
const password = ref('');
const error = ref('');
const busy = ref(false);
const canRegister = ref(false);

onMounted(async () => {
  const config = await auth.config().catch(() => null);
  canRegister.value = config?.registrationOpen ?? false;
  // A fresh installation has no users yet: the first account is created on the sign-up page.
  if (config && !config.hasUsers) await router.replace({ name: 'register', query: route.query });
});

async function submit() {
  busy.value = true;
  error.value = '';
  try {
    await auth.login(email.value, password.value);
    await router.push(nextPath(route.query.next));
  } catch (e) {
    error.value = errorMessage(e);
  } finally {
    busy.value = false;
  }
}
</script>

<template>
  <AuthCard :title="t('auth.title.login')" :text="t('auth.loginText')">
    <form class="space-y-4" @submit.prevent="submit">
      <UiInput
        v-model="email"
        type="email"
        :label="t('auth.email')"
        autocomplete="email"
        required
      />
      <UiInput
        v-model="password"
        type="password"
        :label="t('auth.password')"
        autocomplete="current-password"
        required
      />
      <p v-if="error" class="text-sm text-red-600">{{ error }}</p>
      <UiButton type="submit" block :loading="busy">{{ t('auth.signIn') }}</UiButton>
    </form>
    <p v-if="canRegister" class="mt-6 text-center text-sm text-stone-600 dark:text-stone-400">
      {{ t('auth.noAccount') }}
      <RouterLink
        :to="{ name: 'register', query: route.query }"
        class="font-medium text-accent-600"
      >
        {{ t('auth.title.register') }}
      </RouterLink>
    </p>
  </AuthCard>
</template>
