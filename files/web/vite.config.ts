import { fileURLToPath } from 'node:url';
import tailwindcss from '@tailwindcss/vite';
import vue from '@vitejs/plugin-vue';
import { defineConfig } from 'vite';
import { htmlEnv, readEnv } from './vite/env.ts';

/** The project root holds the single .env shared by the services and the web app. */
const rootDir = fileURLToPath(new URL('..', import.meta.url));

/**
 * The ONLY .env keys exposed to browser code, as import.meta.env.<KEY> (typed in src/env.d.ts).
 * Everything in the browser bundle is public: never add secrets here.
 */
const PUBLIC_ENV_KEYS = ['APP_NAME'] as const;

export default defineConfig(() => {
  // Every key of the root .env, read literally (vite/env.ts), is available to this config;
  // only PUBLIC_ENV_KEYS reach the bundle.
  const env = readEnv(rootDir);
  const missing = PUBLIC_ENV_KEYS.filter((key) => !env[key]);
  if (missing.length > 0) {
    throw new Error(`Missing ${missing.join(', ')} in ${rootDir}.env (copy .env.example to .env)`);
  }

  // Everything the browser calls under /api goes through the edge service.
  const edgeTarget = `http://127.0.0.1:${Number(env.EDGE_PORT || 8700)}`;
  const proxy = { '/api': edgeTarget };
  const webPort = Number(env.WEB_PORT || 5173);
  // WEB_HOST=0.0.0.0 makes the dev server reachable from other devices on the network.
  const webHost = process.env.WEB_HOST || 'localhost';

  return {
    envDir: rootDir,
    plugins: [vue(), tailwindcss(), htmlEnv(env, PUBLIC_ENV_KEYS)],
    define: Object.fromEntries(
      PUBLIC_ENV_KEYS.map((key) => [`import.meta.env.${key}`, JSON.stringify(env[key])]),
    ),
    resolve: {
      alias: { '@': fileURLToPath(new URL('./src', import.meta.url)) },
    },
    server: {
      host: webHost,
      port: webPort,
      strictPort: true,
      proxy,
      // Runtime data (SQLite database, WAL/SHM files) must never trigger a reload.
      watch: {
        ignored: [`${rootDir}data/**`, `${rootDir}target/**`, '**/*.sqlite', '**/*.sqlite-*'],
      },
    },
    preview: {
      port: webPort,
      strictPort: true,
      proxy,
    },
    build: {
      outDir: 'dist',
      emptyOutDir: true,
    },
  };
});
