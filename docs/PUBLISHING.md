# Publishing memless

This is the maintainer's guide to how a version of memless reaches its users:
what the release workflow publishes, the one-time setup only the repository
owner can do, how to rehearse a release with a dry run, and how to cut a real
one.

## What a release publishes

A tag `vX.Y.Z` pushed on `main` runs `.github/workflows/release.yml`, which
builds the C library for the four supported platforms and publishes it in
four places:

| Where | What | Job | Runs when |
| --- | --- | --- | --- |
| GitHub Releases | `memless-capi-X.Y.Z-<target>.tar.gz` ×4, `nascent-tech-memless-X.Y.Z.tgz`, `memless-php-X.Y.Z.zip`, `memless-lib-SHA256SUMS`, `SHA256SUMS` | `publish` | always, on a tag |
| Go module proxy | an orphan commit and tag `vX.Y.Z` in the mirror `nascent-tech/memless-go`, module `github.com/nascent-tech/memless-go` | `publish-go` | repository variable `GO_MIRROR_PUBLISH` is `true` |
| npm | `@nascent-tech/memless-darwin-arm64`, `-darwin-x64`, `-linux-x64-gnu`, `-linux-arm64-gnu`, then `@nascent-tech/memless` | `publish-npm` | repository variable `NPM_PUBLISH` is `true` |
| Packagist | an orphan commit and tag `vX.Y.Z` in the mirror `nascent-tech/memless-php`, package `nascent-tech/memless` | `publish-php` | repository variable `PHP_MIRROR_PUBLISH` is `true` |

The four platforms and their Rust targets:

| Platform | Rust target | Library |
| --- | --- | --- |
| `darwin-arm64` | `aarch64-apple-darwin` | `libmemless_capi.dylib` |
| `darwin-x64` | `x86_64-apple-darwin` | `libmemless_capi.dylib` |
| `linux-x64-gnu` | `x86_64-unknown-linux-gnu` | `libmemless_capi.so` |
| `linux-arm64-gnu` | `aarch64-unknown-linux-gnu` | `libmemless_capi.so` |

The jobs, in order:

1. `plan` reads the version from the tag and checks that every version
   carrier agrees with it: the `version` of `bindings/node/package.json` and
   the `version` of each of the three crates (`crates/*/Cargo.toml`). It fails
   with the file that disagrees otherwise. It also decides whether this run
   publishes (see [Dry run](#dry-run)).
2. `build` (one per target) builds the library, packages the release archive,
   and uploads the raw library.
3. `libraries` gathers the four raw libraries into `lib/<platform>/` and writes
   `lib/SHA256SUMS`. Every package below ships these exact bytes.
4. `npm-packages` builds the four platform packages from
   `bindings/node/platforms/<platform>/package.json` and the main package, to
   which it adds the four platform packages as `optionalDependencies` at
   exactly `X.Y.Z`. It installs them on Linux x86_64, runs a query, and
   uploads the five tarballs. It needs no account, so it always runs.
5. `php-package` assembles the Composer package tree (`bindings/php` without
   `vendor`, `tests/`, `phpunit.xml.dist`, `composer.lock` and
   `lib/.gitignore`, plus `LICENSE`, `lib/<platform>/`, `lib/SHA256SUMS` and
   `lib/memless.h`), installs it with Composer on Linux x86_64 and runs a
   query, then uploads both the tree and `memless-php-X.Y.Z.zip` (root
   `memless-php-X.Y.Z/`) as artifacts. It needs no account, so it always runs;
   `publish` and `publish-php` both consume its output, so the package content
   is defined once.
6. `go-package` assembles the Go module tree (`bindings/go` without
   `*_test.go`, `lib/.gitignore` and the placeholder `lib/<platform>/README.md`,
   plus `LICENSE`, `lib/<platform>/` and `lib/SHA256SUMS`), checks the
   checksums and the module path, runs `go vet` for the four platforms, then,
   outside the repository, builds a program that requires the module through a
   `replace` pointing at the tree (`go build -trimpath`, no `MEMLESS_LIB`, no
   `target/`) on Linux x86_64 and runs a query. It uploads the tree as an
   artifact. It needs no account, so it always runs.
7. `publish` creates the GitHub release, whose npm tarball is the one
   `npm-packages` built (with its `optionalDependencies`) and whose PHP
   archive is the zip `php-package` built. It waits for `go-package` too, so
   no release is created while one package fails its test. When the release
   already exists, it says so and leaves it alone: assets are never replaced.
8. `publish-npm` installs npm 11 and publishes the tarballs of `npm-packages`,
   the platform packages first and the main package last.
9. `publish-php` takes the package tree `php-package` assembled and tested,
   and pushes it to the mirror as a single orphan commit, forced onto the
   mirror's `main`, and tags it `vX.Y.Z`. Packagist picks the tag up through
   its GitHub hook.
10. `publish-go` does the same with the module tree `go-package` assembled
    and tested, in the mirror `nascent-tech/memless-go`. The Go module proxy
    picks the tag up on the first `go get` of that version.

Each job only gets the permissions it needs: `publish` can write the
repository's contents, `publish-npm` can request an OIDC token for npm, the
others can only read. `NPM_TOKEN` is only exposed to the step of
`publish-npm` that publishes, `PHP_MIRROR_DEPLOY_KEY` only to the step of
`publish-php` that pushes, and `GO_MIRROR_DEPLOY_KEY` only to the step of
`publish-go` that pushes. Each deploy key can write to its own mirror and
nothing else.

Every publishing step can be re-run safely: an existing GitHub release is
left as it is, a version already on npm is skipped, and an existing mirror
tag `vX.Y.Z` is left as it is. **Tags are
never moved or deleted, release assets never replaced**; if a published
version is wrong, release a new patch version.

## One-time setup by the owner

Until the owner has done this, `publish-npm`, `publish-php` and `publish-go`
are skipped on a tag and the workflow stays green; the GitHub release is
published regardless. Every command below assumes the GitHub CLI (`gh`) is
logged in as an owner of the `nascent-tech` organization.

### npm

1. **Create the organization.** On [npmjs.com](https://www.npmjs.com), with an
   account that has two-factor authentication enabled: avatar menu → *Add
   Organization* → name `nascent-tech` → the free plan (unlimited public
   packages). The scope `@nascent-tech` now belongs to it.

2. **Create a granular token for the first publication.** A trusted publisher
   can only be attached to a package that already exists, so the very first
   publication needs a token. Avatar menu → *Access Tokens* → *Generate New
   Token* → *Granular Access Token*:
   - name: `memless release workflow`;
   - expiration: 90 days at most (npm caps write tokens there);
   - *Bypass two-factor authentication*: checked (the workflow cannot answer a
     one-time password);
   - *Packages and scopes*: permission *Read and write*, restricted to the
     scope `@nascent-tech`;
   - no organization permission.

   Store it as a repository secret (the command prompts for the value, so it
   never lands in your shell history):

   ```sh
   gh secret set NPM_TOKEN --repo nascent-tech/memless
   ```

3. **Turn npm publication on:**

   ```sh
   gh variable set NPM_PUBLISH --body true --repo nascent-tech/memless
   ```

4. **After the first release**, switch to trusted publishing, which needs no
   long-lived secret. For each of the five packages
   (`@nascent-tech/memless`, `@nascent-tech/memless-darwin-arm64`,
   `@nascent-tech/memless-darwin-x64`, `@nascent-tech/memless-linux-x64-gnu`,
   `@nascent-tech/memless-linux-arm64-gnu`): package page → *Settings* →
   *Trusted Publisher* → *GitHub Actions*, with organization `nascent-tech`,
   repository `memless`, workflow filename `release.yml`, and no environment.
   Then, still under *Settings* → *Publishing access*, choose *Require
   two-factor authentication and disallow tokens*.

   Once all five have a trusted publisher, delete the token on npmjs.com
   (*Access Tokens*) and in the repository:

   ```sh
   gh secret delete NPM_TOKEN --repo nascent-tech/memless
   ```

   From then on the workflow authenticates through OIDC (npm 11.5.1 or later;
   the job installs the latest npm 11), and npm attaches provenance on its own. While
   `NPM_TOKEN` exists, the workflow uses it and passes `--provenance` itself.

### Packagist, through the mirror repository

Packagist reads `composer.json` at the root of a repository, and this
repository keeps the PHP bridge in `bindings/php`, so the package is
published from a mirror that only the release workflow writes to.

1. **Create the mirror**, empty (no README, no license: the workflow
   force-pushes its `main`):

   ```sh
   gh repo create nascent-tech/memless-php --public \
     --description "Packagist mirror of the memless PHP bridge, written by nascent-tech/memless's release workflow"
   ```

2. **Give the workflow a deploy key** that can write to the mirror and
   nothing else. Generate the pair in a scratch directory, register the public
   half on the mirror with write access, store the private half as a secret of
   this repository, then delete both files:

   ```sh
   ssh-keygen -t ed25519 -N "" -C "memless release workflow" -f memless-php-deploy
   gh repo deploy-key add memless-php-deploy.pub --repo nascent-tech/memless-php \
     --title "memless release workflow" --allow-write
   gh secret set PHP_MIRROR_DEPLOY_KEY --repo nascent-tech/memless < memless-php-deploy
   rm memless-php-deploy memless-php-deploy.pub
   ```

3. **Turn mirror publication on:**

   ```sh
   gh variable set PHP_MIRROR_PUBLISH --body true --repo nascent-tech/memless
   ```

4. **After the first release has pushed the mirror** (Packagist needs a
   `composer.json` to read):
   - sign in to [packagist.org](https://packagist.org) with GitHub; if the
     `nascent-tech` organization restricts OAuth apps, grant Packagist access
     to it (GitHub → *Settings* → *Applications* → *Authorized OAuth Apps* →
     Packagist → *Grant* next to `nascent-tech`);
   - *Submit* → repository URL `https://github.com/nascent-tech/memless-php`
     → *Check* → *Submit*. The package is named `nascent-tech/memless`, from
     its `composer.json`, and Packagist imports every tag of the mirror;
   - check the hook: the package page must not warn that it is not
     auto-updated. When Packagist could not install the hook itself, add it
     on the mirror (*Settings* → *Webhooks* → *Add webhook*): payload URL
     `https://packagist.org/api/github?username=<your Packagist username>`,
     content type `application/json`, secret = your Packagist API token
     (packagist.org → your profile → *Show API Token*), event *Just the push
     event*.

### Go module proxy, through the mirror repository

The Go module path names the repository that holds it, and the module
carries the four libraries, so it is published from a mirror that only the
release workflow writes to: the main repository (and every clone of it)
carries no binary.

1. **The mirror exists**: `nascent-tech/memless-go`, public, created empty
   (the workflow force-pushes its `main`). Its deploy key, with write access,
   is registered too; the maintainer keeps the private half in
   `~/.ssh/memless_go_mirror_deploy`. Had they to be recreated, the commands
   are those of the PHP mirror above, with `memless-go` in place of
   `memless-php`.

2. **Store the private key as a secret** of this repository (the command
   reads it from the file, so it never lands in your shell history):

   ```sh
   gh secret set GO_MIRROR_DEPLOY_KEY --repo nascent-tech/memless < ~/.ssh/memless_go_mirror_deploy
   ```

3. **Turn mirror publication on:**

   ```sh
   gh variable set GO_MIRROR_PUBLISH --body true --repo nascent-tech/memless
   ```

Nothing to register with the Go module proxy: the first `go get` of a version
makes `proxy.golang.org` fetch and cache it forever. Two rules:

- **Never push a `vX.Y.Z` tag to the mirror by hand.** The proxy caches a
  version forever: a hand-made tag could publish a module without its
  library.
- **Never create a `bindings/go/*` tag** in this repository any more. The
  versions up to 0.2.1 were published at
  `github.com/nascent-tech/memless/bindings/go` through the tags
  `bindings/go/vX.Y.Z` (`v0.1.0` and `v0.1.1` without a library, `v0.2.0` and
  `v0.2.1` with it); they stay where they are, never moved or deleted.

To make the new version show up at once instead of on the first user's
`go get`:

```sh
GOPROXY=https://proxy.golang.org go list -m github.com/nascent-tech/memless-go@vX.Y.Z
```

## Dry run

A manual run of the workflow is a rehearsal. It builds the four libraries,
assembles every package — `npm pack` of the five npm packages, the PHP and
Go mirror trees and their orphan commits — and runs the same Linux install
tests, then prints what each job would publish instead of publishing it. It
does not create the GitHub release, push a tag, or publish to npm, and it
ignores `NPM_PUBLISH`, `PHP_MIRROR_PUBLISH` and `GO_MIRROR_PUBLISH`, so it
works before any account exists. The version is the one in `bindings/node/package.json`.

```sh
gh workflow run release.yml --repo nascent-tech/memless --ref main -f dry_run=true
gh run watch --repo nascent-tech/memless "$(gh run list --repo nascent-tech/memless \
  --workflow release.yml --limit 1 --json databaseId --jq '.[0].databaseId')"
```

`dry_run` defaults to `true`. A manual run with `dry_run=false` only
publishes when it is started on a tag (`--ref vX.Y.Z`), which re-runs that
release; started on a branch, it stays a dry run.

## A real release

1. **Prepare the version** in a release pull request: the version of the
   three crates, of `bindings/node/package.json` (and the two npm lock files),
   and the `CHANGELOG.md` section. The platform templates under
   `bindings/node/platforms/` carry no version: the workflow gives them the
   tag's. Merge it.

2. **Rehearse** with a [dry run](#dry-run) on `main`, and read what each
   job would publish.

3. **Tag and push** the merged commit:

   ```sh
   git switch main && git pull --ff-only
   git tag -a vX.Y.Z -m "memless X.Y.Z"
   git push origin vX.Y.Z
   ```

   The `plan` job fails at once if the tag does not match
   `bindings/node/package.json`, or if one of the three crates carries another
   version.

4. **Check** each channel once the run is green:

   ```sh
   gh release view vX.Y.Z --repo nascent-tech/memless
   npm view @nascent-tech/memless@X.Y.Z optionalDependencies
   composer show --all nascent-tech/memless
   GOPROXY=https://proxy.golang.org go list -m github.com/nascent-tech/memless-go@vX.Y.Z
   ```

   Then, in an empty directory outside the repository and without
   `MEMLESS_LIB`, install each bridge the way a user would (`npm install
   @nascent-tech/memless`, `composer require nascent-tech/memless`, `go get
   github.com/nascent-tech/memless-go@vX.Y.Z`) and run one query.

5. **After the very first release**, finish the owner's setup: attach the npm
   trusted publishers and delete `NPM_TOKEN`, and submit the mirror to
   Packagist (see [One-time setup](#one-time-setup-by-the-owner)).

If a job fails, fix the cause and re-run the failed jobs from the run page
(or `gh run rerun <run-id> --failed`): what was already published is skipped.

## Checksums

`lib/SHA256SUMS` lists the SHA-256 of each bundled library, with paths
relative to `lib/` (`darwin-arm64/libmemless_capi.dylib`, …). The same file
ships in the Go module (`lib/SHA256SUMS` of the mirror
`nascent-tech/memless-go`), in the Composer
package (`lib/SHA256SUMS`) and on the GitHub release as
`memless-lib-SHA256SUMS`; the release's own `SHA256SUMS` covers every release
asset, that file included. The npm platform packages carry the same bytes;
`npm view @nascent-tech/memless-<platform>@X.Y.Z dist` gives their tarball
integrity.
