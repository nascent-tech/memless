---
title: Writes, transactions and reload
description: When Memless writes the file, how transactions group writes, and how to reread a file.
sidebar:
  order: 2
---

## Each accepted write is saved immediately

Outside a transaction, an `INSERT`, `UPDATE` or `DELETE` that changes something
rewrites the file before the call returns. A write that changes nothing leaves
the file untouched. `execute` returns the number of rows affected.

## Transactions group writes

After `begin`, writes change memory only, and queries already see them.
`commit` checks the whole result and writes the file once; `rollback` throws
the changes away.

- If `commit` is refused, the data goes back to its state before `begin`.
- A refused write inside a transaction leaves the transaction open.

In a test, a transaction opened at the start and rolled back at the end keeps
the fixture file untouched.

## Reload rereads the file

`reload` rereads the file after something else changed it, exactly as a new
`load` would. It is refused while a transaction is open, and a refused reload
keeps the current data.

## The file is replaced safely

Memless writes a complete new copy next to the file (`.<name>.memless-tmp`),
then renames it over the original, so a crash never leaves a half-written
file. Add this line to your `.gitignore`:

```text title=".gitignore"
.*.memless-tmp
```

## One process per file

Memless does not coordinate writers: do not change the same file from two
instances or two processes at once.

## Release the instance

Release the instance when you are done — `release()`, or `Release()` in Go —
or let PHP's destructor do it.
