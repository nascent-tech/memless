import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { estimateTokens, markdownPath, prefersMarkdown } from './negotiation.js';

describe('prefersMarkdown', () => {
	it('serves markdown to an agent that asks only for markdown', () => {
		assert.equal(prefersMarkdown('text/markdown'), true);
	});

	it('serves markdown when it is weighted above html', () => {
		assert.equal(prefersMarkdown('text/markdown, text/html;q=0.9'), true);
	});

	it('keeps html for a browser that does not mention markdown', () => {
		assert.equal(prefersMarkdown('text/html,application/xhtml+xml,*/*;q=0.8'), false);
	});

	it('keeps html when markdown is weighted below it', () => {
		assert.equal(prefersMarkdown('text/html, text/markdown;q=0.5'), false);
	});

	it('keeps html when markdown is refused with q=0', () => {
		assert.equal(prefersMarkdown('text/markdown;q=0'), false);
	});

	it('keeps html when there is no accept header', () => {
		assert.equal(prefersMarkdown(), false);
	});

	it('reads media types and the q parameter without regard to case or spacing', () => {
		assert.equal(prefersMarkdown(' TEXT/MARKDOWN ; Q=0.9 , text/html ; q=0.5'), true);
	});

	it('does not count wildcards as a request for markdown', () => {
		assert.equal(prefersMarkdown('*/*'), false);
		assert.equal(prefersMarkdown('text/*'), false);
	});

	it('serves markdown when an agent names it, even next to a wildcard', () => {
		assert.equal(prefersMarkdown('text/markdown;q=0.1, */*'), true);
	});

	it('serves markdown when it ties with html', () => {
		assert.equal(prefersMarkdown('text/html;q=0.5, text/markdown;q=0.5'), true);
	});

	it('keeps html when the q value is not a number', () => {
		assert.equal(prefersMarkdown('text/markdown;q=abc'), false);
	});
});

describe('markdownPath', () => {
	it('maps the home page to its markdown twin', () => {
		assert.equal(markdownPath('/'), '/index.md');
	});

	it('maps a page with a trailing slash', () => {
		assert.equal(markdownPath('/reference/sql/'), '/reference/sql/index.md');
	});

	it('leaves a page without a trailing slash to the origin redirect', () => {
		assert.equal(markdownPath('/reference/sql'), null);
	});

	it('leaves files with an extension alone', () => {
		assert.equal(markdownPath('/llms.txt'), null);
		assert.equal(markdownPath('/_astro/docs.css'), null);
	});
});

describe('estimateTokens', () => {
	it('counts about one token per four characters', () => {
		assert.equal(estimateTokens('12345678'), 2);
		assert.equal(estimateTokens('123456789'), 3);
	});
});
