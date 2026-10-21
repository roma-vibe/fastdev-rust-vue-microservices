/**
 * Auth store: the signed-in user. The session itself is an HttpOnly cookie set by the identity
 * service, so the web app never sees a token. Shared by the auth feature (guard, sign-in pages)
 * and every feature that needs the user. Endpoints: docs/api.md, "identity".
 */
import { defineStore } from 'pinia';
import { computed, ref } from 'vue';
import { ApiError, http, setUnauthorizedHandler } from '@/shared/api/http.ts';

export interface User {
  id: string;
  email: string;
  name: string;
  role: 'user' | 'admin';
  createdAt: number;
}

export interface AuthConfig {
  registrationOpen: boolean;
  hasUsers: boolean;
}

interface AuthResponse {
  user: User;
}

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null);
  const loaded = ref(false);

  const signedIn = computed(() => user.value !== null);
  const isAdmin = computed(() => user.value?.role === 'admin');

  function accept(response: AuthResponse): User {
    user.value = response.user;
    loaded.value = true;
    return response.user;
  }

  /** Loads the current user once (route guard); signed out when there is no valid session. */
  async function load(): Promise<void> {
    if (loaded.value) return;
    try {
      user.value = await http.get<User>('/api/me');
    } catch (error) {
      if (!(error instanceof ApiError) || error.status !== 401) throw error;
      user.value = null;
    }
    loaded.value = true;
  }

  const config = () => http.get<AuthConfig>('/api/auth/config');

  const login = async (email: string, password: string) =>
    accept(await http.post<AuthResponse>('/api/auth/login', { email, password }));

  const register = async (input: { email: string; password: string; name?: string }) =>
    accept(await http.post<AuthResponse>('/api/auth/register', input));

  async function logout(): Promise<void> {
    try {
      await http.post('/api/auth/logout');
    } finally {
      user.value = null;
    }
  }

  async function update(patch: { name?: string }): Promise<User> {
    user.value = await http.patch<User>('/api/me', patch);
    return user.value;
  }

  setUnauthorizedHandler(() => {
    user.value = null;
  });

  return { user, loaded, signedIn, isAdmin, load, config, login, register, logout, update };
});
