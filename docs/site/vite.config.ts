import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import { footerVersions } from "./scripts/footer-versions";

// Docs version tracks the real project version — source of truth is the
// workspace Cargo.toml, injected at build time, never hand-maintained.
const root = dirname(fileURLToPath(import.meta.url));
const cargo = readFileSync(resolve(root, "../../Cargo.toml"), "utf8");
const match = cargo.match(/^version\s*=\s*"([^"]+)"/m);
if (!match?.[1]) {
  throw new Error("Could not extract version from Cargo.toml");
}
const RUSTEAMS_VERSION: string = match[1];

function git(...args: string[]): string {
  try {
    return execFileSync("git", ["-C", resolve(root, "../.."), ...args], {
      encoding: "utf8",
      stdio: ["ignore", "pipe", "ignore"],
    }).trim();
  } catch {
    return "";
  }
}

// The footer's versions, each linked to the tree this build came from.
const VERSIONS = footerVersions({
  repoUrl: "https://github.com/lucasvmigotto/rusteams",
  name: "rusteams",
  docsVersion: JSON.parse(readFileSync(resolve(root, "package.json"), "utf8")).version,
  docsPath: "docs/site",
  git: {
    exactTag: git("describe", "--tags", "--exact-match", "--match", RUSTEAMS_VERSION),
    lastTag: RUSTEAMS_VERSION,
    sha: git("rev-parse", "HEAD"),
  },
});

export default defineConfig({
  // Deployed under the docs-hub prefix /<repo>/ (ADR 0001 in lucas/docs).
  // CI sets VITE_BASE_PATH from github.event.repository.name; the local
  // default keeps `vite preview` at the same path as production.
  base: process.env["VITE_BASE_PATH"] ?? "/rusteams/",
  plugins: [tailwindcss(), react()],
  resolve: {
    alias: {
      "@": resolve(root, "src"),
    },
  },
  define: {
    __RUSTEAMS_VERSION__: JSON.stringify(RUSTEAMS_VERSION),
    __VERSIONS__: JSON.stringify(VERSIONS),
  },
});
