# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Non publié]

## [0.3.0] — 2026-09-26

### Modifié

- **Breaking change for Go users:** the Go module installs with `go get
  github.com/nascent-tech/memless-go` and is imported as `github.com/nascent-tech/memless-go`: it is
  published from the mirror repository `nascent-tech/memless-go`, so the main repository no longer
  carries its libraries. The versions up to 0.2.1 stay at the former path,
  `github.com/nascent-tech/memless/bindings/go`; to move, change the import path.

## [0.2.1] — 2026-09-26

### Corrigé

- The PHP archive of the GitHub release carries the four bundled libraries and the C header (the
  0.2.0 archive did not; set `MEMLESS_LIB` with it).

## [0.2.0] — 2026-09-26

### Ajouté

- Each bridge installs with its language's own tool and brings the native library with it, with
  nothing to download and no `MEMLESS_LIB` to set, on macOS (Apple silicon and Intel) and Linux with
  glibc (x86_64 and aarch64): `npm install @nascent-tech/memless`, `composer require
  nascent-tech/memless`, and `go get github.com/nascent-tech/memless/bindings/go@v0.2.0` or later.
  npm installs only the platform package matching the machine; the Composer package carries the four
  libraries and the C header; a Go program embeds only the library of the platform it is built for,
  extracted once into the user cache directory and checked against its SHA-256 before every load.
  The bundled Linux libraries need glibc 2.39 or later (Ubuntu 24.04 or later); elsewhere,
  `MEMLESS_LIB` still points the bridge at a library built locally.
- The release workflow publishes to npm, to Packagist (through the mirror repository
  `nascent-tech/memless-php`) and to the Go module proxy, next to the GitHub release, which now also
  carries `memless-lib-SHA256SUMS`. A manual run is a dry run by default: it builds and assembles every
  package and prints what it would publish. `docs/PUBLISHING.md` describes the owner's one-time setup
  and how to release.

### Modifié

- The Composer package is named `nascent-tech/memless` (it was `memless/php`).
- The three bridges look for the library in a new shared order: `MEMLESS_LIB`, then the library the
  package bundles for the current platform, then `target/release` and `target/debug` of a checked-out
  workspace. The PHP bridge finds the C header the same way (`MEMLESS_HEADER`, then the bundled
  `lib/memless.h`, then the workspace). When nothing is found, the error still says to set
  `MEMLESS_LIB`.

## [0.1.1] — 2026-09-26

### Corrigé

- The release workflow packages the library it actually built for each target (`.dylib` on
  macOS, `.so` on Linux). The `v0.1.0` tag failed at this step, so no 0.1.0 archives were published.

## [0.1.0] — 2026-09-26

### Ajouté

- Node.js bridge (`bindings/node`) that loads the memless C library through koffi, with no native
  addon to compile: open an instance, query, execute writes and run transactions, with TypeScript
  declarations and a `npm run typecheck` script.
- Integers beyond 2^53 stay exact in Node.js: they come back as `BigInt` instead of losing precision.
- The parity harness replays fixtures, queries, writes and transactions through the Node.js bridge
  as well, and demands a single answer from all three bridges, rewritten file included.
- `ORDER BY` in queries: one or more column keys, `ASC` (the default) or `DESC`, qualified columns
  across a join, ties kept in file order and rows without the column placed last. Ordering follows
  the single comparison rule: a key whose values differ in type is refused and names the two rows,
  and ordering an aggregate, ordering by position and `NULLS FIRST`/`NULLS LAST` stay outside the
  subset.
- `reload()` on an instance, in PHP, Go and Node.js: it re-reads the file the instance was loaded
  from and replaces the in-memory state, exactly as a fresh load would. It is refused while a
  transaction is open, or when the file would be refused at load (the old state stays readable and
  writable either way), and a released instance faults.
- The C ABI exposes `memless_reload`, and the parity harness replays reload scenarios on all three
  bridges.
- The project is licensed under MIT, and its README explains what Memless is, the guessing rules,
  the SQL subset, transactions and reload, errors, installation and the parity harness, with one
  example per language; each bridge has its own README.
- Continuous integration runs the four Rust gates, the three bridges' tests, the Node.js type check
  and the three-bridge parity harness on `aarch64-apple-darwin`, `x86_64-apple-darwin`,
  `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu`; a `v*` tag publishes a GitHub release
  with the library for each of these targets, the PHP archive and the Node.js package.

### Modifié

- A fault reads the same in PHP, Go and Node.js — `memless fault (<status>): <message>` — and PHP
  now throws a dedicated `Memless\MemlessFault` (with its `status`) instead of a `\LogicException`
  or an `\InvalidArgumentException`, still distinct from a `MemlessRefusal`.
- The three bridges look for the library in the same order — `MEMLESS_LIB`, which must name an
  existing file, then the release build, then the debug build (Go used to prefer the debug build) —
  and load it on the first call that needs it, never at import time.
- The C ABI version is now 5, and the PHP, Go and Node.js bridges require an ABI 5 library.
- The PHP bridge requires PHP 8.1 or later (it used to declare 7.4).

[Non publié]: https://github.com/nascent-tech/memless/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/nascent-tech/memless/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/nascent-tech/memless/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/nascent-tech/memless/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/nascent-tech/memless/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/nascent-tech/memless/releases/tag/v0.1.0
