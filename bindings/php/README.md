# memless — PHP bridge

A PHP bridge to the memless C ABI through [FFI](https://www.php.net/manual/en/book.ffi.php)
— no extension to compile, only `ext-ffi` enabled. It loads the same
`libmemless_capi` cdylib as the Go and Node bridges and speaks the same
contract (ABI version 5), so the three stay in parity from a single shared
surface. See the [project README](../../README.md) for what memless is, the
guessing rules and the supported SQL subset.

## Requirements

- PHP 8.1 or later, with the `ffi` extension enabled.

## Install

Memless is distributed only through GitHub Releases, not Packagist. Download
`memless-php-<version>.zip` from the release, extract it, and either:

- point a Composer *path* repository at the extracted directory, so
  `Memless\` autoloads through Composer's own autoloader; or
- `require` the classes under its `src/` directly — the archive ships without
  a `vendor/` directory, since it has no runtime dependency besides `ext-ffi`.

## Surface

```php
<?php

require 'vendor/autoload.php';

use Memless\Instance;
use Memless\MemlessFault;
use Memless\MemlessRefusal;

$db = Instance::load('data.yaml'); // throws MemlessRefusal or MemlessFault

$rows = $db->query('SELECT name FROM users');
// -> [['name' => 'Ada'], ['name' => 'Grace']]

$affected = $db->execute("UPDATE users SET name = 'Zoe' WHERE id = '01H7B2'");

$db->begin();                                   // BEGIN / COMMIT / ROLLBACK
$db->execute("DELETE FROM wallets WHERE id = 'w_123'");
$db->commit();

$db->reload();                                  // re-reads data.yaml

$db->release();
```

- `Instance::query()` returns an array of rows, each row an associative array
  keyed by (possibly qualified, e.g. `users.id`) column name; a cell is a PHP
  `string`, `int`, `float`, `bool`, or `null` for an absent value.
- `Instance::execute()` returns the affected row count (`0` for a transaction
  verb — `begin()`, `commit()` and `rollback()` are thin wrappers over it).
- `MemlessRefusal` (extends `\RuntimeException`) carries the domain message
  verbatim (D13); a boundary or internal fault instead throws `MemlessFault`
  (also a `\RuntimeException`), whose message reads
  `memless fault (<status>): <message>` — the same text as in the Go and Node
  bridges — and whose `status` property carries the ABI status (`2` for an
  invalid argument, such as a released instance or a NUL byte in the path or
  SQL, `3` for an internal fault).
- A library or header that cannot be found, or a library that speaks another
  ABI version, throws a `\LogicException` on the first call that needs it.
- `Instance::release()` is idempotent, and is also called automatically from
  the destructor if you never call it yourself.
- `Instance::reload()` re-reads the file from disk into a fresh in-memory
  state, exactly as `Instance::load()` would build it. It throws a
  `MemlessRefusal` while a transaction is open
  (`cannot reload while a transaction is open`) or when the file would be
  refused at load; the old state stays usable either way.

## The cdylib

The bridge loads the native library on the first call that needs it, never
at include time, and looks for it in the same order as the Go and Node
bridges: `MEMLESS_LIB`, a trusted (ideally absolute) path that must name an
existing file, then, inside a checked-out workspace, `target/release/`, then
`target/debug/` (`.dylib` before `.so`). The library must speak ABI version 5.
Build it first:

```sh
cargo build --release -p memless-capi
```

`MEMLESS_LIB` loads arbitrary native code, like any FFI library path — only
point it at a library you trust. The C header (`memless.h`) is found the same
way through `MEMLESS_HEADER`, defaulting to
`crates/memless-capi/include/memless.h`.

## Running the tests

```sh
composer install
vendor/bin/phpunit
```
