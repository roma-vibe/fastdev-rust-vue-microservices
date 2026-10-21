import type { FeatureRoute } from '@/app/types.ts';

export default [
  {
    path: '/login',
    name: 'login',
    component: () => import('./pages/LoginPage.vue'),
    meta: { titleKey: 'auth.title.login', layout: 'public', guestOnly: true },
  },
  {
    path: '/register',
    name: 'register',
    component: () => import('./pages/RegisterPage.vue'),
    meta: { titleKey: 'auth.title.register', layout: 'public', guestOnly: true },
  },
] satisfies FeatureRoute[];
