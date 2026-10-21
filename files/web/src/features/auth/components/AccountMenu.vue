<script setup lang="ts">
/** Sidebar footer: the signed-in user, links to the profile and sign out. */
import { LogOut } from '@lucide/vue';
import { useI18n } from 'vue-i18n';
import { RouterLink, useRouter } from 'vue-router';
import { useAuthStore } from '@/shared/auth.ts';

const { t } = useI18n();
const auth = useAuthStore();
const router = useRouter();

async function signOut() {
  await auth.logout();
  await router.push({ name: 'login' });
}
</script>

<template>
  <div class="flex items-center gap-3 px-2">
    <RouterLink to="/app/profile" class="flex min-w-0 flex-1 items-center gap-3">
      <span
        class="grid size-9 shrink-0 place-items-center rounded-full bg-accent-500/15 font-semibold text-accent-700 uppercase dark:text-accent-300"
      >
        {{ (auth.user?.name || auth.user?.email || '?').slice(0, 1) }}
      </span>
      <span class="min-w-0">
        <span class="block truncate text-sm font-medium">
          {{ auth.user?.name || auth.user?.email }}
        </span>
        <span class="block truncate text-xs text-stone-500">{{ auth.user?.email }}</span>
      </span>
    </RouterLink>
    <button
      type="button"
      class="rounded-lg p-1.5 text-stone-500 hover:bg-stone-900/5 hover:text-red-600 dark:hover:bg-white/10"
      :title="t('auth.signOut')"
      :aria-label="t('auth.signOut')"
      @click="signOut"
    >
      <LogOut class="size-4" />
    </button>
  </div>
</template>
