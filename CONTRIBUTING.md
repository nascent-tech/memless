# Contributing to memless

Thank you for wanting to improve memless. Bug reports, fixes, documentation
and ideas are all welcome. This guide explains where to contribute, how the
project is organised, and what a change needs before it can be merged.

By taking part, you agree to follow the [code of conduct](CODE_OF_CONDUCT.md).

## Where to contribute

Everything happens in this repository, `nascent-tech/memless`: the Rust core,
the C ABI, the three bridges (PHP, Go, Node.js) and the parity harness that
keeps them in agreement.

The repositories `nascent-tech/memless-php`, `nascent-tech/memless-go` and
`nascent-tech/memless-node` are **mirrors**: the release workflow rewrites
them at each version from `bindings/php`, `bindings/go` and `bindings/node`,
plus the native libraries. A commit made there would be erased by the next
release, so please open issues and pull requests here, even when they only
concern one bridge.

## Before you start

- **Found a bug?** Open an issue with the *Bug report* form. The exact error
  message, the YAML file (or the smallest excerpt that reproduces it) and the
  SQL statement are what makes a bug fixable. Say which bridges you tried: a
  difference between two bridges is a bug in itself.
- **Want to change a behaviour?** Open an issue with the *Feature request*
  form before writing code. The guessing rules, the SQL subset and the
  refusals are deliberate: memless refuses what it does not understand rather
  than approximating it, and it is built for test fixtures and demos, not for
  production (see [What Memless is not](README.md#what-memless-is-not)). A
  short discussion first saves you work that could not be merged.
- **Found a security issue?** Do not open a public issue; follow
  [`SECURITY.md`](SECURITY.md).
- **Small fixes** — a typo, a broken link, a clearer error message in the
  docs — can go straight to a pull request.

Issues labelled [`good first issue`](https://github.com/nascent-tech/memless/labels/good%20first%20issue)
or [`help wanted`](https://github.com/nascent-tech/memless/labels/help%20wanted)
are a good place to start.

## How the project is organised

| Path | What lives there |
| --- | --- |
| `crates/memless-domain` | The engine's rules: guessing types, identities and relations, executing statements, transactions, refusals. Pure Rust, no dependency, no I/O. |
| `crates/memless-engine` | Around the domain: reading and rewriting the YAML file, parsing SQL (with `sqlparser`), loading, querying, executing and reloading an instance. |
| `crates/memless-capi` | The C ABI (`include/memless.h`) that every bridge calls, built as the `libmemless_capi` cdylib. |
| `bindings/php`, `bindings/go`, `bindings/node` | The three thin bridges, each over the same C ABI. |
| `harness/parity` | The parity harness: one battery of fixtures, queries, writes, transactions and reloads replayed through the three bridges. |
| `docs/PUBLISHING.md` | How a release is published (maintainers). |

**The parity rule.** The same SQL against the same file must give the same
answer, and the same refusal with the same message, in PHP, Go and Node.js. So:

- a change of behaviour belongs in the Rust core, which the three bridges
  share, not in a bridge;
- a change to one bridge's surface (a new method, a new option) lands in all
  three bridges in the same pull request;
- a new behaviour comes with a case in `harness/parity`, so that the harness
  proves the three bridges agree on it.

## Setting up

memless is developed and tested on macOS (Apple silicon and Intel) and Linux
with glibc (x86_64 and aarch64). You need:

- a stable Rust toolchain with `clippy` (via [rustup](https://rustup.rs));
- PHP 8.1 or later with the `ffi` extension enabled, and Composer;
- Go 1.21 or later;
- Node.js 18 or later (the CI uses Node.js 24).

Then build the library and install the bridges' dependencies:

```sh
cargo build --release -p memless-capi
composer install -d bindings/php
npm ci --prefix bindings/node
composer install -d harness/parity/php
npm ci --prefix harness/parity/node
```

In a checked-out repository, the bridges load the library you built under
`target/` (`target/release`, then `target/debug`), so rebuild it after a
change to the core.

## Checks a pull request must pass

The CI runs these on the four supported platforms; run them locally first:

```sh
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release -p memless-capi

(cd bindings/php && vendor/bin/phpunit)
(cd bindings/go && go test ./...)
npm test --prefix bindings/node
npm run typecheck --prefix bindings/node

bash harness/parity/run.sh
for test in harness/parity/tests/detects-*.sh; do bash "$test"; done
```

## Writing the change

- **Follow the surrounding code.** Small files and short functions, one type
  per file, names that say what the code does rather than comments that
  explain it, and each language written the way that language is usually
  written.
- **Test what you change.** Tests mirror the source layout (`tests/` beside
  `src/` in the crates and in the PHP and Node bridges, `_test.go` files in
  the Go bridge). A fix comes with the test that failed
  before it; a refusal is tested as carefully as a success.
- **Keep refusals exact.** An error message is part of the contract: the three
  bridges return it verbatim, and the parity harness compares it.
- **Code, comments, commits and pull requests are written in English.**
- **Update the docs** your change affects: the project `README.md`, the
  bridge's `README.md`, and an entry under `## [Non publié]` in
  [`CHANGELOG.md`](CHANGELOG.md) for anything a user would notice.

## Commits and pull requests

- One pull request carries one change. A maintainer squash-merges it: its
  title (or, for a single commit, that commit's title) becomes the commit on
  `main`, so write both the same way.
- Titles follow [Conventional Commits](https://www.conventionalcommits.org):
  `feat(node): …`, `fix(php): …`, `docs: …`, `test(parity): …`,
  `refactor(domain): …`. Mark a breaking change with `!` (`feat(go)!: …`) and
  describe it in the body; versions follow [Semantic Versioning](https://semver.org).
- Fill in the pull request template: why the change is needed, what it does,
  and which checks you ran.
- Keep your branch up to date by rebasing on `main`; the history stays linear.

A maintainer reviews every pull request. Expect questions: they are about
keeping the three bridges in agreement and the refusals exact, not about you.

## Releases

Releases are cut by the maintainers, from a tag, by the workflow described in
[`docs/PUBLISHING.md`](docs/PUBLISHING.md). You do not need to bump any version
in your pull request.

## License

memless is released under the [MIT License](LICENSE). By contributing, you
agree that your contribution is licensed under the same terms.
