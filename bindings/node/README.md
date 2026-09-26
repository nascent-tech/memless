# memless — Node.js bridge

A Node.js bridge to the memless C ABI through [koffi](https://koffi.dev), without
a native addon. It loads the same `libmemless_capi` cdylib as the Go and PHP
bridges and speaks the same contract (ABI version 4), so the three stay in
parity from a single shared surface.

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

db.release();
```

- A cell is a `number`, `bigint`, `string`, `boolean`, or `null`. An integer
  stays exact: within `[Number.MIN_SAFE_INTEGER, Number.MAX_SAFE_INTEGER]` it
  comes back as a plain JS `number`; beyond that range it comes back as a
  `bigint` instead of silently losing precision.
- `MemlessRefusal` carries the domain message verbatim (D13). `MemlessFault`
  is a boundary or internal fault and carries the ABI status.
- Inside an open transaction, `query` sees the not-yet-committed writes
  (read-your-writes); `commit` rewrites the file once.

## The cdylib

The bridge finds the native library through `MEMLESS_LIB`, a trusted (ideally
absolute) path, or, inside a checked-out workspace, under `target/`. Build it
first:

```sh
cargo build -p memless-capi
```

## Running the tests

```sh
npm install
npm test        # node --test
npm run typecheck  # tsc --noEmit --strict against tests/types-check.ts
```
