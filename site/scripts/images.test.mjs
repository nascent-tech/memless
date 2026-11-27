import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { describe, it } from 'node:test';
import sharp from 'sharp';

const siteRoot = new URL('../', import.meta.url);
const svgLogos = ['src/assets/memless-logo.svg', 'src/assets/memless-logo-light.svg', 'public/favicon.svg'];

function isAtLeast(version, minimum) {
	const [major, minor, patch] = version.split('.').map(Number);
	const [minMajor, minMinor, minPatch] = minimum.split('.').map(Number);
	if (major !== minMajor) return major > minMajor;
	if (minor !== minMinor) return minor > minMinor;
	return patch >= minPatch;
}

describe('build-time image optimisation', () => {
	// 0.35.5 ships the librsvg fix for GHSA-wq5f-xc86-pv6w.
	it('uses a sharp release with the patched librsvg', () => {
		const version = sharp.versions.sharp;
		assert.ok(isAtLeast(version, '0.35.5'), `sharp ${version} is older than 0.35.5`);
	});

	for (const logo of svgLogos) {
		it(`rasterises ${logo} to a non-empty PNG`, async () => {
			const svg = await readFile(new URL(logo, siteRoot));
			const { data, info } = await sharp(svg).png().toBuffer({ resolveWithObject: true });
			assert.equal(info.format, 'png');
			assert.ok(info.width > 0 && info.height > 0);
			assert.ok(data.length > 0);
		});
	}
});
