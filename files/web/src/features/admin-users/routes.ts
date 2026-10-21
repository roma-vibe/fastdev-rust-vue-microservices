import { Users } from '@lucide/vue';
import type { FeatureRoute } from '@/app/types.ts';

export default [
  {
    path: '/app/admin/users',
    name: 'admin-users',
    component: () => import('./pages/AdminUsersPage.vue'),
    meta: {
      titleKey: 'adminUsers.title',
      admin: true,
      nav: { section: 'admin', icon: Users, order: 10 },
    },
  },
] satisfies FeatureRoute[];
