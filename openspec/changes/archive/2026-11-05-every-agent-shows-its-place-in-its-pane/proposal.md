# Every agent shows its place in its pane

## Why

On deneb on 2026-10-31, under flume 0.4.1, the user watched agents
flume had started in herdr panes. From a pane they could not tell which
place it was, its kind or its stage, or whether it was working, waiting
on them or stalled. They could not open the herdr popup of places at
all: it has no key, and the only way in was `herdr plugin action invoke
places --plugin agentplot.flume`. They said "I don't see any of these
statuses".

The user approved the work on 2026-11-01 (Pending You,
req_49c620d8b1ba426bb81a): "Build a Claude Code status line in Flume's
plugin (place, kind, stage, state in every agent's pane) and fix the
herdr popup".

Two readings of the pinned tools shape the change. Claude Code 2.1.296
gives a plugin's hooks module a status line of its own,
`$.ui.status`, pinned under the prompt beside the engine's own notices,
as sluice's board plugin draws its own. The user chose that way on
2026-11-05 in the Plannotator Inbox: "As sluice-board does: flume's
plugin pins its line with `$.ui.status` from a hooks module". herdr 0.9.5's
manifest gives an action an id, a title, a description, contexts, a
command and platforms, and no key. A key is only ever the host's own
`[[keys.command]]` entry in herdr's `config.toml`.

## What Changes

- **A place's line.** The core gives every place one line. It holds the
  place as `<kind>--<name>`, the kind and its version, the current
  stage with its group where it repeats (`code 3/9`), how many times
  that stage started, the change's tasks ticked, and the place's state:
  `working`, `asking`, `waiting on an answer`, `held`, `stalled`,
  `submitted`, `landed` and the rest of design 2's words. The core
  reads it from the place's held state, the change at the place's head
  and the agent's own record, and never from the terminal or the merge
  queue, so it is cheap enough to ask at every render (56, 57, 58, 139,
  142).
- **`flume status --line`.** The command line gives each place's line,
  reading only what the line needs and settling nothing. Status's JSON
  form gains each place's `line`, so the MCP status, the sidebar and
  the popup read the same words (57, 60, 61; S14).
- **The status line.** flume's Claude Code plugin gains a hooks module
  that, in a session flume started, runs `<flume> hook status-line` at
  the session's start, after each tool call, at each turn's end and
  every ten seconds, and pins the line it prints with `$.ui.status`.
  The session's own status line and other plugins' lines stay beside
  it. Outside an agent flume started it pins nothing and runs nothing.
  The hook writes nothing and settles nothing, and the module's clock
  ends with its session (57, 60, 72, C.4; core.13).
- **The popup and the sidebar show the line.** The popup shows each
  place's line, the status line's words. The sidebar row on an agent's
  entry is the line without the place and its kind, which the entry
  already shows. The plugin's own row function goes, so the plugin
  computes nothing (60; core.14). The pane's title stays the stage and
  its agent's state, as S49 has it (139).
- **The popup's key.** The plugin's manifest declares no key, since
  herdr 0.9.5's has none to declare. The README and the site name the
  one `[[keys.command]]` entry herdr's configuration needs, apart from
  the sidebar's rows. The watch reads herdr's configuration at its start
  and at each settle, and where no entry binds the popup, or no sidebar
  row names `$flume`, it records what is missing beside its lease.
  `flume status` then names the entry under the host's watch, so flume
  says it when asked and tells no person by itself (72, 122).
- **Docs.** The README, the herdr plugin's README, and the site's
  herdr, Claude Code, commands and state pages, in the site's neutral
  voice (core.35).

## Capabilities

### New Capabilities

- `place-line`: a place's one line, its parts and its state's words, read
  from held state, the change at the head and the agent's own record
  alone, and given by `flume status --line` and in status.

### Modified Capabilities

- `place-status`: status gives each place's line, and, for the host,
  what its terminal's configuration lacks for flume's plugin.
- `command-line`: `flume status --line`.
- `claude-code-plugin`: the plugin's module pins the place's line under
  the prompt, `flume hook status-line` prints it, nothing is shown
  outside an agent flume started, and the engine the module rests on is
  pinned to the release the tree holds.
- `herdr-plugin`: the sidebar row and the popup show the line, the watch
  records what herdr's configuration lacks, and the popup's key is the
  host's entry, checked against the pinned herdr.
- `terminal-profiles`: the herdr profile gives as the terminal's setup
  what the plugin's watch recorded as missing.
- `seam-contracts`: the terminal seam gives what its configuration lacks
  for flume's plugin.

## Impact

- agentplot/blueprints: the statement has no clause for an agent's own
  status line, for a place's line, or for a watch that names what the
  terminal's configuration lacks. Design 9 drafts each for the user's
  ruling: a status-line part under B.4 and its row in C.4, the plugin's
  module pinning the line;
  the line in 56, beside 139's clues; and 122's watch and C.5's row
  naming the missing setup. The code stands ahead of them until they
  land, and the drafts are asked of the user on one Pending You card
  (core.19).
- `crates/flume-seams`: the terminal seam gains `setup`, none by
  default.
- `crates/flume-core`: a new `line.rs` makes the line and answers
  `Engine::line` and `Engine::lines`; `status.rs` gives it in
  `PlaceStatus`, and the host's `terminal_setup`; `answer.rs` gains
  `PlaceLine` and `Answer::Lines`, only added.
- `crates/flume`: `flume status --line`; the person's form prints the
  line and the watch's setup; the Claude Code profile's `status-line`
  hook, in a new `claude/status_line.rs`, and the plugin's module
  written with the binary's path as its hooks are; the herdr profile
  reads the watch's setup file; walkthroughs and one live walkthrough.
- `crates/flume-fakes`, `crates/flume-conformance`: the fake terminal's
  watch with setup notes, the engine scenarios of the line, and the
  answer's field list.
- `plugins/claude`: the module `hooks/line.ts`, the engine types the
  pinned Claude Code writes, its tests run by the pinned engine, and the
  hooks' tests of `hook status-line`; `plugin-claude` gains its
  `engine-types` and `typecheck` tasks and TypeScript as its language,
  as sluice's board has, and `devenv.nix` the TypeScript it runs.
- `plugins/herdr`: the manifest's description, the watch's setup file,
  the popup and the sidebar token from `line`, `rows_jq` removed, and
  the README.
- `site/`: the guides and references.
- No new crate, plugin or site part, so no new moon project, and no new
  crate dependency; `devenv.nix` pins the TypeScript `plugin-claude`
  checks its module with (core.22).
