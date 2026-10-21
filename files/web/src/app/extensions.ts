/** Shell extensions of the feature modules (`features/<name>/shell.ts`), collected automatically. */
import type { ShellExtension } from './types.ts';

const modules = import.meta.glob<ShellExtension>('../features/*/shell.ts', {
  eager: true,
  import: 'default',
});

export const extensions: ShellExtension[] = Object.keys(modules)
  .sort()
  .map((path) => modules[path])
  .filter((extension): extension is ShellExtension => extension !== undefined);
