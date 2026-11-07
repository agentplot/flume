# Verify: every-place-waiting-on-the-merge-queue-is-settled-at-every-depth

Read at `bf8675d0` on `change/every-place-waiting-on-the-merge-queue-is-settled-at-every-depth`,
against main at `9a838ee1` (#144).

## Verdict

**Falls short: archive nothing.** The code and the tests hold every
scenario. But archiving the change as written fails the gate's
`root:specs` task. Two requirements this change modifies are longer than
`openspec validate --specs --strict` allows once they are applied to
`openspec/specs/`.

## What fails

### The archived specs fail `root:specs`

A trial archive over a copy of `openspec/` applies cleanly (+32, ~14,
−8, →1, with `base-places` new). After it, `openspec validate --specs
--strict`, which is `root:specs`'s command (`moon.yml`), passes 43 of 45
specs. The branch's own specs pass 44 of 44. The two that fail fail on
the same warning, "Requirement text is very long (>500 characters)", and
in both cases the requirement is one this change MODIFIES:

| spec | requirement | text before its scenarios |
|---|---|---|
| `seam-contracts` | The terminal seam's open, tidy and show | 606 characters |
| `terminal-profiles` | The terminal through herdr | 521 characters |

`openspec validate every-place-waiting-on-the-merge-queue-is-settled-at-every-depth --strict`
passes, because the change's own validation does not measure a
requirement's length, so nothing before the archive caught it. To fix
it: shorten or split each requirement in the change's delta spec, for
example moving herdr's bound and `HERDR_*` stripping into a requirement
of their own, and the terminal seam's bound into its own. Then a trial
archive should leave `openspec validate --specs --strict` passing 45 of
45.

## What holds

### Tasks

All 37 tasks in `tasks.md` are ticked. Each one names the checks that
show it done. The gate's run below covers those checks.

### Requirements and scenarios

Each scenario in the 15 delta specs has a test that holds it:

- **settling**: `engine/settling.rs` holds "A watch kept, then lapsed",
  "No terminal keeps watch", S109 (`s109_…`), "A queue that moves on its
  own", "Two depths in one settle", "Killed with nobody watching",
  "Crashed under a watch", "A watched host with one stream unwatched"
  and S118 (`s118_a_host_with_no_watch_runs_attended`). "Two depths"
  submits a chore to `main` beside a chore on the base place's branch,
  not the base place's own submission. As written, the scenario cannot
  happen: 203 refuses a base place's submission while a place on its
  branch is live. The test still asserts what the scenario is about,
  one step of each base in the command.
- **base-places**: `engine/base_places.rs` holds 20 of the spec's 21
  scenarios, S110, S111 and S112 by name among them. The one it lacks is
  "Points given to a base place". Design 6 defers it to the change that
  builds points (165 to 168): no launch carries points today, and the
  clause list names 165 as that change's.
- **command-line**: `crates/flume/tests/commands.rs` holds
  `a_watch_over_a_scratch_repository`, `a_watch_with_a_place_named` and
  `one_places_line_under_the_printed_command`.
- **place-status**: `engine/answers.rs` holds the host's watch, kept
  and lapsed, and each terminal's `watch` none beside the host's bound.
- **end-delivery**: `engine/end_delivery.rs` holds "Under a watch",
  "Attended under any terminal" and "Under the printed command".
- **stream-terminals**: `engine/stream_terminals.rs` holds "One stream
  under another terminal", "Two terminals and no watch" and S91.
- **stage-runs**: `engine/starts.rs` holds "The line alone changes"
  beside "Titles follow a stage".
- **seam-contracts**: `seams/merge_queue.rs` holds open then close, a
  refused open and a submit by name. `seams/terminal.rs` holds the
  attended reading under every terminal and show with the line.
  `seams/store.rs` holds two watches at once. `flume-seams`'
  `round_trip.rs` holds `SubmitHead`, `BranchOpened`, `BranchClosed`,
  `BranchRemoved::Refused`, `HostKey::Watch`, `Watched` and `Clues`'
  line.
- **seam-fakes**: `crates/flume-fakes/tests/fakes.rs` holds scripted
  open and close, a submission by name decided in the record, a watch
  the clock lets lapse, and show's clues in the log.
- **merge-queue-profile**: `tests/profiles/tools/merge_queue_sluice.rs`
  holds the refused open, the refused close, a base place's branch over
  a scratch repository, and S109 by `flume settle --every` against the
  pinned sluice.
- **place-profile**: `tests/profiles/place_git_worktree.rs` holds
  `a_base_place_is_given_its_location_and_nothing_is_made_there`.
- **kinds-profile**: `tests/profiles/kinds_weir.rs` holds the three
  tables.
- **terminal-profiles**: the herdr profile's unit test
  `setup_is_a_running_servers_words_alone`, and
  `tests/profiles/tools/terminal_herdr.rs`'s
  `unattended_by_the_hosts_watch_alone` and `the_row_follows_the_line`.
- **herdr-plugin**: `plugins/herdr/tests/run.sh` holds the startup, ten
  minutes of agents changing, an earlier release's watch ended, the
  three setups, a row from a settle, and the popup's two scenarios.
  `tests/install.sh` holds the pinned herdr's `config check`.
- **clause-coverage**: `clauses.rs` reads blueprints main at `6dc9399`,
  and every clause from 202 to 206 and 208, plus S109 to S112 and S118,
  is cited by this change's scenarios.

### The rules

- **core.53**: `base.rs` tells a base place apart by
  `kind.lands && kind.stages.is_empty()` alone. No code in the core
  names a bolt, a unit or a depth.
- **core.13**: `Engine::attended` reads the host's watch alone.
  `settle_every` writes the watch by compare-and-set and then settles
  every place. No source, test or page names a nudge, and the herdr
  plugin keeps only `startup.sh`, `places.sh` and `open.sh`, with no
  watch loop and no lease library.
- Record 0051 is accepted. Its amendment that no terminal settles is
  proposed until the user sees it as a record, as the record itself says.

### Checks

`sluice exec main` passed 37 of 37 tasks at `bf8675d0`, `flume:tools`
121 of 121 among them. sluice did not record the run: this report was
written in the worktree while the run went on, so the tree no longer
matched its head. `openspec validate
every-place-waiting-on-the-merge-queue-is-settled-at-every-depth
--strict` passes. Run on the branch's own specs, `openspec validate
--specs --strict` passes 44 of 44. That gate is green only until the
archive, as "What fails" says.

## Beside the change

Main has moved to `9a838ee1` (#144) since this branch merged it at
`f6a5ba72`. `git merge-tree` finds conflicts with main in
`crates/flume-conformance/src/clauses.rs` and
`site/src/content/docs/reference/state.md`, so a submission would come
back conflicting. The code stage can merge main while it fixes the
specs.
