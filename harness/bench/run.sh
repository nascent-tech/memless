#!/usr/bin/env bash
# Builds and runs the write bench (crates/memless-engine/examples/bench_write.rs),
# the transaction bench (examples/bench_transaction.rs) and the reload bench
# (examples/bench_reload.rs). The write bench prints, for k = 4, 100, 1000, 10000,
# the p50/p95 (over 20 reps, in microseconds) of a load, one isolated write
# (rewrite included), and a suite of 100 writes. The transaction bench prints, for
# k = 1, 10, 100, 1000 writes, the p50/p95 of a committed transaction of k writes
# (one rewrite) versus k isolated writes (k rewrites). The reload bench prints, for
# k = 4, 100, 1000, 10000, the p50/p95 of a full load, one reload of a live
# instance, and a suite of 100 reloads.
set -uo pipefail

here=$(cd "$(dirname "$0")" && pwd) || exit 1
root=$(cd "$here/../.." && pwd) || exit 1

examples=(--example bench_write --example bench_transaction --example bench_reload)
if ! (cd "$root" || exit 1; cargo build --release "${examples[@]}" >&2); then
	echo "bench build failed" >&2
	exit 1
fi

echo "# write bench"
"$root/target/release/examples/bench_write"
echo "# transaction bench"
"$root/target/release/examples/bench_transaction"
echo "# reload bench"
exec "$root/target/release/examples/bench_reload"
