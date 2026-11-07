# Verify: every-place-waiting-on-the-merge-queue-is-settled-at-every-depth

Read at `70ec2fc9` on `change/every-place-waiting-on-the-merge-queue-is-settled-at-every-depth`,
against main at `9a838ee1` (#144), which the branch holds.

## Verdict

**Holds: archived.** Every task is ticked. Every scenario of the 15
delta specs is held by a test, apart from the one design 6 defers. A
trial archive leaves `openspec validate --specs --strict` passing 46 of
46, and `sluice exec main` passes 37 of 37 tasks.

## What was found short before, and how it stands

The report at `b4450ad8` found that two requirements this change
modified would be over openspec's 500 characters once archived:
seam-contracts' "The terminal seam's open, tidy and show" (606) and
terminal-profiles' "The terminal through herdr" (521). Task 7.4 fixes
this by modifying neither one. Each keeps main's text, and the change
adds what it needs as a requirement of its own:

- **seam-contracts**: "The clues a terminal shows" adds a place's
  line and its count on the base to what show takes (139). Main's text
  already says show takes "a place's clues". `seams/terminal.rs`'s
  `show_gives_clues` holds its scenario, "A place's clues shown", with
  the line `code · run 1 · tasks 2/5 · working` and one place running
  on the base.
- **terminal-profiles**: "herdr settles nothing" (119, 122, 208, C.5).
  Main's text already says the profile "keep[s] no watch".
  `tools/terminal_herdr.rs`'s `unattended_by_the_hosts_watch_alone`
  holds its scenario.

Neither requirement main keeps names a nudge or a lease, so the
archive leaves nothing that contradicts the work. The archived specs
name a nudge only once, in seam-contracts' "The terminal seam starts no
settle", which rules one out.

Task 7.4 also merges main at `9a838ee1` (#144, an agent's effort) in
`e182328f`, with both sides kept in `clauses.rs` and the state
reference. The clause-coverage spec's list now names 197 and S103
built, and 207 and S113 to S117 built too. `clauses.rs` agrees, and its
unit test checks that none of these is named unbuilt.

## What holds

### Tasks

All 38 tasks in `tasks.md` are ticked, 7.4 among them. Its three
checks pass:

- `openspec validate every-place-waiting-on-the-merge-queue-is-settled-at-every-depth --strict`:
  valid.
- The change archived over a copy of `openspec/` (openspec 1.14.3,
  the devenv's): +34, ~12, −8, →1, with `base-places` new. After it,
  `openspec validate --specs --strict` passes 46 of 46.
- `sluice exec main` at `70ec2fc9`: passes 37 of 37 tasks, and sluice
  recorded the result under its key for `70ec2fc`. Among them,
  `flume:tools` passes 124 of 124, `flume:test` and `plugin-herdr:test`
  (27 of 27) pass, and `root:specs` passes 45 of 45 before the archive.

### Requirements and scenarios

Since the report at `b4450ad8`, the code has changed only by main's
#144, merged in. That report's account of each spec still stands, and
each test it names is in the tree:

- **settling**: `engine/settling.rs`, S109 and S118 by name among them.
- **base-places**: `engine/base_places.rs` holds 20 of the 21
  scenarios, S110 to S112 by name among them. The one it lacks is
  "Points given to a base place". Design 6 defers it to the change
  that builds points (165 to 168). flume builds no points yet: a launch
  carries none, and the host's configuration refuses a stream's
  `points`, so no launch can reach that refusal.
- **command-line**, **place-status**, **end-delivery**,
  **stream-terminals**, **stage-runs**, **seam-fakes**: in
  `crates/flume/tests/commands.rs`, `engine/answers.rs`,
  `engine/end_delivery.rs`, `engine/stream_terminals.rs`,
  `engine/starts.rs` and `crates/flume-fakes/tests/fakes.rs`.
- **seam-contracts**: `seams/merge_queue.rs`, `seams/terminal.rs`
  (the attended reading and `show_gives_clues`), `seams/store.rs` and
  `round_trip.rs`.
- **merge-queue-profile**, **place-profile**, **kinds-profile**,
  **terminal-profiles**: in `tests/profiles/tools/merge_queue_sluice.rs`,
  `tests/profiles/place_git_worktree.rs`, `tests/profiles/kinds_weir.rs`,
  and the herdr profile's unit test with
  `tests/profiles/tools/terminal_herdr.rs`.
- **herdr-plugin**: `plugins/herdr/tests/run.sh` and
  `tests/install.sh`.
- **clause-coverage**: `clauses.rs` reads blueprints main at
  `6dc9399`. This change's scenarios cite each of 202 to 206 and 208,
  and S109 to S112 and S118.

### The rules

- **core.53**: the core tells a base place by
  `kind.lands && kind.stages.is_empty()` alone, and names no bolt, unit
  or depth.
- **core.13**: `Engine::attended` reads the host's watch alone. No
  terminal profile or plugin settles, and herdr's plugin reads status
  only at a person's asking. One test message in
  `engine/stream_terminals.rs` says "trial nudging nothing". It says
  that nothing nudges, and names no nudge as a thing that exists.
- Record 0051 is accepted. Its amendment that no terminal settles is
  proposed until the user sees it as a record, as the record says.

## Beside the change

Main is still at `9a838ee1`, which the branch holds, so a submission
meets no conflict.
