/**
 * Router: collects the routes.ts of every feature folder automatically and installs the guards of
 * the feature shell extensions (app/extensions.ts). Page titles come from `meta.titleKey`.
 */
import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router';
import { appName } from '@/config.ts';
import { i18n } from '@/shared/i18n.ts';
import { extensions } from './extensions.ts';
import { installPageTransition } from './pageTransition.ts';
import './types.ts';

const modules = import.meta.glob<RouteRecordRaw[]>('../features/*/routes.ts', {
  eager: true,
  import: 'default',
});

export const featureRoutes: RouteRecordRaw[] = Object.values(modules).flat();

export const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    ...featureRoutes,
    {
      path: '/:pathMatch(.*)*',
      name: 'not-found',
      component: () => import('./NotFoundPage.vue'),
      meta: { titleKey: 'errors.notFoundTitle', layout: 'public' },
    },
  ],
  scrollBehavior: (_to, _from, saved) => saved ?? { top: 0 },
});

installPageTransition(router);

for (const extension of extensions) {
  if (extension.guard) router.beforeEach(extension.guard);
}

router.afterEach((to) => {
  const title = to.meta.titleKey ? i18n.global.t(to.meta.titleKey) : '';
  document.title = title ? `${title} · ${appName}` : appName;
});
