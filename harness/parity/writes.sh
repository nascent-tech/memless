#!/usr/bin/env bash
# Sourced by run.sh: replays writes.txt through all three bridges and requires
# the same rewritten bytes from each. Shares diverged, here and the drivers.

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
		node_out=$("${node_driver[@]}" "$path" "$sql" "$mode" 2>/dev/null)
		node_status=$?
		if [ "$php_status" -ne 0 ] || ! is_write_outcome "$php_out"; then
			echo "WRITE DRIVER FAILURE [$fixture | $sql] (php): status=$php_status out=[$php_out]"
			diverged=1
		elif [ "$go_status" -ne 0 ] || ! is_write_outcome "$go_out"; then
			echo "WRITE DRIVER FAILURE [$fixture | $sql] (go): status=$go_status out=[$go_out]"
			diverged=1
		elif [ "$node_status" -ne 0 ] || ! is_write_outcome "$node_out"; then
			echo "WRITE DRIVER FAILURE [$fixture | $sql] (node): status=$node_status out=[$node_out]"
			diverged=1
		elif [ "$mode" = "write-disk" ] && ! is_disk_refusal "$php_out"; then
			echo "WRITE DISK NOT REFUSED [$fixture | $sql]: [$php_out]"
			diverged=1
		elif [ "$php_out" != "$go_out" ] || [ "$php_out" != "$node_out" ]; then
			echo "WRITE DIVERGENCE [$fixture | $sql]"
			echo "  php =[$php_out]"
			echo "  go  =[$go_out]"
			echo "  node=[$node_out]"
			diverged=1
		fi
	done <"$writes"
fi
