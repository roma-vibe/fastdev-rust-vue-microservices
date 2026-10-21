/** Shared helpers of the project scripts: the root folder, `.env` and `services.json`. */
import { existsSync, readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { parseEnv } from 'node:util';

export const rootDir = resolve(import.meta.dirname, '../..');

export interface Service {
  name: string;
  package: string;
  portEnv: string;
  defaultPort: number;
}

/** Every service of `services.json`, in file order. */
export function readServices(): Service[] {
  const registry = JSON.parse(readFileSync(join(rootDir, 'services.json'), 'utf8')) as {
    services: Service[];
  };
  return registry.services;
}

/** Stops the script with a hint when `.env` is missing. */
export function requireEnvFile(): void {
  if (!existsSync(join(rootDir, '.env'))) {
    console.error('Missing .env in the project root. Create it with: npm run setup');
    process.exit(1);
  }
}

/** Values of the root `.env` (read like dotenv, no `$VAR` expansion); the environment wins. */
export function readEnv(): Record<string, string | undefined> {
  const file = join(rootDir, '.env');
  const values = existsSync(file) ? parseEnv(readFileSync(file, 'utf8')) : {};
  return { ...values, ...process.env };
}
