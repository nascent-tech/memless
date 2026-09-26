# Memless documentation site

The source of the Memless documentation, built with
[Starlight](https://starlight.astro.build) and published to
<https://memless.nascent-tech.co> on every push to `main` that
touches `site/`.

```sh
cd site
npm ci
npm run dev     # http://localhost:4321/
npm run build   # writes dist/
```

Pages live in `src/content/docs/`, as Markdown or MDX. The sidebar is set in
`astro.config.mjs`. Code samples shown in several languages use
`<Tabs syncKey="lang">`, so a reader who picks PHP, Go or Node.js once sees
that language everywhere.

## Brand assets

The logo lives in `public/brand/` and is served at
<https://memless.nascent-tech.co/brand/>. The SVG files are the source; the
PNG files are rendered from them.

| File | Use |
| --- | --- |
| `memless-logo.svg`, `memless-logo.png` | Logo on a light background |
| `memless-logo-light.svg`, `memless-logo-light.png` | Logo on a dark background |
| `memless-mark.svg`, `memless-mark-light.svg` | The mark alone, for light or dark backgrounds |
| `memless-icon.svg`, `memless-icon-1024.png` | App icon and avatar (GitHub organisation, npm, Packagist) |
| `github-social.png` | Repository social preview, 1280 × 640 |

The wordmark is JetBrains Mono Bold, turned into outlines so no font is
needed. Colours: ink `#16181D`, paper `#F3F1EA`, green `#2E8B62`.

The favicons, `og.png` (the card shown when a link is shared),
`site.webmanifest` and `robots.txt` sit at the root of `public/`.

## Markdown for agents

`npm run build` writes an `index.md` next to every `index.html`, converted from
the same page. The Cloudflare Worker in `worker/` answers a request that prefers
`text/markdown` in its `Accept` header with that file, as
`Content-Type: text/markdown` with `Vary: Accept` and an `x-markdown-tokens`
estimate; browsers keep getting HTML.

```sh
npm test                      # the worker's tests
npx wrangler deploy -c worker/wrangler.toml
```

The worker is deployed by hand: the site workflow does not hold Cloudflare
credentials. Its route runs on every request to the site, and the free plan
allows 100,000 a day: the route is set to **fail open**, so past that limit the
site is served as if the worker did not exist.

Only URLs ending in `/` are negotiated; the origin redirects the others there
first, so the relative links in the Markdown resolve.
