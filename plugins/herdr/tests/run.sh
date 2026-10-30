#!/bin/sh
# shellcheck disable=SC2016 # jq programs name their own $variables.
# The herdr plugin's scripts against stand-in `herdr` and `flume` on PATH
# (tests/fake): the status the stand-in flume answers in, the herdr and
# flume calls made checked. Nothing here reaches a running herdr, the
# user's flume, a network or a credential. FLUME_HERDR_SH names the shell
# every script runs under as `sh`, as herdr runs them: dash, as Debian's
# /bin/sh is, or the system's.

set -u
here=$(cd "$(dirname "$0")" && pwd -P)
plugin=$(dirname "$here")

# Under sluice's sandbox only its scratch directory is writable beside the
# checkout.
scratch=$(mktemp -d "${SLUICE_TMP:-${TMPDIR:-/tmp}}/flume-herdr.XXXXXX")
scratch=$(cd "$scratch" && pwd -P)

if [ -n "${FLUME_HERDR_SH:-}" ]; then
  mkdir -p "$scratch/sh"
  ln -s "$(command -v "$FLUME_HERDR_SH")" "$scratch/sh/sh"
  export PATH="$scratch/sh:$PATH"
fi
echo "scripts run under $(command -v sh)"
unset HERDR_PLUGIN_CONTEXT_JSON HERDR_ENV HERDR_PANE_ID HERDR_WORKSPACE_ID HERDR_TAB_ID
unset FLUME_RECIPIENT FLUME_PLACE FLUME_AGENT FLUME_SECRET
export PATH="$here/fake:$PATH" HERDR_BIN_PATH="$here/fake/herdr" HERDR_PLUGIN_ROOT="$plugin"
# The server's socket, as herdr gives a plugin: a path no server listens on.
SOCK=$scratch/herdr.sock
export HERDR_SOCKET_PATH="$SOCK"

failures=0
count=0
name=
version=$(sed -n 's/^version = "\(.*\)"$/\1/p' "$plugin/herdr-plugin.toml")

holder_of() {
  head -1 "$1" 2>/dev/null | cut -d' ' -f1
}

# A watch a test started is ended with it: only those whose lease is in
# this run's scratch directory.
reap() {
  for l in "$scratch"/t*/state/servers/*/watch.lease; do
    [ -f "$l" ] || continue
    p=$(holder_of "$l")
    [ -n "$p" ] && [ "$p" != $$ ] && kill "$p" 2>/dev/null
  done
}
trap 'reap; rm -rf "$scratch"' EXIT

# A fresh state, clock and stand-in directory for one test.
fresh() {
  reap
  name=$1
  failed_before=$failures
  count=$((count + 1))
  t=$scratch/t$count
  mkdir -p "$t/state" "$t/fake"
  export HERDR_PLUGIN_STATE_DIR="$t/state" FAKE="$t/fake" FLUME_HERDR_CLOCK="$t/clock"
  echo 1000 >"$t/clock"
  S=$t/state/servers/$(printf '%s' "$SOCK" | cksum | cut -d' ' -f1)
  mkdir -p "$S"
  : >"$FAKE/herdr-calls"
  : >"$FAKE/flume-calls"
}

fail() {
  failures=$((failures + 1))
  printf 'FAIL %s: %s\n' "$name" "$*"
}

pass() {
  [ "$failures" -eq "$failed_before" ] && printf 'ok   %s\n' "$name"
}

# Wait up to $1 seconds for the shell condition $2.
within() {
  n=$1
  while [ "$n" -gt 0 ]; do
    eval "$2" && return 0
    sleep 1
    n=$((n - 1))
  done
  eval "$2"
}

# One tick of the watch at the clock's time $1.
tick_at() {
  echo "$1" >"$t/clock"
  sh "$plugin/bin/watch.sh" --ticks 1 2>>"$t/log"
}

settles() {
  grep -c '^settle$' "$FAKE/flume-calls"
}

# A status of one change place at its code stage, its agent on pane $1
# of this server, $2 of 5 tasks ticked, its agent $3; and its base's
# workspace w1 with its count.
status_of() {
  jq -n --arg pane "$1" --argjson ticked "$2" --arg state "$3" --arg sock "$SOCK" '
    {status: {host: {watch: {bound: {seconds: 30}},
        bases: [{repository: "/r", base: "main", line: "1 running · 0 asking · 0 waiting on caller · 0 stalled",
                 count: {running: 1, asking: 0, waiting_on_caller: 0, stalled: 0}, shown_at: "herdr:w1@\($sock)"}]},
      places: [{place: "/r/places/change--plan", kind: "change", name: "plan", phase: "live",
        current: "code", tasks: [range(5) | {text: "t", ticked: (. < $ticked)}],
        submission: null, waiting_on: null, stalled: false,
        clues: {stage: "code", state: $state, title: "code · \($state)", display_agent: "change--plan",
                shown_at: {place: "herdr:t1@\($sock)", agent: "herdr:\($pane)@\($sock)", base: null}}}]}}'
}

# herdr's listing of panes, each `<id>:<agent state>`, or `<id>:` for a
# pane with no agent in it.
panes() {
  for p in "$@"; do echo "$p"; done | jq -R -s -c '
    {result: {panes: [split("\n")[] | select(. != "") | split(":")
      | {pane_id: .[0], agent: (if .[1] == "" then null else "claude" end),
         agent_status: (if .[1] == "" then null else .[1] end)}]}}' >"$FAKE/panes.json"
}

no_send() {
  if grep -E '^(agent (prompt|send|send-keys|start)|pane (send|close|split|run))' "$FAKE/herdr-calls"; then
    fail "the stand-in herdr was sent to: $(grep -E '^(agent|pane (send|close|split|run))' "$FAKE/herdr-calls" | head -1)"
  fi
}

## the manifest

fresh 'manifest: the plugin'"'"'s own reads as herdr reads it'
sh "$here/manifest.sh" >"$t/out" 2>&1 || fail "$(cat "$t/out")"
pass

fresh 'manifest: a missing script fails it'
mkdir -p "$t/p/bin"
sed 's|bin/watch.sh|bin/gone.sh|' "$plugin/herdr-plugin.toml" >"$t/p/herdr-plugin.toml"
cp "$plugin/bin/places.sh" "$plugin/bin/open.sh" "$t/p/bin/"
if sh "$here/manifest.sh" "$t/p/herdr-plugin.toml" "$plugin/../../Cargo.toml" >"$t/out" 2>&1; then
  fail "a manifest naming a missing script passed"
fi
grep -q 'runs bin/gone.sh, which is not in the plugin' "$t/out" || fail "the missing script not named: $(cat "$t/out")"
pass

fresh 'manifest: a version other than the workspace'"'"'s fails it'
mkdir -p "$t/p"
cp -R "$plugin/bin" "$t/p/"
sed 's/^version = .*/version = "9.9.9"/' "$plugin/herdr-plugin.toml" >"$t/p/herdr-plugin.toml"
if sh "$here/manifest.sh" "$t/p/herdr-plugin.toml" "$plugin/../../Cargo.toml" >"$t/out" 2>&1; then
  fail "a manifest of another version passed"
fi
grep -q "version 9.9.9, and the cargo workspace's is" "$t/out" || fail "the versions not named: $(cat "$t/out")"
pass

## the watch

fresh 'watch: started twice, one watch runs'
FLUME_HERDR_TICK=1 sh "$plugin/bin/watch.sh" --ensure
within 10 '[ -f "$S/watch.lease" ]' || fail "the first start took no lease"
first=$(holder_of "$S/watch.lease")
FLUME_HERDR_TICK=1 sh "$plugin/bin/watch.sh" --ensure
sleep 2
[ "$(holder_of "$S/watch.lease")" = "$first" ] || fail "the second start took the lease"
[ "$(grep -c 'watching' "$S/watch.log")" -eq 1 ] || fail "two watches started: $(cat "$S/watch.log")"
kill -0 "$first" 2>/dev/null || fail "the first watch is gone"
pass

fresh 'watch: a lease whose pid was reused is stale'
# The lease names this shell's pid, alive, with another start time.
printf '%s %s\n%s\n%s\n%s\n\n' $$ "$(date +%s)" "start:Thu Jan  1 00:00:00 1970" "$plugin" "$version" >"$S/watch.lease"
tick_at 1000
[ "$(settles)" -eq 1 ] || fail "the new watch did not settle: $(cat "$FAKE/flume-calls")"
pass

fresh 'watch: a state change settles before the bound'
status_of p1 2 working >"$FAKE/status.json"
panes p1:working
tick_at 1000
tick_at 1005
[ "$(settles)" -eq 1 ] || fail "settled with nothing changed: $(settles)"
panes p1:idle
tick_at 1010
[ "$(settles)" -eq 2 ] || fail "the agent's change from working to idle was not settled at once"
pass

fresh 'watch: an agent come up since status was written settles before the bound (122)'
status_of p1 2 working >"$FAKE/status.json"
panes p1:working
tick_at 1000
tick_at 1004
[ "$(settles)" -eq 1 ] || fail "settled with nothing changed: $(settles)"
# A retry's agent, started by a command of its own: no status names it.
panes p1:working p2:idle
tick_at 1006
[ "$(settles)" -eq 2 ] || fail "the new agent was not settled until the bound"
panes p1:working p2:idle p3:
tick_at 1008
[ "$(settles)" -eq 2 ] || fail "a pane with no agent in it was settled"
pass

fresh 'watch: one listing a tick, no call per pane'
jq '.status.places = [range(5) as $i | .status.places[0] | .place = "/r/places/change--p\($i)"
  | .clues.shown_at.agent = "herdr:p\($i)@'"$SOCK"'"]' "$(status_of p0 1 working >"$t/s" && echo "$t/s")" >"$FAKE/status.json"
panes p0:working p1:working p2:idle p3:working p4:blocked
tick_at 1000
: >"$FAKE/herdr-calls"
tick_at 1002
[ "$(cat "$FAKE/herdr-calls")" = "pane list" ] ||
  fail "a tick asked herdr more than one listing: $(cat "$FAKE/herdr-calls")"
pass

fresh 'watch: nothing changes for three bounds'
status_of p1 2 working >"$FAKE/status.json"
panes p1:working
at=1000
while [ "$at" -le 1090 ]; do
  tick_at "$at"
  at=$((at + 10))
done
[ "$(settles)" -eq 4 ] || fail "not one settle per bound: $(settles) in 90 seconds"
others=$(grep -v -e '^settle$' -e '^status --json$' "$FAKE/flume-calls")
[ -z "$others" ] || fail "ran more of flume's than settle and status: $others"
pass

fresh 'watch: a pane killed by hand is read within one bound (S28)'
status_of p1 2 working >"$FAKE/status.json"
panes p1:working
tick_at 1000
panes
tick_at 1002
[ "$(settles)" -eq 2 ] || fail "the pane closed by hand was not settled within the bound"
pass

fresh 'watch: herdr not answering stops it'
touch "$FAKE/down"
FLUME_HERDR_TICK=1 sh "$plugin/bin/watch.sh" --ticks 5 2>>"$t/log"
grep -q 'herdr does not answer; stopping' "$t/log" || fail "it kept on: $(cat "$t/log")"
pass

## the clues and the sidebar

fresh 'clues: copied from status, nothing sent to a pane'
status_of p1 2 asking >"$FAKE/status.json"
jq --arg sock "$SOCK" '.status.places += [.status.places[0] | .place = "/r/places/fix--far"
  | .clues.shown_at.agent = "herdr:p9@/another.sock"]' "$FAKE/status.json" >"$t/s" && mv "$t/s" "$FAKE/status.json"
panes p1:blocked
tick_at 1000
grep -qx 'pane report-metadata p1 --source flume --title code · asking --display-agent change--plan --token flume=change · code · asking · 2/5' "$FAKE/herdr-calls" ||
  fail "the pane's title and row not reported as status gives them: $(cat "$FAKE/herdr-calls")"
grep -qx 'workspace report-metadata w1 --source flume --token flume=1 running · 0 asking · 0 waiting on caller · 0 stalled' "$FAKE/herdr-calls" ||
  fail "the base's count not reported as status gives it"
grep -q 'p9' "$FAKE/herdr-calls" && fail "a pane of another server was reported here"
no_send
pass

fresh 'clues: a report that fails is logged and the tick goes on'
status_of p1 2 working >"$FAKE/status.json"
panes p1:working
touch "$FAKE/refuse-report"
tick_at 1000
grep -q 'the clues of the pane p1 were not reported: error: pane_not_found' "$t/log" ||
  fail "the failure not logged: $(cat "$t/log")"
jq -e '.status.places[0].kind == "change"' "$S/status.json" >/dev/null || fail "status was not written"
[ "$(settles)" -eq 1 ] || fail "the tick did not settle"
pass

fresh 'sidebar: a row from status'
status_of p1 2 working >"$S/status.json"
echo q | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
grep -q 'change · code · working · 2/5' "$t/out" || fail "the row reads: $(cat "$t/out")"
[ -s "$FAKE/flume-calls" ] && fail "the popup asked flume: $(cat "$FAKE/flume-calls")"
[ -s "$FAKE/herdr-calls" ] && fail "the popup asked herdr: $(cat "$FAKE/herdr-calls")"
pass

fresh 'sidebar: a place waiting on its caller, stalled'
status_of p1 0 waiting | jq '.status.places[0] |= (.waiting_on = {act: "retry"} | .stalled = true | .tasks = null)' >"$S/status.json"
echo q | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
grep -q 'change · code · waiting · stalled' "$t/out" || fail "the row reads: $(cat "$t/out")"
pass

fresh 'keys: retry from the sidebar, one key and the words as lines'
status_of p1 2 'ended short' | jq '.status.places[0] |= (.current = "construct" | .clues.stage = "construct")' >"$S/status.json"
printf 'r\n\nq' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
[ "$(cat "$FAKE/flume-calls")" = "retry /r/places/change--plan --stage construct" ] ||
  fail "the retry ran: $(cat "$FAKE/flume-calls")"
no_send
pass

fresh 'keys: tell, stop with the force asked, and open, each one key'
status_of p1 2 working >"$S/status.json"
printf 'tthe tests are in tests/\nsy\noq' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
printf 'tell /r/places/change--plan the tests are in tests/\nstop /r/places/change--plan --force\n' >"$t/want"
cmp -s "$t/want" "$FAKE/flume-calls" || fail "the keys ran: $(cat "$FAKE/flume-calls")"
grep -qx 'tab focus t1' "$FAKE/herdr-calls" || fail "open did not focus the place's tab: $(cat "$FAKE/herdr-calls")"
no_send
pass

fresh 'keys: q and Escape close at once, with no Enter'
status_of p1 2 working >"$S/status.json"
printf 'qr' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1 || fail "q did not close it cleanly"
printf 'Q' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1 || fail "Q did not close it cleanly"
printf '\033' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1 || fail "Escape did not close it cleanly"
[ -s "$FAKE/flume-calls" ] && fail "a key after q ran: $(cat "$FAKE/flume-calls")"
pass

# Three places, the second and third named p1 and p2.
three() {
  status_of p1 2 working | jq '.status.places = [range(3) as $i | .status.places[0]
    | .place = "/r/places/change--p\($i)" | .name = "p\($i)"]' >"$S/status.json"
}

fresh 'keys: a number chooses the place a key acts on'
three
printf '3sn\n2sn\n' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
printf 'stop /r/places/change--p2\nstop /r/places/change--p1\n' >"$t/want"
cmp -s "$t/want" "$FAKE/flume-calls" || fail "the keys ran: $(cat "$FAKE/flume-calls")"
grep -q '^▶ 2  p1' "$t/out" || fail "the chosen place not marked: $(cat "$t/out")"
pass

fresh 'keys: j and k, and the arrows, move the choice'
three
printf 'jjsn\nksn\n\033[A\033[Asn\n\033[Bsn\n' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
printf 'stop /r/places/change--p2\nstop /r/places/change--p1\nstop /r/places/change--p0\nstop /r/places/change--p1\n' >"$t/want"
cmp -s "$t/want" "$FAKE/flume-calls" || fail "the keys ran: $(cat "$FAKE/flume-calls")"
pass

fresh 'keys: the choice stays among the places'
three
printf 'jjjjsn\nkkkkksn\n' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
printf 'stop /r/places/change--p2\nstop /r/places/change--p0\n' >"$t/want"
cmp -s "$t/want" "$FAKE/flume-calls" || fail "the keys ran: $(cat "$FAKE/flume-calls")"
pass

fresh 'keys: a number with no place is said, and acts on nothing'
status_of p1 2 working >"$S/status.json"
printf '7' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
grep -q 'no place 7' "$t/out" || fail "the missing place not said: $(cat "$t/out")"
[ -s "$FAKE/flume-calls" ] && fail "flume was asked: $(cat "$FAKE/flume-calls")"
pass

fresh 'keys: tell with no words runs nothing'
status_of p1 2 working >"$S/status.json"
printf 't\nq' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
[ -s "$FAKE/flume-calls" ] && fail "flume was asked: $(cat "$FAKE/flume-calls")"
no_send
pass

## unattended

fresh 'unattended: the watch alone drives a place from its launch to its landing (119, 122, I8)'
flume launch --kind fix --name one >/dev/null
panes p1:idle
at=1000
while [ "$at" -le 1300 ] && [ "$(cat "$FAKE/drive")" != tidied ]; do
  tick_at "$at"
  at=$((at + 30))
done
[ "$(cat "$FAKE/drive")" = tidied ] || fail "the place stands at $(cat "$FAKE/drive")"
after=$(sed 1d "$FAKE/flume-calls" | grep -v -e '^settle$' -e '^status --json$')
[ -z "$after" ] || fail "something but the watch's settles ran after the launch: $after"
no_send
pass

fresh 'unattended: a stage waiting for room starts within the bound once the host has room (70, 122)'
touch "$FAKE/no-room"
flume launch --kind fix --name one >/dev/null
tick_at 1000
tick_at 1010
[ "$(cat "$FAKE/drive")" = waiting ] || fail "the place stands at $(cat "$FAKE/drive") with no room"
# The host's configuration changed: there is room now.
rm "$FAKE/no-room"
tick_at 1020
tick_at 1030
[ "$(cat "$FAKE/drive")" = code-working ] || fail "the place stands at $(cat "$FAKE/drive") a bound after the host had room"
after=$(sed 1d "$FAKE/flume-calls" | grep -v -e '^settle$' -e '^status --json$')
[ -z "$after" ] || fail "something but the watch's settles ran after the launch: $after"
no_send
pass

echo
if [ "$failures" -gt 0 ]; then
  echo "$failures of $count failed"
  exit 1
fi
echo "all $count passed"
