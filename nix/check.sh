#!/bin/sh
# The nix project's type check and test (the change
# flume-publishes-its-own-flake-and-module, design 11, as Boomdrive's
# nix/check.sh): the nix devenv.nix pins evaluates nix/tests, offline and
# read-only, against a dummy store whose store directory is none of the
# host's. So it computes every path and writes none, reads the flake's
# inputs where the devenv shell fetched them (FLUME_FLAKE_INPUTS) as plain
# files, and needs neither the network nor the nix daemon, which the gate's
# sandbox denies. Its configuration, caches and state are a scratch
# folder's, never the host's or the home's. The test also runs the
# scripts' cases, nix/tests/scripts.sh, by sh in scratch folders. Each
# failure is one line, nix: <case>: <what>.
set -eu

case ${1:-} in
typecheck | test) check=$1 ;;
*)
  echo "usage: sh nix/check.sh typecheck|test" >&2
  exit 2
  ;;
esac

if [ -z "${FLUME_FLAKE_INPUTS:-}" ]; then
  echo "nix: $check: FLUME_FLAKE_INPUTS is not set: the devenv shell sets it" >&2
  exit 1
fi

scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
mkdir "$scratch/conf"
unset NIX_CONFIG NIX_PATH
NIX_CONF_DIR="$scratch/conf"
NIX_USER_CONF_FILES="$scratch/conf/nix.conf"
XDG_CACHE_HOME="$scratch/cache"
XDG_CONFIG_HOME="$scratch/config"
XDG_DATA_HOME="$scratch/data"
XDG_STATE_HOME="$scratch/state"
export NIX_CONF_DIR NIX_USER_CONF_FILES XDG_CACHE_HOME XDG_CONFIG_HOME \
  XDG_DATA_HOME XDG_STATE_HOME

failures=$(nix eval --raw --offline --read-only --impure \
  --store 'dummy://?store=/flume-check/store' \
  --extra-experimental-features nix-command \
  --file nix/tests "$check")
if [ "$check" = test ]; then
  scripts=$(sh nix/tests/scripts.sh)
  failures=$(printf '%s\n%s\n' "$failures" "$scripts" | sed '/^$/d')
fi

if [ -n "$failures" ]; then
  printf '%s\n' "$failures"
  exit 1
fi
echo "nix: $check: every case holds"
