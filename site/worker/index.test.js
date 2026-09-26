import assert from 'node:assert/strict';
import { afterEach, beforeEach, describe, it } from 'node:test';
import worker from './index.js';

const ORIGIN = {
	'/reference/sql/': { body: '<h1>Supported SQL</h1>', type: 'text/html' },
	'/reference/sql/index.md': { body: '# Supported SQL\n', type: 'text/markdown' },
	'/guides/draft/': { body: '<h1>Draft</h1>', type: 'text/html' },
	'/llms.txt': { body: '# Memless\n', type: 'text/plain' },
	'/reference/sql': { status: 301, location: '/reference/sql/' },
};

const realFetch = globalThis.fetch;

function fakeOrigin(input) {
	const { pathname } = new URL(input instanceof Request ? input.url : input);
	const page = ORIGIN[pathname];
	if (!page) return new Response('not found', { status: 404 });
	if (page.status) return new Response(null, { status: page.status, headers: { location: page.location } });
	return new Response(page.body, { headers: { 'content-type': page.type, etag: '"v1"' } });
}

function get(path, accept, method = 'GET') {
	return worker.fetch(new Request(`https://memless.nascent-tech.co${path}`, { method, headers: { accept } }));
}

describe('the markdown negotiation worker', () => {
	beforeEach(() => {
		globalThis.fetch = async (input) => fakeOrigin(input);
	});

	afterEach(() => {
		globalThis.fetch = realFetch;
	});

	it('answers an agent asking for markdown with the markdown twin of the page', async () => {
		const response = await get('/reference/sql/', 'text/markdown');
		assert.equal(response.headers.get('content-type'), 'text/markdown; charset=utf-8');
		assert.equal(response.headers.get('vary'), 'Accept');
		assert.equal(response.headers.get('x-markdown-tokens'), '4');
		assert.equal(await response.text(), '# Supported SQL\n');
	});

	it('answers a browser with the html page and marks it as varying on accept', async () => {
		const response = await get('/reference/sql/', 'text/html,*/*;q=0.8');
		assert.equal(response.headers.get('content-type'), 'text/html');
		assert.match(response.headers.get('vary'), /Accept/);
		assert.equal(await response.text(), '<h1>Supported SQL</h1>');
	});

	it('falls back to html when a page has no markdown twin', async () => {
		const response = await get('/guides/draft/', 'text/markdown');
		assert.equal(response.headers.get('content-type'), 'text/html');
		assert.equal(await response.text(), '<h1>Draft</h1>');
	});

	it('keeps the etag of the markdown twin so an agent can revalidate', async () => {
		const response = await get('/reference/sql/', 'text/markdown');
		assert.equal(response.headers.get('etag'), '"v1"');
	});

	it('answers a HEAD request for markdown with the headers and no body', async () => {
		const response = await get('/reference/sql/', 'text/markdown', 'HEAD');
		assert.equal(response.headers.get('content-type'), 'text/markdown; charset=utf-8');
		assert.equal(await response.text(), '');
	});

	it('relays the origin redirect for a page without a trailing slash', async () => {
		const response = await get('/reference/sql', 'text/markdown');
		assert.equal(response.status, 301);
		assert.equal(response.headers.get('location'), '/reference/sql/');
	});

	it('keeps a missing page missing when an agent asks for markdown', async () => {
		const response = await get('/nowhere/', 'text/markdown');
		assert.equal(response.status, 404);
	});

	it('passes files through untouched, whatever the agent asks for', async () => {
		const response = await get('/llms.txt', 'text/markdown');
		assert.equal(response.headers.get('content-type'), 'text/plain');
		assert.equal(response.headers.get('vary'), null);
	});
});
