// @vitest-environment node
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { readEnv, replacePlaceholders } from './env.ts';

describe('readEnv', () => {
  const dirs: string[] = [];
  const folderWith = (env?: string): string => {
    const dir = mkdtempSync(join(tmpdir(), 'web-env-'));
    dirs.push(dir);
    if (env !== undefined) writeFileSync(join(dir, '.env'), env);
    return dir;
  };

  afterEach(() => {
    for (const dir of dirs.splice(0)) rmSync(dir, { recursive: true, force: true });
  });

  it('reads .env literally: quoted values as written, $VAR not expanded', () => {
    const dir = folderWith(
      ['# App', `APP_NAME='Test "Shop" $HOME A$&B Тест'`, 'APP_PORT=3000', '', 'EMPTY='].join('\n'),
    );

    expect(readEnv(dir, {})).toEqual({
      APP_NAME: 'Test "Shop" $HOME A$&B Тест',
      APP_PORT: '3000',
      EMPTY: '',
    });
  });

  it('lets variables of the environment win (Docker build argument, env_file)', () => {
    const dir = folderWith(`APP_NAME='From .env'\nAPP_PORT=3000\n`);

    expect(readEnv(dir, { APP_NAME: 'From the environment' })).toEqual({
      APP_NAME: 'From the environment',
      APP_PORT: '3000',
    });
  });

  it('works without .env (the Docker build has none)', () => {
    expect(readEnv(folderWith(), { APP_NAME: 'Docker' })).toEqual({ APP_NAME: 'Docker' });
  });
});

describe('replacePlaceholders', () => {
  it('inserts HTML-escaped values literally', () => {
    const html = '<title>%APP_NAME%</title><p>%APP_NAME%</p>';
    const value = `A$&B $$ $' <"x">`;
    const escaped = 'A$&#38;B $$ $&#39; &#60;&#34;x&#34;&#62;';

    expect(replacePlaceholders(html, { APP_NAME: value }, ['APP_NAME'])).toBe(
      `<title>${escaped}</title><p>${escaped}</p>`,
    );
  });

  it('replaces a key without a value with an empty string', () => {
    expect(replacePlaceholders('<title>%APP_NAME%</title>', {}, ['APP_NAME'])).toBe(
      '<title></title>',
    );
  });
});
