#!/bin/sh
# The scripts' cases (the change flume-publishes-its-own-flake-and-module,
# design 11), which nix/check.sh test runs beside the evaluated ones: each
# runs nix/check.sh or nix/modules/activate.sh by sh in a scratch folder of
# its own. It prints each failure as one line, nix: <case>: <what>, and
# nothing where every case holds.
set -eu

nix=$(cd "$(dirname "$0")/.." && pwd)
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
cases=0

# A case's folder.
fresh() {
  cases=$((cases + 1))
  dir=$scratch/$cases
  mkdir -p "$dir"
}

# A failure of the case `$1`, saying `$2`, where the test after them fails.
expect() {
  named=$1
  what=$2
  shift 2
  if ! "$@"; then
    echo "nix: $named: $what"
  fi
}

# `$1` holds `$2`.
says() {
  case $1 in *"$2"*) ;; *) return 1 ;; esac
}

named="No inputs given"
fresh
status=0
(cd "$nix/.." && env -u FLUME_FLAKE_INPUTS sh nix/check.sh test) \
  >"$dir/out" 2>"$dir/err" || status=$?
err=$(cat "$dir/err")
expect "$named" "check.sh test with FLUME_FLAKE_INPUTS unset exits $status" \
  [ "$status" -ne 0 ]
expect "$named" "check.sh test with FLUME_FLAKE_INPUTS unset says \"$err\", not that the devenv shell sets it" \
  says "$err" "FLUME_FLAKE_INPUTS is not set: the devenv shell sets it"
