import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { dirname, join } from 'node:path';
import { test } from 'node:test';

// GHSA-mrhx-c7xc-m3w5: zod before 4.6.6 can exhaust memory on oversized
// constrained collections. zod reaches the site through astro and
// @astrojs/sitemap, which validate the docs frontmatter at build time.
const FIRST_FIXED = [4, 6, 6];

const requireFrom = (fromFile) => createRequire(fromFile);

const packageDirectory = (requireFn, name) => {
	const entry = requireFn.resolve(name);
	let directory = dirname(entry);
	while (!directory.endsWith(join('node_modules', name))) {
		directory = dirname(directory);
	}
	return directory;
};

const zodSeenBy = (consumer) => {
	const site = requireFrom(import.meta.url);
	const consumerRequire = requireFrom(join(packageDirectory(site, consumer), 'package.json'));
	const zodDirectory = packageDirectory(consumerRequire, 'zod');
	const { version } = JSON.parse(readFileSync(join(zodDirectory, 'package.json'), 'utf8'));
	return { version, zod: consumerRequire('zod') };
};

const isAtLeast = (version, minimum) => {
	const parts = version.split('-')[0].split('.').map(Number);
	const index = parts.findIndex((part, i) => part !== minimum[i]);
	return index === -1 || parts[index] > minimum[index];
};

for (const consumer of ['astro', '@astrojs/sitemap']) {
	test(`${consumer} loads a zod release with the memory exhaustion fix`, () => {
		const { version } = zodSeenBy(consumer);

		assert.ok(isAtLeast(version, FIRST_FIXED), `${consumer} resolves zod ${version}`);
	});

	test(`${consumer}'s zod still enforces collection bounds`, () => {
		const { zod } = zodSeenBy(consumer);
		const tags = zod.z.array(zod.z.string()).max(3);

		assert.deepEqual(tags.parse(['a', 'b']), ['a', 'b']);
		assert.equal(tags.safeParse(Array.from({ length: 100_000 }, () => 'x')).success, false);
	});
}
