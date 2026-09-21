#!/usr/bin/env bash
# Regression test for run.sh: a Go driver that always accepts must make the
# launcher name a divergence on a refusal fixture, still agree on the accepted
# ones, and exit 1 (a divergence, not a setup error).
set -uo pipefail

here=$(cd "$(dirname "$0")" && pwd) || exit 1
fake="$here/.fake-accept-driver"
printf '#!/usr/bin/env bash\necho accepted\n' >"$fake"
chmod +x "$fake"
trap 'rm -f "$fake"' EXIT

out=$(MEMLESS_GO_DRIVER="$fake" bash "$here/../run.sh" 2>/dev/null)
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
	echo "regression: start.yaml should still agree (both accepted)"
	echo "$out"
	exit 1
fi

echo "regression: divergence detected and named as expected"
exit 0
