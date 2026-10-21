import { defineConfig } from 'vitest/config';

// Web tests; Rust tests run with `cargo test --workspace` (both: `npm test`).
export default defineConfig({
  test: {
    projects: ['web'],
  },
});
