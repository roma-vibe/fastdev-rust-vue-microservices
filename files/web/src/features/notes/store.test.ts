import { createPinia, setActivePinia } from 'pinia';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { notesApi, type Note } from './api.ts';
import { useNotesStore } from './store.ts';

vi.mock('./api.ts', () => ({
  notesApi: { list: vi.fn(), create: vi.fn(), update: vi.fn(), remove: vi.fn() },
}));

const note = (id: string, updatedAt: number, pinned = false): Note => ({
  id,
  title: `Note ${id}`,
  body: '',
  pinned,
  createdAt: updatedAt,
  updatedAt,
});

describe('notes store', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.resetAllMocks();
  });

  it('loads notes and reports errors', async () => {
    const store = useNotesStore();
    expect(store.loaded).toBe(false);
    vi.mocked(notesApi.list).mockResolvedValueOnce([note('a', 1)]);
    await store.load();
    expect(store.count).toBe(1);
    expect(store.loaded).toBe(true);

    vi.mocked(notesApi.list).mockRejectedValueOnce(new Error('offline'));
    await store.load();
    expect(store.error).toBe('offline');
    expect(store.loading).toBe(false);
  });

  it('keeps pinned and recent notes first', async () => {
    const store = useNotesStore();
    store.items = [note('old', 1), note('new', 2)];
    vi.mocked(notesApi.update).mockResolvedValueOnce(note('old', 3, true));
    await store.update('old', { pinned: true });
    expect(store.items.map((n) => n.id)).toEqual(['old', 'new']);

    vi.mocked(notesApi.create).mockResolvedValueOnce(note('fresh', 4));
    await store.create({ title: 'Note fresh', body: '' });
    expect(store.items.map((n) => n.id)).toEqual(['old', 'fresh', 'new']);

    vi.mocked(notesApi.remove).mockResolvedValueOnce(undefined);
    await store.remove('fresh');
    expect(store.items.map((n) => n.id)).toEqual(['old', 'new']);
  });
});
