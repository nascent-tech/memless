#!/usr/bin/env bash
# Sourced by run.sh: replays transactions.txt through both bridges and requires
# the same block (issue per instruction, then sha256 and residue) from each.
# Shares diverged, here, php_driver, go_driver and is_transaction_outcome.

is_transaction_outcome() {
	case "$1" in
	*"sha256:"*) return 0 ;;
	*) return 1 ;;
	esac
}

is_transaction_disk_refused() {
	case "$1" in
	*"refused:cannot write file "*) return 0 ;;
	*) return 1 ;;
	esac
}

transactions="$here/transactions.txt"
if [ -f "$transactions" ]; then
	while IFS=$'\t' read -r first second third; do
		case "$first" in '' | \#*) continue ;; esac
		if [ "$first" = "!disk" ]; then
			fixture="$second"
			suite="$third"
			mode="transaction-disk"
			if [ "$(id -u)" = "0" ]; then
				echo "note: skipping !disk under root (0555 does not block root): $suite"
				continue
			fi
		else
			fixture="$first"
			suite="$second"
			mode="transaction"
		fi
		path="$here/fixtures/$fixture"
		if [ ! -f "$path" ]; then
			echo "TRANSACTION SETUP ERROR: transactions.txt names a missing fixture: $fixture"
			exit 2
		fi
		php_out=$("${php_driver[@]}" "$path" "$suite" "$mode" 2>/dev/null)
		php_status=$?
		go_out=$("${go_driver[@]}" "$path" "$suite" "$mode" 2>/dev/null)
		go_status=$?
		if [ "$php_status" -ne 0 ] || ! is_transaction_outcome "$php_out"; then
			echo "TRANSACTION DRIVER FAILURE [$fixture | $suite] (php): status=$php_status out=[$php_out]"
			diverged=1
		elif [ "$go_status" -ne 0 ] || ! is_transaction_outcome "$go_out"; then
			echo "TRANSACTION DRIVER FAILURE [$fixture | $suite] (go): status=$go_status out=[$go_out]"
			diverged=1
		elif [ "$mode" = "transaction-disk" ] && ! is_transaction_disk_refused "$php_out"; then
			echo "TRANSACTION DISK NOT REFUSED [$fixture | $suite]: [$php_out]"
			diverged=1
		elif [ "$php_out" != "$go_out" ]; then
			echo "TRANSACTION DIVERGENCE [$fixture | $suite]"
			echo "  php=[$php_out]"
			echo "  go =[$go_out]"
			diverged=1
		fi
	done <"$transactions"
fi
