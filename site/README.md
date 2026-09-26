# Memless documentation site

The source of the Memless documentation, built with
[Starlight](https://starlight.astro.build) and published to
<https://nascent-tech.github.io/memless/> on every push to `main` that
touches `site/`.

```sh
cd site
npm ci
npm run dev     # http://localhost:4321/memless/
npm run build   # writes dist/
```

Pages live in `src/content/docs/`, as Markdown or MDX. The sidebar is set in
`astro.config.mjs`. Code samples shown in several languages use
`<Tabs syncKey="lang">`, so a reader who picks PHP, Go or Node.js once sees
that language everywhere.
