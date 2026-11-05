#!/bin/sh
# The plugin linked into the pinned herdr under a scratch configuration,
# and herdr's own list of plugins read naming it at this directory and
# version, so the manifest is proved against herdr itself (C.5; design
# 10); and the README's TOML, the popup's entry and the sidebar's rows,
# written into a scratch file HERDR_CONFIG_PATH names, passed by the
# pinned herdr's own `config check` (122; design 8; core.29). Nothing of
# the user's herdr is read or written: the home, the configuration and the
# state are scratch, and the socket herdr is given is a path no server
# listens on, short enough for a socket's name.
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
    XDG_STATE_HOME="$scratch/state" HERDR_SOCKET_PATH="/tmp/flume-herdr-$$.sock" \
    HERDR_CONFIG_PATH="$scratch/config.toml" "$herdr" "$@"
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
# The README's TOML blocks, the configuration every call above read
# absent.
awk '/^```$/ { inside = 0 } inside { print } /^```toml$/ { inside = 1 }' "$plugin/README.md" >"$scratch/config.toml"
# shellcheck disable=SC2016 # the token herdr's rows name, as written.
if ! grep -q 'command = "agentplot.flume.places"' "$scratch/config.toml" || ! grep -qF '"$flume"' "$scratch/config.toml"; then
  echo "the README gives no entry for the popup or no \$flume row: $(cat "$scratch/config.toml")"
  exit 1
fi
in_scratch config check >"$scratch/checked" 2>&1
if [ "$(head -1 "$scratch/checked")" = "config: ok" ]; then
  echo "herdr's config check passes the README's entry and rows"
else
  echo "herdr's config check of the README's entry and rows: $(cat "$scratch/checked")"
  exit 1
fi
