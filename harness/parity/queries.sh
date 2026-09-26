#!/usr/bin/env bash
# Sourced by run.sh: replays queries.txt through all three bridges and requires
# the same rendering (or refusal) from each. Shares diverged, here and the
# drivers.

is_query_outcome() {
	case "$1" in
	ok* | "refused: "* | "fault: "*) return 0 ;;
	*) return 1 ;;
	esac
}

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
		node_out=$("${node_driver[@]}" "$path" "$sql" 2>/dev/null)
		node_status=$?
		if [ "$php_status" -ne 0 ] || ! is_query_outcome "$php_out"; then
			echo "QUERY DRIVER FAILURE [$fixture | $sql] (php): status=$php_status out=[$php_out]"
			diverged=1
		elif [ "$go_status" -ne 0 ] || ! is_query_outcome "$go_out"; then
			echo "QUERY DRIVER FAILURE [$fixture | $sql] (go): status=$go_status out=[$go_out]"
			diverged=1
		elif [ "$node_status" -ne 0 ] || ! is_query_outcome "$node_out"; then
			echo "QUERY DRIVER FAILURE [$fixture | $sql] (node): status=$node_status out=[$node_out]"
			diverged=1
		elif [ "$php_out" != "$go_out" ] || [ "$php_out" != "$node_out" ]; then
			echo "QUERY DIVERGENCE [$fixture | $sql]"
			echo "  php =[$php_out]"
			echo "  go  =[$go_out]"
			echo "  node=[$node_out]"
			diverged=1
		fi
	done <"$queries"
fi
