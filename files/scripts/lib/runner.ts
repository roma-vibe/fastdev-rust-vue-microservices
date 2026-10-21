/**
 * Runs several long-lived processes with prefixed, colored output. A process that exits on its
 * own is restarted after a pause; `restart` replaces the processes of one kind (e.g. after a
 * rebuild); SIGINT/SIGTERM stop everything, waiting up to 5 s before SIGKILL.
 */
import { type ChildProcess, spawn } from 'node:child_process';
import { createInterface } from 'node:readline';
import { styleText } from 'node:util';

type Color = 'cyan' | 'magenta' | 'yellow' | 'green' | 'blue' | 'red' | 'gray' | 'white';
const COLORS: Color[] = ['cyan', 'yellow', 'green', 'blue', 'magenta', 'red', 'white', 'gray'];

export interface Task {
  name: string;
  command: string;
  args: string[];
  cwd: string;
  /** Group for `restart` (e.g. `rust`, `web`). */
  kind: string;
}

export class Runner {
  private readonly children = new Map<ChildProcess, Task>();
  private readonly restarting = new Set<ChildProcess>();
  private readonly colors = new Map<string, Color>();
  private stopping = false;
  private width = 0;

  constructor() {
    process.on('SIGINT', () => this.stop('SIGINT'));
    process.on('SIGTERM', () => this.stop('SIGTERM'));
    process.on('SIGHUP', () => this.stop('SIGTERM'));
  }

  get isStopping(): boolean {
    return this.stopping;
  }

  /** Starts every task; call once with all of them so the prefixes line up. */
  startAll(tasks: Task[]): void {
    this.width = Math.max(...tasks.map((task) => task.name.length));
    tasks.forEach((task, index) =>
      this.colors.set(task.name, COLORS[index % COLORS.length] ?? 'white'),
    );
    for (const task of tasks) this.start(task);
  }

  /** Stops the running processes of `kind`; each is started again as soon as it exits. */
  restart(kind: string): void {
    for (const [child, task] of this.children) {
      if (task.kind !== kind || this.restarting.has(child)) continue;
      this.restarting.add(child);
      child.kill('SIGTERM');
    }
  }

  private prefix(task: Task): string {
    const color = this.colors.get(task.name) ?? 'white';
    return styleText(color, `[${task.name.padEnd(this.width)}]`, { stream: process.stdout });
  }

  private start(task: Task): void {
    const child = spawn(task.command, task.args, {
      cwd: task.cwd,
      env: { ...process.env, ...(process.stdout.isTTY ? { FORCE_COLOR: '1' } : {}) },
      stdio: ['ignore', 'pipe', 'pipe'],
    });
    this.children.set(child, task);
    const prefix = this.prefix(task);
    const pipe = (stream: NodeJS.ReadableStream, out: NodeJS.WriteStream) =>
      createInterface({ input: stream, crlfDelay: Infinity }).on('line', (line) =>
        out.write(`${prefix} ${line}\n`),
      );
    pipe(child.stdout, process.stdout);
    pipe(child.stderr, process.stderr);
    child.on('error', (error) => console.error(`${prefix} ${error.message}`));
    child.on('close', (code, signal) => this.onClose(child, task, code, signal));
  }

  private onClose(child: ChildProcess, task: Task, code: number | null, signal: string | null) {
    this.children.delete(child);
    if (this.stopping) {
      if (this.children.size === 0) process.exit(process.exitCode ?? 0);
    } else if (this.restarting.delete(child)) {
      this.start(task);
    } else {
      console.log(`${this.prefix(task)} exited (${signal ?? `code ${code}`}), restarting in 3 s`);
      setTimeout(() => {
        if (!this.stopping) this.start(task);
      }, 3_000);
    }
  }

  private stop(signal: NodeJS.Signals): void {
    if (this.stopping) return;
    this.stopping = true;
    if (this.children.size === 0) process.exit(0);
    for (const child of this.children.keys()) child.kill(signal);
    setTimeout(() => {
      for (const child of this.children.keys()) child.kill('SIGKILL');
      process.exit(0);
    }, 5_000).unref();
  }
}
