# memless — Go bridge

A Go bridge to the memless C ABI through [purego](https://github.com/ebitengine/purego)
— no cgo, no build step besides the pure-Go module. It loads the same
`libmemless_capi` cdylib as the PHP and Node bridges and speaks the same
contract (ABI version 5), so the three stay in parity from a single shared
surface. See the [project README](../../README.md) for what memless is, the
guessing rules and the supported SQL subset.

## Install

Memless is distributed only through GitHub Releases, not a module proxy you
need to configure by hand: Go modules resolve straight from the tag.

```sh
go get github.com/nascent-tech/memless/bindings/go@vX.Y.Z
```

Go resolves `vX.Y.Z` from the repository tag `bindings/go/vX.Y.Z`: the module
lives in a subdirectory, so that tag must exist next to the release tag
`vX.Y.Z`. Go 1.21 or later.

## Surface

```go
package main

import (
	"fmt"
	"log"

	memless "github.com/nascent-tech/memless/bindings/go"
)

func main() {
	db, err := memless.Load("data.yaml")
	if err != nil {
		log.Fatal(err) // *memless.RefusalError or *memless.FaultError
	}
	defer db.Release()

	rows, err := db.Query("SELECT name FROM users")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(rows.Columns, rows.Rows) // [name] [[Ada] [Grace]]

	affected, err := db.Execute("UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(affected)

	if err := db.Begin(); err != nil { // BEGIN
		log.Fatal(err)
	}
	if _, err := db.Execute("DELETE FROM wallets WHERE id = 'w_123'"); err != nil {
		log.Fatal(err)
	}
	if err := db.Commit(); err != nil { // COMMIT
		log.Fatal(err)
	}

	if err := db.Reload(); err != nil { // re-reads data.yaml
		log.Fatal(err)
	}
}
```

- `Instance.Query` returns `Rows{Columns []string, Rows [][]any}`; a cell is
  `int64`, `float64`, `string`, `bool`, or `nil` for an absent value.
- `Instance.Execute` returns the affected row count (`0` for `BEGIN` /
  `COMMIT` / `ROLLBACK`, which are also run through it — `Begin`, `Commit` and
  `Rollback` are thin wrappers over it).
- Every method returns a `*RefusalError` (the domain's message, verbatim —
  D13) or a `*FaultError` (a boundary or internal fault); tell them apart with
  `errors.As`. A fault reads `memless fault (<status>): <message>`, the same
  text as in the PHP and Node bridges, and carries the ABI status in
  `FaultError.Status` (`2` for an invalid argument, such as a released
  instance or a NUL byte in the SQL, `3` for an internal fault).
- A library that cannot be found or speaks another ABI version is a plain
  error, returned by the first call that needs the library.
- `Instance.Release` releases the handle once; a second call, a zero-value
  instance, or an unknown handle is ignored.
- `Instance.Reload` re-reads the file from disk into a fresh in-memory state,
  exactly as `Load` would build it. It returns a `*RefusalError` while a
  transaction is open (`cannot reload while a transaction is open`) or when
  the file would be refused by `Load`; the old state stays usable either way.

## The cdylib

The bridge loads the native library on the first call that needs it, never
at import time, and looks for it in the same order as the PHP and Node
bridges: `MEMLESS_LIB`, a trusted (ideally absolute) path that must name an
existing file, then, inside a checked-out workspace, `target/release/`, then
`target/debug/` (`.dylib` before `.so`). The library must speak ABI version 5.
Build it first:

```sh
cargo build --release -p memless-capi
```

`MEMLESS_LIB` loads arbitrary native code, like any FFI library path — only
point it at a library you trust.

## Running the tests

```sh
go test ./...
```
