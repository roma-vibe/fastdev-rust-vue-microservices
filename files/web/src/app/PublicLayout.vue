<script setup lang="ts">
/** Public pages (sign-in, 404): slim glass top bar. */
import { RouterView } from 'vue-router';
import AppLogo from './AppLogo.vue';
import { extensions } from './extensions.ts';
import LocaleSwitch from './LocaleSwitch.vue';
import ThemeToggle from './ThemeToggle.vue';

const actions = extensions.flatMap((extension) => extension.publicActions ?? []);
</script>

<template>
  <div class="flex min-h-screen flex-col">
    <header class="px-4 pt-3 sm:px-6">
      <div class="glass mx-auto flex max-w-6xl items-center gap-3 !rounded-2xl px-4 py-2">
        <AppLogo />
        <div class="ml-auto flex items-center gap-2">
          <LocaleSwitch />
          <ThemeToggle />
          <component :is="action" v-for="(action, index) in actions" :key="index" />
        </div>
      </div>
    </header>
    <main class="mx-auto w-full max-w-6xl flex-1 px-4 py-8 sm:px-6"><RouterView /></main>
  </div>
</template>
