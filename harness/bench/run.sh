#!/usr/bin/env bash
# Builds and runs the write bench (crates/memless-engine/examples/bench_write.rs)
# and prints a TSV: for k = 4, 100, 1000, 10000, the p50/p95 (over 20 reps, in
# microseconds) of a load, one isolated write (rewrite included), and a suite of
# 100 writes. Feed the numbers by hand into .charpente/releves/banc.md.
set -uo pipefail

here=$(cd "$(dirname "$0")" && pwd) || exit 1
root=$(cd "$here/../.." && pwd) || exit 1

if ! (cd "$root" || exit 1; cargo build --release --example bench_write >&2); then
	echo "bench build failed" >&2
	exit 1
fi

exec "$root/target/release/examples/bench_write"
