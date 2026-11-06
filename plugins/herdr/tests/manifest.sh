#!/bin/sh
# herdr-plugin.toml as herdr 0.9.5 reads it (C.5; design 10): every line
# parses, the top-level keys herdr requires are there, its least herdr
# version among them, every command of [[startup]], [[events]],
# [[actions]] and [[panes]] that runs a script names one that exists in
# the plugin and parses under the system's sh, its version is the cargo
# workspace's, so a tag installs the plugin of that flume, and its places
# action's description names the entry that binds it a key. The README
# beside it gives that entry, lib/common.sh's, in a TOML block of its own
# (122; design 8). $1 is the manifest (default: the plugin's own); the
# plugin's directory is its, and $2 the workspace's Cargo.toml (default:
# the repository's).
set -u
here=$(cd "$(dirname "$0")" && pwd -P)
manifest=${1:-$(dirname "$here")/herdr-plugin.toml}
cargo=${2:-$(dirname "$(dirname "$(dirname "$here")")")/Cargo.toml}
dir=$(cd "$(dirname "$manifest")" && pwd -P)
bad=0
awk -v dir="$dir" '
function fail(why) { printf "%s:%d: %s\n", FILENAME, NR, why; bad = 1 }
function trim(s) { sub(/^[ \t]+/, "", s); sub(/[ \t]+$/, "", s); return s }
# The strings of a one-line array, into arr; their count.
function strings(v, arr,   n) {
  n = 0
  v = trim(v)
  if (v !~ /^\[.*\]$/) return -1
  v = substr(v, 2, length(v) - 2)
  while ((v = trim(v)) != "") {
    if (!match(v, /^"([^"\\]|\\.)*"/)) return -1
    arr[++n] = substr(v, 2, RLENGTH - 2)
    v = trim(substr(v, RLENGTH + 1))
    if (substr(v, 1, 1) == ",") v = substr(v, 2)
    else if (v != "") return -1
  }
  return n
}
function runs(path) {
  if (system("test -f \"" dir "/" path "\"") != 0) return "is not in the plugin"
  if (system("sh -n \"" dir "/" path "\"") != 0) return "does not parse under sh"
  return ""
}
BEGIN { table = "" }
{
  line = $0
  sub(/[ \t]+#.*$/, "", line)
  line = trim(line)
  if (line == "" || substr(line, 1, 1) == "#") next
  if (line ~ /^\[\[[a-z_]+\]\]$/) {
    table = substr(line, 3, length(line) - 4)
    if (table !~ /^(build|startup|events|actions|panes|link_handlers)$/) fail("a table herdr does not read: " line)
    next
  }
  if (!match(line, /^[a-z_]+[ \t]*=[ \t]*/)) { fail("not key = value: " $0); next }
  key = line; sub(/[ \t]*=.*/, "", key)
  val = trim(substr(line, RLENGTH + 1))
  if (val !~ /^("([^"\\]|\\.)*"|[0-9]+|true|false|\[.*\])$/) { fail("a value herdr cannot read: " $0); next }
  if (table == "") top[key] = val
  if (key == "command" && table != "") {
    delete c
    n = strings(val, c)
    if (n < 1) { fail("a command that is not an array of strings: " $0); next }
    script = (c[1] == "sh" && n >= 2) ? c[2] : ""
    if (script != "" && (why = runs(script)) != "") fail("runs " script ", which " why)
  }
  if (table == "actions" && key == "id") action = val
  if (table == "actions" && key == "description" && action == "\"places\"") described = val
}
END {
  split("id name version min_herdr_version", need, " ")
  for (i in need) if (!(need[i] in top)) { printf "%s: no %s\n", FILENAME, need[i]; bad = 1 }
  if ("min_herdr_version" in top && top["min_herdr_version"] !~ /^"[0-9]+\.[0-9]+\.[0-9]+"$/)
    { printf "%s: a least herdr version that is no version: %s\n", FILENAME, top["min_herdr_version"]; bad = 1 }
  if (index(described, "plugin_action") == 0 || index(described, "agentplot.flume.places") == 0)
    { printf "%s: the places action'"'"'s description names no plugin_action entry for agentplot.flume.places: %s\n", FILENAME, described; bad = 1 }
  exit bad
}' "$manifest" || bad=1
version=$(sed -n 's/^version = "\(.*\)"$/\1/p' "$manifest" | head -1)
workspace=$(sed -n '/^\[workspace.package\]/,/^\[/s/^version = "\(.*\)"$/\1/p' "$cargo")
if [ "$version" != "$workspace" ]; then
  echo "$manifest: version ${version:-none}, and the cargo workspace's is ${workspace:-none}"
  bad=1
fi
# The popup's entry as lib/common.sh gives it, sourced over a scratch
# state directory.
scratch=$(mktemp -d "${SLUICE_TMP:-${TMPDIR:-/tmp}}/flume-herdr-manifest.XXXXXX")
trap 'rm -rf "$scratch"' EXIT
entry=$(HERDR_PLUGIN_STATE_DIR=$scratch HERDR_PLUGIN_ROOT=$dir sh -c '. "$HERDR_PLUGIN_ROOT/lib/common.sh" && printf "%s\n" "$entry"' 2>/dev/null)
if [ -z "$entry" ]; then
  echo "$dir/lib/common.sh gives no entry for the popup's key"
  bad=1
elif ! ENTRY=$entry awk '
  /^```toml$/ { block = ""; inside = 1; next }
  /^```$/ && inside { if (block == ENVIRON["ENTRY"] "\n") found = 1; inside = 0; next }
  inside { block = block $0 "\n" }
  END { exit !found }' "$dir/README.md" 2>/dev/null; then
  printf '%s\n' "$dir/README.md: no TOML block of its own is the popup's entry lib/common.sh gives:" "$entry"
  bad=1
fi
[ "$bad" -eq 0 ] && echo "the manifest reads as herdr reads it, version $version, and the README gives the popup's entry"
exit "$bad"
