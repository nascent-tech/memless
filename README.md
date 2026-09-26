# Memless

Memless is an **in-memory SQL engine driven by a single YAML file**. The file
holds nothing but data — each table is a list of rows — and Memless guesses the
rest by itself: the type of every value, which column is the row's identity,
and which columns point at another table. It loads that file into memory once,
lets you query and change it in plain SQL text, and rewrites the file every
time a write or a transaction is validated. The file stays the only source of
truth; there is no schema to declare and no migration to write.

One core, written in Rust, is shared by three thin bridges — PHP, Go and
Node.js — so the same SQL against the same file gives the same answer,
byte-for-byte, in all three languages. A parity harness
(`harness/parity/`) enforces that agreement on every change.

## What Memless is not

- **Not for production.** It is built for a developer preparing fixtures for
  automated tests and demos — a single process, a local file, no schema
  migration story. There is no server, no network protocol, and no
  coordination between concurrent writers.
- **Not a server.** Every bridge loads the engine in-process (as a native
  library); nothing listens on a port, and nothing else can be safely running
  against the same file at the same time.
- **Not a general-purpose SQL database.** Only the subset of SQL described
  below is understood; everything else — including most of what a real
  database offers — is refused, not silently approximated.

## The example file

A Memless file is plain YAML: a top-level key per table, each table a list of
field sets (maps of column name to value).

```yaml
users:
  - id: 01H7B2
    name: Ada
  - id: 01H7B3
    name: Grace
wallets:
  - id: w_123
    user_id: 01H7B3
    amount: 100
  - id: w_124
    user_id: 01H7B2
    amount: 250
```

## The guessing rules

Memless never reads a schema; it infers structure from the file's own shape,
and refuses the file when that shape is ambiguous or broken:

- **Value types.** Every cell is one of four kinds: text, integer, decimal or
  boolean. A column that mixes kinds across rows, or a nested value (a list or
  a map instead of a scalar) inside a cell, is refused.
- **Primary key.** Every row must carry an `id` column whose value is text or
  an integer (never a decimal, a boolean, or absent) — Memless refuses a row
  with no `id`, an `id` of the wrong kind, or a duplicate `id` inside the same
  table. Comparisons never mix kinds: an integer `id` of `5` and a text `id`
  of `"5"` are different rows.
- **Relations.** A column literally named `<name>_id` is guessed as a relation
  to the `id` column of table `<name>s` (`user_id` → `users`, `wallet_id` →
  `wallets`). A `<name>_id` value that names no row in that table (a broken
  relation) refuses the file or the write that would create it.
- **Everything else about the file's shape** — a table that is not a list, a
  row that is not a field set, a duplicate table or column key, a key that is
  not text — is refused too, each with its own message (see *Errors* below).

## The SQL subset

Memless accepts exactly this shape of SQL and refuses everything else with the
same message, in all three languages, as *"… is outside the supported SQL
subset"*:

- **`SELECT`** a list of columns or `*`, `FROM` one table, with an optional
  **`WHERE`** built from `=`, `<>`, `<`, `<=`, `>`, `>=`, `IS NULL` /
  `IS NOT NULL` comparisons combined with `AND` / `OR`.
- **One `JOIN`** — `INNER JOIN <table> ON <a>.<col> = <b>.<col>` — where the
  `ON` condition is a guessed relation between the two tables (equality on a
  `<name>_id` column and the target's `id`). Only one `JOIN` per query; every
  column reference in a joined query must be qualified (`table.column`).
- **`COUNT(*)`** and **`SUM(<column>)`**, each alone in the projection (never
  mixed with a plain column, and `SUM` refuses a column that is not numeric
  for any matched row, or an overflowing sum).
- **`ORDER BY <column> [ASC|DESC], …`** — one or more column keys, `ASC` by
  default, qualified (`table.column`) in a joined query. The sort is stable:
  ties keep the file's order, and a row whose value for that column is absent
  always sorts last, in both directions. Ordering follows the single
  comparison rule: a key whose retained rows mix kinds (e.g. text and
  integer) is refused, naming the two rows, rather than ordered arbitrarily.
- **`INSERT INTO <table> (<columns…>) VALUES (<values…>)`** — the column list
  is mandatory; a column count mismatch or a repeated column is refused.
- **`UPDATE <table> SET <col> = <value>, … [WHERE …]`**.
- **`DELETE FROM <table> [WHERE …]`**.
- **`BEGIN`**, **`COMMIT`**, **`ROLLBACK`** — see *Transactions and reload*
  below.

Refused outright, with no partial support: `GROUP BY`, `LIMIT`/`OFFSET`,
`ORDER BY` with an aggregate, by position (`ORDER BY 1`) or with
`NULLS FIRST`/`NULLS LAST`, multiple `JOIN`s, `JOIN … USING`, `NATURAL JOIN`,
a `JOIN` without `ON`, a projection with duplicate output column names, any
operator besides the six comparisons above (`LIKE`, `IN`, arithmetic, …),
CTEs, subqueries, window functions, locking clauses, and any statement that is
not one of those listed above (DDL, `ALTER`, `CREATE`, …).

## One example per language

Each example opens `data.yaml` (the file shown above), transfers 50 from
Grace's wallet to Ada's inside a transaction, then rereads the wallets sorted
by amount.

### Go

```go
package main

import (
	"fmt"
	"log"

	memless "github.com/nascent-tech/memless-go"
)

func main() {
	db, err := memless.Load("data.yaml")
	if err != nil {
		log.Fatal(err)
	}
	defer db.Release()

	if err := db.Begin(); err != nil {
		log.Fatal(err)
	}
	if _, err := db.Execute("UPDATE wallets SET amount = 50 WHERE id = 'w_123'"); err != nil {
		log.Fatal(err)
	}
	if _, err := db.Execute("UPDATE wallets SET amount = 300 WHERE id = 'w_124'"); err != nil {
		log.Fatal(err)
	}
	if err := db.Commit(); err != nil {
		log.Fatal(err)
	}

	rows, err := db.Query("SELECT id, amount FROM wallets ORDER BY amount ASC")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(rows.Columns, rows.Rows)

	// re-reads data.yaml from disk into a fresh in-memory state
	if err := db.Reload(); err != nil {
		log.Fatal(err)
	}
}
```

### PHP

```php
<?php

require 'vendor/autoload.php';

use Memless\Instance;

$db = Instance::load('data.yaml');

$db->begin();
$db->execute("UPDATE wallets SET amount = 50 WHERE id = 'w_123'");
$db->execute("UPDATE wallets SET amount = 300 WHERE id = 'w_124'");
$db->commit();

$rows = $db->query("SELECT id, amount FROM wallets ORDER BY amount ASC");
print_r($rows);

$db->reload(); // re-reads data.yaml from disk into a fresh in-memory state
$db->release();
```

### Node.js

```js
const { load } = require('@nascent-tech/memless');

const db = load('data.yaml');

db.begin();
db.execute("UPDATE wallets SET amount = 50 WHERE id = 'w_123'");
db.execute("UPDATE wallets SET amount = 300 WHERE id = 'w_124'");
db.commit();

const { columns, rows } = db.query("SELECT id, amount FROM wallets ORDER BY amount ASC");
console.log(columns, rows);

db.reload(); // re-reads data.yaml from disk into a fresh in-memory state
db.release();
```

## Transactions and reload

`BEGIN` opens a transaction on the instance; the writes that follow change
only its working state, which a `SELECT` inside the transaction already sees.
`COMMIT` validates the whole working state and rewrites the file once (not at
all when nothing changed); `ROLLBACK` discards the working state and leaves
the file untouched. A second `BEGIN` (`a transaction is already open`) and a
`COMMIT` or `ROLLBACK` with nothing open (`no open transaction`) are refused. A
write refused inside the transaction leaves it open and usable, while a
`COMMIT` always closes it: a failed validation leaves memory and the file as
they were before `BEGIN`.

`reload()` (`Reload()` in Go) re-reads the file the instance was loaded from
and replaces the in-memory state with it, exactly as a fresh load would — use
it after the file was edited outside Memless. It is refused while a
transaction is open (`cannot reload while a transaction is open`), and when
the file would be refused at load (missing, incoherent); either way the old
state stays in place, readable and writable.

## Errors

Every refusal — a broken file, an unsupported statement, an unknown table or
column, a validation failure on a write, a transaction guard — carries the
**same message, verbatim, in all three languages**: the core decides the
wording once, and no bridge is allowed to translate or reword it. A bridge
only translates the *shape* of the error:

- Go returns a `*memless.RefusalError` (domain refusal) or a
  `*memless.FaultError` (a boundary or internal fault, never a file refusal).
- PHP throws `Memless\MemlessRefusal` (domain refusal) or
  `Memless\MemlessFault` (boundary or internal fault).
- Node throws `MemlessRefusal` (domain refusal) or `MemlessFault` (boundary or
  internal fault).

A fault reads the same in all three: `memless fault (<status>): <message>`,
where `<status>` is the ABI status (`2` for an invalid argument, such as a
released instance or a NUL byte in the SQL text; `3` for an internal fault),
also exposed as the fault's `Status` (Go) or `status` (PHP, Node) field. A
library that cannot be found, or that speaks another ABI version, is reported
by a plain error (Go), a `\LogicException` (PHP) or an `Error` (Node) on the
first call that needs it.

A refusal is not a bug: `no table "ghosts"`, `no column "ghost" in table
"users"`, `broken relation "user_id" of row 0 in "wallets": no row w_999 in
"users"`, `GROUP BY is outside the supported SQL subset`, `a transaction is
already open` are all ordinary, expected outcomes of malformed SQL or an
inconsistent file. A **fault** — an invalid handle, a NUL byte in the SQL
text, an internal panic caught at the FFI boundary — is different: it signals
a bridge or caller mistake, not a rule of the data model.

## The file rewrite

An accepted write (or a committed transaction) that actually changes the
state rewrites the file **by substitution**: Memless writes a full new copy of
the file next to the original, `fsync`s it, then renames it over the original,
so a reader never observes a half-written file and a crash mid-write leaves
the original untouched. A write that changes nothing never touches the disk.

That rewrite briefly creates a sibling residue file named `.<name>.memless-tmp`
next to `<name>.yaml`; it is removed by the same rename that completes the
write. Add `.*.memless-tmp` (or the exact residue names your fixtures use) to
your project's `.gitignore` so a fixture directory under test never commits a
leftover temp file.

## Installation

Install the bridge for your language with its own package manager. Each
package brings the native library for your platform with it, so there is
nothing to download or configure by hand:

```sh
npm install @nascent-tech/memless                              # Node.js 18 or later
composer require nascent-tech/memless                          # PHP 8.1 or later, ffi extension
go get github.com/nascent-tech/memless-go                      # Go 1.21 or later
```

The library is bundled for four platforms: macOS on Apple silicon
(`darwin-arm64`) and on Intel (`darwin-x64`), and Linux with glibc on x86_64
(`linux-x64-gnu`) and aarch64 (`linux-arm64-gnu`). The Linux libraries are
built on Ubuntu 24.04 and need glibc 2.39 or later (Ubuntu 24.04 or later, or
a distribution of the same age); on an older glibc, build the library locally
and set `MEMLESS_LIB` (see below). How each package carries it:

- **npm** — `@nascent-tech/memless` ships `lib/<platform>/` for the four
  platforms and loads the one that matches your machine. See
  [`bindings/node/README.md`](bindings/node/README.md).
- **Composer** — the package ships `lib/<platform>/` for the four platforms,
  and the C header the FFI extension needs. See
  [`bindings/php/README.md`](bindings/php/README.md).
- **Go** — the module `github.com/nascent-tech/memless-go`, published from
  `bindings/go` by the mirror repository `nascent-tech/memless-go`, embeds the
  library of the platform you build for (and only that one); on first use it
  is extracted once into your user cache directory
  (`memless/<version>-<checksum>/`) and checked against its SHA-256 before
  every load. Versions up to 0.2.1 were published at
  `github.com/nascent-tech/memless/bindings/go` and stay there. See
  [`bindings/go/README.md`](bindings/go/README.md).

All three bridges look for the library in the same order, on the first call
that needs it (never at import time):

1. `MEMLESS_LIB`, if it is set — it must name an existing file, or the call
   fails rather than falling back;
2. the library the package bundles for the current platform;
3. `target/release/`, then `target/debug/`, inside a checked-out workspace
   (`.dylib` before `.so`).

If none is found, the call fails with a message that tells you to set
`MEMLESS_LIB`. `MEMLESS_LIB` loads arbitrary native code, like any FFI library
path: only point it at a library you trust. The bridges require a library of
ABI version 5.

### Other platforms, or your own build

On Linux with musl or with a libc the bridge cannot tell (no `/usr/bin/ldd`,
no dynamic loader under `/lib` and, for Node, no diagnostic report) — neither
has a bundled library —, on a glibc older than 2.39,
or to use a library you built yourself, set `MEMLESS_LIB` to its path (Windows is not supported).
Either:

- **build it from source** (needs a Rust toolchain):

  ```sh
  cargo build --release -p memless-capi
  ```

  which produces `target/release/libmemless_capi.{dylib,so}`; inside a
  checked-out workspace every bridge finds it there with no configuration;
- or **download a release archive** — `memless-capi-<version>-<target>.tar.gz`
  from the project's GitHub Releases, which also carry the checksums
  (`SHA256SUMS`, and `memless-lib-SHA256SUMS` for the bundled libraries) — and
  point `MEMLESS_LIB` at the extracted library (an absolute path is safest).

## Parity and running the tests

The three bridges are held to the **same** behaviour by
`harness/parity/run.sh`, which replays a shared battery of fixtures, queries,
writes, transactions and reloads through all three and requires the same issue
and the same message from each, rewritten file included. Build the cdylib
first, install the Node bridge's own dependencies (koffi) before the Node
driver, which links to it by path, install the PHP driver, then run it:

```sh
cargo build --release -p memless-capi
npm ci --prefix bindings/node
npm ci --prefix harness/parity/node
composer install -d harness/parity/php
bash harness/parity/run.sh
```

It uses the most recently built library under `target/` (or `MEMLESS_LIB`) and
ends, when all three agree, with:

```text
parity: all fixtures, queries, writes, transactions and reloads agree on all three bridges
```

Its own regression tests check that it does catch a divergence:

```sh
for test in harness/parity/tests/detects-*.sh; do bash "$test"; done
```

Every bridge also carries its own unit tests:

```sh
# Rust workspace: the four gates
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release

# PHP bridge
(cd bindings/php && composer install && vendor/bin/phpunit)

# Go bridge
(cd bindings/go && go test ./...)

# Node bridge
npm ci --prefix bindings/node
npm test --prefix bindings/node
npm run typecheck --prefix bindings/node
```

## Supported targets

Memless is built and tested for four target families: `aarch64-apple-darwin`,
`x86_64-apple-darwin`, `x86_64-unknown-linux-gnu` and
`aarch64-unknown-linux-gnu`, which the packages call `darwin-arm64`,
`darwin-x64`, `linux-x64-gnu` and `linux-arm64-gnu`. Windows and musl targets
are not supported. How a release reaches npm, Packagist and the Go module
proxy is described for maintainers in
[`docs/PUBLISHING.md`](docs/PUBLISHING.md).

## License

MIT — see [`LICENSE`](LICENSE).
