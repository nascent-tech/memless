import assert from 'node:assert/strict';
import { createServer } from 'node:http';
import { createRequire } from 'node:module';
import { after, before, describe, it } from 'node:test';

// unifont, astro's remote font loader, is the only package that ships undici in
// the site, so the test loads the copy that unifont itself resolves.
const requireFromUnifont = createRequire(new URL('../node_modules/unifont/package.json', import.meta.url));
const { Agent, interceptors, request } = requireFromUnifont('undici');

function listen(handler) {
	return new Promise((resolve) => {
		const server = createServer(handler).listen(0, '127.0.0.1', () => resolve(server));
	});
}

function urlOf(server) {
	return `http://127.0.0.1:${server.address().port}`;
}

async function getText(url, dispatcher) {
	const response = await request(url, { dispatcher });

	return response.body.text();
}

describe('undici redirect and cache interceptors (GHSA-4w7w-5xrc-9h26)', () => {
	let otherOrigin;
	let requestedOrigin;
	let requestedOriginRedirects = true;

	before(async () => {
		otherOrigin = await listen((_request, response) => {
			response.writeHead(200, { 'cache-control': 'public, max-age=60' });
			response.end('body from the other origin');
		});
		requestedOrigin = await listen((_request, response) => {
			if (requestedOriginRedirects) {
				response.writeHead(302, { location: `${urlOf(otherOrigin)}/data` });
				response.end();

				return;
			}
			response.writeHead(200);
			response.end('body from the requested origin');
		});
	});

	after(() => {
		requestedOrigin.close();
		otherOrigin.close();
	});

	it('does not serve a cross-origin redirect target from the cache of the requested url', async () => {
		const agent = new Agent().compose(interceptors.redirect({ maxRedirections: 2 }), interceptors.cache());
		const url = `${urlOf(requestedOrigin)}/page`;
		await getText(url, agent);
		requestedOriginRedirects = false;

		const body = await getText(url, agent);

		assert.equal(body, 'body from the requested origin');
		await agent.close();
	});
});
