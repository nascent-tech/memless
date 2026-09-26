#!/usr/bin/env bash
# Regression test for run.sh: a Go driver whose sha256 line differs on the
# "restore" reload scenario must make the launcher name a RELOAD DIVERGENCE
# and exit 1, while load, query, write and transactions still agree (their
# lines are untouched).
set -uo pipefail

here=$(cd "$(dirname "$0")" && pwd) || exit 1

if ! (cd "$here/../go" || exit 1; go build -o driver .); then
	echo "go driver build failed" >&2
	exit 2
fi
real="$here/../go/driver"

fake="$here/.fake-reload-driver"
cat >"$fake" <<EOF
#!/usr/bin/env bash
out=\$("$real" "\$@")
case "\${3:-}" in
reload)
	case "\${2:-}" in
	restore) out=\$(printf '%s\n' "\$out" | sed 's/^sha256:.*/sha256:tampered/') ;;
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
if ! grep -q '^RELOAD DIVERGENCE' <<<"$out"; then
	echo "regression: a tampered restore reload (different rewritten bytes) was not caught"
	echo "$out"
	exit 1
fi
if grep -qE '^(DIVERGENCE|QUERY DIVERGENCE|WRITE DIVERGENCE|TRANSACTION DIVERGENCE)' <<<"$out"; then
	echo "regression: only the restore reload was tampered, yet another category diverged"
	echo "$out"
	exit 1
fi

echo "regression: a reload divergence was detected and named as expected"
exit 0
