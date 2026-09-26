import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

/**
 * Social preview contract: crawlers read the static index.html (no JS run),
 * so the Open Graph / Twitter image and URL tags must be absolute https://
 * values. Vite substitutes the `%VITE_APP_SITE_NAME%` placeholder at build
 * time; the deploy workflow additionally asserts that the built dist contains
 * no leftover placeholder.
 */
const SITE_ROOT = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const INDEX_HTML = readFileSync(join(SITE_ROOT, "index.html"), "utf-8");

function hasTag(attr: string, name: string): boolean {
  return INDEX_HTML.includes(`${attr}="${name}"`);
}

describe("social preview tags", () => {
  it("declares the Open Graph basics", () => {
    for (const property of [
      "og:type",
      "og:site_name",
      "og:title",
      "og:description",
      "og:url",
      "og:image",
    ]) {
      expect(hasTag("property", property)).toBe(true);
    }
  });

  it("uses the build-time base URL for the absolute image and url", () => {
    expect(INDEX_HTML).toContain('property="og:image" content="%VITE_APP_SITE_NAME%/og-image.png"');
    expect(INDEX_HTML).toContain('property="og:url" content="%VITE_APP_SITE_NAME%/"');
    expect(INDEX_HTML).toContain(
      'name="twitter:image" content="%VITE_APP_SITE_NAME%/og-image.png"',
    );
  });

  it("declares a large-image Twitter card", () => {
    for (const name of ["twitter:card", "twitter:title", "twitter:description", "twitter:image"]) {
      expect(hasTag("name", name)).toBe(true);
    }
    expect(INDEX_HTML).toContain('content="summary_large_image"');
  });

  it("pins the image dimensions for layout stability", () => {
    expect(INDEX_HTML).toContain('property="og:image:width" content="1200"');
    expect(INDEX_HTML).toContain('property="og:image:height" content="630"');
  });

  it("links a favicon that exists in public/", () => {
    expect(INDEX_HTML).toContain('rel="icon"');
    expect(INDEX_HTML).toContain('href="/favicon.svg"');
    expect(existsSync(join(SITE_ROOT, "public", "favicon.svg"))).toBe(true);
  });
});
