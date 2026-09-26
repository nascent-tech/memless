#!/usr/bin/env bash
# Sourced by run.sh: replays reloads.txt through all three bridges and
# requires the same block (issue per line, then sha256 and residue) from
# each. Shares diverged, here, php_driver, go_driver and node_driver.

is_reload_outcome() {
	case "$1" in
	*"sha256:"*) return 0 ;;
	*) return 1 ;;
	esac
}

reloads="$here/reloads.txt"
if [ -f "$reloads" ]; then
	while IFS=$'\t' read -r fixture scenario; do
		case "$fixture" in '' | \#*) continue ;; esac
		path="$here/fixtures/$fixture"
		if [ ! -f "$path" ]; then
			echo "RELOAD SETUP ERROR: reloads.txt names a missing fixture: $fixture"
			exit 2
		fi
		php_out=$("${php_driver[@]}" "$path" "$scenario" reload 2>/dev/null)
		php_status=$?
		go_out=$("${go_driver[@]}" "$path" "$scenario" reload 2>/dev/null)
		go_status=$?
		node_out=$("${node_driver[@]}" "$path" "$scenario" reload 2>/dev/null)
		node_status=$?
		if [ "$php_status" -ne 0 ] || ! is_reload_outcome "$php_out"; then
			echo "RELOAD DRIVER FAILURE [$fixture | $scenario] (php): status=$php_status out=[$php_out]"
			diverged=1
		elif [ "$go_status" -ne 0 ] || ! is_reload_outcome "$go_out"; then
			echo "RELOAD DRIVER FAILURE [$fixture | $scenario] (go): status=$go_status out=[$go_out]"
			diverged=1
		elif [ "$node_status" -ne 0 ] || ! is_reload_outcome "$node_out"; then
			echo "RELOAD DRIVER FAILURE [$fixture | $scenario] (node): status=$node_status out=[$node_out]"
			diverged=1
		elif [ "$php_out" != "$go_out" ] || [ "$php_out" != "$node_out" ]; then
			echo "RELOAD DIVERGENCE [$fixture | $scenario]"
			echo "  php =[$php_out]"
			echo "  go  =[$go_out]"
			echo "  node=[$node_out]"
			diverged=1
		fi
	done <"$reloads"
fi
