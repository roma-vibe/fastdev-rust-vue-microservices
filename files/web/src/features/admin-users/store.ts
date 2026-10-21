/** Admin user list. The last result stays on screen while it refreshes. */
import { defineStore } from 'pinia';
import { ref } from 'vue';
import { errorMessage } from '@/shared/api/http.ts';
import { type AdminUser, adminUsersApi } from './api.ts';

export const useAdminUsersStore = defineStore('adminUsers', () => {
  const items = ref<AdminUser[]>([]);
  const loading = ref(false);
  const loaded = ref(false);
  const error = ref('');

  async function load() {
    loading.value = true;
    error.value = '';
    try {
      items.value = await adminUsersApi.list();
    } catch (e) {
      error.value = errorMessage(e);
    } finally {
      loading.value = false;
      loaded.value = true;
    }
  }

  return { items, loading, loaded, error, load };
});
