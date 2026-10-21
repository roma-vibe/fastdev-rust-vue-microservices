import { createPinia, setActivePinia } from 'pinia';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { type ApiKey, keysApi } from './api.ts';
import { useKeysStore } from './store.ts';

const key = (id: string, revokedAt: number | null = null): ApiKey => ({
  id,
  name: `Key ${id}`,
  prefix: 'sk-abcd',
  createdAt: 1,
  lastUsedAt: null,
  revokedAt,
});

describe('keys store', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.restoreAllMocks();
  });

  it('lists, creates and revokes keys', async () => {
    vi.spyOn(keysApi, 'list').mockResolvedValue([key('1'), key('2', 5)]);
    vi.spyOn(keysApi, 'create').mockResolvedValue({ key: key('3'), secret: 'sk-secret' });
    vi.spyOn(keysApi, 'revoke').mockResolvedValue(undefined);
    const store = useKeysStore();

    await store.load();
    expect(store.active.map((k) => k.id)).toEqual(['1']);

    const created = await store.create('CI');
    expect(created.secret).toBe('sk-secret');
    expect(store.items[0]?.id).toBe('3');

    await store.revoke('1');
    expect(store.active.map((k) => k.id)).toEqual(['3']);
  });

  it('keeps the error of a failed load', async () => {
    vi.spyOn(keysApi, 'list').mockRejectedValue(new Error('offline'));
    const store = useKeysStore();
    await store.load();
    expect(store.error).toBe('offline');
  });
});
