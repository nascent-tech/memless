# Memless

**Query and change a YAML file with SQL, from PHP, Go or Node.js.**

Memless loads a YAML file of test data into memory, lets you run plain SQL
against it, and writes every accepted change back to the file. There is no
schema to declare and no server to start: Memless reads the structure from the
data itself — value types, row identities, and relations between tables.

It is made for **test fixtures and demos**: data you can read, review and diff
in Git, and query from your tests with the SQL you already know. The same file
and the same SQL give the same result, and the same error message, in all
three languages.

```yaml
# data.yaml
users:
  - id: 1
    name: Ada
  - id: 2
    name: Grace
wallets:
  - id: w1
    user_id: 2        # points at users.id = 2, because the column is named user_id
    amount: 100
```

```js
const { load } = require('@nascent-tech/memless');

const db = load('data.yaml');
db.query('SELECT users.name, wallets.amount FROM wallets INNER JOIN users ON wallets.user_id = users.id');
// { columns: ['users.name', 'wallets.amount'], rows: [['Grace', 100]] }

db.execute("UPDATE wallets SET amount = 150 WHERE id = 'w1'"); // data.yaml is rewritten
db.release();
```

## Contents

- [Install](#install)
- [Quick start](#quick-start)
- [Writing the YAML file](#writing-the-yaml-file)
- [Supported SQL](#supported-sql)
- [Writes, transactions and reload](#writes-transactions-and-reload)
- [Errors](#errors)
- [Platforms and troubleshooting](#platforms-and-troubleshooting)
- [When not to use Memless](#when-not-to-use-memless)
- [Contributing](#contributing)

## Install

Pick your language. Each package ships the native engine for your platform:
there is nothing else to download or configure.

| Language | Command | Requires |
| --- | --- | --- |
| Node.js | `npm install @nascent-tech/memless` | Node.js 18 or later |
| PHP | `composer require nascent-tech/memless` | PHP 8.1 or later, with the `ffi` extension enabled |
| Go | `go get github.com/nascent-tech/memless-go` | Go 1.21 or later; no cgo needed |

Supported platforms: macOS (Apple silicon and Intel) and Linux with glibc 2.39
or later (x86_64 and aarch64). Windows and musl-based Linux (Alpine) are not
supported out of the box; see [Platforms and troubleshooting](#platforms-and-troubleshooting).

Each bridge has its own guide, with its full API:
[Node.js](bindings/node/README.md) · [PHP](bindings/php/README.md) · [Go](bindings/go/README.md).

## Quick start

The three examples below load `data.yaml` from above, move money between two
wallets inside a transaction, then read the wallets back.

**Node.js**

```js
const { load } = require('@nascent-tech/memless');

const db = load('data.yaml');
try {
  db.begin();
  db.execute("UPDATE wallets SET amount = 50 WHERE id = 'w1'");
  db.execute("INSERT INTO wallets (id, user_id, amount) VALUES ('w2', 1, 50)");
  db.commit(); // data.yaml is rewritten once, here

  const { columns, rows } = db.query('SELECT id, amount FROM wallets ORDER BY amount DESC');
  console.log(columns, rows); // [ 'id', 'amount' ] [ [ 'w1', 50 ], [ 'w2', 50 ] ]
} finally {
  db.release();
}
```

**PHP**

```php
<?php

require 'vendor/autoload.php';

use Memless\Instance;

$db = Instance::load('data.yaml');

$db->begin();
$db->execute("UPDATE wallets SET amount = 50 WHERE id = 'w1'");
$db->execute("INSERT INTO wallets (id, user_id, amount) VALUES ('w2', 1, 50)");
$db->commit(); // data.yaml is rewritten once, here

print_r($db->query('SELECT id, amount FROM wallets ORDER BY amount DESC'));
// [['id' => 'w1', 'amount' => 50], ['id' => 'w2', 'amount' => 50]]

$db->release();
```

**Go**

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

	check := func(err error) {
		if err != nil {
			log.Fatal(err)
		}
	}

	check(db.Begin())
	_, err = db.Execute("UPDATE wallets SET amount = 50 WHERE id = 'w1'")
	check(err)
	_, err = db.Execute("INSERT INTO wallets (id, user_id, amount) VALUES ('w2', 1, 50)")
	check(err)
	check(db.Commit()) // data.yaml is rewritten once, here

	rows, err := db.Query("SELECT id, amount FROM wallets ORDER BY amount DESC")
	check(err)
	fmt.Println(rows.Columns, rows.Rows) // [id amount] [[w1 50] [w2 50]]
}
```

### Using it in tests

Memless writes to the file it loaded. In a test, load a **copy** of your
fixture, so that every test starts from the same data and the original stays
untouched:

```js
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { load } = require('@nascent-tech/memless');

function loadFixture(name) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'fixture-'));
  const copy = path.join(dir, path.basename(name));
  fs.copyFileSync(name, copy);
  return load(copy);
}
```

To keep the original instead, open a transaction at the start of the test and
roll it back at the end: nothing is written until a `COMMIT`.

## Writing the YAML file

A Memless file is a YAML map: **each key is a table, each table is a list of
rows, each row is a map of column names to values.** From that shape alone,
Memless works out the rest.

| Rule | What Memless expects |
| --- | --- |
| **Row identity** | Every row has an `id`, either text or an integer, unique within its table. Text and integer ids never match each other: `5` and `"5"` are two different ids. |
| **Relations** | A column named `<name>_id` points at the `id` of the table `<name>s`, when that table exists: `user_id` → `users`, `wallet_id` → `wallets`. The plural is always `<name>` + `s`: `category_id` points at a table named `categorys`. When no such table exists, the column is an ordinary value. Every value of a relation column must name an existing row. |
| **Values** | Each value is text, an integer, a decimal or a boolean. Lists and maps inside a row are refused. |
| **Missing values** | A row may leave out any column except `id`. A value of `null` (or `~`) counts as missing. Missing values read back as `null` (`nil` in Go) and match `IS NULL`. |
| **Mixed types** | A column may hold different types in different rows, but sorting on it or summing it is refused, rather than guessed. |

When the file breaks a rule — no `id`, a duplicate `id`, a relation to a row
that does not exist, a nested value, invalid YAML — `load` refuses it with a
message that names the table and the row, such as
`row 2 in "users" has no id` or
`broken relation "user_id" of row "w2" in "wallets": no row 9 in "users"`.

## Supported SQL

Memless understands a deliberate subset of SQL. Anything outside it is refused
with `<construct> is outside the supported SQL subset`, never approximated.

| Statement | Supported form |
| --- | --- |
| `SELECT` | `SELECT <columns> \| * FROM <table>`, with optional `WHERE`, `ORDER BY` and one `JOIN` |
| `WHERE` | `=`, `<>`, `<`, `<=`, `>`, `>=`, `IS NULL`, `IS NOT NULL`, combined with `AND`, `OR` and parentheses |
| `JOIN` | One `INNER JOIN <table> ON <a>.<name>_id = <b>.id`, on a relation Memless recognised; every column is then written `table.column` |
| `ORDER BY` | One or more columns, each `ASC` (default) or `DESC`. Ties keep the file order; missing values always come last |
| Aggregates | `SELECT COUNT(*)` or `SELECT SUM(<column>)`, alone in the select list |
| `INSERT` | `INSERT INTO <table> (<columns>) VALUES (<values>)`; the column list is required |
| `UPDATE` | `UPDATE <table> SET <column> = <value>, … [WHERE …]` |
| `DELETE` | `DELETE FROM <table> [WHERE …]` |
| Transactions | `BEGIN`, `COMMIT`, `ROLLBACK` (also available as methods) |

Not supported: `GROUP BY`, `LIMIT` and `OFFSET`, more than one `JOIN`,
`JOIN … USING`, `NATURAL JOIN`, `LIKE`, `IN`, arithmetic, subqueries, `WITH`,
window functions, `CREATE`, `ALTER`, `DROP` and any other statement not listed
above.

A `SELECT` returns the column names and the rows. Query results never contain
the same column name twice: `SELECT name, name` is refused.

## Writes, transactions and reload

- **Each accepted write is saved immediately.** Outside a transaction, an
  `INSERT`, `UPDATE` or `DELETE` that changes something rewrites the file
  before the call returns. A write that changes nothing leaves the file
  untouched. `execute` returns the number of rows affected.
- **Transactions group writes.** After `begin`, writes change memory only, and
  queries already see them. `commit` checks the whole result and writes the
  file once; `rollback` throws the changes away. If `commit` is refused, the
  data goes back to its state before `begin`. A refused write inside a
  transaction leaves the transaction open.
- **`reload` rereads the file** after something else changed it, exactly as a
  new `load` would. It is refused while a transaction is open, and a refused
  reload keeps the current data.
- **The file is replaced safely.** Memless writes a complete new copy next to
  the file (`.<name>.memless-tmp`), then renames it over the original, so a
  crash never leaves a half-written file. Add `.*.memless-tmp` to your
  `.gitignore`.
- **One process per file.** Memless does not coordinate writers: do not change
  the same file from two instances or two processes at once.
- **Release the instance** when you are done (`release()`, `Release()` in Go),
  or let PHP's destructor do it.

## Errors

Every failure is one of two kinds, with the **same message in every
language**:

| Kind | Meaning | Node.js | PHP | Go |
| --- | --- | --- | --- | --- |
| **Refusal** | Your file or your SQL breaks a rule: an unknown table or column, a broken relation, unsupported SQL, a transaction already open… | `MemlessRefusal` | `Memless\MemlessRefusal` | `*memless.RefusalError` |
| **Fault** | A misuse of the bridge, such as calling a released instance, or an internal error. The message reads `memless fault (<status>): <message>`. | `MemlessFault` (`.status`) | `Memless\MemlessFault` (`->status`) | `*memless.FaultError` (`.Status`) |

A refusal is a normal outcome that you can assert on in a test:

```js
assert.throws(() => db.query('SELECT nope FROM users'), { message: 'no column "nope" in table "users"' });
```

If the native library cannot be found, or is of an incompatible version, the
first call fails with a plain error (`Error` in Node.js, `\LogicException` in
PHP, `error` in Go) that says what to do.

## Platforms and troubleshooting

The packages include the engine for macOS (Apple silicon, Intel) and for Linux
with glibc 2.39 or later (x86_64, aarch64), such as Ubuntu 24.04, Debian 13 or
Fedora 40 and later. Each bridge finds it on its own, in this order:

1. the file named by the `MEMLESS_LIB` environment variable, if it is set;
2. the library bundled in the package for your platform;
3. inside a clone of this repository, `target/release/`, then `target/debug/`.

**Your platform is not covered** — Alpine or another musl-based Linux, an
older glibc, or a platform whose libc Memless cannot identify: build the
library yourself and point `MEMLESS_LIB` at it.

```sh
git clone https://github.com/nascent-tech/memless.git
cd memless
cargo build --release -p memless-capi   # needs a Rust toolchain
export MEMLESS_LIB="$PWD/target/release/libmemless_capi.so"   # .dylib on macOS
```

You can also download a prebuilt library, `memless-capi-<version>-<target>.tar.gz`,
from the [GitHub releases](https://github.com/nascent-tech/memless/releases),
with its checksums in `SHA256SUMS`.

**PHP says the `FFI` class does not exist** — enable the extension:
`extension=ffi` in `php.ini`. On the command line, FFI is allowed by default
(`ffi.enable=preload` covers the CLI); for a web server, see the
[PHP guide](bindings/php/README.md).

`MEMLESS_LIB` loads native code into your process: only point it at a library
you trust.

## When not to use Memless

- **Not for production data.** There is no concurrency control, no
  durability guarantee beyond the atomic file replacement, and no access
  control.
- **Not a server.** The engine runs inside your process; nothing listens on a
  port.
- **Not a general SQL database.** It supports the subset above and refuses the
  rest. If you need `GROUP BY`, several joins or large data, use SQLite or a
  real database.

## Contributing

Contributions are welcome — bug reports, fixes, documentation, and ideas
discussed in an issue first. Everything happens in this repository: the Rust
engine, the three bridges and the test harness that keeps them in agreement.

- Read [`CONTRIBUTING.md`](CONTRIBUTING.md) to set up the project and run the
  checks.
- [Open an issue](https://github.com/nascent-tech/memless/issues/new/choose)
  for a bug, a feature request or a question. Issues labelled
  [`good first issue`](https://github.com/nascent-tech/memless/labels/good%20first%20issue)
  are a good start.
- Report a vulnerability privately, as described in [`SECURITY.md`](SECURITY.md).
- Follow the [code of conduct](CODE_OF_CONDUCT.md).

The repositories `nascent-tech/memless-php`, `nascent-tech/memless-go` and
`nascent-tech/memless-node` are read-only copies published by each release;
please open issues and pull requests here. Release notes are in
[`CHANGELOG.md`](CHANGELOG.md).

## License

[MIT](LICENSE). By contributing, you agree that your contribution is released
under the same license.
