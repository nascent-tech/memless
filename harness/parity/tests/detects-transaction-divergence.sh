#!/usr/bin/env bash
# Regression test for run.sh: a Go driver whose committed sha256 line differs on
# a transaction that COMMITs must make the launcher name a TRANSACTION DIVERGENCE
# and exit 1, while load, query and write still agree (their lines are untouched).
set -uo pipefail

here=$(cd "$(dirname "$0")" && pwd) || exit 1

if ! (cd "$here/../go" || exit 1; go build -o driver .); then
	echo "go driver build failed" >&2
	exit 2
fi
real="$here/../go/driver"

fake="$here/.fake-transaction-driver"
cat >"$fake" <<EOF
#!/usr/bin/env bash
out=\$("$real" "\$@")
case "\${3:-}" in
transaction | transaction-disk)
	case "\${2:-}" in
	*COMMIT*) out=\$(printf '%s\n' "\$out" | sed 's/^sha256:.*/sha256:tampered/') ;;
	esac
	;;
esac
printf '%s' "\$out"
EOF
chmod +x "$fake"
trap 'rm -f "$fake"' EXIT

out=$(MEMLESS_GO_DRIVER="$fake" bash "$here/../run.sh" 2>/dev/null)
status=$?

if [ "$status" -ne 1 ]; then
	echo "regression: expected exit 1 (divergence), got $status"
	echo "$out"
	exit 1
fi
if ! grep -q '^TRANSACTION DIVERGENCE' <<<"$out"; then
	echo "regression: a tampered commit (different committed bytes) was not caught"
	echo "$out"
	exit 1
fi
if grep -qE '^(DIVERGENCE|QUERY DIVERGENCE|WRITE DIVERGENCE)' <<<"$out"; then
	echo "regression: only the transaction commit was tampered, yet another category diverged"
	echo "$out"
	exit 1
fi

echo "regression: a transaction commit divergence was detected and named as expected"
exit 0
