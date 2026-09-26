// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import starlightLlmsTxt from 'starlight-llms-txt';

const llmsDescription = [
	'Memless loads a YAML file of test data into memory,',
	'runs a deliberate subset of SQL against it from PHP, Go or Node.js,',
	'and writes every accepted change back to the file.',
].join(' ');

const llmsDetails = [
	'- Install it as `@nascent-tech/memless` (npm), `nascent-tech/memless` (Composer)'
		+ ' or `github.com/nascent-tech/memless-go` (Go). Each package bundles the native engine.',
	'- The three languages share one engine: the same file and SQL give the same result'
		+ ' and the same error message.',
	'- SQL outside the documented subset is refused with a message, never approximated.'
		+ ' There is no GROUP BY, LIMIT, LIKE, IN or subquery.',
	'- Types never convert: `2` and `2.0` are different values, and so are `5` and `"5"`.',
	'- It is made for test fixtures and demos, not for production data.',
].join('\n');

export default defineConfig({
	site: 'https://memless.nascent-tech.co',
	integrations: [
		starlight({
			title: 'Memless',
			plugins: [
				starlightLlmsTxt({
					description: llmsDescription,
					details: llmsDetails,
					promote: [
						'index*',
						'getting-started/install*',
						'getting-started/quick-start*',
						'guides/yaml-file*',
						'guides/writes-transactions*',
						'reference/sql*',
						'reference/errors*',
					],
				}),
			],
			components: { Hero: './src/components/landing/Hero.astro' },
			customCss: [
				'@fontsource-variable/instrument-sans',
				'@fontsource-variable/jetbrains-mono',
				'./src/styles/theme.css',
			],
			expressiveCode: {
				themes: ['github-dark'],
				styleOverrides: { borderRadius: '0.75rem', codeFontFamily: 'var(--__sl-font-mono)' },
			},
			description: 'Query and change a YAML file with SQL, from PHP, Go or Node.js.',
			logo: {
				light: './src/assets/memless-logo.svg',
				dark: './src/assets/memless-logo-light.svg',
				replacesTitle: true,
			},
			head: [
				{ tag: 'link', attrs: { rel: 'icon', href: '/favicon.ico', sizes: '32x32' } },
				{ tag: 'link', attrs: { rel: 'apple-touch-icon', href: '/apple-touch-icon.png' } },
				{ tag: 'link', attrs: { rel: 'manifest', href: '/site.webmanifest' } },
				{ tag: 'link', attrs: { rel: 'service-doc', href: '/getting-started/install/' } },
				{ tag: 'meta', attrs: { name: 'theme-color', content: '#16181D' } },
				{ tag: 'meta', attrs: { property: 'og:image', content: 'https://memless.nascent-tech.co/og.png' } },
				{ tag: 'meta', attrs: { property: 'og:image:width', content: '1200' } },
				{ tag: 'meta', attrs: { property: 'og:image:height', content: '630' } },
				{ tag: 'meta', attrs: { property: 'og:image:alt', content: 'memless — Query and change a YAML file with SQL' } },
				{ tag: 'meta', attrs: { name: 'twitter:card', content: 'summary_large_image' } },
			],
			social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/nascent-tech/memless' }],
			editLink: { baseUrl: 'https://github.com/nascent-tech/memless/edit/main/site/' },
			sidebar: [
				{
					label: 'Getting started',
					items: [
						{ label: 'Install', slug: 'getting-started/install' },
						{ label: 'Quick start', slug: 'getting-started/quick-start' },
					],
				},
				{
					label: 'Guides',
					items: [{ autogenerate: { directory: 'guides' } }],
				},
				{
					label: 'Reference',
					items: [{ autogenerate: { directory: 'reference' } }],
				},
			],
		}),
	],
});
