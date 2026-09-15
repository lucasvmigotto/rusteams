import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

// Docs version tracks the real project version — source of truth is the
// workspace Cargo.toml, injected at build time, never hand-maintained.
const root = dirname(fileURLToPath(import.meta.url));
const cargo = readFileSync(resolve(root, "../Cargo.toml"), "utf8");
const match = cargo.match(/^version\s*=\s*"([^"]+)"/m);
if (!match?.[1]) {
  throw new Error("Could not extract version from Cargo.toml");
}
const RUSTEAMS_VERSION: string = match[1];

export default defineConfig({
  plugins: [tailwindcss(), react()],
  resolve: {
    alias: {
      "@": resolve(root, "src"),
    },
  },
  define: {
    __RUSTEAMS_VERSION__: JSON.stringify(RUSTEAMS_VERSION),
  },
});
