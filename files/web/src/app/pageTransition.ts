/**
 * Fade between pages with the View Transitions API: the browser cross-fades screenshots of the old
 * and the new page. A `Transition` with `opacity` on the page would switch off the backdrop blur of
 * the glass cards for the length of the fade. Browsers without the API just navigate instantly.
 */
import { nextTick } from 'vue';
import type { Router } from 'vue-router';

export function installPageTransition(router: Router) {
  let finish: (() => void) | undefined;

  router.beforeResolve((to, from) => {
    const start = document.startViewTransition?.bind(document);
    const reduced = window.matchMedia?.('(prefers-reduced-motion: reduce)').matches;
    // The first navigation, same-page changes (query, hash) and reduced motion are not animated.
    if (!start || reduced || !from.matched.length || to.path === from.path) return;
    return new Promise<void>((resolve) => {
      start(
        () =>
          new Promise<void>((done) => {
            finish = done;
            resolve();
          }),
      );
    });
  });

  // Runs after every navigation, successful or not, so a transition never stays open.
  router.afterEach(() => {
    const done = finish;
    finish = undefined;
    if (done) void nextTick(done);
  });
}
