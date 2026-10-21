/** Personal API keys: the list and the actions. The secret of a new key is kept only in memory. */
import { defineStore } from 'pinia';
import { computed, ref } from 'vue';
import { errorMessage } from '@/shared/api/http.ts';
import { type ApiKey, type CreatedKey, keysApi } from './api.ts';

export const useKeysStore = defineStore('keys', () => {
  const items = ref<ApiKey[]>([]);
  const loading = ref(false);
  /** The first load finished (successfully or not): later loads refresh in the background. */
  const loaded = ref(false);
  const error = ref('');

  const active = computed(() => items.value.filter((key) => key.revokedAt === null));

  async function load(): Promise<void> {
    loading.value = true;
    error.value = '';
    try {
      items.value = await keysApi.list();
    } catch (e) {
      error.value = errorMessage(e);
    } finally {
      loading.value = false;
      loaded.value = true;
    }
  }

  async function create(name: string): Promise<CreatedKey> {
    const created = await keysApi.create(name);
    items.value = [created.key, ...items.value];
    return created;
  }

  async function revoke(id: string): Promise<void> {
    await keysApi.revoke(id);
    const now = Date.now();
    items.value = items.value.map((key) => (key.id === id ? { ...key, revokedAt: now } : key));
  }

  return { items, loading, loaded, error, active, load, create, revoke };
});
