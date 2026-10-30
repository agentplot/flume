# An agent's mode is a layered setting

## Why

Nothing flume says decides an agent's reach in its place today. Every
Claude Code agent flume starts runs in whatever permission mode its
account's own configuration sets. A kind cannot ask for less, and a
host cannot run one kind with more care than another. A start whose
program came up in a mode nobody meant is still taken as the stage's
agent.

Record 0024 (accepted 2026-10-30) makes the mode a setting, layered as
the model is, its values flume's own four, which every agent profile
maps to its program's setting or refuses. The user ordered the mode
built first, before the crew refactor. The plan stage comes after it,
with points (record 0025) and the other agent profiles (record 0026)
(162, 169, C.2, C.4, C.7, S66, S67).

## What Changes

- **`mode` in the six layers.** Every agents table may give `mode`: a
  kind's and its stage's, the repository's for a kind it does not
  declare, and the host's `[kinds.<kind>.agents]` and
  `[repos."<repository>".kinds.<kind>.agents]`. So may the launch. The
  mode is taken whole from the first layer that gives it, a stage's own
  before one for every stage, and read at each start as the other
  settings are (32, 162).
- **flume's four modes.** `plan`, `ask`, `edits` and `full` mean the
  same on every program (169). A layer may give, in place of one, a
  table of programs, `mode = { claude = "auto" }`. The table is taken
  whole, and the agent's entry is its program's. A program the table
  names no entry for refuses the stage, and the mode never passes to a
  later layer. A program's own value given bare, `mode = "auto"`, is
  refused where it is read, naming it.
- **The command line.** `flume launch --agent [<stage>.]mode=<mode>` and
  `--agent [<stage>.]mode.<program>=<value>`.
- **Claude Code's map.** The profile passes `--permission-mode`: `plan`
  to `plan`, `ask` to `manual`, `edits` to `acceptEdits` and `full` to
  `bypassPermissions`. Its own values under `claude` are the pinned
  2.1.296's choices: `acceptEdits`, `auto`, `bypassPermissions`,
  `manual`, `dontAsk` and `plan`. A mode with no exact setting refuses,
  naming the mode and the program, and is never read wider or narrower.
  A `check-tools` test pins the choices to the release the tree holds
  (C.4; core.22).
- **None where no layer gives one.** With no mode in any layer flume
  passes no `--permission-mode`, and the account's own configuration
  decides, as today (162).
- **Refusals.** A launch is refused, naming the mode, the program, the
  stage and the layer, where a stage's agent resolves to a mode its
  profile cannot honour, or to a table with no entry for its program. So
  is a launch that gives `plan` to a stage that is not a conversation,
  naming the stage. A host's table changed after the launch leaves the
  stage refused at its start in the same words, until a retry (4, 32,
  169).
- **The start checked against its mode.** A start records the mode as
  its profile passes it. The SessionStart hook compares it with the
  `permission_mode` its input carries before it takes the started ack.
  A start that began in another mode, as Claude Code does when the
  account's configuration disables the mode asked, is not the stage's
  agent. flume ends it once quiet and starts the stage afresh. A second
  such start in a row is never begun, naming the mode asked and the mode
  it began in (35, 135, 161, 162; core.10).
- **Status.** The current agent's mode, with its layer, the table's
  entry it took and the value passed, and whether the mode was read
  back. A start that began in another mode names both modes (56).
- **Docs.** The site's agent-settings guide and its kinds guide, and
  the configuration and commands references, in the site's neutral
  voice (core.35).

## Capabilities

### New Capabilities

- `agent-modes`: flume's four modes and the table of programs, the mode
  in every layer, none passed where no layer gives one, each profile's
  map or refusal, `plan` kept to a stage that can take it, and the
  refusals at the launch and at a later start.

### Modified Capabilities

- `launching`: the six layers resolve the mode beside the program, the
  model and the instructions, and the host's two give the mode.
- `start-integrity`: a start's record holds its mode. The started ack
  is taken only where the mode matches. A start in another mode starts
  afresh, and a second in a row is never begun naming both modes.
- `agent-profile`: `--permission-mode` from Claude Code's map, its own
  values under `claude`, and the map pinned to the release.
- `claude-code-plugin`: SessionStart compares `permission_mode` at the
  start's first word, and `flume ack started` takes `--mode`.
- `place-status`: an agent's mode with its layer, entry, value passed
  and read-back; a start in another mode named with both modes.
- `command-line`: `--agent` gives `mode` and `mode.<program>`.
- `seam-contracts`: the agent seam's mode part and the types of a mode;
  `started_without` names the mode.
- `seam-fakes`: the fake agent scripts a map of modes, its read-back and
  a start begun in another mode.
- `store-profile`: the host's agents tables give `mode`.
- `kinds-profile`: a declaration's agents tables give `mode`.
- `clause-coverage`: the suite's list at the statement's current
  revision, 162, 169, S66 and S67 built, and the plan stage's, points'
  and record 0026's clauses named unbuilt.

## Impact

- agentplot/blueprints: nothing changes. The mode is stated at 9c7490c
  and a11f6d5 (4, 32, 88, 135, 161, 162, 169, C.2, C.4, C.7, S66, S67),
  and the suite reads the statement at main's revision. Nothing in weir
  changes either: weir-kinds returns an agents table as written, `mode`
  "the reader's to check" (weir C.4), so flume's pin stays (core.4).
- `crates/flume-seams`: `Mode`, `ModeSetting`, `Settings::mode`,
  `StartRecord::mode`, `ModeBegan` and `Unsettled::mode`; the agent
  seam's `mode` and `mode_read_back`; status fields, only added.
- `crates/flume-core`: `settings.rs` resolves and maps the mode,
  `launch.rs` refuses it, `start.rs` records it, and `status.rs` gives
  it.
- `crates/flume`: the kinds profile's and the host's agents tables;
  `--agent` and `flume ack started --mode`; the Claude Code profile's
  map, its command and its SessionStart comparison; the person's form
  of status; the stand-in's `--permission-mode` and `permission_mode`;
  the walkthroughs and one live walkthrough.
- `crates/flume-fakes`, `crates/flume-conformance`: the fake agent's
  modes, the engine scenarios citing 162, 169, S66 and S67, and the
  clause list.
- `plugins/claude`: the hooks' tests of the mode's comparison.
- `site/`: the guides and references.
- `docs/architecture/_open.md`: the item on records 0024 and 0025 keeps
  only the plan stage and points.
- No new crate, plugin or site part, so no new moon project; no new
  dependency.
