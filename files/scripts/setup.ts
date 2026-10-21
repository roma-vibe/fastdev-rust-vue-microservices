/**
 * `npm run setup [-- --port-base 9000]`: creates .env from .env.example with a fresh MASTER_KEY.
 * Keeps an existing .env (only fills an empty MASTER_KEY). With --port-base N the ports become
 * N, N+1, … in services.json order and WEB_PORT N+90, so several copies of the project (git
 * worktrees) can run side by side.
 */
import { randomBytes } from 'node:crypto';
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { parseArgs } from 'node:util';
import { readServices, rootDir } from './lib/project.ts';

const envPath = join(rootDir, '.env');
const { values } = parseArgs({ options: { 'port-base': { type: 'string' } } });

let text = readFileSync(existsSync(envPath) ? envPath : join(rootDir, '.env.example'), 'utf8');

function setVar(key: string, value: string): void {
  const line = new RegExp(`^${key}=.*$`, 'm');
  text = line.test(text)
    ? text.replace(line, `${key}=${value}`)
    : `${text.trimEnd()}\n${key}=${value}\n`;
}

if (!/^MASTER_KEY=\S+/m.test(text)) {
  setVar('MASTER_KEY', randomBytes(32).toString('hex'));
  console.log('MASTER_KEY generated');
}

if (values['port-base'] !== undefined) {
  const base = Number(values['port-base']);
  if (!Number.isInteger(base) || base < 1024 || base > 65000) {
    throw new Error('--port-base must be an integer 1024..65000');
  }
  const services = readServices();
  services.forEach((service, index) => setVar(service.portEnv, String(base + index)));
  setVar('WEB_PORT', String(base + 90));
  console.log(`Ports: services ${base}..${base + services.length - 1}, web ${base + 90}`);
}

writeFileSync(envPath, text);
console.log(`Wrote ${envPath}`);
