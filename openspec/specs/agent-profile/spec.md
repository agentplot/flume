# agent-profile Specification

## Purpose
The agent seam, Part B.4, through Claude Code: the command that starts
the pinned program with flume's plugin and the start's variables, its
folder-trust question and answer, and the readings of a start (started,
received, asking, ended, last words) from the plugin's acknowledgements
and the program's transcript.

## Requirements

### Requirement: An agent started as Claude Code with flume's plugin

The agent profile SHALL start an agent as `claude` in the place, with
the plugin of the flume that started it loaded by `--plugin-dir`, the
settings' model and instructions, an environment of the host user's
login environment with its account's variables over it, and
`FLUME_AGENT`, `FLUME_SECRET`, `FLUME_PLACE` and `FLUME_STAGE`, the
identity naming the repository by an id from its common git directory;
and the prompt as its argument under the printed command (29, 32, 33,
88, 144, C.4).

#### Scenario: The command for a code stage

- **WHEN** the profile gives the command for a stage with a model and instructions under an account giving `CLAUDE_CONFIG_DIR`
- **THEN** it runs `claude` with `--plugin-dir`, `--model` and `--append-system-prompt-file`, naming a file of the start's own beside its record that holds the instructions as given, newlines and all, so no argument holds a newline, the account's `CLAUDE_CONFIG_DIR`, the four `FLUME_*` words, and nothing of the caller's session

### Requirement: Readings from the agent's own record

The profile SHALL read own-work and asking from the transcript of the
session the plugin's SessionStart hook recorded for that start in the
place's own folder and the plugin's marks; last words from the
transcript's last message or the reason the StopFailure hook recorded;
and whether the session ended from the SessionEnd hook's mark (134); an
answer the record does not give SHALL be unknown (43, 77, 90, 91, 93).

#### Scenario: Seam scenarios over fixture transcripts

- **WHEN** the agent seam's scenarios run against the profile over fixture transcripts of the pinned Claude Code
- **THEN** each passes: a prompt received by its ack, a background shell started and not finished as own work, an open question as asking, and a missing transcript as unknown

### Requirement: Started and received from the plugin's acks

The profile SHALL read started from the start's started ack, carrying
its identity, secret, the plugin's version and the session, and
received from the start's received ack, carrying the submitted prompt's
digest; only where no ack can come SHALL it read received from the
transcript's user turn or queued `enqueue` record (38, 77, 89, 135;
record 0016).

#### Scenario: A bare resume

- **WHEN** herdr resumes a start's session with a bare `claude --resume`, no `FLUME_*` in its environment and no `--plugin-dir`
- **THEN** no started ack comes for the start, and `started` reads no (77, S41)

#### Scenario: A prompt queued while the agent worked

- **WHEN** a prompt is typed while the agent works and Claude Code submits it after the turn
- **THEN** the received ack comes at the submission, and `received` reads yes then and not before (38, 89)

#### Scenario: A plugin of another version

- **WHEN** the started ack names a plugin version other than the binary's
- **THEN** status reports the difference with the agent

### Requirement: The stop check, the guard and on-quiet through hooks

The profile SHALL offer send-back, the guard and on-quiet: the Stop hook
blocks a stop with the core's stop-check words and, when it lets the
agent stop, starts `flume settle <place> --on-quiet` detached; the
PreToolUse hook refuses a shell command that runs `sluice submit`, `wt
submit`, `wt merge` or `git push`, saying flume submits the place, and
records the refusal status reads, and never refuses `git merge` into the
place's branch (108, 110, 112, 115, 116, 121, C.4).

#### Scenario: An agent that would stop with nothing made

- **WHEN** the Stop hook runs for a code stage's agent with no commit since its start
- **THEN** it blocks the stop with words naming commits and `flume needs` (S24)

#### Scenario: An agent tries to push

- **WHEN** the PreToolUse hook is handed `git add -A && git push origin HEAD`
- **THEN** it refuses the command, and status lists the refused act (S25)

#### Scenario: An agent merges the base for a conflict

- **WHEN** the PreToolUse hook is handed `git fetch origin && git merge 4f2a9c1` in the place
- **THEN** it allows the command and records no refusal (112)

#### Scenario: A conversation's agent stops

- **WHEN** the Stop hook runs for a conversation's agent
- **THEN** it lets it stop and blocks nothing (109)

### Requirement: A wrapped Claude Code starts as the program it wraps

The Claude Code agent profile SHALL name as the program the one a nix
wrapper the settings name execs, with the variables the wrapper sets,
so that herdr detects the agent and flume's foreground check takes the
program that runs (32; record 0018; C.5).

#### Scenario: A wrapper exec'ing its program

- **WHEN** the settings name a wrapper `bin/claude` that execs `bin/.claude` with variables of its own
- **THEN** the start runs `bin/.claude` under the name `claude` with those variables, herdr reads it as Claude Code, and the start is taken

### Requirement: A prompt submitted inside paste tags is received as sent

The Claude Code agent profile SHALL compare a submitted prompt with the
prompt sent with Claude Code's paste tags taken off and its whitespace
then collapsed, in the received ack and the transcript alike (38, 89;
record 0016).

#### Scenario: A prompt submitted inside paste tags

- **WHEN** a terminal pastes the prompt and Claude Code submits it inside `<pasted_content id="fad0">` and `</pasted_content id="fad0">`, newlines around them
- **THEN** the received ack carries the digest of the prompt as sent, `received` reads yes, and the stage begins

#### Scenario: Text that only looks like a paste tag

- **WHEN** a prompt holds `<pasted_content id="FAD0">`, whose id is not four lowercase hex digits
- **THEN** the text stays as it is in the comparison

### Requirement: A usage limit read from Claude Code's own words

The Claude Code profile's `spent` SHALL read yes only where the
StopFailure hook recorded `rate_limit` with words that open as the
pinned release's usage-limit messages do, with the reset their
` · resets <when> (<zone>)` part names, or, for a budget, their
` · your <limit> resets <when> (<zone>)` part, read in that zone from
when the hook ran. Words that name no reset, or one in
another form, SHALL read yes with none; a `rate_limit` with other words
SHALL read no (146, 151, C.4).

#### Scenario: A session limit with a time of day

- **WHEN** the hook ran at 13:10 in New York and recorded `rate_limit` with "You've hit your session limit · resets 3pm (America/New_York)"
- **THEN** `spent` reads yes until 15:00 that day in New York (151, C.4)

#### Scenario: A weekly limit with a date

- **WHEN** the hook ran on 2026-11-05 at 20:40 UTC and recorded `rate_limit` with "You've hit your weekly limit · resets Nov 8 at 12am (America/New_York)"
- **THEN** `spent` reads yes until 2026-11-08 at 05:00 UTC (151, C.4)

#### Scenario: A reset past midnight

- **WHEN** the hook ran at 23:40 in UTC and the words say "resets 1am (UTC)"
- **THEN** `spent` reads yes until 01:00 UTC the next day (151)

#### Scenario: A budget with its plan limit's reset

- **WHEN** the words are "You've hit your team's shared budget · raise it at https://claude.ai/admin-settings/usage · your weekly limit resets Nov 8 at 12am (America/New_York)"
- **THEN** `spent` reads yes until 2026-11-08 at 05:00 UTC (151, C.4)

#### Scenario: No reset in the words

- **WHEN** the words are "You've hit your limit"
- **THEN** `spent` reads yes with no reset (146, 151)

#### Scenario: A reset in a form the release does not write

- **WHEN** the words are "You've hit your weekly limit · resets Nov 2, 3:30pm (UTC) · progress saved"
- **THEN** `spent` reads yes with no reset (146, 151, C.4)

#### Scenario: A model under high load

- **WHEN** the hook recorded `rate_limit` with "Opus is experiencing high load, please use /model to switch to Sonnet"
- **THEN** `spent` reads no, and the words stay the agent's last words (43, 151)

### Requirement: The credentials Claude Code keeps from its hooks

The Claude Code profile's `withheld` SHALL name the credential
variables the pinned release keeps from every process it starts, its
hooks among them: its own list of credentials, the file-descriptor ones
it spreads into that list, and the memory service's token (161, C.4).

#### Scenario: The account's token

- **WHEN** a start's account gives `CLAUDE_CONFIG_DIR` and `CLAUDE_CODE_OAUTH_TOKEN`
- **THEN** `withheld` names `CLAUDE_CODE_OAUTH_TOKEN` and not `CLAUDE_CONFIG_DIR`, the start's record names `CLAUDE_CONFIG_DIR` alone of the two, and the SessionStart hook, whose environment holds no token, acks (161)

### Requirement: Own work read before the first turn

The Claude Code profile's `own_work` SHALL read no for a session whose
transcript the release has not yet written, since it writes the
transcript at the session's first message and own work starts only in a
turn; a transcript that cannot be read SHALL read unknown (91, 93).

#### Scenario: Idle at its prompt

- **WHEN** an agent's SessionStart hook recorded its session and transcript, and the agent sits at its prompt with no turn taken and no transcript written
- **THEN** `own_work` reads no and `asking` reads no (31, 90, 91)

### Requirement: The words pinned to the release the tree holds

A `check-tools` test SHALL read the pinned Claude Code and require the
source of its usage-limit words to stand byte for byte as the profile
reads them: its list of usage-limit openings, the `You've hit your`
template, the ` · resets ` part, the budget's ` · your <limit> resets `
part, the zone's suffix, and its reset formatter's choice of a date past
a day off and its options for a date and for a time of day (core.22,
C.4).

#### Scenario: The pinned release read

- **WHEN** the test reads the program devenv.nix pins, the file its `claude` wrapper runs
- **THEN** each fragment the profile's reading rests on is found in it, and the test names any that is not (C.4)

### Requirement: The credentials pinned to the release the tree holds

A `check-tools` test SHALL require the source the profile's `withheld`
rests on to stand byte for byte in the pinned Claude Code: its lists of
credentials, the memory service's pair, the test of the environment
against the list, the names its child environment drops and the drop,
and the hook runner's environment built on that child environment
(core.22, C.4).

#### Scenario: What the pinned release keeps from its hooks

- **WHEN** the test reads the program devenv.nix pins
- **THEN** each fragment `withheld` rests on is found in it, and the test names any that is not (161, C.4)

### Requirement: The settings' mode passed by --permission-mode

The agent profile SHALL start `claude` with `--permission-mode` and the
value its map gives the settings' mode where they give one, and with no
`--permission-mode` where they give none (162, 169, C.4).

#### Scenario: The command for a stage with no mode

- **WHEN** the profile gives the command for a stage whose settings give no mode
- **THEN** it runs `claude` with no `--permission-mode` (162)

#### Scenario: The command for a stage in edits

- **WHEN** the profile gives the command for a stage whose settings give `edits`
- **THEN** it runs `claude` with `--permission-mode acceptEdits` (S66, 169)

### Requirement: Claude Code's map of flume's modes and its own values

The Claude Code profile SHALL map `plan` to `plan`, `ask` to `manual`,
`edits` to `acceptEdits` and `full` to `bypassPermissions`, and SHALL
take its own values under `claude` in a table of programs, the pinned
2.1.296's `--permission-mode` choices `acceptEdits`, `auto`,
`bypassPermissions`, `manual`, `dontAsk` and `plan`, each passed as
written; any other own value SHALL be refused naming it, `claude` and
the values it names; and it SHALL read the mode back (161, 169, C.4).

#### Scenario: Each of the four

- **WHEN** the profile maps `plan`, `ask`, `edits` and `full`
- **THEN** it gives `plan`, `manual`, `acceptEdits` and `bypassPermissions` (169, C.4)

#### Scenario: Its own value under its name

- **WHEN** the profile maps `{ claude = "auto" }`'s entry, and then `{ claude = "yolo" }`'s
- **THEN** the first gives `auto`, and the second is refused naming `yolo`, `claude` and its six values (169, C.4)

#### Scenario: The mode read back

- **WHEN** the profile reads back `default`, `acceptEdits` and `plan`
- **THEN** it reads `manual`, `acceptEdits` and `plan` (161, C.4)

### Requirement: The modes pinned to the release the tree holds

A `check-tools` test SHALL read the pinned Claude Code and require the
source of its modes to stand byte for byte as the profile's map rests
on it: the release's own modes, its alias reading `manual` as
`default`, the `--permission-mode` option, the hook input's builder
naming the mode, UserPromptSubmit's input built with the session's
mode, and a hook's block holding back the prompt (core.22, C.4).

#### Scenario: The pinned release's modes read

- **WHEN** the test reads the program devenv.nix pins, the file its `claude` wrapper runs
- **THEN** each fragment the map rests on is found in it, and the test names any that is not (169, C.4)

### Requirement: Claude Code's command resumes the session a start carries

Where a start carries a conversation, the Claude Code profile's command
SHALL add `--resume` with the session the start resumes, and SHALL add
neither where the start carries none (88, 179, C.4).

#### Scenario: The command of a carried start

- **WHEN** the profile gives the command for a start resuming the session `s-1`, and for a start resuming none
- **THEN** the first holds `--resume s-1` beside `--plugin-dir`, and the second holds no `--resume` (88, 179)

### Requirement: Each start reads only its own part of its transcript

The Claude Code profile SHALL read own work, last words, asking's answer
and a slash command's receipt only from the records its start's
transcript holds past the length the start's first started ack
recorded, and the whole transcript where the file is shorter than that
length (43, 89, 90, 91, C.4).

#### Scenario: A background task the stopped agent left

- **WHEN** a carried start's transcript holds a background shell the stopped agent started and never finished, before the length its started ack recorded, and nothing of the carried agent's own still runs
- **THEN** `own_work` reads no for the carried start (31, 91)

#### Scenario: Last words before the carried agent wrote any

- **WHEN** a carried start's agent stops before it writes a message, and the transcript's last message is the stopped agent's
- **THEN** the carried start's last words are none, and the stopped start's are its own (43)

#### Scenario: A fresh start reads its whole transcript

- **WHEN** a start that carries nothing recorded a length of 0 at its started ack
- **THEN** every reading reads its whole transcript, as before (91)

### Requirement: Claude Code's carry pinned to the release the tree holds

A `check-tools` test SHALL read the pinned Claude Code and require the
source its carry rests on to stand byte for byte: `--resume`'s and
`--fork-session`'s option text, the transcript's folder under the
configuration home's `projects`, and the words it ends with when it
finds no such session (core.22, C.4).

#### Scenario: The pinned release's resume read

- **WHEN** the test reads the program devenv.nix pins, the file its `claude` wrapper runs
- **THEN** each fragment the carry rests on is found in it, and the test names any that is not (C.4)

### Requirement: A reset read as the pinned release's formatter writes it

The Claude Code profile SHALL read a reset's `<when>` as the pinned
release's reset formatter writes it: a time of day being the first such
time no more than a minute before the hook ran, and a date and a time
joined by ` at ` being that date, in the year the hook ran in the zone
where the date names none (151, C.4).

#### Scenario: A reset in the minute the hook ran

- **WHEN** the hook ran 20 seconds past 15:00 UTC and the words say "resets 3pm (UTC)"
- **THEN** `spent` reads yes until 15:00 UTC that day, not the next (151)

#### Scenario: A date with its minutes

- **WHEN** the hook ran on 2026-11-05 in UTC and the words say "resets Nov 12 at 3:30pm (UTC)"
- **THEN** `spent` reads yes until 2026-11-12 at 15:30 UTC (151, C.4)

#### Scenario: A date in another year

- **WHEN** the hook ran on 2026-12-30 in London and the words say "resets Jan 3, 2027 at 9am (Europe/London)"
- **THEN** `spent` reads yes until 3 January 2027 at 09:00 in London (151, C.4)

### Requirement: Claude Code's own settings passed by --settings

Where the agent's resolved keys for `claude` are not empty, the agent
profile SHALL write them as one JSON object to `own-settings.json` in
the start's folder and start `claude` with `--settings` naming that
file; where they are empty it SHALL pass no `--settings` and write no
file. It SHALL never pass a file a layer named. The release lays them
over the user's, the project's and the local settings (88, 195, 196,
C.4).

#### Scenario: The command for a stage with own settings

- **WHEN** the profile gives the command for a stage whose resolved keys for `claude` give `tui = "fullscreen"` from a named file and `statusLine` from a table's `settings`
- **THEN** it runs `claude` with `--settings <folder>/claude/<identity>/own-settings.json` after `--plugin-dir`, its files hold that path with both keys as JSON, and its line names no path of the named file (S100, 196)

#### Scenario: The command for a stage with none

- **WHEN** the profile gives the command for a stage whose settings give `claude` no key
- **THEN** it runs `claude` with no `--settings`, and its files are empty (195)

### Requirement: Claude Code's reserved keys named

The agent profile SHALL name as reserved, each with what owns it,
Claude Code's `env`, `model`, `effortLevel` and
`permissions.defaultMode`, its credential helpers `apiKeyHelper`, `awsAuthRefresh`,
`awsCredentialExport`, `gcpAuthRefresh` and `otelHeadersHelper`, its
sign-in keys `forceLoginMethod` and `forceLoginOrgUUID`,
`disableAllHooks`, and `skipDangerousModePermissionPrompt` and
`skipAutoPermissionPrompt`; and take every other key as written (34,
144, 162, 172, 195, 197; core.12, core.30).

#### Scenario: Each reserved key

- **WHEN** the profile's reserved keys are read
- **THEN** they hold each of those keys, `permissions.defaultMode` by its path, each with its owner, the account, the model, the effort, the mode, the account's sign-in, flume's plugin or a person (195, 197)

#### Scenario: A people's settings file

- **WHEN** a host's named file gives `tui = "fullscreen"`, `statusLine`, `hooks`, `env`, `permissions = { allow = ["Bash(git log:*)"], defaultMode = "bypassPermissions" }` and `skipDangerousModePermissionPrompt = true`
- **THEN** the command's file holds `tui`, `statusLine`, `hooks` and `permissions = { allow = ["Bash(git log:*)"] }`, and no `env`, `defaultMode` or `skipDangerousModePermissionPrompt` (S100, 195)

#### Scenario: Keys taken as written

- **WHEN** the profile is asked to take `statusLine`, `tui`, `hooks`, `permissions = { allow = ["Bash(git log:*)"] }` and `enableAllProjectMcpServers = true`
- **THEN** it takes them, and its command's file holds each as written (195)

### Requirement: A repository's tagline reaches Claude Code's settings file

Claude Code's `attribution`, its commit and pull request text and its
session link, SHALL be an own setting like any other: given by a
repository's layers for every kind, taken whole, and written into the
`--settings` file of each agent of that repository as the layer wrote
it, with nothing of flume's added to it or read in it; a repository no
layer gives it for SHALL get no `attribution` from flume (195, C.4).

#### Scenario: A fork that carries no tagline

- **WHEN** the host's `[repos."<fork>".agents.programs.claude.settings]` gives `attribution = { commit = "", pr = "", sessionUrl = false }`, and a fix and a chore run in the fork
- **THEN** the `--settings` file of every stage of each holds `attribution` as `{"commit":"","pr":"","sessionUrl":false}`, the stand-in read it, and status names `programs.claude.settings.attribution` from `host-repository` (S102, 195)

#### Scenario: A repository's own words from its file

- **WHEN** the repository's `.config/flume.toml` gives `[agents.programs.claude.settings] attribution = { commit = "Assisted-by: Claude Code", pr = "" }` and no host table gives `attribution`
- **THEN** a fix's `--settings` file holds that `attribution` as written, and status names it from `repository` (S102, 195, C.2)

#### Scenario: A repository nothing names

- **WHEN** no layer gives `attribution` and no other own setting for a fix's repository
- **THEN** the fix's agent starts with no `--settings`, and the stand-in read no `attribution` but its account's (S102, 195)

### Requirement: Own settings pinned to the release the tree holds

A check-tools test SHALL find in the pinned Claude Code, byte for byte,
the `--settings <file-or-json>` option and its words, its sources'
order with `flagSettings` after the user's, the project's and the local
settings, the merge that joins a source's lists with those beneath it,
each reserved key as its settings schema names it, and the
`attribution` schema's commit and pull request texts, each saying an
empty string hides attribution, naming any it does not find (195, C.4;
core.22).

#### Scenario: The pinned release read

- **WHEN** the test reads the program the `claude` wrapper from devenv.nix execs
- **THEN** every fragment of the profile's settings stands in it (C.4)

### Requirement: Claude Code reads a session live from its records

The Claude Code profile SHALL answer live runs, naming each process,
where a `sessions/<pid>.json` under a home it is given, each account's
`CLAUDE_CONFIG_DIR` or the login environment's `~/.claude`, names the
session its start's started ack names, and a process of the host runs
with its `pid` started at its `procStart`. A record whose process is
gone or started otherwise SHALL count nothing (207, C.4).

#### Scenario: A session record and its process

- **WHEN** a scratch home's `sessions/<pid>.json` names the session with the `pid` and `procStart` of a process the test runs
- **THEN** live answers runs with that process; once the process ends, it answers none, the record still on disk (207, C.4)

#### Scenario: A reused process id

- **WHEN** the record names a `pid` the host runs, started at another time than its `procStart`
- **THEN** live answers none (207)

### Requirement: Claude Code reads a session live from the host's processes

The Claude Code profile SHALL answer live runs, naming each process,
where a process of the host whose program is `claude` holds the session
after `--resume` or `-r` in its arguments, under whatever home (207,
C.4).

#### Scenario: A resume under a home not given

- **WHEN** a process `claude --resume <session>` runs under a home the profile is not given
- **THEN** live answers runs with that process (207, S117)

### Requirement: Claude Code's live says what it cannot read

The Claude Code profile SHALL answer live unknown where a home's
`sessions` folder exists and cannot be read, naming `CLAUDE_CONFIG_DIR`
and no path, or where the host's processes cannot be listed, naming
them (207; core.30).

#### Scenario: A sessions folder that cannot be read

- **WHEN** a home's `sessions` folder exists and cannot be read
- **THEN** live answers unknown, naming `CLAUDE_CONFIG_DIR` and no path (207; core.30)

### Requirement: Claude Code's live pinned to the release the tree holds

A `check-tools` test SHALL read the pinned Claude Code and require the
source its live rests on to stand: the session record's fields
`sessionId`, `pid` and `procStart`, its `sessions` folder under the
configuration home, and `--resume`'s short form `-r` (core.22, C.4).

#### Scenario: The pinned release's session record read

- **WHEN** the test reads the program devenv.nix pins
- **THEN** each fragment live rests on is found in it, and the test names any that is not (C.4)

### Requirement: Claude Code tells its agent what a found is

Claude Code's way to stop short SHALL be the same for every start, and
SHALL say to run `flume needs "<words>"` when the agent cannot go on
without an answer, saying what it needs, and `flume needs --found
"<words>"` when it found work a stage before its own must do again,
saying what it found, and then to stop (28, 41, 131; core.31).

#### Scenario: The words a fix's code is told

- **WHEN** the profile's way to stop short is asked for the agent of a fix's code stage and for a change's verify stage
- **THEN** each is the same text, which names `flume needs --found` for work a stage before this one must do again, and holds neither "cannot deliver past" nor anything of either agent's identity (28, 41; core.31)

### Requirement: The settings' effort passed by --effort

The agent profile SHALL start `claude` with `--effort` and the
settings' effort where they give one, and with no `--effort` where they
give none (197, C.4).

#### Scenario: The command for a stage with no effort

- **WHEN** the profile gives the command for a stage whose settings give no effort
- **THEN** it runs `claude` with no `--effort` (197)

#### Scenario: The command for a stage at xhigh

- **WHEN** the profile gives the command for a stage whose settings give the model `opus` and the effort `xhigh`
- **THEN** it runs `claude` with `--model opus --effort xhigh` (S103, C.4)

### Requirement: Claude Code takes the pinned release's five efforts

The Claude Code profile SHALL take `low`, `medium`, `high`, `xhigh` and
`max`, the pinned 2.1.296's `--effort` words, and SHALL refuse any
other, naming it, `claude` and the five; and SHALL name
`CLAUDE_CODE_EFFORT_LEVEL` as the variable its program reads over
`--effort` (197, C.4).

#### Scenario: Each of the five, and two the release reads otherwise

- **WHEN** the profile is asked of `low`, `medium`, `high`, `xhigh`, `max`, `med` and `ultracode`
- **THEN** it takes the first five, and refuses `med` and `ultracode`, each naming it, `claude` and the five (197, C.4)

### Requirement: Claude Code's effortLevel is flume's effort's

The Claude Code profile SHALL reserve its settings' `effortLevel` with
the owner flume's `effort`, beside `model`: a layer's `settings` that
writes it SHALL be refused naming the key and its owner, and a named
file's SHALL be left out and named in status; `modelSettings` and
`maxEffortLevel` SHALL NOT be reserved (195, 197, C.4).

#### Scenario: effortLevel written in a table

- **WHEN** the host's `[agents.programs.claude.settings]` gives `effortLevel = "xhigh"`
- **THEN** every launch and start of an agent on Claude Code is refused naming `effortLevel`, `host` and flume's `effort` (195, 197)

#### Scenario: effortLevel in a named file

- **WHEN** the host's `[agents.programs.claude]` names a file giving `effortLevel`, `maxEffortLevel` and `tui`
- **THEN** the agent's `--settings` file holds `maxEffortLevel` and `tui`, and status names `effortLevel` left out with the file, `host` and flume's `effort` (195)

### Requirement: The efforts pinned to the release the tree holds

A `check-tools` test SHALL read the pinned Claude Code and require the
source the profile's efforts rest on to stand byte for byte: the
`--effort` option with its words, the release's list of the five, the
variable read over it, and `effortLevel`'s declaration in the settings
schema (core.22, C.4).

#### Scenario: The pinned release's efforts read

- **WHEN** the test reads the program devenv.nix pins, the file its `claude` wrapper runs
- **THEN** each fragment the efforts rest on is found in it, and the test names any that is not (197, C.4)
