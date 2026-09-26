#!/usr/bin/env bash
# Regression test for run.sh: a Node driver that always accepts must make the
# launcher name a divergence on a refusal fixture, still agree on the accepted
# ones, and exit 1 (a divergence, not a setup error). Mirrors
# detects-divergence.sh, which does the same for MEMLESS_GO_DRIVER.
set -uo pipefail

here=$(cd "$(dirname "$0")" && pwd) || exit 1
fake="$here/.fake-accept-node-driver"
printf '#!/usr/bin/env bash\necho accepted\n' >"$fake"
chmod +x "$fake"
trap 'rm -f "$fake"' EXIT

out=$(MEMLESS_NODE_DRIVER="$fake" bash "$here/../run.sh" 2>/dev/null)
status=$?

if [ "$status" -ne 1 ]; then
	echo "regression: expected exit 1 (divergence), got $status"
	echo "$out"
	exit 1
fi
if ! grep -q '^DIVERGENCE empty.yaml' <<<"$out"; then
	echo "regression: the divergence on empty.yaml was not named"
	echo "$out"
	exit 1
fi
if grep -q '^DIVERGENCE start.yaml' <<<"$out"; then
	echo "regression: start.yaml should still agree (all three accepted)"
	echo "$out"
	exit 1
fi
if ! grep -q '^QUERY DRIVER FAILURE' <<<"$out"; then
	echo "regression: a bad query driver (output not ok/refused/fault) was not caught"
	echo "$out"
	exit 1
fi

echo "regression: load divergence and bad query driver detected on the node leg and named as expected"
exit 0
