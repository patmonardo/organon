import { defineConfig } from "vitest/config";
import { resolve } from "node:path";

export default defineConfig({
  test: {
    environment: "node",
    include: ["test/**/*.test.ts", "test/**/*.spec.ts", "src/**/*.spec.ts"],
    exclude: ["node_modules", "dist", "test/form/morph"],
    globals: true
  },
  resolve: {
    alias: {
      "@organon/logic": resolve(__dirname, "src/index.ts"),
      "@organon/logic/schema": resolve(__dirname, "src/schema/index.ts"),
      "@organon/logic/repository": resolve(__dirname, "src/repository/index.ts")
    }
  }
});
