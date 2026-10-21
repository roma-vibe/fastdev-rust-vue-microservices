/// <reference types="vite/client" />

/** Public values from the root .env, whitelisted in vite.config.ts (PUBLIC_ENV_KEYS). */
interface ImportMetaEnv {
  readonly APP_NAME: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
