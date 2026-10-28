# A kind runs through

## Why

The core landed over fakes (`the-core`): nothing a person can run.
crew is taking its conductors out (records 0010, 0011; afterthought/crew
records 0045, 0047), so flume's first runnable build must carry a
unit's whole course on one host: launch a fix, a chore or a change, run
it from its first stage to its landing with nobody speaking between the
stages, send it back on a failed submission or a finding, keep the
host's bound on agents by its callers' priorities, and tidy the place
when the work is over. Since the core was built, the statement moved
under it: a submission is read at the settles that follow it and never
waited on (111, 124, 133; record 0012), a kind runs through on its own
(131; record 0011), the host keeps one queue (70; record 0010), a
stop-short needs an answer or found work, the one place profile is
plain `git worktree` (C.1, C.4; record 0014), and a launch names its
recipients, a conflict's fix merges the base in, and the agent seam
reads whether a session ended (7, 8, 112, 134; record 0015). This change brings the
core to the statement and builds every profile and surface a kind needs
to run through.

## What Changes

- **The gate brought to sluice's setup.** The commit hooks find their
  config in every worktree, as sluice's do, so no commit needs
  `PREK_ALLOW_NO_CONFIG`; clippy `-D warnings` joins them; treefmt fails
  on a file no formatter claims, every exclude carrying its reason; a
  root moon project runs treefmt over the tree, the toolchain check,
  the records' numbering and `openspec validate` of the specs and the
  open changes; the tools the profiles drive (Claude Code, OpenSpec,
  herdr, sluice) are pinned in `devenv.nix`; the gate gains a
  `check-tools` layer, a sandboxed executor and CI on Linux and macOS,
  as sluice's has (core.21, core.22).
- **The seams brought to the statement.** The merge queue's submit
  holds no wait; `inbox` and `step` join it, and `wait` goes (117, 125,
  133). A stop-short says whether it needs an answer or found work
  (41, 92). The agent seam's readings are handed the place's own
  folder, where the agent profile keeps its marks (128). The host's
  bound and a lock over the room join the host's facts. flume's own
  declaration holds three kinds (22).
- **A kind runs through on its own.** Delivery starts the next stage,
  the last stage's delivery submits, every settle collects the place's
  pending outcomes and drives the queue one round where nothing else
  does, a failed or conflicting submission and found work from the last
  stage start the kind's send-back stage with the words, counted
  together toward `max_send_backs`, needing an answer waits on a retry
  whose prompt carries the question with the answer, and flume's tidy
  removes every landed place (40, 41, 42, 49, 111 to 113, 124, 131,
  133).
- **The host's queue.** A stage starts only while the host's agents
  running are under `[host] agents`; otherwise it waits for room and
  starts by priority, given at the launch and changed after; status
  shows the agents running, the bound, the room and the queue (70).
- **The profiles.** The kinds through weir's `weir-kinds` crate,
  flume's own kinds and their prompts built in; the change through
  OpenSpec; the place through plain `git worktree`, readied and torn
  down as the merge queue's profile declares; the store in the host's
  state directory; git's history and the host's facts read for real;
  the agent through Claude Code with flume's own plugin; the terminal
  through herdr and the printed command; the merge queue through sluice
  (C.1 to C.7).
- **The surfaces.** The command line: `launch`, `status`, `prompt`,
  `tell`, `stop`, `retry`, `finish`, `drop`, `done`, `tidy`, `settle`,
  `needs` and `priority`, each answering as JSON on request. flume's
  Claude Code plugin: the asking mark, the stop check and send-back,
  the guard, the on-quiet settle, `/launch` from any session and
  `status` over MCP (section 9).

Not here: holds put after the launch, a stalled place, delivering an
end to its recipient, `flume inbox`, a role's address and the herdr
plugin's watch and sidebar, which are the next change's; a stage's made
things (129; record 0007, proposed).

## Capabilities

### New Capabilities

- `the-gate`: the commit hooks, treefmt, the root project, the pinned
  tools, the layers and CI, aligned with sluice's.
- `running-through`: a kind's course from launch to landing with no
  caller's word, the fix round and its bound, and the two stop-shorts.
- `host-queue`: the host's bound, waiting for room, priority, and the
  queue in status.
- `kinds-profile`: B.2 through `weir-kinds`, the binding, and flume's
  own three kinds with their prompts.
- `change-profile`: B.3 through OpenSpec's files.
- `place-profile`: B.1 through plain `git worktree`.
- `store-profile`: B.7 in the host's state directory, and the host's
  configuration.
- `history-and-host`: git's history and the host's facts, read for
  real.
- `agent-profile`: B.4 through Claude Code.
- `terminal-profiles`: B.5 through herdr and the printed command.
- `merge-queue-profile`: B.6 through sluice.
- `command-line`: flume's commands, each answering from the one core.
- `claude-code-plugin`: flume's plugin, loaded into every agent and
  installable in any session.

### Modified Capabilities

- `seam-contracts`: the merge queue's operations and the types the
  seams pass, the stop-short's two kinds among them.
- `settling`: the submission made with no wait, its outcome collected
  at the settles that follow, and the send-back to the kind's fix
  round.
- `stage-runs`: a stop-short that needs an answer or found work.
- `caller-acts`: a retry carrying the question with its answer.
- `launching`: a first stage waiting for room, the launch's priority,
  and a binding that names a place profile flume does not have.
- `place-status`: the stop-short's kind, the move a place waits on,
  and the host's queue.
- `clause-coverage`: the clause list read at the statement's revision
  e5f92f6.

## Impact

- `crates/flume-seams`, `crates/flume-core`, `crates/flume-fakes` and
  `crates/flume-conformance` change where the statement moved; their
  JSON answers only gain fields (61).
- `crates/flume` gains its profiles under `src/profiles/`, its commands
  under `src/commands/`, its readers of git and the host, and its
  dependencies: `weir-kinds` from agentplot/weir at a pinned revision,
  `toml`, `clap`.
- New `plugins/claude/`, a moon project of its own, and a marketplace
  at `.claude-plugin/` so a session can install it.
- The gate: `devenv.nix`, `treefmt.nix`, `moon.yml` at the root,
  `.moon/workspace.yml`, `.config/sluice.toml`,
  `.github/workflows/ci.yml`, `check-toolchain.sh`, `check-adrs.sh`,
  `CLAUDE.md`.
- External: the sluice profile calls `sluice inbox --json` (sluice
  clause 51; sluice record 0070), which sluice's main does not hold yet
  (see design, Risks).

## Touches

```
devenv.nix
devenv.lock
treefmt.nix
moon.yml
check-toolchain.sh
check-adrs.sh
.moon/workspace.yml
.config/sluice.toml
.config/flume.toml
.claude-plugin/marketplace.json
.github/workflows/ci.yml
.gitignore
Cargo.toml
Cargo.lock
CLAUDE.md
README.md
docs/architecture/_open.md
crates/flume-seams/**
crates/flume-core/**
crates/flume-fakes/**
crates/flume-conformance/**
crates/flume/**
plugins/claude/**
```
