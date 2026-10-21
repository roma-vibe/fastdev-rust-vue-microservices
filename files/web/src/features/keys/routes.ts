import { KeyRound } from '@lucide/vue';
import type { FeatureRoute } from '@/app/types.ts';

export default [
  {
    path: '/app/keys',
    name: 'keys',
    component: () => import('./pages/KeysPage.vue'),
    meta: { titleKey: 'keys.title', nav: { icon: KeyRound, order: 70 } },
  },
] satisfies FeatureRoute[];
