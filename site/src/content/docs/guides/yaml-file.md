---
title: Writing the YAML file
description: The shape Memless expects, and the rules it reads from your data.
sidebar:
  order: 1
---

A Memless file is a YAML map: **each key is a table, each table is a list of
rows, each row is a map of column names to values.** From that shape alone,
Memless works out the rest.

## Row identity

Every row has an `id`, either text or an integer, unique within its table.
Text and integer ids never match each other: `5` and `"5"` are two different
ids.

## Relations

A column named `<name>_id` points at the `id` of the table `<name>s`, when that
table exists: `user_id` → `users`, `wallet_id` → `wallets`. The plural is
always `<name>` + `s`: `category_id` points at a table named `categorys`.

When no such table exists, the column is an ordinary value. Every value present
in a relation column must name an existing row; a missing value is allowed.

## Values

Each value is text, an integer, a decimal or a boolean. Lists and maps inside a
row are refused.

A row may leave out any column except `id`. A value of `null` (or `~`) counts
as missing. Missing values read back as `null` (`nil` in Go) and match
`IS NULL`.

## Types never convert

An integer (`2`) and a decimal (`2.0`) are two different types, like text and
integer. A comparison only matches values of the same type: `WHERE amount > 1`
skips rows whose `amount` is `1.5`, and `WHERE amount = 2.0` does not match
`2`. Keep one type per column: write `2.0`, not `2`, in a decimal column.

A column may hold different types in different rows, but sorting or summing
rows of different types is refused, rather than guessed.

## When the file is refused

When the file breaks a rule — no `id`, a duplicate `id`, a relation to a row
that does not exist, a nested value, invalid YAML — `load` refuses it with a
message that names the table and the row:

```text
row 2 in "users" has no id
broken relation "user_id" of row "w2" in "wallets": no row 9 in "users"
```
