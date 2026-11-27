import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { test } from "node:test";

// GHSA-587h-7wmj-h5pm (and the other launch-editor advisories bundled in vite)
// affect vite 8.3.0 to 8.3.2, which runs the `astro dev` server.
const FIRST_FIXED_VITE = [8, 3, 3];

function viteVersionUsedByAstro() {
  const requireFromAstro = createRequire(
    createRequire(import.meta.url).resolve("astro/package.json"),
  );
  const manifest = requireFromAstro.resolve("vite/package.json");
  return JSON.parse(readFileSync(manifest, "utf8")).version;
}

function isAtLeast(version, minimum) {
  const parts = version.split(/[.-]/).slice(0, 3).map(Number);
  const firstDifference = parts.findIndex((part, i) => part !== minimum[i]);
  return firstDifference === -1 || parts[firstDifference] > minimum[firstDifference];
}

test("the vite used by astro includes the launch-editor fixes", () => {
  const version = viteVersionUsedByAstro();

  assert.ok(
    isAtLeast(version, FIRST_FIXED_VITE),
    `vite ${version} is affected by GHSA-587h-7wmj-h5pm, need >= ${FIRST_FIXED_VITE.join(".")}`,
  );
});
