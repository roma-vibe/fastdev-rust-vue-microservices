/**
 * Sign-in guard: pages of the `app` layout (the default) need a signed-in user, `guestOnly`
 * pages (sign-in, sign-up) send signed-in users to the app, `admin` pages need the admin role. Public and bare pages are open to all.
 */
import type { RouteLocationNormalized, RouteLocationRaw } from 'vue-router';
import { useAuthStore } from '@/shared/auth.ts';

declare module 'vue-router' {
  interface RouteMeta {
    /** Only for signed-out visitors (sign-in, sign-up): signed-in users go to the app. */
    guestOnly?: boolean;
  }
}

export async function authGuard(to: RouteLocationNormalized): Promise<true | RouteLocationRaw> {
  const auth = useAuthStore();
  await auth.load();
  const needsUser = (to.meta.layout ?? 'app') === 'app';
  if (needsUser && !auth.signedIn) return { name: 'login', query: { next: to.fullPath } };
  if (to.meta.admin && !auth.isAdmin) return { path: '/app' };
  if (to.meta.guestOnly && auth.signedIn) return { path: '/app' };
  return true;
}

/** Where to go after signing in: the `next` query when it is a local path, else the app. */
export function nextPath(next: unknown): string {
  return typeof next === 'string' && next.startsWith('/') && !next.startsWith('//') ? next : '/app';
}
