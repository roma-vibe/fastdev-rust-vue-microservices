/**
 * Translations (vue-i18n, Composition API). Messages are collected automatically from
 * `shared/locales/<lang>.json` and every `features/<feature>/locales/<lang>.json`; a feature's file
 * holds one top-level key named after the feature (`{"notes": {...}}`), so features never collide.
 * Use `const { t } = useI18n()` in components, `i18n.global.t` elsewhere.
 */
import { createI18n } from 'vue-i18n';
import { ref } from 'vue';

export const LOCALES = ['ru', 'en'] as const;
export type Locale = (typeof LOCALES)[number];

interface Messages {
  [key: string]: string | Messages;
}

const files = import.meta.glob<Messages>(['./locales/*.json', '../features/*/locales/*.json'], {
  eager: true,
  import: 'default',
});

function merge(target: Messages, source: Messages): Messages {
  for (const [key, value] of Object.entries(source)) {
    const current = target[key];
    if (isObject(value) && isObject(current)) merge(current, value);
    else target[key] = value;
  }
  return target;
}

function isObject(value: unknown): value is Messages {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

const messages: Record<Locale, Messages> = { ru: {}, en: {} };
for (const [path, content] of Object.entries(files)) {
  const lang = /\/(\w+)\.json$/.exec(path)?.[1];
  if (lang === 'ru' || lang === 'en') merge(messages[lang], content);
}

const STORAGE_KEY = 'app-locale';

function initialLocale(): Locale {
  const stored = localStorage.getItem(STORAGE_KEY);
  if (stored === 'ru' || stored === 'en') return stored;
  return navigator.language.toLowerCase().startsWith('ru') ? 'ru' : 'en';
}

export const i18n = createI18n<false>({
  legacy: false,
  locale: initialLocale(),
  fallbackLocale: 'en',
  messages,
  missingWarn: false,
  fallbackWarn: false,
});

/** The current UI language (reactive). */
export const locale = ref<Locale>(i18n.global.locale.value as Locale);

export function setLocale(value: Locale): void {
  i18n.global.locale.value = value;
  locale.value = value;
  localStorage.setItem(STORAGE_KEY, value);
  document.documentElement.lang = value;
}

document.documentElement.lang = locale.value;
