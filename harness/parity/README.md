# Parity harness

The **battery** is the set of fixtures under `fixtures/` that palier 1 must load
or refuse. The **launcher** (`run.sh`) replays each one through the PHP bridge
and then the Go bridge and requires the **same issue and the same message** from
both. The **parity record** is its output: either `parity: all fixtures agree on
both bridges` (exit 0), or one `DIVERGENCE <fixture>: php=[…] go=[…]` line per
disagreement (exit non-zero).

## Running

```sh
cargo build -p memless-capi        # produce the cdylib the bridges load
composer install -d harness/parity/php
bash harness/parity/run.sh
```

`run.sh` resolves the cdylib under `target/` and exports `MEMLESS_LIB` and
`MEMLESS_HEADER` for both bridges, builds the Go driver once, runs every
fixture through both, and finally checks that the neighbouring residue
(`.start.yaml.memless-tmp`) is **byte-for-byte intact** after a successful load
(decision D16).

The regression test proves the launcher actually detects a disagreement:

```sh
bash harness/parity/tests/detects-divergence.sh
```

## What zero divergence opens — and does not close

Zero divergence across these fixtures shows the PHP and Go bridges speak with
one voice on one family of systems. It **opens** hypothesis §17.1 (the real
parity of PHP), it does not close it: that is settled at palier 4, across every
family of systems. This harness measures no duration — timing is palier 3–4 —
and replays no Node bridge.
