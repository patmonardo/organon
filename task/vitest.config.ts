import { defineConfig } from 'vitest/config';

export default defineConfig({
  resolve: {
    alias: {
      '@schema': '/home/pat/VSCode/organon/logic/src/schema/index.ts',
      '@schema/*': '/home/pat/VSCode/organon/logic/src/schema/*',
      '@repository': '/home/pat/VSCode/organon/logic/src/repository/index.ts',
      '@repository/*': '/home/pat/VSCode/organon/logic/src/repository/*',
      '@relative': '/home/pat/VSCode/organon/logic/src/relative/index.ts',
      '@relative/*': '/home/pat/VSCode/organon/logic/src/relative/*',
      '@absolute': '/home/pat/VSCode/organon/logic/src/absolute/index.ts',
      '@absolute/*': '/home/pat/VSCode/organon/logic/src/absolute/*',
    },
  },
  test: {
    environment: 'node',
    include: ['test/**/*.test.ts', 'test/**/*.spec.ts', 'src/**/*.spec.ts'],
    exclude: [
      'node_modules',
      'dist',
    ],
    globals: true,
  },
});
