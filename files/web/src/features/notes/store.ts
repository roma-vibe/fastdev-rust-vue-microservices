/** Notes state: the list, loading and errors. Pages call the actions; the API is in api.ts. */
import { defineStore } from 'pinia';
import { computed, ref } from 'vue';
import { errorMessage } from '@/shared/api/http.ts';
import { notesApi, type Note, type NoteInput } from './api.ts';

/** Pinned first, then the most recently updated (the order of the API). */
function sorted(notes: Note[]): Note[] {
  return [...notes].sort(
    (a, b) => Number(b.pinned) - Number(a.pinned) || b.updatedAt - a.updatedAt,
  );
}

export const useNotesStore = defineStore('notes', () => {
  const items = ref<Note[]>([]);
  const loading = ref(false);
  /** The first load finished (successfully or not): later loads refresh in the background. */
  const loaded = ref(false);
  const error = ref('');

  const count = computed(() => items.value.length);

  async function load(): Promise<void> {
    loading.value = true;
    error.value = '';
    try {
      items.value = await notesApi.list();
    } catch (e) {
      error.value = errorMessage(e);
    } finally {
      loading.value = false;
      loaded.value = true;
    }
  }

  async function create(input: NoteInput): Promise<Note> {
    const note = await notesApi.create(input);
    items.value = sorted([note, ...items.value]);
    return note;
  }

  async function update(id: string, patch: Partial<NoteInput>): Promise<Note> {
    const note = await notesApi.update(id, patch);
    items.value = sorted(items.value.map((item) => (item.id === id ? note : item)));
    return note;
  }

  async function remove(id: string): Promise<void> {
    await notesApi.remove(id);
    items.value = items.value.filter((item) => item.id !== id);
  }

  return { items, loading, loaded, error, count, load, create, update, remove };
});
