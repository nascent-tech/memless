// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
	site: 'https://memless.nascent-tech.co',
	integrations: [
		starlight({
			title: 'Memless',
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
