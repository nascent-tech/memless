const MARKDOWN = 'text/markdown';
const HTML = 'text/html';

function quality(accept, type) {
	const range = accept
		.split(',')
		.map((part) => part.split(';').map((piece) => piece.trim().toLowerCase()))
		.find(([media]) => media === type);
	if (!range) return 0;
	const weight = range.slice(1).find((param) => param.startsWith('q='));
	return weight ? Number(weight.slice(2)) : 1;
}

export function prefersMarkdown(accept = '') {
	const markdown = quality(accept, MARKDOWN);
	return markdown > 0 && markdown >= quality(accept, HTML);
}

export function markdownPath(pathname) {
	return pathname.endsWith('/') ? `${pathname}index.md` : null;
}

export function estimateTokens(text) {
	return Math.ceil(text.length / 4);
}
