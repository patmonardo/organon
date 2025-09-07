import { defineConfig } from 'vitest/config'

export default defineConfig({
  test: {
    dir: 'test',
    include: ['**/*.test.ts', '**/*.spec.ts'],
    environment: 'node',
    globals: true,
    passWithNoTests: false,
    coverage: {
      provider: 'v8',
      reporter: ['text', 'lcov'],
    },
    // allow TS paths if used; uses project's tsconfig
    deps: {
      inline: ['zod'],
    },
  },
})
