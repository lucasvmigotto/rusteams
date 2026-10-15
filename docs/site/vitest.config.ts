import { resolve } from "node:path";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vitest/config";

export default defineConfig({
  plugins: [react()],
  resolve: { alias: { "@": resolve(import.meta.dirname, "src") } },
  define: {
    __RUSTEAMS_VERSION__: JSON.stringify("0.0.0-test"),
    __VERSIONS__: JSON.stringify([
      {
        name: "rusteams",
        version: "0.2.0",
        label: "rusteams@v0.2.0",
        href: "https://github.com/lucasvmigotto/rusteams/tree/0.2.0",
      },
      {
        name: "docs",
        version: "0.2.0",
        label: "docs@v0.2.0",
        href: "https://github.com/lucasvmigotto/rusteams/tree/0.2.0/docs/site",
      },
    ]),
  },
  test: {
    environment: "jsdom",
    globals: true,
    setupFiles: ["./vitest.setup.ts"],
    include: ["tests/unit/**/*.test.{ts,tsx}", "tests/a11y/**/*.test.{ts,tsx}"],
  },
});
