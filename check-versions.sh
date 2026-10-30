#!/bin/sh
# Every file of the tree that carries flume's version gives the
# workspace's, which each crate takes: Cargo.lock's entries for the
# crates, the Claude Code plugin's manifest and the marketplace's entry
# for it, and the herdr plugin's manifest. So a release's plugins carry
# the release's version, which `claude plugin update` and herdr read, and
# never drift from its binary's. And set-version.sh, which the release
# job runs on the commit it releases, run on a scratch copy of those
# files with a version no release has, leaves each giving that version
# and changes no line but the ones that give it.
set -u
root=$(cd "$(dirname "$0")" && pwd)
cd "$root" || exit 1

files="Cargo.toml Cargo.lock .claude-plugin/marketplace.json plugins/claude/.claude-plugin/plugin.json plugins/herdr/herdr-plugin.toml"

# The workspace's version in the tree at $1.
workspace() {
  awk '
    /^\[/ { section = $0 }
    section == "[workspace.package]" && /^version = "/ {
      gsub(/^version = "|"$/, "")
      print
      exit
    }
  ' "$1/Cargo.toml"
}

# Each other place the tree at $1 gives flume's version, a line of where
# and the version it gives, "none" where it gives none.
carried() {
  awk '
    function flush() {
      if (package && !sourced) print "Cargo.lock: " name "\t" (version == "" ? "none" : version)
      package = 0
      sourced = 0
      name = ""
      version = ""
    }
    /^\[\[package\]\]$/ { flush(); package = 1; next }
    package && /^name = "/ { name = $0; gsub(/^name = "|"$/, "", name) }
    package && /^version = "/ { version = $0; gsub(/^version = "|"$/, "", version) }
    package && /^source = / { sourced = 1 }
    END { flush() }
  ' "$1/Cargo.lock"
  jq -r '"plugins/claude/.claude-plugin/plugin.json\t\(.version // "none")"' \
    "$1/plugins/claude/.claude-plugin/plugin.json"
  jq -r '.plugins[] | ".claude-plugin/marketplace.json: \(.name)\t\(.version // "none")"' \
    "$1/.claude-plugin/marketplace.json"
  awk '
    /^\[/ { exit }
    /^version = "/ { version = $0; gsub(/^version = "|"$/, "", version) }
    END { print "plugins/herdr/herdr-plugin.toml\t" (version == "" ? "none" : version) }
  ' "$1/plugins/herdr/herdr-plugin.toml"
}

# Each place in the tree at $1 that does not give the version $2, said
# with $3 ahead of it; fails where there is one.
agree() {
  carried "$1" | awk -F '\t' -v want="$2" -v said="$3" '
    $2 != want {
      print said $1 " gives " $2 ", and the workspace " want > "/dev/stderr"
      failed = 1
    }
    END { exit failed }
  '
}

status=0
version=$(workspace .)
if [ -z "$version" ]; then
  echo "versions: Cargo.toml gives no version under [workspace.package]" >&2
  exit 1
fi
agree . "$version" "versions: " || status=1

# The release's writer on a scratch copy.
scratch=$(mktemp -d "${SLUICE_TMP:-${TMPDIR:-/tmp}}/flume-versions.XXXXXX")
trap 'rm -rf "$scratch"' EXIT
for file in $files; do
  mkdir -p "$scratch/$(dirname "$file")"
  cp "$file" "$scratch/$file"
done
release=98.76.54
if ! sh set-version.sh "$release" "$scratch"; then
  echo "versions: set-version.sh $release failed" >&2
  exit 1
fi
written=$(workspace "$scratch")
if [ "$written" != "$release" ]; then
  echo "versions: set-version.sh $release left Cargo.toml giving ${written:-none}" >&2
  status=1
fi
agree "$scratch" "$release" "versions: set-version.sh $release left " || status=1
for file in $files; do
  # A line it changed gives the version; it changes as many as it writes.
  changes=$(diff "$file" "$scratch/$file")
  removed=$(printf '%s\n' "$changes" | grep -c '^<')
  added=$(printf '%s\n' "$changes" | grep -c '^>')
  stray=$(printf '%s\n' "$changes" | grep '^>' | grep -vF "\"$release\"")
  if [ "$removed" != "$added" ] || [ -n "$stray" ]; then
    echo "versions: set-version.sh $release changed $file beyond its versions:" >&2
    printf '%s\n' "$changes" >&2
    status=1
  fi
done

exit $status
