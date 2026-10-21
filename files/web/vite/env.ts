/**
 * Build-time helpers of vite.config.ts: reading the root .env and putting public values into
 * index.html. They run in Node (in the dev server and the build), never in the browser.
 */
import { existsSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { parseEnv } from 'node:util';
import type { Plugin } from 'vite';

export type Env = Record<string, string | undefined>;

/**
 * Reads `<rootDir>/.env` with Node's built-in dotenv parser, the one the server uses
 * (`process.loadEnvFile`), so both sides see exactly the same values: quoted values are taken
 * literally and `$VAR` is never expanded (Vite's `loadEnv` would expand it, even in single quotes).
 * Variables already set in the environment win, as on the server: the Docker build gets APP_NAME
 * as a build argument (the image has no .env), the dev container gets .env through `env_file`.
 */
export function readEnv(rootDir: string, environment: Env = process.env): Env {
  const file = join(rootDir, '.env');
  const values: Env = existsSync(file) ? parseEnv(readFileSync(file, 'utf8')) : {};
  return { ...values, ...environment };
}

/** Vite plugin: replaces `%KEY%` placeholders of `keys` in index.html (HTML-escaped). */
export function htmlEnv(env: Env, keys: readonly string[]): Plugin {
  return {
    name: 'html-public-env',
    transformIndexHtml: {
      order: 'pre',
      handler: (html) => replacePlaceholders(html, env, keys),
    },
  };
}

/** Replaces `%KEY%` with the HTML-escaped value of every key in `keys` (missing keys: empty). */
export function replacePlaceholders(html: string, env: Env, keys: readonly string[]): string {
  return keys.reduce(
    // A replacer function inserts the value as is: in a replacement string, `$&`, `$'` or `$$`
    // in a value would be read as patterns.
    (result, key) => result.replaceAll(`%${key}%`, () => escapeHtml(env[key] ?? '')),
    html,
  );
}

function escapeHtml(value: string): string {
  return value.replace(/[&<>"']/g, (char) => `&#${char.charCodeAt(0)};`);
}
