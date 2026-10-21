/** Colour theme: light, dark or system (follows the OS). Stored in localStorage. */
import { ref, watchEffect } from 'vue';

export type ThemeMode = 'light' | 'dark' | 'system';

const STORAGE_KEY = 'app-theme';
const media = window.matchMedia('(prefers-color-scheme: dark)');
const stored = localStorage.getItem(STORAGE_KEY);

export const themeMode = ref<ThemeMode>(
  stored === 'light' || stored === 'dark' || stored === 'system' ? stored : 'system',
);
const systemDark = ref(media.matches);
media.addEventListener('change', (event) => {
  systemDark.value = event.matches;
});

/** True when the dark theme is shown. */
export const isDark = ref(false);

watchEffect(() => {
  isDark.value = themeMode.value === 'dark' || (themeMode.value === 'system' && systemDark.value);
  document.documentElement.classList.toggle('dark', isDark.value);
  localStorage.setItem(STORAGE_KEY, themeMode.value);
});

/** Cycles light → dark → system. */
export function cycleTheme(): void {
  const order: ThemeMode[] = ['light', 'dark', 'system'];
  themeMode.value = order[(order.indexOf(themeMode.value) + 1) % order.length] ?? 'system';
}
