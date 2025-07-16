import { defineConfig } from 'vitest/config';
import { resolve } from 'path';

export default defineConfig({
  test: {
    globals: true,
    environment: 'node',
    include: ['**/*.{test,spec,tool}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
    exclude: ['./node_modules/**', '**/src/core/compression/packed/**'],
    typecheck: {
      enabled: true,
      tsconfig: './tsconfig.json',
    }
  },
  resolve: {
    alias: {
      '@': resolve(__dirname, './src')
    }
  }
});
