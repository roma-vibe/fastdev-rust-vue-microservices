import { LayoutDashboard } from '@lucide/vue';
import type { FeatureRoute } from '@/app/types.ts';

export default [
  {
    path: '/',
    name: 'landing',
    component: () => import('./pages/LandingPage.vue'),
    meta: { titleKey: 'home.landing.title', layout: 'public' },
  },
  {
    path: '/app',
    name: 'home',
    component: () => import('./pages/HomePage.vue'),
    meta: { titleKey: 'home.title', nav: { icon: LayoutDashboard, order: 0 } },
  },
] satisfies FeatureRoute[];
