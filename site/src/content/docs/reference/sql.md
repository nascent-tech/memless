---
title: Supported SQL
description: The SQL subset Memless understands, and what it refuses.
sidebar:
  order: 1
---

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

## Not supported

`GROUP BY`, `LIMIT` and `OFFSET`, more than one `JOIN`, `JOIN … USING`,
`NATURAL JOIN`, `ORDER BY` by position (`ORDER BY 1`) or with
`NULLS FIRST`/`NULLS LAST`, `LIKE`, `IN`, arithmetic, subqueries, `WITH`,
window functions, `CREATE`, `ALTER`, `DROP` and any other statement not listed
above. A `SUM` that overflows is refused too.

## Results

A `SELECT` returns the column names and the rows. Query results never contain
the same column name twice: `SELECT name, name` is refused.
