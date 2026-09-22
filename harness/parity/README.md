# Parity harness

The **battery** has four parts: the fixtures under `fixtures/` that must load or
refuse (palier 1); the queries in `queries.txt` (`<fixture><TAB><sql>`) that must
return the same rows or the same refusal (palier 2); the writes in `writes.txt`
(`<fixture><TAB><sql>`, or `!disk<TAB><fixture><TAB><sql>`) that must apply, refuse
or fail identically and rewrite the same bytes (palier 3); and the transactions in
`transactions.txt` (`<fixture><TAB><sql1>;;<sql2>;;…`, or `!disk<TAB>…`) that must
agree instruction by instruction and commit the same bytes (palier 4). The
**launcher** (`run.sh`) replays each through the PHP bridge and then the Go bridge
and requires the **same issue and the same message** from both:

- loads compare `accepted` / `refused: ` / `fault: `;
- queries compare a canonical rendering (columns, rows, per-cell kind — decimals as
  IEEE-754 bits so both languages agree — or `refused: ` / `fault: `);
- writes compare one block: `accepted:<n>` / `refused:<msg>` / `fault:<msg>`, then
  `sha256:<hex|->` of the rewritten file, then `residue:<yes|no>`;
- transactions (mode `transaction`, or `transaction-disk` for a `!disk` line) run
  the `;;`-separated suite in order on one instance and print each instruction's
  issue — a write verb as `accepted:<n>` / `refused:<msg>`, a `SELECT` as its rows
  (it sees the open transaction's working state) — then `sha256:` and `residue:`.

Two conventions the bridges share, so a fix on one side must match the other: a
write or transaction-verb refusal is `refused:<msg>` (no space), a load/query/
`SELECT` refusal is `refused: <msg>` (with a space); a disk-failure message has its
temp path normalised to the fixture basename. The **parity record** is the output:
either `parity: all fixtures, queries, writes and transactions agree on both
bridges` (exit 0), or a `DIVERGENCE` / `QUERY DIVERGENCE` / `WRITE DIVERGENCE` /
`TRANSACTION DIVERGENCE` / `… DRIVER FAILURE` / `… DISK NOT REFUSED` line per
problem (exit non-zero). It uses the newest built `libmemless_capi`, or
`MEMLESS_LIB`.

## Running

```sh
cargo build -p memless-capi        # produce the cdylib the bridges load
composer install -d harness/parity/php
bash harness/parity/run.sh
```

`run.sh` resolves the cdylib under `target/` and exports `MEMLESS_LIB` and
`MEMLESS_HEADER` for both bridges, builds the Go driver once, runs every fixture,
query, write and transaction through both, checks that the neighbouring residue
(`.start.yaml.memless-tmp`) is **byte-for-byte intact** after a successful load
(decision D16), and sources `transactions.sh` for the transaction loop. A `!disk`
line forces the atomic rewrite to fail (`chmod 0555`) and is skipped under root.

The regression tests prove the launcher actually detects a disagreement:

```sh
bash harness/parity/tests/detects-divergence.sh              # a load/query
bash harness/parity/tests/detects-write-divergence.sh        # a rewritten write
bash harness/parity/tests/detects-transaction-divergence.sh  # a committed transaction
```

## What zero divergence opens — and does not close

Zero divergence across these fixtures shows the PHP and Go bridges speak with one
voice on one family of systems. It **opens** hypothesis §17.1 (the real parity of
PHP), it does not close it: that is settled across every family of systems. Timing
lives in the bench (`harness/bench/run.sh`, `.charpente/releves/banc.md`); this
harness replays no Node bridge.
