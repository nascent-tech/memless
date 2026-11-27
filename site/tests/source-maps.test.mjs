import { strict as assert } from "node:assert";
import { createRequire } from "node:module";
import { test } from "node:test";

const require = createRequire(import.meta.url);
const requireFromPostcss = createRequire(require.resolve("postcss"));
const postcss = requireFromPostcss("postcss");
const sourceMapJs = requireFromPostcss("source-map-js");

const firstPatchedSourceMapJs = [1, 2, 2];
const keepDeclarations = { postcssPlugin: "keep-declarations", Declaration() {} };

function isAtLeast(version, minimum) {
  const parts = version.split(".").map(Number);
  const firstDifference = parts.findIndex((part, i) => part !== minimum[i]);

  return firstDifference === -1 || parts[firstDifference] > minimum[firstDifference];
}

function indexedSourceMapOf(sourceName) {
  const generator = new sourceMapJs.SourceMapGenerator({ file: "site.css" });

  generator.addMapping({
    generated: { line: 1, column: 0 },
    original: { line: 3, column: 2 },
    source: sourceName,
  });
  generator.setSourceContent(sourceName, "\n\n  a { color: red }\n");

  return {
    version: 3,
    file: "site.css",
    sections: [{ offset: { line: 0, column: 0 }, map: generator.toJSON() }],
  };
}

test("the CSS build uses a source-map-js release patched against GHSA-68fv-2mgg-jv7q", () => {
  const { version } = requireFromPostcss("source-map-js/package.json");

  assert.ok(isAtLeast(version, firstPatchedSourceMapJs), `source-map-js ${version} is vulnerable`);
});

test("postcss still maps the generated CSS back through an indexed previous source map", async () => {
  const css = "a { color: red }";
  const previousMap = indexedSourceMapOf("theme.scss");

  const result = await postcss([keepDeclarations]).process(css, {
    from: "site.css",
    to: "site.out.css",
    map: { prev: previousMap, inline: false, annotation: false },
  });

  const consumer = new sourceMapJs.SourceMapConsumer(result.map.toJSON());
  const colorDeclaration = consumer.originalPositionFor({ line: 1, column: 4 });
  assert.equal(colorDeclaration.source, "theme.scss");
  assert.equal(colorDeclaration.line, 3);
});
