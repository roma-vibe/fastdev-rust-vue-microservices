/**
 * `npm run dev`: builds the Rust workspace, starts every service of services.json and the Vite
 * dev server with prefixed output. When Rust sources, migrations, services.json or .env change it
 * rebuilds and restarts the services (the web app hot-reloads by itself).
 *
 *   npm run dev                         all services + web
 *   npm run dev -- --only notes,edge    a subset (+ web)
 *   npm run dev -- --no-web             services only
 */
import { spawnSync } from 'node:child_process';
import { watch } from 'node:fs';
import { createRequire } from 'node:module';
import { dirname, join } from 'node:path';
import { parseArgs, styleText } from 'node:util';
import { readServices, requireEnvFile, rootDir } from './lib/project.ts';
import { Runner, type Task } from './lib/runner.ts';

const { values } = parseArgs({
  options: { only: { type: 'string' }, 'no-web': { type: 'boolean', default: false } },
});
requireEnvFile();

const only = values.only?.split(',').map((name) => name.trim());
const services = readServices().filter((service) => !only || only.includes(service.name));

function build(): boolean {
  const packages = services.flatMap((service) => ['-p', service.package]);
  return (
    spawnSync('cargo', ['build', ...packages], { cwd: rootDir, stdio: 'inherit' }).status === 0
  );
}

const tasks: Task[] = services.map((service) => ({
  name: service.name,
  command: join(rootDir, 'target/debug', service.package),
  args: [],
  cwd: rootDir,
  kind: 'rust',
}));
if (!values['no-web']) {
  const vitePackage = createRequire(join(rootDir, 'web/package.json')).resolve('vite/package.json');
  tasks.push({
    name: 'web',
    command: process.execPath,
    args: [join(dirname(vitePackage), 'bin/vite.js')],
    cwd: join(rootDir, 'web'),
    kind: 'web',
  });
}

/** Rebuilds and restarts the services when Rust sources, migrations, services.json or .env change. */
function watchRust(runner: Runner): void {
  let timer: NodeJS.Timeout | undefined;
  const schedule = (file: string | null) => {
    if (runner.isStopping || !file || file.includes('target')) return;
    if (!/\.(rs|toml|sql|json)$|^\.env$/.test(file)) return;
    clearTimeout(timer);
    timer = setTimeout(() => {
      console.log(styleText('gray', `change in ${file}: rebuilding…`));
      if (build()) runner.restart('rust');
      else console.log(styleText('red', 'build failed: services keep running the previous build'));
    }, 400);
  };
  for (const dir of ['crates', 'services', 'migrations']) {
    watch(join(rootDir, dir), { recursive: true }, (_event, file) => schedule(file));
  }
  watch(rootDir, (_event, file) => {
    if (file === 'services.json' || file === '.env' || file === 'Cargo.toml') schedule(file);
  });
}

if (!build()) {
  console.error('cargo build failed');
  process.exit(1);
}
const runner = new Runner();
runner.startAll(tasks);
watchRust(runner);
