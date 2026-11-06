#!/bin/sh
# shellcheck disable=SC1091,SC2154,SC2016 # common.sh, sourced, sets root, state, status and bound; jq programs.
# The plugin's one watch on its herdr server (122; design 10). `--ensure`,
# herdr's startup hook, starts it unless a watch holds a fresh lease.
# Running, on each short tick it reads every pane's agent state in one
# herdr listing, and when a flume agent's state changed since the last
# tick, an agent came up or went in one of the server's panes, as a new
# stage's does before any status names it, or the bound passed since its
# last settle, it runs `flume settle`,
# writes `flume status --json` for the sidebar and the popup, and reports
# the display clues and the sidebar rows status gives (46, 119, 120, 139;
# core.13, core.14). At its start and after each settle it writes what
# herdr's configuration lacks for the plugin, for flume's status to give,
# and logs it once at its start, telling no person (72, 122). It runs
# nothing else of flume's. What it holds between ticks is in its state
# directory, so `--ticks N`, which stops it after N ticks (the tests),
# goes on where the last left off.
set -u
. "$(dirname "$0")/../lib/common.sh"
. "$root/lib/lease.sh"
. "$root/lib/procs.sh"

lease=$state/watch.lease
tick=${FLUME_HERDR_TICK:-2}
# A lease is fresh for this long, and renewed four times as often.
term=${FLUME_HERDR_LEASE:-60}
every=$((term / 4))
[ "$every" -ge 1 ] || every=1
version=$(sed -n 's/^version = "\(.*\)"$/\1/p' "$root/herdr-plugin.toml")

# The herdr server, known by its socket's inode: another server at the
# same path has another socket. Nothing where there is no socket.
socket_id() {
  [ -e "${HERDR_SOCKET_PATH:-}" ] || return 0
  # shellcheck disable=SC2012 # the inode comes first, whatever the name.
  ls -di "$HERDR_SOCKET_PATH" 2>/dev/null | awk '{ print $1 }'
}

# The lease: the watch's pid and when it last renewed; its identity, so a
# pid the system gives another process since is never taken for it
# (record 0016); the plugin directory and version it runs, so a reinstall
# replaces it; and its server's socket, so a server started in a stopped
# one's place does too.
fresh() {
  [ -f "$lease" ] || return 1
  read -r pid at <"$lease" || return 1
  [ $(($(date +%s) - at)) -lt "$term" ] || return 1
  kill -0 "$pid" 2>/dev/null || return 1
  was=$(sed -n 2p "$lease")
  case $was in
  # Where no identity could be read when it was written, the pid alone.
  pid:*) return 0 ;;
  esac
  [ "$(identity "$pid")" = "$was" ]
}

ours() {
  fresh && [ "$(sed -n 3p "$lease")" = "$root" ] && [ "$(sed -n 4p "$lease")" = "$version" ] &&
    [ "$(sed -n 5p "$lease")" = "$(socket_id)" ]
}

holder() {
  head -1 "$lease" 2>/dev/null | cut -d' ' -f1
}

# Written only while the lock is still this process's.
write_lease() {
  printf '%s %s\n%s\n%s\n%s\n%s\n' $$ "$(date +%s)" "$who" "$root" "$version" "$socket" >"$lease.tmp.$$" || return 1
  if holds_lock; then mv -f "$lease.tmp.$$" "$lease"; else rm -f "$lease.tmp.$$"; fi
}

# What herdr's configuration lacks for the plugin, written as a JSON
# array into setup.json, which flume's herdr profile gives in status
# beside this watch's lease; and logged where $1 is `said`. Nothing is
# sent to herdr for it (72, 122; design 8).
setup() {
  lacking "$(herdr_config)" >"$state/setup.lines"
  jq -R -s -c 'split("\n") | map(select(. != ""))' <"$state/setup.lines" | put "$state/setup.json" ||
    say "what herdr's configuration lacks was not written"
  if [ "${1:-}" = said ]; then
    while IFS= read -r words; do say "$words"; done <"$state/setup.lines"
  fi
}

case ${1:-} in
--ensure)
  ours && exit 0
  nohup sh "$root/bin/watch.sh" </dev/null >>"$state/watch.log" 2>&1 &
  exit 0
  ;;
esac

ticks=-1
[ "${1:-}" = --ticks ] && ticks=$2
socket=$(socket_id)
who=$(identity $$)

# The watch's end: its renewer ended, and its lease removed where it is
# still this watch's, under the lock, which it may hold already.
stop() {
  if [ -n "${renewer:-}" ]; then
    kill "$renewer" 2>/dev/null
    wait "$renewer" 2>/dev/null
  fi
  if holds_lock || take_lock; then
    rm -f "$lease.tmp.$$"
    [ "$(holder)" = $$ ] && rm -f "$lease"
    drop_lock
  fi
}

take_lock || exit 0
if ours; then
  drop_lock
  exit 0
fi
# Set before the lease is written, so a watch ended at any moment once
# its lease can be read takes the lease with it: a lease left by a watch
# that is gone is read as a live one until it is stale (122).
trap 'stop; exit 0' INT TERM
trap stop EXIT
write_lease
drop_lock
say "watching (pid $$)"
setup said

# The lease is renewed on its own clock, however long a settle takes.
(
  # The watch's traps are its own: a subshell's end removes no lease.
  trap - TERM EXIT
  # shellcheck disable=SC2034 # read by take_lock and drop_lock
  lock_owner=$(exec sh -c 'echo $PPID')
  while sleep "$every" && kill -0 $$ 2>/dev/null; do
    take_lock || continue
    if [ "$(holder)" = $$ ]; then
      write_lease
    else
      drop_lock
      exit 0
    fi
    drop_lock
  done
) &
renewer=$!

# Each flume pane on this server status names, with the agent state
# herdr's listing $1 reads in it, or gone where it lists none: one line
# per pane, in order. Then every pane the listing holds an agent in, so
# an agent come up since status was written, which it cannot name yet,
# changes what is seen too (122).
states() {
  if [ -f "$status" ]; then
    panes=$(jq -r '.status.places[]?.clues.shown_at.agent // empty' "$status" 2>/dev/null |
      while IFS= read -r name; do on_this_server "$name"; done | sort -u)
    for p in $panes; do
      s=$(printf '%s' "$1" | jq -r --arg p "$p" \
        '[.result.panes[]? | select(.pane_id == $p) | (.agent_status // "none")][0] // "gone"' 2>/dev/null)
      echo "$p $s"
    done
  fi
  printf '%s' "$1" | jq -r '"agents:" + ([.result.panes[]? | select(.agent != null) | " " + .pane_id] | sort | add // "")' 2>/dev/null
}

# Each clue and line status gives, reported where status says the
# terminal shows it on this server: a pane's title, the agent's name and
# its row, the short form of its place's line; a base's workspace's count
# line. Copied from status and computed by nothing here (56, 60, 139;
# core.14). A report that fails is logged and the tick goes on.
report() {
  jq -r '
    (.status.places[]? | select(.clues != null and .clues.shown_at.agent != null)
      | ["pane", .clues.shown_at.agent, .clues.title, .clues.display_agent, .line.short]),
    (.status.host.bases[]? | select(.shown_at != null) | ["workspace", .shown_at, .line])
    | @tsv' "$status" 2>/dev/null >"$state/reports"
  while IFS='	' read -r what at a b c; do
    id=$(on_this_server "$at")
    [ -n "$id" ] || continue
    if [ "$what" = pane ]; then
      said=$(hd pane report-metadata "$id" --source flume --title "$a" --display-agent "$b" --token "flume=$c" 2>&1 >/dev/null) ||
        say "the clues of the pane $id were not reported: ${said:-no answer within ${bound}s}"
    else
      said=$(hd workspace report-metadata "$id" --source flume --token "flume=$a" 2>&1 >/dev/null) ||
        say "the count of the workspace $id was not reported: ${said:-no answer within ${bound}s}"
    fi
  done <"$state/reports"
}

# flume settle, then status written for the sidebar, then the clues, then
# what herdr's configuration lacks.
settle() {
  flume settle >/dev/null 2>"$state/settle.err" ||
    say "flume settle failed: $(tail -1 "$state/settle.err")"
  now >"$state/settled"
  if flume status --json 2>"$state/status.err" | put "$status.new"; then
    if jq -e .status "$status.new" >/dev/null 2>&1; then
      mv -f "$status.new" "$status"
      report
    else
      rm -f "$status.new"
      say "flume status gave no status: $(head -c 200 "$status.err")"
    fi
  else
    say "flume status failed: $(tail -1 "$state/status.err")"
  fi
  setup
}

gone=0
while [ "$ticks" -ne 0 ]; do
  # The watch lives as long as the herdr server it was started from.
  if [ "$(socket_id)" != "$socket" ]; then
    say "the herdr server it was started from is gone; stopping"
    exit 0
  fi
  if listing=$(hd pane list); then gone=0; else
    gone=$((gone + 1))
    listing='{}'
    if [ "$gone" -ge 2 ]; then
      say "herdr does not answer; stopping"
      exit 0
    fi
  fi
  seen=$(states "$listing")
  # The bound the herdr terminal profile states, as status gives it.
  every_settle=$(jq -r '.status.host.watch.bound.seconds // 30' "$status" 2>/dev/null || echo 30)
  last=$(cat "$state/settled" 2>/dev/null || echo 0)
  if [ "$seen" != "$(cat "$state/seen" 2>/dev/null)" ] || [ $(($(now) - last)) -ge "${every_settle:-30}" ]; then
    settle
    # The panes the new status names, read in the same listing, so only
    # a change herdr reads after this settle moves the next tick.
    seen=$(states "$listing")
  fi
  printf '%s\n' "$seen" >"$state/seen"
  [ "$ticks" -gt 0 ] && ticks=$((ticks - 1))
  [ "$ticks" -eq 0 ] && break
  i=0
  while [ "$i" -lt "$tick" ]; do
    sleep 1
    i=$((i + 1))
    [ "$(holder)" = $$ ] || exit 0
  done
done
