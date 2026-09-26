// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
	site: 'https://nascent-tech.github.io',
	base: '/memless',
	integrations: [
		starlight({
			title: 'Memless',
			description: 'Query and change a YAML file with SQL, from PHP, Go or Node.js.',
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
