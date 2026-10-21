import { createPinia, setActivePinia } from 'pinia';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { adminUsersApi } from './api.ts';
import { useAdminUsersStore } from './store.ts';

describe('admin users store', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.restoreAllMocks();
  });

  it('loads the users and marks the first load done', async () => {
    const user = {
      id: '1',
      email: 'a@b.c',
      name: '',
      role: 'admin' as const,
      createdAt: 1,
      disabled: false,
      keyCount: 2,
    };
    vi.spyOn(adminUsersApi, 'list').mockResolvedValue([user]);
    const store = useAdminUsersStore();
    await store.load();
    expect(store.items).toEqual([user]);
    expect(store.loaded).toBe(true);
    expect(store.error).toBe('');
  });

  it('keeps the error and still ends the first load', async () => {
    vi.spyOn(adminUsersApi, 'list').mockRejectedValue(new Error('nope'));
    const store = useAdminUsersStore();
    await store.load();
    expect(store.error).toBe('nope');
    expect(store.loaded).toBe(true);
  });
});
