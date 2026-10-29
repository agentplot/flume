#!/bin/sh
# The plugin linked into the pinned herdr under a scratch configuration,
# and herdr's own list of plugins read naming it at this directory and
# version, so the manifest is proved against herdr itself (C.5; design
# 10). Nothing of the user's herdr is read or written: the home, the
# configuration and the state are scratch, and the socket herdr is given
# is a path no server listens on, short enough for a socket's name.
set -u
here=$(cd "$(dirname "$0")" && pwd -P)
plugin=$(dirname "$here")
scratch=$(mktemp -d "${SLUICE_TMP:-${TMPDIR:-/tmp}}/flume-herdr-install.XXXXXX")
trap 'rm -rf "$scratch"' EXIT
herdr=$(command -v herdr) || {
  echo "herdr is not on PATH: run this in the devenv shell, which pins it"
  exit 1
}
in_scratch() {
  env -i PATH="$PATH" HOME="$scratch/home" XDG_CONFIG_HOME="$scratch/config" \
    XDG_STATE_HOME="$scratch/state" HERDR_SOCKET_PATH="/tmp/flume-herdr-$$.sock" "$herdr" "$@"
}
"$herdr" --version
in_scratch plugin link "$plugin" >"$scratch/linked" 2>&1 || {
  echo "herdr refused to link the plugin: $(cat "$scratch/linked")"
  exit 1
}
in_scratch plugin list --json >"$scratch/list" 2>&1 || {
  echo "herdr listed no plugins: $(cat "$scratch/list")"
  exit 1
}
version=$(sed -n 's/^version = "\(.*\)"$/\1/p' "$plugin/herdr-plugin.toml")
if jq -e --arg root "$plugin" --arg version "$version" '
  [.result.plugins[] | select(.plugin_id == "agentplot.flume" and .plugin_root == $root
    and .version == $version and .enabled)] | length == 1' "$scratch/list" >/dev/null; then
  echo "herdr lists agentplot.flume $version, linked from $plugin"
else
  echo "herdr's list does not name agentplot.flume $version at $plugin: $(cat "$scratch/list")"
  exit 1
fi
