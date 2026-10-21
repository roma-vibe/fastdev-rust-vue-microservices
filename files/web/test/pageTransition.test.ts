import { afterEach, describe, expect, it, vi } from 'vitest';
import { createMemoryHistory, createRouter } from 'vue-router';
import { installPageTransition } from '@/app/pageTransition.ts';

const page = { template: '<div />' };

function makeRouter() {
  const router = createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: '/', component: page },
      { path: '/a', component: page },
      { path: '/b', component: page },
    ],
  });
  installPageTransition(router);
  return router;
}

describe('page transition', () => {
  afterEach(() => {
    Reflect.deleteProperty(document, 'startViewTransition');
  });

  it('navigates normally without the View Transitions API', async () => {
    const router = makeRouter();
    await router.push('/a');
    await router.push('/b');
    expect(router.currentRoute.value.path).toBe('/b');
  });

  it('wraps a page change in a view transition and finishes it', async () => {
    let finished = false;
    const start = vi.fn((update: () => Promise<void>) => {
      void update().then(() => (finished = true));
    });
    Object.assign(document, { startViewTransition: start });

    const router = makeRouter();
    await router.push('/a'); // first navigation: not animated
    expect(start).not.toHaveBeenCalled();

    await router.push('/b');
    expect(start).toHaveBeenCalledOnce();
    await vi.waitFor(() => expect(finished).toBe(true));
    expect(router.currentRoute.value.path).toBe('/b');
  });

  it('does not animate a change of the query only', async () => {
    const start = vi.fn();
    Object.assign(document, { startViewTransition: start });
    const router = makeRouter();
    await router.push('/a');
    await router.push({ path: '/a', query: { q: '1' } });
    expect(start).not.toHaveBeenCalled();
  });
});
