import { defineConfig } from 'tsup';

export default defineConfig({
  entry: ['src/cli-entry.ts'],
  format: ['esm'],
  platform: 'node',
  target: 'node20',
  bundle: true,
  sourcemap: true,
  clean: true,
  outDir: 'dist',
  outExtension: () => ({ js: '.js' }),
  banner: {
    js: '#!/usr/bin/env node',
  },
  esbuildOptions(options) {
    options.jsx = 'automatic';
  },
});
