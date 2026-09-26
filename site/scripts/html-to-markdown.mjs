import { matches, select, selectAll } from 'hast-util-select';
import rehypeParse from 'rehype-parse';
import rehypeRemark from 'rehype-remark';
import remarkGfm from 'remark-gfm';
import remarkStringify from 'remark-stringify';
import { unified } from 'unified';
import { remove } from 'unist-util-remove';

const parser = unified().use(rehypeParse);
const converter = unified().use(rehypeRemark).use(remarkGfm).use(remarkStringify);
const noise = ['script', 'style', 'svg', 'button', 'footer', '.sl-anchor-link', '.sr-only'];
const MAX_DEPTH = 64;

function textOf(node, depth = 0) {
	if (node.type === 'text') return node.value;
	if (depth > MAX_DEPTH) return '';
	return (node.children ?? []).map((child) => textOf(child, depth + 1)).join('');
}

function stripNoise(tree) {
	remove(tree, (node) => node.type === 'comment' || noise.some((selector) => matches(selector, node)));
}

function codeText(code) {
	return selectAll('.ec-line', code)
		.map((line) => textOf(line).replace(/\n+$/, ''))
		.join('\n');
}

function normalizeCodeBlocks(tree) {
	for (const pre of selectAll('.expressive-code pre[data-language]', tree)) {
		const code = select('code', pre);
		if (!code) continue;
		code.properties.className = [`language-${pre.properties.dataLanguage}`];
		code.children = [{ type: 'text', value: codeText(code) }];
	}
}

function tabEntry(tab, panel) {
	const label = { type: 'element', tagName: 'p', properties: {}, children: [{ type: 'text', value: textOf(tab).trim() }] };
	return { type: 'element', tagName: 'li', properties: {}, children: [label, panel] };
}

function flattenTabs(tree) {
	for (const tabs of selectAll('starlight-tabs', tree)) {
		const labels = selectAll('[role="tab"]', tabs);
		const panels = selectAll('[role="tabpanel"]', tabs);
		tabs.tagName = 'ul';
		tabs.properties = {};
		tabs.children = labels.map((tab, index) => tabEntry(tab, panels[index])).filter((entry) => entry.children[1]);
	}
}

async function contentMarkdown(content) {
	stripNoise(content);
	normalizeCodeBlocks(content);
	flattenTabs(content);
	const mdast = await converter.run({ type: 'root', children: [content] });
	return converter.stringify(mdast).trim();
}

export async function toMarkdown(html) {
	const page = parser.parse(html);
	const title = textOf(select('h1#_top', page) ?? { type: 'text', value: '' }).trim();
	const description = select('meta[name="description"]', page)?.properties.content ?? '';
	const content = select('.sl-markdown-content', page);
	const body = content ? await contentMarkdown(content) : '';
	return [`# ${title}`, description, body].filter(Boolean).join('\n\n') + '\n';
}
