#!/bin/sh
# shellcheck disable=SC1091,SC2154,SC2016 # common.sh, sourced, sets state, status and rows_jq; jq programs.
# The popup: every place in flight, drawn from the status the watch last
# wrote, each row the sidebar's, and the keys that act on one: open
# focuses its tab, tell, stop and retry each run one flume command, the
# words or the force asked for first (53, 56, 60, 107; core.14). No key
# sends to, opens or closes an agent's pane. Each key acts at once, as
# sluice's popup's do: the arrows, j and k or a number choose a place,
# o, t, s or r act on it, q or Escape closes; only words are read as a
# line.
set -u
. "$(dirname "$0")/../lib/common.sh"

sel=1
note=

count() {
  jq -r '.status.places | length' "$status" 2>/dev/null || echo 0
}

draw() {
  [ -t 0 ] && printf '\033[H\033[2J'
  if [ ! -f "$status" ]; then
    echo "flume: no status yet; the watch writes it at its next settle."
  else
    n=$(count)
    [ "$sel" -le "$n" ] || sel=$n
    [ "$sel" -ge 1 ] || sel=1
    jq -r --argjson sel "$sel" "$rows_jq"'
      .status.places | to_entries[]
      | (if .key + 1 == $sel then "▶ " else "  " end) as $mark
      | "\($mark)\(.key + 1)  \(.value.name)  \(.value | row)\n     \(.value.place)"' "$status"
  fi
  echo
  echo "↑↓, j/k or a number to choose · o open · t tell · s stop · r retry · q close"
  [ -n "$note" ] && printf '%s\n' "$note"
}

# Field $2 of the place numbered $1, as status gives it.
field() {
  jq -r --argjson n "$1" ".status.places[\$n - 1]$2 // empty" "$status" 2>/dev/null
}

# The terminal as the keys want it: each key at once, unechoed, a second
# without one a redraw.
raw() {
  [ -t 0 ] || return 0
  stty -icanon -echo min 0 time 10
  printf '\033[?25l' >&2
}

# The terminal as a line of words wants it, until Enter.
cooked() {
  [ -t 0 ] || return 0
  stty "$saved" 2>/dev/null
  printf '\033[?25h' >&2
}

# A question on the terminal, whatever captures the answer's use: words
# are text, read as a line.
ask() {
  cooked
  printf '%s ' "$1" >&2
  read -r answer || answer=
  raw
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
    flume tell "$place" "$answer" 2>&1
    ;;
  s)
    force=$(forced)
    # shellcheck disable=SC2086 # --force or nothing.
    flume stop "$place" $force 2>&1
    ;;
  r)
    stage=$(field "$n" .current)
    ask "words for its agent (none for none):"
    words=$answer
    force=$(forced)
    # shellcheck disable=SC2086 # --force or nothing.
    if [ -n "$words" ]; then
      flume retry "$place" --stage "$stage" --words "$words" $force 2>&1
    else
      flume retry "$place" --stage "$stage" $force 2>&1
    fi
    ;;
  esac
}

# One byte of input, a newline kept; nothing at the end of input or a
# second without a key.
key() {
  k=$(
    dd bs=1 count=1 2>/dev/null
    echo .
  )
  printf '%s' "${k%.}"
}

if [ -t 0 ]; then
  saved=$(stty -g)
  trap 'stty "$saved" 2>/dev/null; printf "\033[?1049l\033[?25h"' EXIT
  trap 'exit 0' INT TERM HUP
  # The alternate screen, so closing leaves the pane as it was.
  printf '\033[?1049h'
  raw
fi

esc=$(printf '\033')
draw
while :; do
  key=$(key)
  if [ -z "$key" ]; then
    # A second without a key on a terminal; the end of input otherwise.
    [ -t 0 ] || exit 0
    draw
    continue
  fi
  case $key in
  q | Q) exit 0 ;;
  "$esc")
    # Escape alone closes; an arrow key's bytes follow at once. Off a
    # terminal, a key's two bytes after its escape are read as one.
    if [ -t 0 ]; then
      stty time 1
      more=$(dd bs=64 count=1 2>/dev/null)
      stty time 10
    else
      more=$(dd bs=1 count=2 2>/dev/null)
    fi
    [ -z "$more" ] && exit 0
    # The wheel sends several arrows at once: each one is a place.
    case $more in
    *A*) sel=$((sel - $(printf '%s' "$more" | tr -cd A | wc -c))) ;;
    *B*) sel=$((sel + $(printf '%s' "$more" | tr -cd B | wc -c))) ;;
    esac
    note=
    ;;
  k)
    sel=$((sel - 1))
    note=
    ;;
  j)
    sel=$((sel + 1))
    note=
    ;;
  [1-9])
    if [ "$key" -le "$(count)" ]; then
      sel=$key
      note=
    else
      note="no place $key"
    fi
    ;;
  o | t | s | r)
    note=$(act "$sel" "$key")
    ;;
  esac
  draw
done
