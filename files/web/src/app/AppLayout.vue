<script setup lang="ts">
/** Main shell: glass sidebar (feature nav from route meta) + top bar + page. */
import { Menu, PanelLeftClose, PanelLeftOpen, X } from '@lucide/vue';
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { RouterLink, RouterView, useRoute } from 'vue-router';
import AppLogo from './AppLogo.vue';
import { extensions } from './extensions.ts';
import LocaleSwitch from './LocaleSwitch.vue';
import { useNav } from './nav.ts';
import { useSidebar } from './sidebar.ts';
import ThemeToggle from './ThemeToggle.vue';

const { t } = useI18n();
const route = useRoute();
const mobileOpen = ref(false);
const nav = useNav();
const sidebar = useSidebar();
const footers = extensions.flatMap((extension) => extension.sidebarFooter ?? []);
watch(
  () => route.fullPath,
  () => (mobileOpen.value = false),
);
</script>

<template>
  <div class="flex min-h-screen">
    <div
      v-if="mobileOpen"
      class="fixed inset-0 z-30 bg-stone-950/30 backdrop-blur-sm lg:hidden"
      @click="mobileOpen = false"
    />
    <aside
      class="glass fixed inset-y-3 left-3 z-40 flex w-64 flex-col !rounded-3xl p-3 transition-transform duration-300"
      :class="[
        mobileOpen ? 'translate-x-0' : '-translate-x-[110%]',
        sidebar.collapsed.value ? 'lg:-translate-x-[110%]' : 'lg:translate-x-0',
      ]"
    >
      <div class="flex items-center justify-between px-2 py-2">
        <AppLogo to="/app" />
        <button type="button" class="lg:hidden" @click="mobileOpen = false">
          <X class="size-5" />
        </button>
      </div>
      <nav class="mt-4 flex-1 space-y-6 overflow-y-auto">
        <template v-for="section in ['main', 'admin'] as const" :key="section">
          <div v-if="nav[section].length">
            <p
              v-if="nav.admin.length"
              class="px-3 pb-2 text-[11px] font-semibold tracking-wider text-stone-500 uppercase"
            >
              {{ t(`nav.${section}`) }}
            </p>
            <RouterLink
              v-for="link in nav[section]"
              :key="link.name"
              :to="link.path"
              class="flex items-center gap-3 rounded-xl px-3 py-2 text-sm text-stone-700 transition hover:bg-white/60 dark:text-stone-300 dark:hover:bg-white/10"
              exact-active-class="!bg-accent-500/15 !text-accent-700 font-medium dark:!text-accent-300 shadow-[inset_0_0_0_1px] shadow-accent-500/25"
            >
              <component :is="link.icon" class="size-[18px]" />
              {{ t(link.titleKey) }}
            </RouterLink>
          </div>
        </template>
      </nav>
      <div
        v-for="(footer, index) in footers"
        :key="index"
        class="mt-3 border-t border-stone-900/5 pt-3 dark:border-white/10"
      >
        <component :is="footer" />
      </div>
    </aside>

    <div
      class="flex min-w-0 flex-1 flex-col transition-[padding] duration-300"
      :class="sidebar.collapsed.value ? 'lg:pl-0' : 'lg:pl-[17.5rem]'"
    >
      <header class="sticky top-0 z-20 flex items-center gap-3 px-4 pt-3 sm:px-6 lg:px-8">
        <div class="glass flex w-full items-center gap-2 !rounded-2xl px-3 py-2">
          <button
            type="button"
            class="rounded-lg p-1.5 lg:hidden"
            :aria-label="t('nav.showSidebar')"
            @click="mobileOpen = true"
          >
            <Menu class="size-5" />
          </button>
          <button
            type="button"
            class="hidden rounded-lg p-1.5 hover:bg-stone-900/5 lg:inline-flex dark:hover:bg-white/10"
            :aria-label="sidebar.collapsed.value ? t('nav.showSidebar') : t('nav.hideSidebar')"
            :title="sidebar.collapsed.value ? t('nav.showSidebar') : t('nav.hideSidebar')"
            @click="sidebar.toggle()"
          >
            <PanelLeftOpen v-if="sidebar.collapsed.value" class="size-5" />
            <PanelLeftClose v-else class="size-5" />
          </button>
          <p class="truncate font-medium">
            {{ route.meta.titleKey ? t(route.meta.titleKey) : '' }}
          </p>
          <div class="ml-auto flex items-center gap-2">
            <LocaleSwitch />
            <ThemeToggle />
          </div>
        </div>
      </header>
      <main class="mx-auto w-full max-w-7xl flex-1 px-4 py-6 sm:px-6 lg:px-8">
        <RouterView />
      </main>
    </div>
  </div>
</template>
