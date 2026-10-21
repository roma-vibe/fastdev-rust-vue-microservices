<script setup lang="ts">
/** Picks the layout of the current route (`meta.layout`). */
import { computed } from 'vue';
import { RouterView, useRoute } from 'vue-router';
import AppBackdrop from '@/app/AppBackdrop.vue';
import AppLayout from '@/app/AppLayout.vue';
import PublicLayout from '@/app/PublicLayout.vue';
import ToastHost from '@/app/ToastHost.vue';

const route = useRoute();
const layout = computed(() => route.meta.layout ?? 'app');
</script>

<template>
  <AppBackdrop v-if="layout !== 'bare'" />
  <AppLayout v-if="layout === 'app' && route.matched.length" />
  <PublicLayout v-else-if="layout === 'public'" />
  <RouterView v-else />
  <ToastHost />
</template>
