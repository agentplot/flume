# shellcheck shell=sh disable=SC2034 # root, status and rows_jq are read by the scripts that source this.
# What every script of the plugin shares: where its state is, how it calls
# herdr and flume under a bound, and the clock. Sourced, never run.

: "${HERDR_PLUGIN_ROOT:=$(cd "$(dirname "$0")/.." && pwd -P)}"
: "${HERDR_PLUGIN_STATE_DIR:=${XDG_STATE_HOME:-$HOME/.local/state}/herdr/plugins/agentplot.flume}"
root=$HERDR_PLUGIN_ROOT
# Each herdr server, told apart by its socket, has its own watch, lease,
# status and log. flume's herdr profile reads every server's lease here
# (watch.lease's first line: the watch's pid and when it last renewed).
state=$HERDR_PLUGIN_STATE_DIR/servers/$(printf '%s' "${HERDR_SOCKET_PATH:-none}" | cksum | cut -d' ' -f1)
mkdir -p "$state"
# The status the watch last wrote, which the sidebar and the popup draw.
status=$state/status.json
# The bound on every herdr call, in seconds (core.29).
bound=${FLUME_HERDR_BOUND:-10}

# The one source of the time, so the tests can move it: the clock file's
# number where FLUME_HERDR_CLOCK names one.
now() {
  if [ -n "${FLUME_HERDR_CLOCK:-}" ]; then cat "$FLUME_HERDR_CLOCK"; else date +%s; fi
}

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

# A place's row, as the sidebar and the popup show it, from its status
# alone: kind, stage, the stage's state, the change's tasks ticked, the
# submission, and whether it waits on its caller, on a person to answer
# its agent's dialog, or is stalled (34, 35, 56, 60; core.14). A jq
# function, defined for every program that includes it.
rows_jq='
def phase_word: if type == "string" then . else keys[0] end;
def submission_word:
  if . == null then empty
  elif .verdict != null then (.verdict | if type == "string" then . else keys[0] end)
  elif (.read | type) == "object" and .read.found != null then "submitted"
  else empty end;
def row:
  [ .kind,
    (.clues.stage // .current // "-"),
    (.clues.state // (.phase | phase_word)),
    (if .tasks then "\([.tasks[] | select(.ticked)] | length)/\(.tasks | length)" else empty end),
    (.submission | submission_word),
    (if .stalled then "stalled"
     elif .waiting_on.act == "answer" then "waits on a person"
     elif .waiting_on then "waits on caller"
     else empty end)
  ] | join(" · ");
'
