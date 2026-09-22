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
		if [ -f "$candidate" ] && { [ -z "$lib" ] || [ "$candidate" -nt "$lib" ]; }; then
			lib="$candidate"
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

is_query_outcome() {
	case "$1" in
	ok* | "refused: "* | "fault: "*) return 0 ;;
	*) return 1 ;;
	esac
}

is_write_outcome() {
	case "$1" in
	"accepted:"* | "refused:"* | "fault:"*) return 0 ;;
	*) return 1 ;;
	esac
}

is_disk_refusal() {
	case "$1" in
	"refused:cannot write file "*) return 0 ;;
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

queries="$here/queries.txt"
if [ -f "$queries" ]; then
	while IFS=$'\t' read -r fixture sql; do
		case "$fixture" in '' | \#*) continue ;; esac
		path="$here/fixtures/$fixture"
		if [ ! -f "$path" ]; then
			echo "QUERY SETUP ERROR: queries.txt names a missing fixture: $fixture"
			exit 2
		fi
		php_out=$("${php_driver[@]}" "$path" "$sql" 2>/dev/null)
		php_status=$?
		go_out=$("${go_driver[@]}" "$path" "$sql" 2>/dev/null)
		go_status=$?
		if [ "$php_status" -ne 0 ] || ! is_query_outcome "$php_out"; then
			echo "QUERY DRIVER FAILURE [$fixture | $sql] (php): status=$php_status out=[$php_out]"
			diverged=1
		elif [ "$go_status" -ne 0 ] || ! is_query_outcome "$go_out"; then
			echo "QUERY DRIVER FAILURE [$fixture | $sql] (go): status=$go_status out=[$go_out]"
			diverged=1
		elif [ "$php_out" != "$go_out" ]; then
			echo "QUERY DIVERGENCE [$fixture | $sql]"
			echo "  php=[$php_out]"
			echo "  go =[$go_out]"
			diverged=1
		fi
	done <"$queries"
fi

writes="$here/writes.txt"
if [ -f "$writes" ]; then
	while IFS=$'\t' read -r first second third; do
		case "$first" in '' | \#*) continue ;; esac
		if [ "$first" = "!disk" ]; then
			fixture="$second"
			sql="$third"
			mode="write-disk"
			if [ "$(id -u)" = "0" ]; then
				echo "note: skipping !disk under root (0555 does not block root): $sql"
				continue
			fi
		else
			fixture="$first"
			sql="$second"
			mode="write"
		fi
		path="$here/fixtures/$fixture"
		if [ ! -f "$path" ]; then
			echo "WRITE SETUP ERROR: writes.txt names a missing fixture: $fixture"
			exit 2
		fi
		php_out=$("${php_driver[@]}" "$path" "$sql" "$mode" 2>/dev/null)
		php_status=$?
		go_out=$("${go_driver[@]}" "$path" "$sql" "$mode" 2>/dev/null)
		go_status=$?
		if [ "$php_status" -ne 0 ] || ! is_write_outcome "$php_out"; then
			echo "WRITE DRIVER FAILURE [$fixture | $sql] (php): status=$php_status out=[$php_out]"
			diverged=1
		elif [ "$go_status" -ne 0 ] || ! is_write_outcome "$go_out"; then
			echo "WRITE DRIVER FAILURE [$fixture | $sql] (go): status=$go_status out=[$go_out]"
			diverged=1
		elif [ "$mode" = "write-disk" ] && ! is_disk_refusal "$php_out"; then
			echo "WRITE DISK NOT REFUSED [$fixture | $sql]: [$php_out]"
			diverged=1
		elif [ "$php_out" != "$go_out" ]; then
			echo "WRITE DIVERGENCE [$fixture | $sql]"
			echo "  php=[$php_out]"
			echo "  go =[$go_out]"
			diverged=1
		fi
	done <"$writes"
fi

if [ "$diverged" -eq 0 ]; then
	echo "parity: all fixtures, queries and writes agree on both bridges"
fi
exit "$diverged"
