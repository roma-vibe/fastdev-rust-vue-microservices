import { UserRound } from '@lucide/vue';
import type { FeatureRoute } from '@/app/types.ts';

export default [
  {
    path: '/app/profile',
    name: 'profile',
    component: () => import('./pages/ProfilePage.vue'),
    meta: { titleKey: 'profile.title', nav: { icon: UserRound, order: 80 } },
  },
] satisfies FeatureRoute[];
