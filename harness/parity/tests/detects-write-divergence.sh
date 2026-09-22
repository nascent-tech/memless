#!/usr/bin/env bash
# Regression test for run.sh: a Go driver whose rewritten bytes differ by one
# (a tampered sha256 line) must make the launcher name a WRITE DIVERGENCE and
# exit 1, while load and query still agree.
set -uo pipefail

here=$(cd "$(dirname "$0")" && pwd) || exit 1

if ! (cd "$here/../go" || exit 1; go build -o driver .); then
	echo "go driver build failed" >&2
	exit 2
fi
real="$here/../go/driver"

fake="$here/.fake-write-driver"
cat >"$fake" <<EOF
#!/usr/bin/env bash
out=\$("$real" "\$@")
case "\${3:-}" in
write | write-disk) out=\$(printf '%s\n' "\$out" | sed 's/^sha256:.*/sha256:tampered/') ;;
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
if ! grep -q '^WRITE DIVERGENCE' <<<"$out"; then
	echo "regression: a tampered write (different rewritten bytes) was not caught"
	echo "$out"
	exit 1
fi

echo "regression: a write divergence was detected and named as expected"
exit 0
