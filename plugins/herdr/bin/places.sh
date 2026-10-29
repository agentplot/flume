#!/bin/sh
# shellcheck disable=SC1091,SC2154,SC2016 # common.sh, sourced, sets state, status and rows_jq; jq programs.
# The popup: every place in flight, drawn from the status the watch last
# wrote, each row the sidebar's, and the keys that act on one: open
# focuses its tab, tell, stop and retry each run one flume command, the
# words or the force asked for first (53, 56, 60, 107; core.14). No key
# sends to, opens or closes an agent's pane. Reads its keys a line at a
# time: a place's number and a key, `2r`, or `q`.
set -u
. "$(dirname "$0")/../lib/common.sh"

draw() {
  if [ ! -f "$status" ]; then
    echo "flume: no status yet; the watch writes it at its next settle."
    return
  fi
  jq -r "$rows_jq"'
    .status.places | to_entries[]
    | "\(.key + 1)  \(.value.name)  \(.value | row)\n   \(.value.place)"' "$status"
  echo
  echo "a number and a key: o open · t tell · s stop · r retry; q to close (2r)"
}

# Field $2 of the place numbered $1, as status gives it.
field() {
  jq -r --argjson n "$1" ".status.places[\$n - 1]$2 // empty" "$status" 2>/dev/null
}

# A question on the terminal, whatever captures the answer's use.
ask() {
  printf '%s ' "$1" >&2
  read -r answer || answer=
}

forced() {
  ask "force, if its agent is not quiet? [y/N]"
  case $answer in [yY]*) echo --force ;; esac
}

act() {
  n=$1
  key=$2
  place=$(field "$n" .place)
  if [ -z "$place" ]; then
    echo "no place $n"
    return
  fi
  case $key in
  o)
    tab=$(on_this_server "$(field "$n" .clues.shown_at.place)")
    if [ -z "$tab" ]; then
      echo "its tab is not on this herdr server"
      return
    fi
    hd tab focus "$tab" >/dev/null || echo "herdr did not focus its tab"
    ;;
  t)
    ask "words for its agent:"
    [ -n "$answer" ] || return
    flume tell "$place" "$answer"
    ;;
  s)
    force=$(forced)
    # shellcheck disable=SC2086 # --force or nothing.
    flume stop "$place" $force
    ;;
  r)
    stage=$(field "$n" .current)
    ask "words for its agent (none for none):"
    words=$answer
    force=$(forced)
    # shellcheck disable=SC2086 # --force or nothing.
    if [ -n "$words" ]; then
      flume retry "$place" --stage "$stage" --words "$words" $force
    else
      flume retry "$place" --stage "$stage" $force
    fi
    ;;
  *) echo "no key $key" ;;
  esac
}

while :; do
  draw
  read -r line || exit 0
  case $line in
  q | Q) exit 0 ;;
  *[0-9]*[a-z])
    act "${line%[a-z]}" "${line##*[0-9]}"
    ;;
  esac
done
