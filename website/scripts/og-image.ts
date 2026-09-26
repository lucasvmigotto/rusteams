import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import sharp from "sharp";

// Source of truth is scripts/og-image.svg; the built site (and dist/)
// reference only the generated public/og-image.png.
const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

await sharp(resolve(root, "scripts/og-image.svg"))
  .resize(1200, 630)
  .png()
  .toFile(resolve(root, "public/og-image.png"));
console.log("og-image.png generated");
