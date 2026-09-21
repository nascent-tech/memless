#!/usr/bin/env bash
# Replays every fixture through the PHP bridge and the Go bridge and requires
# the same issue and message from both. Exits 0 when all agree, non-zero with
# the list of divergences or driver failures otherwise.
set -uo pipefail
shopt -s nullglob

here=$(cd "$(dirname "$0")" && pwd) || exit 1
root=$(cd "$here/../.." && pwd) || exit 1

lib=""
for profile in debug release; do
	for ext in dylib so; do
		candidate="$root/target/$profile/libmemless_capi.$ext"
		if [ -f "$candidate" ]; then
			lib="$candidate"
			break 2
		fi
	done
done
if [ -z "$lib" ]; then
	echo "cdylib not found under $root/target; build memless-capi first" >&2
	exit 2
fi
export MEMLESS_LIB="${MEMLESS_LIB:-$lib}"
export MEMLESS_HEADER="${MEMLESS_HEADER:-$root/crates/memless-capi/include/memless.h}"

if [ -n "${MEMLESS_PHP_DRIVER:-}" ]; then
	php_driver=("$MEMLESS_PHP_DRIVER")
else
	if [ ! -f "$here/php/vendor/autoload.php" ]; then
		echo "php driver not installed; run: composer install -d $here/php" >&2
		exit 2
	fi
	php_driver=(php -d display_errors=stderr "$here/php/load.php")
fi

if [ -n "${MEMLESS_GO_DRIVER:-}" ]; then
	go_driver=("$MEMLESS_GO_DRIVER")
else
	if ! (cd "$here/go" || exit 1; go build -o driver .); then
		echo "go driver build failed" >&2
		exit 2
	fi
	go_driver=("$here/go/driver")
fi

fixtures=("$here"/fixtures/*.yaml)
if [ "${#fixtures[@]}" -eq 0 ]; then
	echo "no fixtures found under $here/fixtures" >&2
	exit 2
fi

residue="$here/fixtures/.start.yaml.memless-tmp"
if [ ! -f "$residue" ]; then
	echo "residue fixture missing: $residue" >&2
	exit 2
fi
residue_before=$(mktemp) || exit 2
cp "$residue" "$residue_before"

is_outcome() {
	case "$1" in
	accepted | "refused: "* | "fault: "*) return 0 ;;
	*) return 1 ;;
	esac
}

diverged=0
for fixture in "${fixtures[@]}"; do
	name=$(basename "$fixture")
	php_out=$("${php_driver[@]}" "$fixture" 2>/dev/null)
	php_status=$?
	go_out=$("${go_driver[@]}" "$fixture" 2>/dev/null)
	go_status=$?
	if [ "$php_status" -ne 0 ] || ! is_outcome "$php_out"; then
		echo "DRIVER FAILURE $name (php): status=$php_status out=[$php_out]"
		diverged=1
	elif [ "$go_status" -ne 0 ] || ! is_outcome "$go_out"; then
		echo "DRIVER FAILURE $name (go): status=$go_status out=[$go_out]"
		diverged=1
	elif [ "$php_out" != "$go_out" ]; then
		echo "DIVERGENCE $name: php=[$php_out] go=[$go_out]"
		diverged=1
	fi
done

if ! cmp -s "$residue" "$residue_before"; then
	echo "RESIDUE MODIFIED after loading"
	diverged=1
fi
rm -f "$residue_before"

if [ "$diverged" -eq 0 ]; then
	echo "parity: all fixtures agree on both bridges"
fi
exit "$diverged"
