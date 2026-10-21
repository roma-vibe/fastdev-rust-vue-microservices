import { mount } from '@vue/test-utils';
import { describe, expect, it, vi } from 'vitest';
import { featureRoutes } from '@/app/router.ts';
import { withQuery } from '@/shared/api/http.ts';
import { i18n, setLocale } from '@/shared/i18n.ts';
import { formatMs, formatPercent } from '@/shared/lib/format.ts';
import { UiLoader, UiSpinner } from '@/shared/ui/index.ts';

describe('shell', () => {
  it('collects feature routes with titles', () => {
    const names = featureRoutes.map((route) => route.name);
    expect(names).toContain('landing');
    expect(names).toContain('home');
    expect(names).toContain('notes');
    for (const route of featureRoutes) expect(route.meta?.titleKey).toBeTruthy();
  });

  it('serves the landing page publicly and the app under /app', () => {
    const byName = new Map(featureRoutes.map((route) => [route.name, route]));
    expect(byName.get('landing')?.path).toBe('/');
    expect(byName.get('landing')?.meta?.layout).toBe('public');
    for (const route of featureRoutes) {
      if (route.meta?.nav) expect(route.path.startsWith('/app'), route.path).toBe(true);
    }
  });

  it('renders the landing page with its two sections', async () => {
    const { default: LandingPage } = await import('@/features/home/pages/LandingPage.vue');
    setLocale('en');
    const page = mount(LandingPage, {
      global: { plugins: [i18n], stubs: { RouterLink: { template: '<a><slot /></a>' } } },
    });
    expect(page.findAll('h2')).toHaveLength(2);
    expect(page.findAll('li').length).toBeGreaterThan(2);
    expect(page.text()).toContain('simple skeleton');
  });

  it('merges feature locales into i18n in both languages', () => {
    setLocale('ru');
    expect(i18n.global.t('common.save')).toBe('Сохранить');
    for (const route of featureRoutes) {
      const key = route.meta?.titleKey ?? '';
      expect(i18n.global.te(key, 'ru'), key).toBe(true);
      expect(i18n.global.te(key, 'en'), key).toBe(true);
    }
    setLocale('en');
    expect(i18n.global.t('common.save')).toBe('Save');
  });

  it('builds query strings without empty values', () => {
    expect(withQuery('/api/notes', { q: 'a b', page: 2, empty: '', none: null })).toBe(
      '/api/notes?q=a+b&page=2',
    );
    expect(withQuery('/api/notes')).toBe('/api/notes');
  });

  it('formats durations and ratios', () => {
    expect(formatMs(850)).toBe('850 ms');
    expect(formatMs(1234)).toBe('1.2 s');
    expect(formatPercent(0.876)).toBe('88%');
  });

  it('sizes the spinner and shows the loader only after its delay', async () => {
    expect(mount(UiSpinner).classes()).toContain('size-5');
    expect(mount(UiSpinner, { props: { size: 'sm' } }).classes()).toContain('size-4');

    vi.useFakeTimers();
    const loader = mount(UiLoader, { global: { plugins: [i18n] } });
    expect(loader.find('svg').exists()).toBe(false);
    await vi.advanceTimersByTimeAsync(300);
    expect(loader.find('svg').exists()).toBe(true);
    vi.useRealTimers();
  });
});
