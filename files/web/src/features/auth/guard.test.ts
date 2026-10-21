import { createPinia, setActivePinia } from 'pinia';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { RouteLocationNormalized } from 'vue-router';
import { ApiError, http } from '@/shared/api/http.ts';
import { authGuard, nextPath } from './guard.ts';

const route = (meta: RouteLocationNormalized['meta'], fullPath = '/app/notes') =>
  ({ meta, fullPath }) as RouteLocationNormalized;

describe('auth guard', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.restoreAllMocks();
  });

  it('sends visitors to sign in and keeps the target', async () => {
    vi.spyOn(http, 'get').mockRejectedValue(new ApiError(401, 'unauthorized', 'Sign in'));
    expect(await authGuard(route({}))).toEqual({ name: 'login', query: { next: '/app/notes' } });
    expect(await authGuard(route({ layout: 'public', guestOnly: true }))).toBe(true);
  });

  it('lets users in and keeps them out of the sign-in pages', async () => {
    vi.spyOn(http, 'get').mockResolvedValue({ id: '1', email: 'a@b.c', name: '', role: 'user' });
    expect(await authGuard(route({}))).toBe(true);
    expect(await authGuard(route({ layout: 'public', guestOnly: true }))).toEqual({ path: '/app' });
  });

  it('keeps non-admins out of admin pages', async () => {
    const user = { id: '1', email: 'a@b.c', name: '', role: 'user' };
    const spy = vi.spyOn(http, 'get').mockResolvedValue(user);
    expect(await authGuard(route({ admin: true }))).toEqual({ path: '/app' });
    setActivePinia(createPinia());
    spy.mockResolvedValue({ ...user, role: 'admin' });
    expect(await authGuard(route({ admin: true }))).toBe(true);
  });

  it('only follows local next paths', () => {
    expect(nextPath('/app/notes?x=1')).toBe('/app/notes?x=1');
    expect(nextPath('//evil.example')).toBe('/app');
    expect(nextPath('https://evil.example')).toBe('/app');
    expect(nextPath(undefined)).toBe('/app');
  });
});
