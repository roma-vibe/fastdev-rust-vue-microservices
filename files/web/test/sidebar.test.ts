import { beforeEach, describe, expect, it, vi } from 'vitest';

describe('sidebar state', () => {
  beforeEach(() => {
    localStorage.clear();
    vi.resetModules();
  });

  it('starts expanded, toggles and remembers the choice', async () => {
    const { useSidebar } = await import('@/app/sidebar.ts');
    const sidebar = useSidebar();
    expect(sidebar.collapsed.value).toBe(false);
    sidebar.toggle();
    await vi.waitFor(() => expect(localStorage.getItem('sidebar-collapsed')).toBe('1'));

    vi.resetModules();
    const again = (await import('@/app/sidebar.ts')).useSidebar();
    expect(again.collapsed.value).toBe(true);
  });
});
