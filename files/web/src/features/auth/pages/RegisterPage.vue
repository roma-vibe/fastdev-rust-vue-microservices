<script setup lang="ts">
/** Sign up. The first account of a new installation becomes the administrator. */
import { onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { RouterLink, useRoute, useRouter } from 'vue-router';
import { errorMessage } from '@/shared/api/http.ts';
import { type AuthConfig, useAuthStore } from '@/shared/auth.ts';
import { UiButton, UiInput } from '@/shared/ui/index.ts';
import AuthCard from '../components/AuthCard.vue';
import { nextPath } from '../guard.ts';

const { t } = useI18n();
const auth = useAuthStore();
const route = useRoute();
const router = useRouter();
const name = ref('');
const email = ref('');
const password = ref('');
const error = ref('');
const busy = ref(false);
const config = ref<AuthConfig | null>(null);

onMounted(async () => {
  config.value = await auth.config().catch(() => null);
});

async function submit() {
  busy.value = true;
  error.value = '';
  try {
    await auth.register({ name: name.value, email: email.value, password: password.value });
    await router.push(nextPath(route.query.next));
  } catch (e) {
    error.value = errorMessage(e);
  } finally {
    busy.value = false;
  }
}
</script>

<template>
  <AuthCard
    :title="t('auth.title.register')"
    :text="config && !config.hasUsers ? t('auth.firstUserText') : t('auth.registerText')"
  >
    <p v-if="config && !config.registrationOpen" class="text-sm text-stone-600">
      {{ t('auth.registrationClosed') }}
    </p>
    <form v-else class="space-y-4" @submit.prevent="submit">
      <UiInput v-model="name" :label="t('auth.name')" autocomplete="name" />
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
        :hint="t('auth.passwordHint')"
        autocomplete="new-password"
        required
      />
      <p v-if="error" class="text-sm text-red-600">{{ error }}</p>
      <UiButton type="submit" block :loading="busy">{{ t('auth.createAccount') }}</UiButton>
    </form>
    <p v-if="config?.hasUsers" class="mt-6 text-center text-sm text-stone-600 dark:text-stone-400">
      {{ t('auth.haveAccount') }}
      <RouterLink :to="{ name: 'login', query: route.query }" class="font-medium text-accent-600">
        {{ t('auth.signIn') }}
      </RouterLink>
    </p>
  </AuthCard>
</template>
