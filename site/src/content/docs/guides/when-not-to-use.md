---
title: When not to use Memless
description: What Memless is not built for, and what to use instead.
sidebar:
  order: 4
---

- **Not for production data.** There is no concurrency control, no durability
  guarantee beyond the atomic file replacement, and no access control.
- **Not a server.** The engine runs inside your process; nothing listens on a
  port.
- **Not a general SQL database.** It supports [a subset of SQL](../../reference/sql/)
  and refuses the rest. If you need `GROUP BY`, several joins or large data,
  use SQLite or a real database.
