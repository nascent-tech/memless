import { readdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { toMarkdown } from './html-to-markdown.mjs';

const dist = fileURLToPath(new URL('../dist/', import.meta.url));

async function* pages(dir) {
	for (const entry of await readdir(dir, { withFileTypes: true })) {
		const path = join(dir, entry.name);
		if (entry.isDirectory()) yield* pages(path);
		else if (entry.name === 'index.html') yield path;
	}
}

async function writeMarkdownTwin(page) {
	const markdown = await toMarkdown(await readFile(page, 'utf8'));
	await writeFile(page.replace(/index\.html$/, 'index.md'), markdown);
}

await Promise.all((await Array.fromAsync(pages(dist))).map(writeMarkdownTwin));
