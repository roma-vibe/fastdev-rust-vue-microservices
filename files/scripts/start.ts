/**
 * `npm start`: runs the release builds of every service (`npm run build` first). The edge serves
 * the built web app (web/dist) and the API on EDGE_PORT.
 */
import { existsSync } from 'node:fs';
import { join } from 'node:path';
import { readEnv, readServices, requireEnvFile, rootDir } from './lib/project.ts';
import { Runner, type Task } from './lib/runner.ts';

requireEnvFile();
const services = readServices();
const missing = services.filter((s) => !existsSync(join(rootDir, 'target/release', s.package)));
if (missing.length > 0 || !existsSync(join(rootDir, 'web/dist/index.html'))) {
  console.error('Release build not found. Build it first with: npm run build');
  process.exit(1);
}

const tasks: Task[] = services.map((service) => ({
  name: service.name,
  command: join(rootDir, 'target/release', service.package),
  args: [],
  cwd: rootDir,
  kind: 'rust',
}));
const edge = services.find((service) => service.name === 'edge');
const port = edge ? (readEnv()[edge.portEnv] ?? String(edge.defaultPort)) : '';
console.log(`Open http://localhost:${port}`);
new Runner().startAll(tasks);
