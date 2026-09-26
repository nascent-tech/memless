import { estimateTokens, markdownPath, prefersMarkdown } from './negotiation.js';

const NEGOTIABLE = new Set(['GET', 'HEAD']);
const REVALIDATION = ['etag', 'last-modified'];

function markdownHeaders(body, origin) {
	const headers = new Headers({
		'content-type': 'text/markdown; charset=utf-8',
		vary: 'Accept',
		'x-markdown-tokens': String(estimateTokens(body)),
		'cache-control': origin.headers.get('cache-control') ?? 'max-age=600',
	});
	for (const name of REVALIDATION) if (origin.headers.has(name)) headers.set(name, origin.headers.get(name));
	return headers;
}

async function markdownResponse(request, path) {
	const origin = await fetch(new URL(path, request.url));
	if (!origin.ok) return null;
	const body = await origin.text();
	const headers = markdownHeaders(body, origin);
	return new Response(request.method === 'HEAD' ? null : body, { headers });
}

async function htmlResponse(request) {
	const origin = await fetch(request);
	const response = new Response(origin.body, origin);
	response.headers.append('vary', 'Accept');
	return response;
}

function negotiable(request) {
	return NEGOTIABLE.has(request.method) ? markdownPath(new URL(request.url).pathname) : null;
}

export default {
	async fetch(request) {
		const path = negotiable(request);
		if (!path) return fetch(request);
		const markdown = prefersMarkdown(request.headers.get('accept') ?? '') && (await markdownResponse(request, path));
		return markdown || htmlResponse(request);
	},
};
