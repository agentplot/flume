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
# The processes a test started as an earlier release's watch, ended with
# the run.
olds=
trap 'for p in $olds; do kill "$p" 2>/dev/null; done; rm -rf "$scratch"' EXIT

# A fresh state and stand-in directory for one test.
fresh() {
  name=$1
  failed_before=$failures
  count=$((count + 1))
  t=$scratch/t$count
  mkdir -p "$t/state" "$t/fake"
  # herdr's configuration a scratch file, none until a test writes it.
  export HERDR_PLUGIN_STATE_DIR="$t/state" FAKE="$t/fake" HERDR_CONFIG_PATH="$t/herdr.toml"
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

# herdr's startup of the plugin on this server.
startup() {
  sh "$plugin/bin/startup.sh" 2>>"$t/log"
}

# Every process running a script of the plugin's; none once the startup
# is gone (core.13).
plugin_processes() {
  pgrep -f "$plugin/bin/"
}

# A status of one change place at its code stage, its agent on pane $1
# of this server, $2 of 5 tasks ticked, its agent $3, and its line saying
# so; and its base's workspace w1 with its count.
status_of() {
  jq -n --arg pane "$1" --argjson ticked "$2" --arg state "$3" --arg sock "$SOCK" '
    {status: {host: {bases: [{repository: "/r", base: "main",
        line: "1 running · 0 asking · 0 waiting on caller · 0 stalled",
        count: {running: 1, asking: 0, waiting_on_caller: 0, stalled: 0}, shown_at: "herdr:w1@\($sock)"}]},
      places: [{place: "/r/places/change--plan", kind: "change", name: "plan", phase: "live",
        current: "code", tasks: [range(5) | {text: "t", ticked: (. < $ticked)}],
        submission: null, waiting_on: null, stalled: false,
        clues: {stage: "code", state: $state, title: "code · \($state)", display_agent: "change--plan",
                shown_at: {place: "herdr:t1@\($sock)", agent: "herdr:\($pane)@\($sock)", base: null}},
        line: {name: "change--plan", kind: "change@2", stage: "code", run: 1, tasks: "\($ticked)/5", state: $state,
               full: "change--plan · change@2 · code · run 1 · tasks \($ticked)/5 · \($state)",
               short: "code · run 1 · tasks \($ticked)/5 · \($state)"}}]}}'
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

# The plugin copied to $t/p, its manifest edited by the sed program $1.
copied() {
  mkdir -p "$t/p"
  cp -R "$plugin/bin" "$plugin/lib" "$plugin/README.md" "$t/p/"
  sed "$1" "$plugin/herdr-plugin.toml" >"$t/p/herdr-plugin.toml"
}

fresh 'manifest: a missing script fails it'
copied 's|bin/startup.sh|bin/gone.sh|'
if sh "$here/manifest.sh" "$t/p/herdr-plugin.toml" "$plugin/../../Cargo.toml" >"$t/out" 2>&1; then
  fail "a manifest naming a missing script passed"
fi
grep -q 'runs bin/gone.sh, which is not in the plugin' "$t/out" || fail "the missing script not named: $(cat "$t/out")"
pass

fresh 'manifest: a version other than the workspace'"'"'s fails it'
copied 's/^version = .*/version = "9.9.9"/'
if sh "$here/manifest.sh" "$t/p/herdr-plugin.toml" "$plugin/../../Cargo.toml" >"$t/out" 2>&1; then
  fail "a manifest of another version passed"
fi
grep -q "version 9.9.9, and the cargo workspace's is" "$t/out" || fail "the versions not named: $(cat "$t/out")"
pass

fresh 'manifest: a places action whose description names no entry fails it (122)'
copied 's/^description = "flume'"'"'s places in flight;.*/description = "the places"/'
if sh "$here/manifest.sh" "$t/p/herdr-plugin.toml" "$plugin/../../Cargo.toml" >"$t/out" 2>&1; then
  fail "a places action with no entry described passed"
fi
grep -q "the places action's description names no plugin_action entry" "$t/out" || fail "the description not named: $(cat "$t/out")"
pass

fresh 'manifest: a README whose entry is not the plugin'"'"'s fails it (122)'
copied ''
sed 's/^key = "prefix+f"$/key = "prefix+g"/' "$plugin/README.md" >"$t/p/README.md"
if sh "$here/manifest.sh" "$t/p/herdr-plugin.toml" "$plugin/../../Cargo.toml" >"$t/out" 2>&1; then
  fail "a README with another entry passed"
fi
grep -q "no TOML block of its own is the popup's entry lib/common.sh gives" "$t/out" || fail "the README not named: $(cat "$t/out")"
pass

## the startup

fresh 'startup: herdr'"'"'s startup writes setup.json and exits, no process left, flume asked nothing (122, C.5; core.13)'
status_of p1 2 working >"$FAKE/status.json"
panes p1:working
startup || fail "the startup exited $?"
jq -e --arg sock "$SOCK" '.socket == $sock and (.lacking | length) == 2' "$S/setup.json" >/dev/null ||
  fail "setup.json holds: $(cat "$S/setup.json" 2>&1)"
left=$(plugin_processes)
[ -z "$left" ] || fail "a process of the plugin's is left: $left"
[ -s "$FAKE/flume-calls" ] && fail "the stand-in flume was asked: $(cat "$FAKE/flume-calls")"
no_send
pass

fresh 'startup: agents changing for ten minutes run nothing of the plugin'"'"'s (122, 208)'
# Beside the agent's place, a chore submitted to its base, no agent in it.
status_of p1 2 working | jq '.status.places += [.status.places[0] | .place = "/r/places/chore--tidy"
  | .kind = "chore" | .name = "tidy" | .current = null | .tasks = null | .clues = null
  | .submission = {read: {found: {}}}
  | .line = {name: "chore--tidy", kind: "chore@1", state: "submitted",
             full: "chore--tidy · chore@1 · submitted", short: "submitted"}]' >"$FAKE/status.json"
panes p1:working
startup
: >"$FAKE/herdr-calls"
# Ten minutes of herdr's listing at a reading every ten seconds: the
# agent works and idles, another comes up and goes, a pane closes. The
# plugin keeps no process, so nothing reads them.
n=0
while [ "$n" -lt 60 ]; do
  case $((n % 4)) in
  0) panes p1:working ;;
  1) panes p1:idle p2:working ;;
  2) panes p1:blocked p2:idle p3: ;;
  3) panes ;;
  esac
  n=$((n + 1))
done
sleep 2
grep -q -e '^settle$' -e '^status' "$FAKE/flume-calls" && fail "flume was asked: $(cat "$FAKE/flume-calls")"
grep -q 'report-metadata' "$FAKE/herdr-calls" && fail "the plugin reported to herdr: $(cat "$FAKE/herdr-calls")"
left=$(plugin_processes)
[ -z "$left" ] || fail "a process of the plugin's is left: $left"
pass

# An earlier release's watch on this server: a process of the test's
# own, its lease naming its pid and identity as that release wrote it.
# Started from a shell that exits at once, so the system reaps it once it
# ends.
old_watch() {
  old=$(sh -c 'sleep 600 </dev/null >/dev/null 2>&1 & echo $!')
  olds="$olds $old"
  printf '%s %s\n%s\n%s\n%s\n%s\n' "$old" "$(date +%s)" "$(sh -c '. "$1" && identity "$2"' _ "$plugin/lib/procs.sh" "$old")" \
    "$plugin" "0.0.0" "77" >"$S/watch.lease"
}

fresh 'startup: a watch an earlier release left is ended by its pid and identity, its lease removed (record 0016)'
old_watch
startup
within 5 '! kill -0 "$old" 2>/dev/null' || fail "the earlier watch $old is still running"
[ -f "$S/watch.lease" ] && fail "its lease is left: $(cat "$S/watch.lease")"
grep -q "ended the watch an earlier release left (pid $old)" "$S/startup.log" || fail "the startup logged: $(cat "$S/startup.log" 2>&1)"
left=$(plugin_processes)
[ -z "$left" ] || fail "a process of the plugin's is left: $left"
pass

fresh 'startup: a lease whose pid another process holds ends no process, and is removed (record 0016)'
# The lease names this shell's pid, alive, with another start time.
printf '%s %s\n%s\n%s\n%s\n\n' $$ "$(date +%s)" "start:Thu Jan  1 00:00:00 1970" "$plugin" "0.0.0" >"$S/watch.lease"
startup
kill -0 $$ 2>/dev/null || fail "the process holding the pid was ended"
[ -f "$S/watch.lease" ] && fail "the stale lease is left"
pass

## what herdr's configuration lacks

# The TOML the README gives, the popup's entry and the sidebar's rows.
readme_toml() {
  awk '/^```$/ { inside = 0 } inside { print } /^```toml$/ { inside = 1 }' "$plugin/README.md"
}

no_notification() {
  if grep -i 'notif' "$FAKE/herdr-calls"; then
    fail "the stand-in herdr was asked to notify: $(grep -i 'notif' "$FAKE/herdr-calls" | head -1)"
  fi
}

fresh 'setup: a configuration with neither the key nor the rows is said in setup.json and the startup'"'"'s log (72, 122)'
# Comments name both, and a key of another plugin's is bound.
cat >"$HERDR_CONFIG_PATH" <<'TOML'
# command = "agentplot.flume.places"
#   ["$flume"],
[[keys.command]]
key = "prefix+m"
type = "plugin_action"
command = "agentplot.sluice.board"
TOML
startup
jq -e '.lacking | length == 2' "$S/setup.json" >/dev/null || fail "setup.json holds: $(cat "$S/setup.json" 2>&1)"
jq -e --arg file "$HERDR_CONFIG_PATH" '.lacking[0] | contains("binds no key to flume'"'"'s places") and contains($file)
  and contains("[[keys.command]] entry with key = \"prefix+f\", type = \"plugin_action\", command = \"agentplot.flume.places\", description = \"flume: places in flight\"")' \
  "$S/setup.json" >/dev/null || fail "the key's words: $(jq -r '.lacking[0]' "$S/setup.json")"
jq -e --arg file "$HERDR_CONFIG_PATH" '.lacking[1] | contains("shows no row of flume'"'"'s in the sidebar: add \"$flume\"") and contains($file)' \
  "$S/setup.json" >/dev/null || fail "the rows' words: $(jq -r '.lacking[1]' "$S/setup.json")"
jq -r '.lacking[]' "$S/setup.json" >"$t/words"
while IFS= read -r words; do
  [ "$(grep -cF "$words" "$S/startup.log")" -eq 1 ] || fail "startup.log does not hold once: $words"
done <"$t/words"
no_notification
no_send
pass

fresh 'setup: a configuration with the README'"'"'s entry and rows lacks nothing (122)'
readme_toml >"$HERDR_CONFIG_PATH"
startup
jq -e --arg sock "$SOCK" '.socket == $sock and .lacking == []' "$S/setup.json" >/dev/null ||
  fail "setup.json holds: $(cat "$S/setup.json" 2>&1)"
no_notification
pass

fresh 'setup: no configuration file lacks both, naming its path (122)'
startup
jq -e --arg file "$HERDR_CONFIG_PATH" '.lacking | length == 2 and all(.[]; contains($file))' "$S/setup.json" >/dev/null ||
  fail "setup.json holds: $(cat "$S/setup.json" 2>&1)"
grep -qF "$HERDR_CONFIG_PATH, binds no key to flume's places" "$S/startup.log" || fail "the startup logged: $(cat "$S/startup.log" 2>&1)"
no_notification
pass

fresh 'setup: herdr'"'"'s file found as herdr finds it'
(
  unset HERDR_CONFIG_PATH
  export XDG_CONFIG_HOME="$t/xdg"
  startup
)
jq -e --arg file "$t/xdg/herdr/config.toml" '.lacking | all(.[]; contains($file))' "$S/setup.json" >/dev/null ||
  fail "with XDG_CONFIG_HOME, setup.json names: $(cat "$S/setup.json" 2>&1)"
(
  unset HERDR_CONFIG_PATH XDG_CONFIG_HOME
  export HOME="$t/home"
  startup
)
jq -e --arg file "$t/home/.config/herdr/config.toml" '.lacking | all(.[]; contains($file))' "$S/setup.json" >/dev/null ||
  fail "with HOME alone, setup.json names: $(cat "$S/setup.json" 2>&1)"
pass

## the sidebar

fresh 'sidebar: a row from a settle, reported by flume and nothing of the plugin'"'"'s (139, C.5)'
status_of p1 2 working >"$FAKE/status.json"
panes p1:working
startup
printf 'p1\tcode 1/2 · working\tchange--plan\tcode 1/2 · run 1 · tasks 2/5 · working\n' >"$FAKE/shows"
flume settle >/dev/null
[ "$(grep -c 'report-metadata' "$FAKE/herdr-calls")" -eq 1 ] || fail "the reports: $(cat "$FAKE/herdr-calls")"
grep -qx 'pane report-metadata p1 --source flume --title code 1/2 · working --display-agent change--plan --token flume=code 1/2 · run 1 · tasks 2/5 · working' "$FAKE/herdr-calls" ||
  fail "the row not reported as the settle shows it: $(cat "$FAKE/herdr-calls")"
[ "$(cat "$FAKE/flume-calls")" = settle ] || fail "flume was asked more than the settle: $(cat "$FAKE/flume-calls")"
no_send
pass

## the popup

fresh 'popup: two places, each its number and its line with its location beneath (S14)'
# One working, its line given; the other held.
status_of p1 2 working | jq '.status.places += [.status.places[0]
  | .place = "/r/places/change--held" | .name = "held" | .waiting_on = {act: "retry", words: "after the review"}
  | .line = {name: "change--held", kind: "change@2", stage: "code", run: 1, tasks: "0/5", state: "held",
             full: "change--held · change@2 · code · run 1 · tasks 0/5 · held",
             short: "code · run 1 · tasks 0/5 · held"}]
  | .status.places[0].place = "/r/places/fix--one" | .status.places[0].name = "one"
  | .status.places[0].line |= (.name = "fix--one" | .kind = "fix@1" | .tasks = null
    | .full = "fix--one · fix@1 · code · run 1 · working" | .short = "code · run 1 · working")' >"$FAKE/status.json"
echo q | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
printf '%s\n' '▶ 1  fix--one · fix@1 · code · run 1 · working' '     /r/places/fix--one' \
  '  2  change--held · change@2 · code · run 1 · tasks 0/5 · held' '     /r/places/change--held' >"$t/want"
sed -n 1,4p "$t/out" | cmp -s "$t/want" - || fail "the popup reads: $(cat "$t/out")"
[ "$(cat "$FAKE/flume-calls")" = "status --json" ] || fail "the popup asked flume: $(cat "$FAKE/flume-calls")"
[ -s "$FAKE/herdr-calls" ] && fail "the popup asked herdr: $(cat "$FAKE/herdr-calls")"
pass

fresh 'popup: left open with no key, then a retry, it reads status at its opening and after the retry alone (122)'
status_of p1 2 'ended short' | jq '.status.places[0] |= (.current = "construct" | .clues.stage = "construct")' >"$FAKE/status.json"
{
  sleep 3
  printf 'r\n\nq'
} | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
printf 'status --json\nretry /r/places/change--plan --stage construct\nstatus --json\n' >"$t/want"
cmp -s "$t/want" "$FAKE/flume-calls" || fail "flume was asked: $(cat "$FAKE/flume-calls")"
pass

fresh 'popup: a status that cannot be read is said'
echo 'not json' >"$FAKE/status.json"
echo q | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
grep -q '^flume: flume status gave no status' "$t/out" || fail "the popup reads: $(cat "$t/out")"
pass

fresh 'popup: a place waiting on its caller, stalled, reads as its line says'
status_of p1 0 waiting | jq '.status.places[0] |= (.waiting_on = {act: "retry"} | .stalled = true | .tasks = null
  | .line |= (.tasks = null | .state = "stalled" | .full = "change--plan · change@2 · code · run 1 · stalled"))' >"$FAKE/status.json"
echo q | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
grep -qx '▶ 1  change--plan · change@2 · code · run 1 · stalled' "$t/out" || fail "the row reads: $(cat "$t/out")"
pass

fresh 'popup: a place whose agent shows a dialog reads as its line says'
status_of p1 0 starting | jq '.status.places[0] |= (.waiting_on = {act: "answer", why: "dialog"} | .tasks = null
  | .line |= (.tasks = null | .run = null | .state = "asking" | .full = "change--plan · change@2 · code · asking"))' >"$FAKE/status.json"
echo q | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
grep -qx '▶ 1  change--plan · change@2 · code · asking' "$t/out" || fail "the row reads: $(cat "$t/out")"
pass

fresh 'keys: retry from the sidebar, one key and the words as lines'
status_of p1 2 'ended short' | jq '.status.places[0] |= (.current = "construct" | .clues.stage = "construct")' >"$FAKE/status.json"
printf 'r\n\nq' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
printf 'status --json\nretry /r/places/change--plan --stage construct\nstatus --json\n' >"$t/want"
cmp -s "$t/want" "$FAKE/flume-calls" || fail "the retry ran: $(cat "$FAKE/flume-calls")"
no_send
pass

fresh 'keys: tell, stop with the force asked, and open, each one key'
status_of p1 2 working >"$FAKE/status.json"
printf 'tthe tests are in tests/\nsy\noq' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
printf 'status --json\ntell /r/places/change--plan the tests are in tests/\nstatus --json\nstop /r/places/change--plan --force\nstatus --json\n' >"$t/want"
cmp -s "$t/want" "$FAKE/flume-calls" || fail "the keys ran: $(cat "$FAKE/flume-calls")"
grep -qx 'tab focus t1' "$FAKE/herdr-calls" || fail "open did not focus the place's tab: $(cat "$FAKE/herdr-calls")"
no_send
pass

fresh 'keys: q and Escape close at once, with no Enter'
status_of p1 2 working >"$FAKE/status.json"
printf 'qr' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1 || fail "q did not close it cleanly"
printf 'Q' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1 || fail "Q did not close it cleanly"
printf '\033' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1 || fail "Escape did not close it cleanly"
[ "$(grep -vc '^status --json$' "$FAKE/flume-calls")" -eq 0 ] || fail "a key after q ran: $(cat "$FAKE/flume-calls")"
pass

# Three places, the second and third named p1 and p2.
three() {
  status_of p1 2 working | jq '.status.places = [range(3) as $i | .status.places[0]
    | .place = "/r/places/change--p\($i)" | .name = "p\($i)"
    | .line.name = "change--p\($i)" | .line.full = "change--p\($i) · change@2 · code · run 1 · working"]' >"$FAKE/status.json"
}

fresh 'keys: a number chooses the place a key acts on'
three
printf '3sn\n2sn\n' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
printf 'stop /r/places/change--p2\nstop /r/places/change--p1\n' >"$t/want"
grep -v '^status --json$' "$FAKE/flume-calls" | cmp -s "$t/want" - || fail "the keys ran: $(cat "$FAKE/flume-calls")"
grep -q '^▶ 2  change--p1 · ' "$t/out" || fail "the chosen place not marked: $(cat "$t/out")"
pass

fresh 'keys: j and k, and the arrows, move the choice'
three
printf 'jjsn\nksn\n\033[A\033[Asn\n\033[Bsn\n' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
printf 'stop /r/places/change--p2\nstop /r/places/change--p1\nstop /r/places/change--p0\nstop /r/places/change--p1\n' >"$t/want"
grep -v '^status --json$' "$FAKE/flume-calls" | cmp -s "$t/want" - || fail "the keys ran: $(cat "$FAKE/flume-calls")"
pass

fresh 'keys: the choice stays among the places'
three
printf 'jjjjsn\nkkkkksn\n' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
printf 'stop /r/places/change--p2\nstop /r/places/change--p0\n' >"$t/want"
grep -v '^status --json$' "$FAKE/flume-calls" | cmp -s "$t/want" - || fail "the keys ran: $(cat "$FAKE/flume-calls")"
pass

fresh 'keys: a number with no place is said, and acts on nothing'
status_of p1 2 working >"$FAKE/status.json"
printf '7' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
grep -q 'no place 7' "$t/out" || fail "the missing place not said: $(cat "$t/out")"
[ "$(cat "$FAKE/flume-calls")" = "status --json" ] || fail "flume was asked: $(cat "$FAKE/flume-calls")"
pass

fresh 'keys: tell with no words runs nothing'
status_of p1 2 working >"$FAKE/status.json"
printf 't\nq' | sh "$plugin/bin/places.sh" >"$t/out" 2>&1
[ "$(cat "$FAKE/flume-calls")" = "status --json" ] || fail "flume was asked: $(cat "$FAKE/flume-calls")"
no_send
pass

echo
if [ "$failures" -gt 0 ]; then
  echo "$failures of $count failed"
  exit 1
fi
echo "all $count passed"
