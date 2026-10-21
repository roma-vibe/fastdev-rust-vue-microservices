/**
 * Contract between the shell and feature modules. A feature is a folder `features/<name>/` with:
 *   routes.ts                  `export default [...] satisfies FeatureRoute[]` (auto-registered)
 *   locales/ru.json, en.json   `{"<name>": {...}}` (auto-merged into i18n)
 *   shell.ts (optional)        `export default {...} satisfies ShellExtension` (guards, shell widgets)
 *   pages/, components/, api.ts, store.ts   private to the feature
 * Features import only from `@/shared/**`, their own folder and the types of this file.
 */
import type { Component } from 'vue';
import type { NavigationGuard, RouteRecordRaw } from 'vue-router';

export interface NavItem {
  /** Sidebar section: `main` (default) or `admin` (shown to administrators only). */
  section?: 'main' | 'admin';
  /** Lucide icon component. */
  icon: Component;
  /** Position in the sidebar (lower first). */
  order: number;
}

declare module 'vue-router' {
  interface RouteMeta {
    /** i18n key of the page title (document title, top bar and sidebar label). */
    titleKey?: string;
    /** `app` (sidebar shell; default), `public` (slim top bar), `bare` (nothing). */
    layout?: 'app' | 'public' | 'bare';
    /** Administrators only (needs the Accounts feature; others are sent to `/app`). */
    admin?: boolean;
    /** Show in the sidebar. */
    nav?: NavItem;
  }
}

export type FeatureRoute = RouteRecordRaw;

/** What a feature may add to the shell (all optional). */
export interface ShellExtension {
  /** A global navigation guard (runs before every navigation, in feature-name order). */
  guard?: NavigationGuard;
  /** Whether the current user is an administrator (reactive getter; shows the `admin` nav section). */
  isAdmin?: () => boolean;
  /** Shown at the bottom of the sidebar (e.g. the signed-in user). */
  sidebarFooter?: Component;
  /** Shown at the right of the public top bar (e.g. a sign-in button). */
  publicActions?: Component;
}
