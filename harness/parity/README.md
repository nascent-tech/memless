# Parity harness

The **battery** has five parts: the fixtures under `fixtures/` that must load or
refuse (palier 1); the queries in `queries.txt` (`<fixture><TAB><sql>`) that must
return the same rows or the same refusal (palier 2); the writes in `writes.txt`
(`<fixture><TAB><sql>`, or `!disk<TAB><fixture><TAB><sql>`) that must apply, refuse
or fail identically and rewrite the same bytes (palier 3); and the transactions in
`transactions.txt` (`<fixture><TAB><sql1>;;<sql2>;;…`, or `!disk<TAB>…`) that must
agree instruction by instruction and commit the same bytes (palier 4); and the
reload scenarios in `reloads.txt` (`<fixture><TAB><scenario>`, one of `restore`,
`open`, `broken`, `missing`) that must agree line by line and rewrite nothing to
disk on their own (palier 5). The **launcher** (`run.sh`) replays each through the
PHP bridge, the Go bridge and the Node bridge and requires the **same issue and the
same message** from all three:

- loads compare `accepted` / `refused: ` / `fault: `;
- queries compare a canonical rendering (columns, rows, per-cell kind — decimals as
  IEEE-754 bits, and an integer beyond the safe range in its exact decimal text
  (Go's int64, PHP's platform int64, Node's BigInt), so all three languages agree —
  or `refused: ` / `fault: `);
- writes compare one block: `accepted:<n>` / `refused:<msg>` / `fault:<msg>`, then
  `sha256:<hex|->` of the rewritten file, then `residue:<yes|no>`;
- transactions (mode `transaction`, or `transaction-disk` for a `!disk` line) run
  the `;;`-separated suite in order on one instance and print each instruction's
  issue — a write verb as `accepted:<n>` / `refused:<msg>`, a `SELECT` as its rows
  (it sees the open transaction's working state) — then `sha256:` and `residue:`;
- reloads (mode `reload`) run one named scenario's fixed script on `start.yaml` and
  print each step's issue — `reloaded` on Ok, `refused:<msg>` / `fault:<msg>`
  otherwise, a `SELECT` as its rows — then `sha256:` and `residue:`.

Two conventions the bridges share, so a fix on one side must match the others: a
write or transaction-verb refusal is `refused:<msg>` (no space), a load/query/
`SELECT` refusal is `refused: <msg>` (with a space); a disk-failure message has its
temp path normalised to the fixture basename. The **parity record** is the output:
either `parity: all fixtures, queries, writes, transactions and reloads agree on
all three bridges` (exit 0), or a `DIVERGENCE` / `QUERY DIVERGENCE` /
`WRITE DIVERGENCE` / `TRANSACTION DIVERGENCE` / `RELOAD DIVERGENCE` /
`… DRIVER FAILURE` / `… DISK NOT REFUSED` line per problem (exit non-zero). It
uses the newest built `libmemless_capi`, or `MEMLESS_LIB`.

The Node driver (`node/`) reads cells through the bridge's own internal modules
(`@nascent-tech/memless/src/library`, `.../kind`, `.../cell-value`) instead of its
ergonomic `Instance#query`: a plain JS number cannot tell an integer from a decimal
apart the way Go's `int64`/`float64` or PHP's typed FFI can, so the driver keeps the
kind tag alongside the value until it renders the `int:`/`dec:` tag.

## Running

```sh
cargo build -p memless-capi        # produce the cdylib the bridges load
composer install -d harness/parity/php
npm ci --prefix bindings/node             # the Node bridge's own dependency (koffi)
npm install --prefix harness/parity/node  # the driver's file: dependency on the bridge
bash harness/parity/run.sh
```

A fresh clone has no `node_modules` anywhere: `npm install --prefix harness/parity/node`
only links the driver to the bridge (a `file:` dependency), it does not install the
bridge's own `koffi` — that needs its own `npm ci` in `bindings/node` first, or the
Node driver fails to load the library. `run.sh` checks for both and names whichever
is missing.

`run.sh` resolves the cdylib under `target/` and exports `MEMLESS_LIB` and
`MEMLESS_HEADER` for all three bridges, builds the Go driver once, runs every
fixture, query, write, transaction and reload through PHP, Go and Node, checks
that the neighbouring residue (`.start.yaml.memless-tmp`) is **byte-for-byte
intact** after a successful load (decision D16), and sources `queries.sh`,
`writes.sh`, `transactions.sh` and `reloads.sh` for the four tabular parts. A
`!disk` line forces the atomic
rewrite to fail (`chmod 0555`) and is skipped under root. Each driver can be
overridden — `MEMLESS_PHP_DRIVER`, `MEMLESS_GO_DRIVER`, `MEMLESS_NODE_DRIVER` — with
a command to run instead of the real one, which the regression tests use to inject
a divergence.

The regression tests prove the launcher actually detects a disagreement:

```sh
bash harness/parity/tests/detects-divergence.sh              # a load/query (go)
bash harness/parity/tests/detects-node-divergence.sh         # a load/query (node)
bash harness/parity/tests/detects-write-divergence.sh        # a rewritten write
bash harness/parity/tests/detects-transaction-divergence.sh  # a committed transaction
bash harness/parity/tests/detects-reload-divergence.sh       # a restored reload
```

## What zero divergence opens — and does not close

Zero divergence across these fixtures shows the PHP, Go and Node bridges speak with
one voice on one family of systems. It **opens** hypothesis §17.1 (the real parity
of PHP), it does not close it: that is settled across every family of systems.
Timing lives in the bench (`harness/bench/run.sh`, `.charpente/releves/banc.md`).
