# A stream runs its agents in Rex

## Why

The operator asked for Rex as flume's second terminal beside herdr
("I'm ready to land the Rex research and get a bolt queued for that
research/rex-is-flumes-second-terminal", 2026-11-12, signal
2026-11-12-agentplot-planner-c2cd5624/01-rex-bolt-queued). The research
read Rex 0.1.0-preview.14 on deneb and settled with the operator how
flume drives it (boomdrivedev/blueprints
`design/research/2026-11-11-rex-as-flumes-second-terminal.md`), and the
statement now holds C.5's Rex rows and C.6's `REX_*` among the variables
sluice's profile strips (on blueprints main since 79f7b9e). S91's
uncarried example moved to `cmux`.

flume carries no Rex profile: a stream naming `multiplexer = "rex"` is
refused as 190 refuses a multiplexer flume carries no profile for. This
change builds the profile, so a stream that names Rex has its agents
started, sent prompts, read and closed in a Rex server of flume's own,
as a stream on herdr has, with no process of flume's running between
commands (120; core.13) and no variable of an agent's in any process's
arguments.

## What Changes

- **A Rex terminal profile (C.5, 94 to 98, 132, 136 to 138, 201).**
  `crates/flume/src/profiles/terminal/rex`, named `rex` by `[host]
  multiplexer` and a stream's `multiplexer`, its records under
  `<state>/terminal/rex`. Each of the host's terminal sessions under Rex
  is a Rex server of flume's own at `<state>/terminal/rex/<session>.sock`,
  started by `rex server start --restore=false` where none answers, and
  never the server the Rex app runs for its user. Each base is a Rex
  session labelled `<repository> <base>`, each place a window
  `<kind>--<name>`, each agent a block labelled its Rex name.
- **open** makes the block with no shell, its command a start script of
  the start's own, mode 0700, that removes itself and execs `/usr/bin/env
  -i` with Rex's terminal variables, the login environment and the
  agent's environment, so no variable stands in the block's arguments;
  claims the identity in the host's state before it makes the block (98);
  reads Rex's ready from the program's first Program Status report, a
  second signal and never the start (35); answers the folder-trust
  question from the screen (34); and checks the program in the block is
  the one the settings name (32).
- **send** clears held input with ctrl+c only while the agent waits,
  types the text in pieces Rex takes and presses Enter (95). **state**
  reads the program's own report as C.5's state row reads it (96).
  **screen**, **close** and **tidy** as C.5 says (97, 136 to 138).
- **Sessions by name.** `rex:<block>@<socket>` names a recipient's
  session and the calling command's, from `REX_BLOCK` and `REX_SERVER`
  (7, 43); a role run's address reaches the block its live start holds,
  a standing agent the block its server lists under its name (132); a
  name in another terminal's form names no session of Rex's.
- **runs** asks each part's server at once, bounded at a second, and
  starts nothing (201).
- **attach** refuses both views and **show** shows nothing: telling
  Rex's app to show a session and labelling a place's window are the
  bolt's next unit, `a-person-sees-a-rex-place-in-the-app`.
- **Rex pinned** from Rex's flake in the devenv shell, every command and
  field the profile reads checked against it (core.22).
- **sluice strips `REX_*`** with the other multiplexers' variables
  (C.6; core.6).
- **The stand-in for Claude Code** reports its state to Rex by OSC 7501
  in a Rex block, so the walkthroughs run under Rex.
- **An uncarried multiplexer is `cmux`** in every scenario and test that
  held `rex` uncarried (190, S91).
- **Record and rules.** A record, proposed, binding Rex's calls as
  core.29 binds herdr's and naming `rex` among core.1's strings.
- **The site** names Rex among the multiplexers a stream may name.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `terminal-profiles`: the terminal through Rex, its servers, its
  start, its readings, its tidy, its sessions by name, its runs, its
  attach and show; the caller's session under Rex.
- `stream-terminals`: a stream naming Rex runs its places in Rex; the
  uncarried multiplexer is `cmux`.
- `launching`: the uncarried multiplexer is `cmux`.
- `the-list`: the uncarried multiplexer is `cmux`.
- `seam-contracts`: the resolver asked for `cmux` gives none.
- `merge-queue-profile`: a submission from a Rex block names no
  session.

## Impact

- `crates/flume/src/profiles/terminal/rex/` (new), `profiles.rs`
  (registration), `profiles/terminal/common.rs` (new: what herdr's and
  Rex's profiles read alike), `profiles/terminal/herdr/mod.rs` (calls
  it), `profiles/merge_queue/sluice/mod.rs` (`REX_`).
- `crates/flume/tests/profiles/tools/terminal_rex.rs` (new), `walk.rs`,
  `walkthroughs.rs`, `merge_queue_sluice.rs`,
  `stand-in/claude.rs`, `profiles/built_in.rs`, `commands.rs`.
- `crates/flume-core/src/list.rs`, `crates/flume/src/commands/print.rs`,
  `crates/flume-fakes/tests/fakes.rs`, `crates/flume-conformance/src/engine/{launching,list}.rs`:
  `cmux` for `rex`.
- `devenv.yaml`, `devenv.lock`, `devenv.nix`: Rex pinned.
- `docs/adr/`, `docs/architecture/core.md`: the record and core.1,
  core.29.
- `site/src/content/docs/`: configuration, roadmap and the guide to
  places and multiplexers.
- A host that names `rex` for a stream, refused before, runs it.
