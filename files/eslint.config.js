import js from '@eslint/js';
import { defineConfig, globalIgnores } from 'eslint/config';
import prettier from 'eslint-config-prettier/flat';
import vue from 'eslint-plugin-vue';
import globals from 'globals';
import tseslint from 'typescript-eslint';

export default defineConfig(
  globalIgnores(['**/dist/', '**/coverage/', 'data/', 'target/']),

  js.configs.recommended,
  tseslint.configs.strict,
  tseslint.configs.stylistic,
  vue.configs['flat/recommended'],

  {
    files: ['**/*.vue'],
    languageOptions: {
      parserOptions: { parser: tseslint.parser },
    },
  },
  {
    files: ['web/src/**/*.{ts,vue}', 'web/test/**/*.ts'],
    languageOptions: { globals: globals.browser },
  },
  {
    files: ['scripts/**/*.ts', 'web/vite/**/*.ts', '**/*.config.{js,ts}'],
    languageOptions: { globals: globals.node },
  },

  {
    rules: {
      '@typescript-eslint/consistent-type-imports': [
        'error',
        { prefer: 'type-imports', fixStyle: 'inline-type-imports' },
      ],
      '@typescript-eslint/no-unused-vars': [
        'error',
        { argsIgnorePattern: '^_', varsIgnorePattern: '^_', caughtErrors: 'none' },
      ],
      eqeqeq: ['error', 'always'],
      'object-shorthand': 'error',
      'prefer-const': 'error',
    },
  },
  {
    files: ['**/*.vue'],
    rules: {
      'vue/block-lang': ['error', { script: { lang: 'ts' } }],
      'vue/component-api-style': ['error', ['script-setup']],
      'vue/define-macros-order': 'error',
      'vue/no-unused-refs': 'error',
      'vue/no-useless-v-bind': 'error',
      'vue/padding-line-between-blocks': 'error',
      // Optional TypeScript props are simply undefined; no runtime default needed.
      'vue/require-default-prop': 'off',
    },
  },
  {
    // Browser code must go through web/src/shared/api/http.ts for HTTP (see docs/frontend.md).
    files: ['web/src/**/*.{ts,vue}'],
    ignores: ['web/src/shared/api/**'],
    rules: {
      'no-restricted-globals': [
        'error',
        { name: 'fetch', message: 'Use http from @/shared/api/http.ts.' },
      ],
      'no-restricted-properties': [
        'error',
        { object: 'window', property: 'fetch', message: 'Use http from @/shared/api/http.ts.' },
      ],
    },
  },

  prettier,
);
