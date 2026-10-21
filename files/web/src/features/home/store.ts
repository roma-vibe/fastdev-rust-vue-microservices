/** Health of the backend services; kept between visits so the page shows it at once. */
import { defineStore } from 'pinia';
import { ref } from 'vue';
import { errorMessage } from '@/shared/api/http.ts';
import { fetchHealth, type Health } from './api.ts';

export const useHomeStore = defineStore('home', () => {
  const health = ref<Health | null>(null);
  const loading = ref(false);
  const error = ref('');

  async function load(): Promise<void> {
    loading.value = true;
    error.value = '';
    try {
      health.value = await fetchHealth();
    } catch (e) {
      error.value = errorMessage(e);
    } finally {
      loading.value = false;
    }
  }

  return { health, loading, error, load };
});
