# memless — Node.js bridge

A Node.js bridge to the memless C ABI through [koffi](https://koffi.dev), without
a native addon. It loads the same `libmemless_capi` cdylib as the Go and PHP
bridges and speaks the same contract (ABI version 5), so the three stay in
parity from a single shared surface. Node.js 18 or later. See the
[project README](../../README.md) for what memless is, the guessing rules and
the supported SQL subset.

## Install

```sh
npm install @nascent-tech/memless
```

That is all: the package lists four platform packages as optional
dependencies, at its own exact version, and npm installs only the one that
matches your machine, which carries the native library:

| Package | Platform |
| --- | --- |
| `@nascent-tech/memless-darwin-arm64` | macOS on Apple silicon |
| `@nascent-tech/memless-darwin-x64` | macOS on Intel |
| `@nascent-tech/memless-linux-x64-gnu` | Linux x86_64 with glibc |
| `@nascent-tech/memless-linux-arm64-gnu` | Linux aarch64 with glibc |

The Linux libraries need glibc 2.39 or later (Ubuntu 24.04 or later). Elsewhere
(Linux with musl or an older glibc, for instance), see [The cdylib](#the-cdylib).
Installing with `--omit=optional` (or `--no-optional`) skips the platform
package too; the bridge then needs `MEMLESS_LIB`.

## Surface

```js
const { load, MemlessRefusal, MemlessFault } = require('@nascent-tech/memless');

const db = load('data.yaml');                 // throws MemlessRefusal / MemlessFault
const result = db.query('SELECT name FROM users');
// -> { columns: ['name'], rows: [['Ada'], ['Grace']] }
const affected = db.execute("UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'");

db.begin();                                   // BEGIN / COMMIT / ROLLBACK
db.execute("DELETE FROM wallets WHERE id = 'w_123'");
db.commit();

db.reload();                                  // re-reads the file, refused if
                                               // a transaction is open

db.release();
```

- A cell is a `number`, `bigint`, `string`, `boolean`, or `null`. An integer
  stays exact: within `[Number.MIN_SAFE_INTEGER, Number.MAX_SAFE_INTEGER]` it
  comes back as a plain JS `number`; beyond that range it comes back as a
  `bigint` instead of silently losing precision.
- `MemlessRefusal` carries the domain message verbatim (D13). `MemlessFault`
  is a boundary or internal fault: its message reads
  `memless fault (<status>): <message>`, the same text as in the Go and PHP
  bridges, and its `status` property carries the ABI status (`2` for an
  invalid argument, such as a released instance or a NUL byte in the SQL, `3`
  for an internal fault).
- `reload()` re-reads the file into a fresh in-memory state, exactly as
  `load()` would build it; it throws a `MemlessRefusal` while a transaction is
  open or when the file would be refused at load, and the old state stays
  usable either way.
- A library that cannot be found or speaks another ABI version throws a plain
  `Error` on the first call that needs it.
- TypeScript declarations ship in `src/index.d.ts`.
- Inside an open transaction, `query` sees the not-yet-committed writes
  (read-your-writes); `commit` rewrites the file once.

## The cdylib

The bridge loads the native library on the first call that needs it, never
at `require()` time, and looks for it in the same order as the Go and PHP
bridges:

1. `MEMLESS_LIB`, a trusted (ideally absolute) path that must name an
   existing file;
2. the library of the platform package npm installed, found with
   `require.resolve('@nascent-tech/memless-<platform>/package.json')`. On
   Linux the bridge checks the libc itself, once (`/usr/bin/ldd`, then
   `process.report`), because an older npm ignores the `libc` field of the
   platform packages: musl, or a libc it cannot tell, has no platform package;
3. inside a checked-out workspace, `target/release/`, then `target/debug/`
   (`.dylib` before `.so`).

When nothing is found, the error says to set `MEMLESS_LIB`. The library must
speak ABI version 5. On another platform, or with your own build, set
`MEMLESS_LIB`:

```sh
cargo build --release -p memless-capi
export MEMLESS_LIB="$PWD/target/release/libmemless_capi.so"   # .dylib on macOS
```

`MEMLESS_LIB` loads arbitrary native code, like any FFI library path — only
point it at a library you trust.

In this repository, `package.json` does not list the platform packages: the
release workflow adds them when it publishes, from the templates under
`platforms/`, so that installing the bridge for development never pulls a
published library over the one you build under `target/`.

## Running the tests

```sh
npm ci
npm test           # node --test
npm run typecheck  # tsc --noEmit --strict against tests/types-check.ts
```

The parity driver (`harness/parity/node`) depends on this package by path and
reuses its `node_modules`, so run `npm ci` here before
`npm ci --prefix harness/parity/node`.
