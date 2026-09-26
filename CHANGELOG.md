# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Non publié]

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

### Modifié

- A fault reads the same in PHP, Go and Node.js — `memless fault (<status>): <message>` — and PHP
  now throws a dedicated `Memless\MemlessFault` (with its `status`) instead of a `\LogicException`
  or an `\InvalidArgumentException`, still distinct from a `MemlessRefusal`.
- The three bridges look for the library in the same order — `MEMLESS_LIB`, which must name an
  existing file, then the release build, then the debug build (Go used to prefer the debug build) —
  and load it on the first call that needs it, never at import time.
