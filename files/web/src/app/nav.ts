/** Sidebar items built from route meta (`meta.nav`), grouped in sections and sorted by `order`. */
import { computed } from 'vue';
import { extensions } from './extensions.ts';
import { featureRoutes } from './router.ts';
import type { NavItem } from './types.ts';

export interface NavLink extends NavItem {
  section: 'main' | 'admin';
  name: string;
  path: string;
  titleKey: string;
}

const links: NavLink[] = featureRoutes
  .filter((route) => route.meta?.nav && typeof route.name === 'string')
  .map((route) => {
    const nav = route.meta?.nav as NavItem;
    return {
      ...nav,
      section: nav.section ?? 'main',
      name: route.name as string,
      path: route.path,
      titleKey: route.meta?.titleKey ?? String(route.name),
    };
  })
  .sort((a, b) => a.order - b.order);

/** The sidebar sections for the current user; `admin` is empty unless a feature says so. */
export function useNav() {
  return computed(() => {
    const isAdmin = extensions.some((extension) => extension.isAdmin?.());
    return {
      main: links.filter((link) => link.section === 'main'),
      admin: isAdmin ? links.filter((link) => link.section === 'admin') : [],
    };
  });
}
