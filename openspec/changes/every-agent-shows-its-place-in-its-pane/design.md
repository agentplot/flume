# Design

## Context

flume's tree at 201086c shows a place in three ways. The settle's
`show` gives the terminal a place's clues: the current stage, labelled
with its group where it repeats (`label`, `code 3/7`), and that stage's
state, its agent's as the terminal reads it while one is live
(`crates/flume-core/src/clues.rs`). The herdr profile reports them as
the pane's title `<stage> · <state>` and its name `<kind>--<name>`
(139, S49). The herdr plugin's watch writes `flume status --json`, and
its sidebar token and popup rows are made in shell by `rows_jq` in
`plugins/herdr/lib/common.sh`: the kind, `clues.stage`, `clues.state`,
the tasks ticked, a submission word, and a waiting word. Status itself
(`Engine::status`, `crates/flume-core/src/status.rs`) settles nothing,
but it reads every fact where it lives: the change at the head through
the change seam, the submission from the merge queue, every agent's
state through the terminal, and the host's queue, pools and bases. That
is too much to ask at every render of a pane.

Claude Code's plugin is written by the Claude Code profile once per
version of flume under `$XDG_DATA_HOME/flume/plugins/claude/` from
`plugins/claude` (`PLUGIN` in `crates/flume/src/profiles/agent/claude/mod.rs`),
its hooks naming the binary by its absolute path, and loaded by
`--plugin-dir`. Two facts of the pinned 2.1.296 decide how a status
line reaches the agent:

- a plugin's `settings.json` is read through a schema of two keys,
  `["agent","subagentStatusLine"]`, so a plugin cannot set a session's
  `statusLine`;
- `--settings <file-or-json>` loads settings as the `flagSettings`
  source, which the release orders after `userSettings`,
  `projectSettings` and `localSettings` and before `policySettings`
  (`return["userSettings","projectSettings","localSettings","flagSettings","policySettings"]`).
  Its `statusLine` is `{type: "command", command, padding?,
  refreshInterval?}`: the command is run at each update of the session,
  and again every `refreshInterval` seconds where one is given.

herdr 0.9.5's manifest action (`PluginManifestAction`) holds `id`,
`title`, `description`, `contexts`, `command` and `platforms`. A key
reaches a plugin's action only through the host's own `[[keys.command]]`
entry, `type = "plugin_action"`, in herdr's `config.toml`:
`HERDR_CONFIG_PATH`, else `$XDG_CONFIG_HOME/herdr/config.toml`, else
`~/.config/herdr/config.toml`. `herdr config check` parses an entry's
`type` and passes `config: ok`. The plugin's README already shows the
entry beside the sidebar's rows, under "Setting it up". A host whose
configuration is generated, as the user's is, lost it there.

The rules this design relies on are core.1 and core.2 (only the Claude
Code profile names `claude`, its settings or its status line; only the
herdr profile and the plugin name herdr), core.13 (nothing the status
line starts outlives its run), core.14 (one core makes the line, every
surface copies it, and the JSON form only gains fields), core.15 (the
line's tasks are read at the head when asked and kept by nothing),
core.16 (nothing is written by a status line or by `--line`), core.22
and core.39 (the profile's reading of Claude Code's settings is flume's
code, pinned to the release), core.29 (the herdr entry is checked
against the pinned herdr), core.30 (`--settings` carries no setting of
the six layers), core.35 (the site's and the READMEs' voice), and
clause 72 with core.24 and core.25 (flume tells no person anything by
itself: what herdr's configuration lacks is said in status when asked,
never pushed). The one rules page is `docs/architecture/core.md`; no
area page exists for the paths this change touches. Where flume has no
rule, the kit standard's code profile binds (kit:specs.3,
kit:testing.5).

## Goals / Non-Goals

**Goals:**

- Every agent flume starts under Claude Code shows in its own pane its
  place, its kind and version, its stage with the stage's run and the
  change's progress, and the place's state, the same words the CLI, the
  MCP status, the sidebar and the popup give (56, 60; S14).
- The read behind it is cheap enough for every render: held state, the
  change at one head, and the agent's own record, with no call to the
  terminal or the merge queue (57, 58).
- A status line the user or the repository set is kept, beneath
  flume's, with no step of theirs.
- A host with no key for the popup is told the entry it needs, by the
  READMEs, the site and `flume status`, while the plugin changes none of
  herdr's configuration and tells no person by itself (72, 122).

**Non-Goals:**

- The pane's title. It stays the stage and its agent's state as the
  terminal reads it, which S49 fixes (139).
- A status line for any program but Claude Code. The common profile and
  the other programs are record 0026's, not built. A program with no
  status line shows none, which design 9 drafts as a degraded part.
- A key declared by the plugin, which herdr 0.9.5 cannot take, or any
  write to herdr's configuration.
- A switch to turn flume's status line off. Nothing asks for one; the
  session's own status line keeps its place beneath flume's.

## Decisions

### 1. A place's line, made by the core

`flume-core` gains `PlaceLine` in `answer.rs`:

| field | what it holds |
|---|---|
| `name` | `<kind>--<name>` |
| `kind` | `<kind>@<version>` |
| `stage` | the current stage as `label` gives it, `code 3/9` for a repeating stage's group (142): the stage the place most recently started, else the first it reached with no start, held, waiting on its readiness or refused; none where the place is not live or reached none |
| `run` | how many times the current stage started (48); none where it has not started |
| `tasks` | `<ticked>/<all>` of the change's tasks at the head; none where the place has no change, or it holds no tasks |
| `state` | design 2's word |
| `full` | `name · kind · stage · run <n> · tasks <t>/<n> · state`, a part with none left out |
| `short` | `full` without `name` and `kind` |

`full` is the line the status line and the popup show; `short` is the
sidebar's row, beside the entry's own `<kind>--<name>`.

A new `crates/flume-core/src/line.rs` makes it. `Engine::place_line(held,
change, asking, identity)` takes the held state, the change's facts
where the place has a change, whether the live agent's profile reads it
asking, and the asking session's identity where one asks. Status calls
it with what it has read already. `Engine::line(place, identity)`
reads only what the line needs. It reads the place's held state through
the store seam (the state-directory profile). Where the place's kind
has a change and the place is live, it reads the place's head through
the history seam and the change at that head through the change seam, the OpenSpec profile
where the repository binds it, as status reads it, once (58). It asks
the live agent's profile for its asking mark through the agent seam's
`asking`, which the Claude Code profile reads from the agent's own
record. It calls no terminal operation, no merge-queue operation and no
host reading. It writes nothing and starts nothing (57). A change that
cannot be read leaves `tasks` and the group out, never failing the
line.

Where `identity` is given and is not the current stage's live start,
`full` ends `· this session is not its agent`, so a session that
outlives its start never shows a later start's stage as its own (35,
140). `short` never carries it.

### 2. The state's words

The first row that holds gives the word. Each comes from held state or
the agent's own record, never the terminal or the merge queue.

| the place | word |
|---|---|
| being made by a launch | `being made` |
| its work over, landed, dropped or done (63), whether removed or tidied | `landed`, `dropped`, `done` |
| waiting on its caller, stalled (130) | `stalled` |
| waiting on its caller: a hold (47) | `held` |
| a stop-short that needs an answer, or found work outside the last stage (42, 131) | `waiting on an answer` |
| a dialog of its agent's program before its prompt (34) | `asking` |
| a lost agent (45) | `lost` |
| a prompt never begun (38) | `never begun` |
| its readiness failed (12) | `not ready` |
| a refused stage (27) | `refused` |
| its agent stopped by its caller (51) | `stopped` |
| past its kind's bound (112) | `past its bound` |
| its submission withdrawn or refused (112) | `submission refused` |
| its last stage delivered on a kind that does not land (114) | `waiting on done` |
| its stage waiting for room (70) | `waiting for room` |
| its live agent not yet started (35) | `starting` |
| its live agent's profile reads it asking a person (90) | `asking` |
| its stage stuck past its limit, its agent live and quiet (118) | `stuck` |
| its live agent in a conversation stage | `in conversation` |
| its live agent otherwise | `working` |
| submitted by flume at least once, the submission not due, no stage running (48) | `submitted` |
| otherwise | the current stage's state as `word` spells it, `delivered` or `repeating` |

`submitted` and `landed` are flume's own record of its acts: the count
of its submits (48) and the place's work being over (63), both in held
state. They are not the merge queue's record read where it lives, which
status's `submission` still gives (58). While the merge queue holds a
landing no settle has read yet, the line says `submitted`. The next
settle reads the landing and the place's phase moves. Design 9 names
this for the user's ruling.

The pane's title keeps the terminal's reading. An idle agent's title
can say `waiting` while its line says `working`: the title is the
stage's agent, the line is the place. A stage that has not delivered
does not sit idle, since its Stop hook sends it back (110).

### 3. `flume status --line`, and the line in status

`flume status [<place>] --line` asks `Engine::lines(place)`. That is
`Engine::line` for the place named, or for every place on the host,
with no identity. The answer is `Answer::Lines { lines: Vec<PlaceLine> }`.
The person's form prints each `full` on its own line, and `--json` gives
the answer's JSON (61). It settles nothing: the settle-first of an
attended host (119) is for commands that act on places, and status acts
on none.

`PlaceStatus` gains `line: PlaceLine` for every place status gives, made
by `place_line` from the facts status read. The JSON form gains it, so
the MCP status, the watch's status file, the sidebar and the popup read
it (61; core.14). The person's form prints a place's `full` under its
heading. The conformance suite's field list gains `line` and its
fields.

### 4. The status line reaches the agent by `--settings`

The Claude Code profile's `command` passes, after `--plugin-dir`,
`--settings` with one JSON string:

```json
{"statusLine":{"type":"command","command":"'<flume>' hook status-line","refreshInterval":10}}
```

`<flume>` is the binary's absolute path, as the written plugin's hooks
name it, quoted for the shell. It is a JSON string rather than a file,
so a start writes nothing more and the printed command carries it
whole. `refreshInterval` re-runs the line every ten seconds while the
session is idle, so a hold, a stall or a stop-short shows without a
message. Ten seconds is under the watch's bound of thirty and well over
the line's cost. The JSON holds no setting of the six layers (32;
core.30), and the program's own configuration still decides every
other setting. A `statusLine` in the host's managed settings comes after
the flag in the release's order and wins; flume's line is not shown
there.

The JSON, the hook of design 5 and the `fragments()` of design 6 live
in `claude/status_line.rs`, flume's own code (core.39).

### 5. `flume hook status-line`

`flume hook <event>` already dispatches to the agent profile
(`crate::profiles::hook`), so the commands name no program (core.1).
`status-line` is one more of its events, in `claude/status_line.rs`. It
reads 2.1.296's status line input on standard input, passes it on
unread, and exits 0 whatever happens:

1. Where `FLUME_STATUS_LINE` is set, it was run beneath flume's own
   line (step 3). It prints nothing.
2. In a session flume started, `FLUME_PLACE` and `FLUME_AGENT` set, it
   prints `Engine::line` for the place `FLUME_PLACE` names with the
   identity `FLUME_AGENT` names: `full`, one line. A place the core
   cannot read prints nothing of flume's. The words come from the core
   alone (core.14).
3. Then, in that session only, it runs the session's own status line.
   That is the `statusLine` of type `command` given by the last of the
   release's sources below the flag: `$CLAUDE_CONFIG_DIR/settings.json`,
   or `~/.claude/settings.json` where `CLAUDE_CONFIG_DIR` is unset; then
   `<place>/.claude/settings.json`; then
   `<place>/.claude/settings.local.json`. A `statusLine` whose command
   is flume's own `hook status-line` is passed over. It runs the command
   as the release runs a status line's command, with the same input on
   standard input, in the place, with `FLUME_STATUS_LINE=1`, under a
   bound of two seconds. It prints what it writes, beneath flume's
   line. A command that fails, writes nothing or passes the bound adds
   nothing, and one past the bound is killed with every process it
   started, so nothing outlives the hook (core.13).
4. Outside a session flume started it prints nothing and runs nothing.
   flume set no status line there, so this only happens when a person's
   own command calls it, and their own line stands.

It writes nothing under flume's state, the place or the account's
configuration, settles nothing and starts no agent (57; core.16). A
person's own status line that already calls `flume hook status-line`
gets nothing from that call, by step 1, so flume's line shows once.

### 6. The release's facts, pinned and stood in

`status_line::fragments()` lists what the hook and the command rest on,
each byte for byte as 2.1.296 holds it, and
`crates/flume/tests/profiles/tools/claude_words.rs` requires each to
stand in the pinned program (C.4; core.22):

- the plugin settings' two keys, `["agent","subagentStatusLine"]`, the
  reason the line is passed by `--settings`;
- `--settings <file-or-json>`'s option and its words;
- the `statusLine` schema with `refreshInterval`;
- the order of the settings' sources, flag over local over project over
  user;
- how the release runs a status line's command and hands it its input,
  as design 5 step 3 runs the session's own.

The stand-in for Claude Code (`crates/flume/tests/profiles/stand-in/claude.rs`)
reads `--settings`, as a JSON string or a file. It takes the session's
`statusLine` from the flag, else the local, project and user settings
in the release's order. It runs its command as the release does, with
a status line input in the release's shape on standard input: at the
session's start, after each act, and at the turn's end. It appends each
run's output, as one block, to the file `STAND_IN_STATUS_LINES` names.
Its doc comment names this beside its acts.

### 7. The popup and the sidebar show the line

The popup draws, for each place in the status the watch last wrote,
`▶ 1  <line.full>` with the place's location beneath. Those are the
status line's words, read from status and computed by nothing (60). The
watch's report gives each agent's pane the sidebar token
`flume=<line.short>`. `rows_jq` is removed from `lib/common.sh`, and so
are its uses in `watch.sh` and `places.sh`. The pane's title and name,
and the workspace's count line, are reported as before (139, S49).

### 8. The popup's key

The manifest declares no key, since herdr 0.9.5's action has no field
for one. The action gains a `description`: "flume's places in flight;
bind it with a [[keys.command]] entry of type plugin_action and command
agentplot.flume.places in herdr's config.toml". herdr lists it with the
action.

The READMEs and the site name the entry plainly, in a part of its own,
before the sidebar's rows:

```toml
[[keys.command]]
key = "prefix+f"
type = "plugin_action"
command = "agentplot.flume.places"
description = "flume: places in flight"
```

`prefix+f` is free among herdr 0.9.5's default keys and the keys
sluice's herdr plugin asks for, `prefix+m` and `prefix+shift+m`. The
entry carries a `description` as sluice's entries do, so the family's
entries read alike in herdr's configuration.

The watch, at its start and after each settle, reads herdr's
configuration file, `HERDR_CONFIG_PATH`, else
`$XDG_CONFIG_HOME/herdr/config.toml`, else `$HOME/.config/herdr/config.toml`,
from the server's own environment. It finds whether a line that is not a
comment gives `command = "agentplot.flume.places"`, and whether one
names `"$flume"`. It writes `setup.json` into its server's state
directory, a JSON array of what is missing, each in words a person acts
on and naming the file read. An absent file is read as holding neither.
The array is empty where nothing is missing. The words and the entry
are `lib/common.sh`'s, and the plugin's tests hold the README to them.
The reading is for display only: nothing of flume's decides on it, and
a reading that errs only says or omits a note, so it parses no TOML. It
calls no herdr command, and so holds no bound of its own (core.29).
The watch logs the same words to `watch.log` once per start.

The terminal seam gains `setup(&self) -> Vec<String>`: what the
terminal's own configuration lacks for flume's plugin there, in words a
person acts on, none by default. The herdr profile gives the words of
`setup.json` beside each fresh lease it reads for `watch`, in order and
once each. The printed command gives none, and the fake terminal scripts
them. `HostStatus` gains `terminal_setup: Vec<String>`, read from the
terminal by `host_status`. The person's form prints each under the
line that names the watch's bound, so `flume status`, asked of any
place or of every place, says the entry the popup needs (122). Nothing
is pushed to a person: no herdr notification, no notice (72).

### 9. For the user's ruling

The statement has no clause for what this change builds. Each draft
below is the user's to rule in agentplot/blueprints
`design/flume/requirements.md`, asked on one Pending You card, as the
user ruled on 2026-11-04 that an open decision is asked so and kept in
no file. Until one lands, the code stands ahead of the statement
(core.19).

- **A place's line, beside 56.** "For every place, status gives one
  line: its kind and name, its kind's version, its current stage with
  its group and how many times it started, its change's tasks ticked,
  and its state in one word. The state is read from the place's held
  state and its agent's own record alone, flume's count of its submits
  and the place's work being over among them (48, 63), and never from
  the terminal or the merge queue. The line may be asked alone, which
  reads only what it holds."
- **The agent's status line, under B.4 and in C.4.** A part of the
  agent seam: "status line: the program shows the place's line in the
  agent's own pane, asking the core whenever it draws the pane, above
  the status line its own configuration gives; or the part is degraded
  and shows none (171)." For Claude Code, the command row gains
  `--settings` with `statusLine` naming `flume hook status-line` and a
  refresh of ten seconds. A status-line row says what design 5 does,
  and the givens' Claude Code plugin names its status line.
- **What the watch says it lacks, in 122 and C.5.** "watch: ..., with
  what the terminal's own configuration lacks for flume's plugin there,
  which status gives when asked (72)." C.5's watch row names the popup's
  `[[keys.command]]` entry and the sidebar rows' `$flume`, read from
  herdr's configuration file by the plugin's watch.

### 10. Docs

`README.md` and `plugins/herdr/README.md` give the key's entry in a
part of its own, the line's words in the sidebar and the popup, and the
status line. On the site, `guides/places-and-herdr.mdx` and
`install.mdx` give the entry plainly, apart from the rows, and say
`flume status` names what herdr's configuration lacks.
`guides/claude-code.mdx` gives the status line: what it shows, that it
reads no terminal and settles nothing, that the session's own status
line stays beneath it, and that managed settings win over it.
`reference/commands.md` gives `flume status --line` and `flume hook
status-line`. `reference/state.md` gives status's `line` and
`host.terminal_setup`. All of it speaks to any host's operator (core.35).

## Risks / Trade-offs

- **A slow status line of the user's own** delays the render by up to
  the two-second bound. Past it, flume's line shows alone.
- **The words of a submission lag the merge queue** by up to a settle,
  since the line reads flume's own record (design 2). Status's
  `submission` gives the merge queue's at once.
- **A config reading by lines** can miss an entry written as an inline
  table, or read a key bound in a file herdr does not load. Then
  `flume status` gives a note the host does not need, or omits one,
  and decides nothing.
- **Managed settings** that set a status line hide flume's. The site
  says so.
