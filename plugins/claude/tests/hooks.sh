#!/usr/bin/env bash
# The plugin's hooks, each run by the built binary as Claude Code 2.1.296
# runs it: `flume hook <event>` with the hook's input on standard input
# (fixtures/, as that release handed them to a hook) and the agent's
# environment, over a fixture transcript and a scratch state directory.
# No Claude Code runs: this script's own shell stands in for the program
# a hook runs under, a scratch `claude` on the hooks' PATH naming it. Nothing
# of the user's state, configuration or git is read or written.

set -u

here=$(cd "$(dirname "$0")" && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
fixtures=$here/fixtures

cargo build --quiet -p flume --bin flume --manifest-path "$root/Cargo.toml" || exit 1
target=$(cargo metadata --format-version 1 --no-deps --manifest-path "$root/Cargo.toml" | jq -r .target_directory)
flume=$target/debug/flume

scratch=$(mktemp -d "${SLUICE_TMP:-${TMPDIR:-/tmp}}/flume-plugin.XXXXXX")
trap 'rm -rf "$scratch"' EXIT
scratch=$(cd "$scratch" && pwd -P)

# Nothing of the caller's flume, terminal or Claude Code session, and git
# with no configuration but the scratch's.
for name in $(env | grep -Eo '^(FLUME_|HERDR_|CLAUDE)[A-Za-z0-9_]*=' | tr -d =); do
  unset "$name"
done
export XDG_STATE_HOME=$scratch/state XDG_CONFIG_HOME=$scratch/config \
  XDG_DATA_HOME=$scratch/data HOME=$scratch/home
export GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null GIT_TERMINAL_PROMPT=0 \
  GIT_AUTHOR_NAME="flume tests" GIT_AUTHOR_EMAIL=tests@flume.invalid \
  GIT_COMMITTER_NAME="flume tests" GIT_COMMITTER_EMAIL=tests@flume.invalid
mkdir -p "$HOME" "$XDG_STATE_HOME" "$XDG_CONFIG_HOME" "$XDG_DATA_HOME"

failures=0
fail() {
  echo "FAIL: $*" >&2
  failures=$((failures + 1))
}

# The hook for event $1 with the fixture $2, the transcript and the place
# filled in, in the environment the rest of the words set.
hook() {
  local event=$1 fixture=$2
  shift 2
  sed -e "s#__TRANSCRIPT__#$scratch/transcript.jsonl#" -e "s#__CWD__#${place:-$scratch}#" \
    "$fixtures/$fixture" | env "$@" "$flume" hook "$event"
}

# Every file under flume's state directory, with its digest.
state() {
  (cd "$XDG_STATE_HOME" && find . -type f -exec shasum {} + | sort)
}

cp "$fixtures/transcript.jsonl" "$scratch/transcript.jsonl"

# The hook's parent is this script's shell, so the program a start's record
# names, `claude`, is found on the hooks' PATH as that shell (161).
mkdir -p "$scratch/bin"
ln -s "$BASH" "$scratch/bin/claude"
hookpath=$scratch/bin:$PATH

# A repository bound to git-worktree alone, and a place launched in it
# under the printed command, its agent Claude Code.
repo=$scratch/repository
git init --quiet -b main "$repo"
mkdir -p "$repo/.config"
printf '[profiles]\nplace = "git-worktree"\n' >"$repo/.config/flume.toml"
printf 'places/\n' >"$repo/.gitignore"
git -C "$repo" add -A
git -C "$repo" commit --quiet -m "the binding"
(cd "$repo" && "$flume" launch --kind fix --name one --brief "fix the parser" \
  --kinds "$fixtures/kinds.toml" --json >/dev/null) || fail "the launch"
place=$repo/places/fix--one

status() {
  (cd "$repo" && "$flume" status "$place" --json) | jq '.status.places[0]'
}
printed=$(status | jq -r '.agent.printed')
# The command's environment is in the file the line names, never in the
# line (32, 56).
envfile=$(printf '%s\n' "$printed" | tr ' ' '\n' | tr -d "'" | grep '\.env$' | head -n 1)
[ -f "$envfile" ] || fail "the printed command names no environment file: $printed"
word() {
  sed -n "s/^$1=//p" "$envfile" | tr -d "'"
}
plugin=$(printf '%s\n' "$printed" | tr ' ' '\n' | sed -n '/^--plugin-dir$/{n;p;}')
agent=(FLUME_AGENT="$(word FLUME_AGENT)" FLUME_SECRET="$(word FLUME_SECRET)"
FLUME_PLACE="$(word FLUME_PLACE)" FLUME_STAGE="$(word FLUME_STAGE)"
CLAUDE_PLUGIN_ROOT="$plugin" PATH="$hookpath")
[ -n "$(word FLUME_SECRET)" ] || fail "the printed command's environment holds no secret"
case $printed in *"$(word FLUME_SECRET)"*) fail "the printed command holds its secret" ;; esac
[ -f "$plugin/hooks/hooks.json" ] || fail "no plugin at $plugin"
grep -q "\"command\": \"$flume\"" "$plugin/hooks/hooks.json" ||
  grep -q "\"command\": \"$(cd "$(dirname "$flume")" && pwd -P)/flume\"" "$plugin/hooks/hooks.json" ||
  fail "the plugin's hooks do not name the binary by its path"

# A session the operator started: no FLUME_* in its environment. Each hook
# allows what it was asked, writes nothing and exits 0.
before=$(state)
for pair in session-start:session-start.json user-prompt-submit:user-prompt-submit.json \
  pre-tool-use:pre-tool-use-push.json pre-tool-use:pre-tool-use-ask.json \
  permission-request:permission-request.json post-tool-use:post-tool-use.json \
  stop:stop.json stop-failure:stop-failure.json session-end:session-end.json; do
  out=$(hook "${pair%%:*}" "${pair#*:}")
  code=$?
  [ "$code" -eq 0 ] || fail "${pair%%:*} outside flume's agent exited $code"
  [ -z "$out" ] || fail "${pair%%:*} outside flume's agent answered: $out"
done
[ "$(state)" = "$before" ] || fail "a hook outside flume's agent wrote flume's state"

# The started ack.
[ "$(status | jq -r .agent.started)" = no ] || fail "started before the ack"
hook session-start session-start.json "${agent[@]}" >/dev/null || fail "session-start exited non-zero"
[ "$(status | jq -r .agent.started)" = yes ] || fail "the started ack was not taken"

# The asking mark: a question put to a person, answered.
hook pre-tool-use pre-tool-use-ask.json "${agent[@]}" >/dev/null
[ "$(status | jq -r .agent.asking)" = yes ] || fail "a question open is not asking"
hook post-tool-use post-tool-use.json "${agent[@]}" >/dev/null
[ "$(status | jq -r .agent.asking)" = no ] || fail "a question answered is still asking"
# A permission prompt, then the next prompt clears it.
hook permission-request permission-request.json "${agent[@]}" >/dev/null
[ "$(status | jq -r .agent.asking)" = yes ] || fail "a permission prompt is not asking"
hook user-prompt-submit user-prompt-submit.json "${agent[@]}" >/dev/null
[ "$(status | jq -r .agent.asking)" = no ] || fail "the next prompt left the mark"

# The guard: a push refused and reported; a merge of the base allowed.
out=$(hook pre-tool-use pre-tool-use-push.json "${agent[@]}")
[ "$(printf '%s' "$out" | jq -r .hookSpecificOutput.permissionDecision)" = deny ] ||
  fail "a push was not refused: $out"
status | jq -e '.refused_acts | map(select(contains("git push"))) | length == 1' >/dev/null ||
  fail "status does not report the refused push"
out=$(hook pre-tool-use pre-tool-use-merge.json "${agent[@]}")
[ -z "$out" ] || fail "a merge of the base was refused: $out"

# The stop check: nothing made, so the stop is sent back.
out=$(hook stop stop.json "${agent[@]}")
[ "$(printf '%s' "$out" | jq -r .decision)" = block ] || fail "a stop with nothing made: $out"
printf '%s' "$out" | jq -r .reason | grep -q "flume needs" || fail "the stop's words: $out"

# The failed turn's reason is the agent's last words.
hook stop-failure stop-failure.json "${agent[@]}" >/dev/null
status | jq -e '.agent.last_words | startswith("Not logged in")' >/dev/null ||
  fail "the failed turn's reason is not the last words"

# A usage limit's failure, its words the test's: the last words, and the
# agent's record holding its error, its words and when the hook ran.
ran=$(date +%s)
hook stop-failure stop-failure-limit.json "${agent[@]}" >/dev/null
status | jq -e '.agent.last_words | startswith("You'"'"'ve hit your session limit")' >/dev/null ||
  fail "the usage limit's words are not the last words"
failure=$(find "$XDG_STATE_HOME" -name failure.json | head -n 1)
[ -n "$failure" ] || fail "the failed turn is not in the agent's record"
jq -e --argjson ran "$ran" '.error == "rate_limit"
  and (.words | startswith("You'"'"'ve hit your session limit")) and .at >= $ran' "${failure:-/dev/null}" >/dev/null ||
  fail "the agent's record does not hold the failure's error, words and time: $(cat "${failure:-/dev/null}")"

# On quiet: the prompt received, a commit made, the stop let through, and
# once the session ended the place settles itself on to the next stage.
prompt=$(cd "$repo" && "$flume" prompt "$place" code --json | jq -r .prompt.text)
jq -c --arg prompt "$prompt" --arg transcript "$scratch/transcript.jsonl" --arg cwd "$place" \
  '.prompt = $prompt | .transcript_path = $transcript | .cwd = $cwd' \
  "$fixtures/user-prompt-submit.json" | env "${agent[@]}" "$flume" hook user-prompt-submit
printf 'fixed\n' >"$place/parser.txt"
git -C "$place" add parser.txt
git -C "$place" -c core.hooksPath=/dev/null commit --quiet -m "fix the parser"
out=$(hook stop stop.json "${agent[@]}")
[ -z "$out" ] || fail "a stop with the deliverable made was blocked: $out"
hook session-end session-end.json "${agent[@]}" >/dev/null
for _ in $(seq 1 40); do
  [ "$(status | jq -r .current)" = review ] && break
  sleep 0.25
done
[ "$(status | jq -r .current)" = review ] || fail "the stop did not settle the place: $(status)"

# The start checked against its settings: a place whose start runs under
# an account giving CLAUDE_CONFIG_DIR a scratch folder.
mkdir -p "$XDG_CONFIG_HOME/flume" "$scratch/account-1"
cat >"$XDG_CONFIG_HOME/flume/config.toml" <<TOML
[host]
pool = "work"

[pools.work]
strategy = "fill-first"
accounts = [{ name = "work-1", program = "claude", env = { CLAUDE_CONFIG_DIR = "$scratch/account-1" } }]
TOML
(cd "$repo" && "$flume" launch --kind fix --name two --brief "fix the lexer" \
  --kinds "$fixtures/kinds.toml" --json >/dev/null) || fail "the launch under an account"
place=$repo/places/fix--two
printed=$(status | jq -r '.agent.printed')
envfile=$(printf '%s\n' "$printed" | tr ' ' '\n' | tr -d "'" | grep '\.env$' | head -n 1)
two=(FLUME_AGENT="$(word FLUME_AGENT)" FLUME_SECRET="$(word FLUME_SECRET)"
FLUME_PLACE="$(word FLUME_PLACE)" FLUME_STAGE="$(word FLUME_STAGE)"
CLAUDE_PLUGIN_ROOT="$plugin")
unsettled() {
  find "$XDG_STATE_HOME" -path "*fix--two*" -name unsettled.json | head -n 1
}

# A variable with another value: no started ack, the agent's record naming
# the variable as different and holding no value, a settle of the place
# started, which reads the start begun without its settings.
hook session-start session-start.json "${two[@]}" PATH="$hookpath" \
  CLAUDE_CONFIG_DIR="$scratch/account-elsewhere" >/dev/null || fail "session-start exited non-zero"
[ "$(status | jq -r .agent.started)" = no ] || fail "the ack was taken with another CLAUDE_CONFIG_DIR"
word_file=$(unsettled)
jq -e '.different == ["CLAUDE_CONFIG_DIR"] and .missing == [] and .program == false' \
  "${word_file:-/dev/null}" >/dev/null || fail "the record does not name CLAUDE_CONFIG_DIR as different: $(cat "${word_file:-/dev/null}")"
grep -q "account-" "${word_file:-/dev/null}" && fail "the record holds a value"
for _ in $(seq 1 40); do
  [ "$(status | jq '.unsettled | length')" = 1 ] && break
  sleep 0.25
done
[ "$(status | jq -r '.unsettled[0].different[0]')" = CLAUDE_CONFIG_DIR ] ||
  fail "the settle did not read the start begun without its settings: $(status | jq -c .unsettled)"

# Another program: the hook's parent runs other than the `claude` its PATH
# finds.
mkdir -p "$scratch/elsewhere"
ln -s /bin/sh "$scratch/elsewhere/claude"
hook session-start session-start.json "${two[@]}" PATH="$scratch/elsewhere:$PATH" \
  CLAUDE_CONFIG_DIR="$scratch/account-1" >/dev/null || fail "session-start exited non-zero"
[ "$(status | jq -r .agent.started)" = no ] || fail "the ack was taken under another program"
jq -e '.program == true' "$(unsettled)" >/dev/null || fail "the record does not say the program differed"

# Another mode: a place launched with flume's full, which the start passes
# as bypassPermissions, its session begun in `default`, as Claude Code
# begins where the account's configuration disables the mode asked.
# SessionStart's input names no mode, so the started ack is taken on the
# program and the variables; the start's first prompt names `default`, so
# it is blocked, no received ack is written, and the agent's record names
# the mode asked and the mode begun, `default` read as `manual` (161, 162,
# S67).
(cd "$repo" && "$flume" launch --kind fix --name three --brief "fix the reader" \
  --kinds "$fixtures/kinds.toml" --agent mode=full --json >/dev/null) || fail "the launch in full"
place=$repo/places/fix--three
[ "$(status | jq -r .agent.mode.passed)" = bypassPermissions ] ||
  fail "the start does not record bypassPermissions: $(status | jq -c .agent.mode)"
printed=$(status | jq -r '.agent.printed')
envfile=$(printf '%s\n' "$printed" | tr ' ' '\n' | tr -d "'" | grep '\.env$' | head -n 1)
three=(FLUME_AGENT="$(word FLUME_AGENT)" FLUME_SECRET="$(word FLUME_SECRET)"
FLUME_PLACE="$(word FLUME_PLACE)" FLUME_STAGE="$(word FLUME_STAGE)"
CLAUDE_PLUGIN_ROOT="$plugin" PATH="$hookpath" CLAUDE_CONFIG_DIR="$scratch/account-1")
hook session-start session-start.json "${three[@]}" >/dev/null || fail "session-start exited non-zero"
[ "$(status | jq -r .agent.started)" = yes ] || fail "the started ack was not taken on the program and the variables"
blocked=$(hook user-prompt-submit user-prompt-submit-default.json "${three[@]}") || fail "user-prompt-submit exited non-zero"
printf '%s' "$blocked" | jq -e '.decision == "block"' >/dev/null ||
  fail "the first prompt in another mode was not blocked: $blocked"
received=$(find "$XDG_STATE_HOME" -path "*fix--three*" -name received | head -n 1)
[ ! -s "${received:-/dev/null}" ] || fail "the prompt held back was acked received"
word_file=$(find "$XDG_STATE_HOME" -path "*fix--three*" -name unsettled.json | head -n 1)
jq -e '.mode == {"asked": "bypassPermissions", "began": "manual"} and .different == [] and .missing == [] and .program == false' \
  "${word_file:-/dev/null}" >/dev/null || fail "the record does not name both modes: $(cat "${word_file:-/dev/null}")"

if [ "$failures" -gt 0 ]; then
  echo "$failures of the plugin's hook checks failed" >&2
  exit 1
fi
echo "the plugin's hooks pass"
