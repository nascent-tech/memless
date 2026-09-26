# Memless for Node.js

Query and change a YAML file with SQL, from Node.js. Memless loads the file
into memory, runs your SQL against it and writes every accepted change back to
the file — no schema, no server. It is made for test fixtures and demos.

This guide covers the Node.js API. The file format, the supported SQL and the
behaviour shared by all languages are described in the
[project README](https://github.com/nascent-tech/memless#readme).

## Install

```sh
npm install @nascent-tech/memless
```

Requires Node.js 18 or later. The package ships the native engine for macOS
(Apple silicon, Intel) and Linux with glibc 2.39 or later (x86_64, aarch64), so
there is nothing else to install. It calls the engine through
[koffi](https://koffi.dev): no compiler, no native addon build. TypeScript
declarations are included.

## Quick start

```yaml
# data.yaml
users:
  - id: 1
    name: Ada
  - id: 2
    name: Grace
```

```js
const { load } = require('@nascent-tech/memless');

const db = load('data.yaml');
try {
  db.execute("INSERT INTO users (id, name) VALUES (3, 'Linus')"); // data.yaml is rewritten

  const { columns, rows } = db.query('SELECT name FROM users ORDER BY name');
  console.log(columns, rows); // [ 'name' ] [ [ 'Ada' ], [ 'Grace' ], [ 'Linus' ] ]
} finally {
  db.release();
}
```

With ES modules: `import { load } from '@nascent-tech/memless';`.

## API

| Call | Returns | What it does |
| --- | --- | --- |
| `load(path)` | `Instance` | Reads the YAML file into memory. |
| `db.query(sql)` | `{ columns, rows }` | Runs a `SELECT`. `columns` is an array of names, `rows` an array of arrays of values. |
| `db.execute(sql)` | `number` | Runs an `INSERT`, `UPDATE` or `DELETE` and returns the number of rows affected. Outside a transaction, the file is rewritten before it returns. |
| `db.begin()` / `db.commit()` / `db.rollback()` | — | Groups writes: nothing is written until `commit()`; `rollback()` discards them. |
| `db.reload()` | — | Rereads the file, after something else changed it. Refused while a transaction is open. |
| `db.release()` | — | Frees the instance. Calling it again does nothing; any other call afterwards throws. |

## Values

| In the file | In JavaScript |
| --- | --- |
| text | `string` |
| integer | `number`, or `bigint` beyond `Number.MAX_SAFE_INTEGER`, so that it stays exact |
| decimal | `number` |
| boolean | `boolean` |
| missing value | `null` |

## Errors

Every method throws one of two errors. Their messages are identical in the
PHP and Go versions of Memless.

- **`MemlessRefusal`** — the file or the SQL breaks a rule: unknown table or
  column, broken relation, unsupported SQL, transaction already open. It is an
  expected outcome, which you can assert on in a test.
- **`MemlessFault`** — the bridge was misused (for example, a call after
  `release()`) or an internal error happened. Its `status` property is `2` for
  an invalid argument and `3` for an internal error.

```js
const { load, MemlessRefusal } = require('@nascent-tech/memless');

try {
  db.query('SELECT nope FROM users');
} catch (error) {
  if (!(error instanceof MemlessRefusal)) throw error;
  console.log(error.message); // no column "nope" in table "users"
}
```

If the native library cannot be found or has an incompatible version, the
first call throws a plain `Error` that explains what to do.

## Troubleshooting

**`npm install` warns about install scripts.** Recent npm versions ask you to
approve the install script of `koffi`. Memless does not need it: the query
runs either way.

**Your platform is not covered** (Alpine and other musl-based Linux, glibc
older than 2.39). Build the engine and set `MEMLESS_LIB`:

```sh
git clone https://github.com/nascent-tech/memless.git && cd memless
cargo build --release -p memless-capi
export MEMLESS_LIB="$PWD/target/release/libmemless_capi.so"   # .dylib on macOS
```

The bridge looks for the engine in this order: `MEMLESS_LIB`, then the library
bundled for your platform under `lib/<platform>/`, then, inside a clone of the
repository, `target/release/` and `target/debug/`. On Linux it checks the libc
first (`/usr/bin/ldd`, the diagnostic report, then the dynamic loader under
`/lib`): musl, or a libc it cannot identify, gets no bundled library.
`MEMLESS_LIB` loads native code into your process; only point it at a library
you trust.

**Upgrading from 0.3.0 or earlier.** The engine used to come in separate
packages, `@nascent-tech/memless-<platform>`; they are deprecated and no
longer needed. Remove them from your `package.json` if you added them
yourself.

## Contributing

This package is developed in
[`bindings/node`](https://github.com/nascent-tech/memless/tree/main/bindings/node)
of [nascent-tech/memless](https://github.com/nascent-tech/memless), with the
engine and the PHP and Go versions. Open issues and pull requests there — the
repository `nascent-tech/memless-node` is a read-only copy published with each
release. See the
[contributing guide](https://github.com/nascent-tech/memless/blob/main/CONTRIBUTING.md)
and the [security policy](https://github.com/nascent-tech/memless/security/policy).

To work on the bridge in a clone of the repository:

```sh
cargo build --release -p memless-capi   # the bridge loads target/release/ in a clone
npm ci --prefix bindings/node
npm test --prefix bindings/node
npm run typecheck --prefix bindings/node
```
