<script setup lang="ts">
/** Profile: the account details and the password. */
import { useI18n } from 'vue-i18n';
import { PageHeader } from '@/shared/ui/index.ts';
import PasswordForm from '../components/PasswordForm.vue';
import ProfileForm from '../components/ProfileForm.vue';

const { t } = useI18n();
</script>

<template>
  <div>
    <PageHeader :title="t('profile.title')" :description="t('profile.description')" />
    <div class="grid gap-6 lg:grid-cols-2">
      <ProfileForm />
      <PasswordForm />
    </div>
  </div>
</template>
