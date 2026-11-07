# shellcheck shell=sh disable=SC2034 # root and bound are read by the scripts that source this.
# What every script of the plugin shares: where its state is, how it calls
# herdr under a bound, and what herdr's configuration lacks for it.
# Sourced, never run.

: "${HERDR_PLUGIN_ROOT:=$(cd "$(dirname "$0")/.." && pwd -P)}"
: "${HERDR_PLUGIN_STATE_DIR:=${XDG_STATE_HOME:-$HOME/.local/state}/herdr/plugins/agentplot.flume}"
root=$HERDR_PLUGIN_ROOT
# Each herdr server, told apart by its socket, has its own state: the
# setup.json its startup writes, which flume's herdr profile reads while
# the socket it names is present, and the startup's log.
state=$HERDR_PLUGIN_STATE_DIR/servers/$(printf '%s' "${HERDR_SOCKET_PATH:-none}" | cksum | cut -d' ' -f1)
mkdir -p "$state"
# The bound on every herdr call, in seconds (core.29).
bound=${FLUME_HERDR_BOUND:-10}

say() {
  printf '%s agentplot.flume: %s\n' "$(date +%H:%M:%S)" "$*" >&2
}

# "$@" run under the bound; its status, or 124 when the bound ended it.
# The timer writes nothing, so a command substitution around this waits
# on the command alone.
bounded() {
  "$@" &
  _job=$!
  (
    trap 'kill "$_nap" 2>/dev/null; exit 0' TERM
    sleep "$bound" &
    _nap=$!
    wait "$_nap"
    kill "$_job" 2>/dev/null
  ) >/dev/null 2>&1 &
  _timer=$!
  wait "$_job"
  _status=$?
  if kill "$_timer" 2>/dev/null; then
    wait "$_timer" 2>/dev/null
  else
    _status=124
  fi
  return "$_status"
}

# herdr on this plugin's server, under the bound.
hd() {
  bounded "${HERDR_BIN_PATH:-herdr}" "$@"
}

# Replace a file in one step, from stdin; nothing in, nothing replaced.
put() {
  _tmp="$1.tmp.$$"
  if cat >"$_tmp" && [ -s "$_tmp" ]; then mv -f "$_tmp" "$1"; else
    rm -f "$_tmp"
    return 1
  fi
}

# The id in a name status gives in the terminal's words,
# `herdr:<id>@<socket>`, where the socket is this plugin's server's;
# nothing for another server's or none.
on_this_server() {
  case $1 in
  herdr:*@"${HERDR_SOCKET_PATH:-}") _id=${1#herdr:} && echo "${_id%@*}" ;;
  esac
}

# The popup's key, which herdr 0.9.5's manifest cannot declare: the
# entry a host adds to herdr's configuration, as the README gives it and
# tests/manifest.sh holds it (122, C.5).
entry='[[keys.command]]
key = "prefix+f"
type = "plugin_action"
command = "agentplot.flume.places"
description = "flume: places in flight"'

# herdr's configuration file, as herdr finds it from its server's
# environment.
herdr_config() {
  if [ -n "${HERDR_CONFIG_PATH:-}" ]; then
    echo "$HERDR_CONFIG_PATH"
  elif [ -n "${XDG_CONFIG_HOME:-}" ]; then
    echo "$XDG_CONFIG_HOME/herdr/config.toml"
  else
    echo "$HOME/.config/herdr/config.toml"
  fi
}

# What herdr's configuration file $1 lacks for the plugin, one line each,
# in words a person acts on, naming the file: a line that is not a
# comment giving the popup's command, and one naming "$flume" in the
# sidebar's rows. An absent file holds neither. Read by lines for display
# alone, so it parses no TOML and nothing decides on it (72, 122, C.5).
lacking() {
  _read=$(grep -v '^[[:space:]]*#' "$1" 2>/dev/null)
  # The entry's keys on one line: `key = "prefix+f", type = ...`.
  _keys=$(printf '%s\n' "$entry" | sed 1d | awk '{ printf "%s%s", sep, $0; sep = ", " }')
  printf '%s\n' "$_read" | grep -q 'command[[:space:]]*=[[:space:]]*"agentplot\.flume\.places"' ||
    echo "herdr's configuration, $1, binds no key to flume's places: add a [[keys.command]] entry with $_keys, as the README of flume's herdr plugin gives it"
  # shellcheck disable=SC2016 # the token herdr's rows name, as written.
  printf '%s\n' "$_read" | grep -qF '"$flume"' ||
    echo "herdr's configuration, $1, shows no row of flume's in the sidebar: add \"\$flume\" to the rows of [ui.sidebar.agents] and of [ui.sidebar.spaces], as the README of flume's herdr plugin gives them"
}
