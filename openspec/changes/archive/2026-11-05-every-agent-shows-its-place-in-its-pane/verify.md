# Verify: every-agent-shows-its-place-in-its-pane

Checked on 2026-11-05 at
`change/every-agent-shows-its-place-in-its-pane` 1c2e247. The branch
last took main at 323158a (12e45cc), and main's head on `origin` is
still 323158a, so no merge was needed. The check covers this change's
proposal, design, seven specs and tasks.

**The work holds, and the change is archived.** Every task is ticked.
The gate passes. Every requirement and every scenario is held by a test
that reads what its scenario says. The archive syncs the seven specs and
leaves `openspec validate --specs --strict` passing 38 of 38.

## Tasks

All 21 tasks of the seven groups are ticked, `[x]`, and none is open.
The gate run below backs each group's closing task: 1.3, 2.2, 3.6, 4.1's
`flume:lint`, 5.5, 6.3 and 7.1. 7.1 also names `root:specs` and
`conformance:coverage`; the gate runs both.

- **4.1's live run** is the operator's, by hand, as `CLAUDE.md` says:
  `check-live` is required by no branch, and no test account's token is
  on this host for this stage. The walkthrough builds under
  `--all-features` in `flume:lint`, and reads the pinned Claude Code's
  pane under the prompt for `fix--one · fix@1 · code · run 1 ·` (fc80716).
  Until the operator runs `moon run flume:live`, the module's pinning in
  a real Claude Code is held by the pinned engine's `plugin test` and
  `plugin validate --strict`, not by a real session.
- **6.2's card** is req_bd53bfb96318472b835a, named by b90fe22. Design
  9's three drafts are not yet in agentplot/blueprints
  `design/flume/requirements.md` at its `origin/main` a465ba7: the code
  stands ahead of the statement until the user rules, as the task and
  core.19 allow.
- **3.5's mirror.** core.13's own line is unchanged; only its details
  gain the Claude Code plugin's clock. The site's rules page mirrors the
  rules' lines and leaves their details to the binding copy, so it
  stands as it is (7f12537).

## Checks

- **The gate.** `sluice exec main` at 1c2e247 ran the checks 1c2e247
  affects against main at 323158a. It passed 37 tasks of `#check-fast`
  and `#check-tools`, with `site:install` run for them:
  - `seams`, `core`, `fakes` and `conformance`: format, lint and test,
    and `conformance:coverage`;
  - `flume`: format, lint, test and `tools`, the walkthroughs over the
    stand-in and the herdr profile against the pinned herdr;
  - `plugin-claude`: format, validate, engine-types, typecheck and test;
  - `plugin-herdr`: manifest, lint, test and install;
  - `site`: prose and build;
  - `root`: format, toolchain, changes, specs, adrs, hooks, layers,
    marketplace and versions.

  That is every check task 7.1 names. sluice kept no record of the
  pass under its key, since this report stood in the worktree beyond
  1c2e247 when the run ended. The submission's own run records it.
- **The change.** `openspec validate every-agent-shows-its-place-in-its-pane
  --strict` passes.
- **The archive.** `openspec archive every-agent-shows-its-place-in-its-pane
  --yes`, over a copy of `openspec/`, applies 17 requirements added and
  one modified across the seven specs. `openspec validate --specs
  --strict` then passes 38 of 38.

## Requirements and scenarios

Each scenario below is held by the test named, which reads what the
scenario says.

### place-line

- **A place's line holds its parts.** `crates/flume-core/src/line.rs`'s
  `place_line` joins name, kind, stage with its group, `run`, `tasks`
  and the state by ` · `, a part with none left out, and `short` is the
  same less name and kind. Its three scenarios are
  `engine/place_line.rs`'s `a_change_at_its_first_group`
  (`change--parser · change@2 · code 1/2 · run 1 · tasks 0/3 · working`
  and its short form), `a_fix_at_its_second_start` (`run 2`, no tasks)
  and `a_landed_place` (`fix--parser · fix@1 · landed`, a tidied place's
  document kept for an end).
- **A place's state in one word.** `state_words` takes the first that
  holds in the spec's order: being made; landed, dropped, done; stalled;
  then each `Waits` (held, waiting on an answer, asking, lost, never
  begun, not ready, refused, stopped, past its bound, submission
  refused, waiting on done), matched exhaustively; waiting for room;
  starting; the profile's asking; stuck; in conversation; working;
  submitted; else the stage's word. Its five scenarios are
  `held_then_stalled`, `waiting_on_an_answer`, `asking_then_working`,
  `starting_and_waiting_for_room` and `submitted_then_landed`.
- **A line asked alone reads only what it holds.** `Engine::line` reads
  the store, the history seam's head and the change seam once for a live
  place whose kind reads a change, and the agent profile's `asking`.
  `nothing_called_and_nothing_written` reads the fake terminal's and
  merge queue's logs empty and the store's version unchanged;
  `a_change_that_cannot_be_read` gives `code` alone with no tasks.
- **A session that is not its stage's agent says so.**
  `a_repetitions_earlier_agent` reads `code 2/2` ending
  `· this session is not its agent`, its short form not.
- **The line alone and in status agree.** Status makes its line by the
  same `place_line` from the facts it read, and reads the change under
  the same conditions; `asked_two_ways` holds them equal for a fix and a
  change (S14).

### place-status

- **Status gives every place's line.** `PlaceStatus.line`, read with a
  default. `engine/place_status.rs`'s `a_line_in_status` reads every
  field in the JSON form and `full`; `a_line_field_dropped` fails naming
  the answer and the field, the suite's field list in `answers.rs`
  holding `line` and its fields; `print.rs`'s
  `a_places_heading_with_its_line` prints the full form under the
  place's heading.
- **Status gives what the host's terminal lacks.**
  `HostStatus.terminal_setup`, read from `Terminal::setup` by
  `host_status`. `a_popup_with_no_key` gives the fake's words and records
  nothing sent to a pane; `print.rs`'s `a_missing_key_under_the_watch`
  prints them under the watch's line.

### command-line

- **status --line gives each place's line.** `commands/mod.rs` asks
  `Engine::lines`, which settles nothing. `tests/commands.rs`'s
  `one_places_line_under_the_printed_command` reads
  `fix--one · fix@1 · code · run 1 · starting` alone, the same in
  `--json`'s `lines`, the held state's version and the stage's starts
  unchanged, and every file under the state as it was.

### claude-code-plugin

- **The plugin pins the place's line under the prompt.**
  `plugins/claude/hooks/line.ts`, named by `hooks/hooks.json`'s
  `modules`, registers on `session.start`, `tool.call` and
  `turn.complete` and asks one `$.clock.every(10_000)`.
  `tests/line.test.ts`, run by the pinned engine's `plugin test`, holds
  the four scenarios: the line pinned at the start, redrawn after ten
  seconds with no tool call or turn between, removed where the hook
  prints nothing, and, for each of three environments lacking
  `FLUME_PLACE` or `FLUME_AGENT`, nothing pinned, run or clocked. It
  also holds a failed run changing nothing, the first line alone, and
  the redraw after a tool call and a turn. The profile writes the module
  with the binary's absolute path (`written`'s `MODULE_FLUME`), and
  `tests/profiles/agent_claude.rs` reads it so.
- **hook status-line prints the place's line.**
  `profiles/agent/claude/status_line.rs`, dispatched first in `hook`,
  reads no standard input (`hook_reads_input`). `hooks_claude.rs`'s
  `the_status_line_of_a_code_agent` reads `starting`, then `working`,
  then the not-its-agent ending; `the_status_line_outside_an_agent_flume_started`
  prints nothing and exits 0; `plugins/claude/tests/hooks.sh` runs it
  in a code agent's environment against `status --line` and outside a
  session.
- **The status line writes nothing.** `the_status_line_writes_nothing`
  runs it ten times and reads every file's digest and the held state's
  version unchanged; `hooks.sh` does the same three times.
- **The engine pinned to the release.** `plugin-claude:engine-types`
  (`tests/engine-types.sh`) has the pinned 2.1.296 write its
  `claude-code` declarations over a scratch mod, with no account and no
  traffic, compares `engine-types/` byte for byte, fails naming the file
  and the release, and fails itself where a copy with one line changed
  reads the same. `plugin-claude:typecheck` types the module and its
  tests by the TypeScript 7.0.2 `devenv.nix` pins and asserts, which
  `root:toolchain` checks; `treefmt.nix` excludes the vendored types by
  name with its reason (core.21, core.22).

### herdr-plugin

- **The sidebar shows what status gives.** `watch.sh`'s `report` copies
  `.line.short` as the pane's `flume` token; `rows_jq` is gone from
  `lib/common.sh`, `watch.sh` and `places.sh`. `tests/run.sh` reads the
  token as status gives it and nothing of flume's run but `settle` and
  `status`.
- **The popup shows each place's line.** `places.sh` draws
  `▶ 1  <line.full>` with the location beneath; `tests/run.sh` reads the
  two places' rows.
- **The watch records what herdr's configuration lacks.** `setup` runs at
  the watch's start, logged once, and after each settle, reading
  `HERDR_CONFIG_PATH`, else `$XDG_CONFIG_HOME`, else `$HOME`.
  `tests/run.sh` holds the three scenarios (neither, both, no file), the
  two other paths, the log's words once, and no `notification` call to
  the stand-in herdr.
- **The popup's key is the host's entry.** The manifest's `places`
  action gains its `description` and no key. `tests/manifest.sh` fails a
  description naming no entry and a README whose entry is not
  `lib/common.sh`'s; `tests/install.sh` passes the README's entry and
  rows through the pinned herdr's `config check`, requiring
  `config: ok`.

### terminal-profiles

- **herdr's setup read beside a fresh lease.** The herdr profile's
  `setup` reads `setup.json` beside each fresh lease, in the servers'
  order, each words once, and runs no herdr command. Its unit tests
  `setup_is_a_fresh_leases_words_alone`,
  `setup_gives_each_servers_words_in_order_and_once` and
  `setup_is_none_beside_no_lease_or_nothing_lacking` hold the scenario
  and its edges; `tests/profiles/tools/terminal_herdr.rs`'s
  `a_watchs_setup_is_given_by_status_while_its_lease_is_fresh` reads a
  real watch's words through `flume status --json`'s
  `host.terminal_setup` on the pinned herdr, and none once it stopped.

### seam-contracts

- **The terminal seam gives what its configuration lacks.**
  `Terminal::setup`, none by default. `seams/terminal.rs`'s
  `setup_gives_what_is_lacking`, generic over the harness, gives none
  for a terminal with no plugin; `engine/place_status.rs`'s
  `the_terminals_setup` scripts the fake with one entry and reads it in
  status with no open, send or close recorded.

## Rules

- **core.1, core.2.** Only the Claude Code profile and the plugin name
  the program; `flume hook` dispatches through `crate::profiles::hook`.
  ec6bb17 keeps the conformance suite's search of the crates clean.
- **core.13.** The module's clock runs in Claude Code's own process,
  each tick one command that reads; core.13's details say so (7f12537).
- **core.14, core.15, core.16.** The line is made by one core function,
  copied by every surface, read when asked, and written nowhere.
- **core.29.** The watch's setup calls no herdr command; the entry is
  checked against the pinned herdr's `config check`.
- **core.35.** The README, the herdr plugin's README and the site's
  pages the change touches name no host, team or word of ours.
- **core.19.** The code stands ahead of design 9's drafts, asked on the
  card above.

## Decisions

This stage made no decision of the person's and none a later reader
would not find in the change: it read and ran checks.
