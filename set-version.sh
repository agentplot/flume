#!/bin/sh
# shellcheck disable=SC2016 # awk programs name their own $fields.
# Writes a release's version into every file of the tree that carries
# flume's version: the workspace's in Cargo.toml, which each crate takes,
# and Cargo.lock's entries for the crates; the Claude Code plugin's
# manifest and the marketplace's entry for it; and the herdr plugin's
# manifest. main gives 0.0.0 in each. The release job runs this on the
# commit it releases, and its tag names the release commit it makes of
# what this wrote; the binaries job builds the same tree
# (.github/workflows/ci.yml). root:versions holds each file to the
# workspace's version, and runs this on a scratch copy of them
# (check-versions.sh).
#
#   sh set-version.sh <major.minor.patch> [<root>]
set -eu

[ $# -ge 1 ] && [ $# -le 2 ] || {
  echo "usage: set-version.sh <major.minor.patch> [<root>]" >&2
  exit 2
}
version=$1
root=${2:-.}
printf '%s\n' "$version" | grep -Eqx '[0-9]+\.[0-9]+\.[0-9]+' || {
  echo "set-version: $version is not a version major.minor.patch" >&2
  exit 2
}

# The file $1 under the root, rewritten by the awk program $2 with the
# version as `v`; the program counts what it wrote in `written`, and
# a file it wrote nothing in is refused.
put() {
  file=$root/$1
  awk -v v="$version" "$2"'
    END { exit (written ? 0 : 1) }
  ' "$file" >"$file.set-version" || {
    rm -f "$file.set-version"
    echo "set-version: $1 gives no version where flume's is" >&2
    exit 1
  }
  mv "$file.set-version" "$file"
}

put Cargo.toml '
  /^\[/ { section = $0 }
  section == "[workspace.package]" && /^version = "/ && !written {
    $0 = "version = \"" v "\""
    written++
  }
  { print }
'

# The crates are the entries of the lock with no source.
put Cargo.lock '
  function flush(i) {
    for (i = 1; i <= n; i++) {
      if (package && !sourced && lines[i] ~ /^version = "/) {
        lines[i] = "version = \"" v "\""
        written++
      }
      print lines[i]
    }
    n = 0
    sourced = 0
  }
  /^\[\[package\]\]$/ { flush(); package = 1 }
  /^source = / { sourced = 1 }
  { lines[++n] = $0 }
  END { flush() }
'

for json in plugins/claude/.claude-plugin/plugin.json .claude-plugin/marketplace.json; do
  put "$json" '
    /^ *"version": "[^"]*",?$/ {
      sub(/"version": "[^"]*"/, "\"version\": \"" v "\"")
      written++
    }
    { print }
  '
done

# The manifest's own version, ahead of its first table.
put plugins/herdr/herdr-plugin.toml '
  /^\[/ { tables = 1 }
  !tables && /^version = "/ && !written {
    $0 = "version = \"" v "\""
    written++
  }
  { print }
'
