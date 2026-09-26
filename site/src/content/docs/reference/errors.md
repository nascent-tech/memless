---
title: Errors
description: The two kinds of failure, and how each language exposes them.
sidebar:
  order: 2
---

Every failure is one of two kinds, with the **same message in every
language**.

| Kind | Meaning | Node.js | PHP | Go |
| --- | --- | --- | --- | --- |
| **Refusal** | Your file or your SQL breaks a rule: an unknown table or column, a broken relation, unsupported SQL, a transaction already open… | `MemlessRefusal` | `Memless\MemlessRefusal` | `*memless.RefusalError` |
| **Fault** | A misuse of the bridge, such as calling a released instance, or an internal error. The message reads `memless fault (<status>): <message>`. | `MemlessFault` (`.status`) | `Memless\MemlessFault` (`->status`) | `*memless.FaultError` (`.Status`) |

## Asserting on a refusal

A refusal is a normal outcome that you can assert on in a test:

```js
assert.throws(() => db.query('SELECT nope FROM users'), { message: 'no column "nope" in table "users"' });
```

## When the library cannot be loaded

If the native library cannot be found, or is of an incompatible version, the
first call fails with a plain error — `Error` in Node.js, `\LogicException` in
PHP, `error` in Go — that says what to do. See
[Platforms and troubleshooting](../../guides/platforms/).

## Language guides

Each bridge documents its full API in its own guide:
[Node.js](https://github.com/nascent-tech/memless/blob/main/bindings/node/README.md) ·
[PHP](https://github.com/nascent-tech/memless/blob/main/bindings/php/README.md) ·
[Go](https://github.com/nascent-tech/memless/blob/main/bindings/go/README.md).
