#!/bin/sh
# The commit hooks run in a worktree whose devenv shell never ran (the-gate
# spec): a scratch clone laid out as flume's, a bare repository with the
# worktree `main` beside it holding the hook config the shell links, takes
# the hooks devenv.nix installs ($FLUME_HOOKS, $FLUME_HOOKS_CONFIG); a
# fresh worktree beside it, with no config of its own, commits a Rust file;
# convco, treefmt and clippy must each run and pass, with no
# PREK_ALLOW_NO_CONFIG. Every git call is the scratch clone's alone: no
# global or system config, no credential helper, no prompt.
set -eu

fail() {
  echo "check-hooks: $*" >&2
  exit 1
}

[ -n "${FLUME_HOOKS:-}" ] || fail "FLUME_HOOKS is unset: run it in the devenv shell"
[ -n "${FLUME_HOOKS_CONFIG:-}" ] || fail "FLUME_HOOKS_CONFIG is unset: run it in the devenv shell"

root=$(git rev-parse --show-toplevel)
head=$(git rev-parse HEAD)
proto_home=$(git rev-parse --path-format=absolute --git-common-dir)/proto

scratch=$(mktemp -d "${TMPDIR:-/tmp}/flume-hooks.XXXXXX")
trap 'rm -rf "$scratch"' EXIT

unset PREK_ALLOW_NO_CONFIG GIT_DIR GIT_WORK_TREE GIT_INDEX_FILE
GIT_CONFIG_GLOBAL=/dev/null
GIT_CONFIG_NOSYSTEM=1
GIT_TERMINAL_PROMPT=0
export GIT_CONFIG_GLOBAL GIT_CONFIG_NOSYSTEM GIT_TERMINAL_PROMPT

bare="$scratch/flume.git"
git init -q --bare "$bare"
git -C "$bare" config credential.helper ''
git -C "$bare" config user.name 'flume hooks check'
git -C "$bare" config user.email 'hooks@flume.invalid'
git -C "$bare" fetch -q "$root" "$head:refs/heads/main"
git -C "$bare" worktree add -q "$scratch/main" main
ln -s "$FLUME_HOOKS_CONFIG" "$scratch/main/.pre-commit-config.yaml"
git -C "$bare" worktree add -q -b fresh "$scratch/fresh" main

# The hooks devenv installs into the common hooks directory.
hooks=$(git -C "$scratch/fresh" rev-parse --path-format=absolute --git-path hooks)
mkdir -p "$hooks"
for type in pre-commit commit-msg; do
  install -m 755 "$FLUME_HOOKS/$type" "$hooks/$type"
done

# The clone's proto home is this repository's installs, and its cargo
# builds under this checkout's target/, kept between runs.
ln -s "$proto_home" "$(git -C "$bare" rev-parse --path-format=absolute --git-common-dir)/proto"
CARGO_TARGET_DIR="$root/target/check-hooks"
export CARGO_TARGET_DIR

[ ! -e "$scratch/fresh/.pre-commit-config.yaml" ] || fail "the fresh worktree holds a hook config"

file=crates/flume-seams/src/lib.rs
printf '\n// A line the hooks check commits.\n' >>"$scratch/fresh/$file"
git -C "$scratch/fresh" add -- "$file"
if ! out=$(cd "$scratch/fresh" && git commit -m 'test(hooks): a commit in a fresh worktree' -- "$file" 2>&1); then
  printf '%s\n' "$out" >&2
  fail "the commit in a fresh worktree failed"
fi
printf '%s\n' "$out"

printf '%s\n' "$out" | grep -F "/main/.pre-commit-config.yaml" >/dev/null ||
  fail "the hooks did not read the main checkout's config"
for hook in convco treefmt 'clippy (-D warnings)'; do
  printf '%s\n' "$out" | grep -E "^$(printf '%s' "$hook" | sed 's/[()]/\\&/g')\.+Passed" >/dev/null ||
    fail "$hook did not run and pass in the fresh worktree"
done
[ "$(git -C "$scratch/fresh" rev-list --count main..fresh)" = 1 ] || fail "no commit was made"
echo "check-hooks: convco, treefmt and clippy ran in a fresh worktree on the main checkout's config"
