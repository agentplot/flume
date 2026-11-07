# Verify: every-place-waiting-on-the-merge-queue-is-settled-at-every-depth

Read at `f9381310` on `change/every-place-waiting-on-the-merge-queue-is-settled-at-every-depth`,
against main at `b1f6a617` (#147), which the branch holds. main is
still at `b1f6a617` at origin.

## Verdict

**Holds: archived.** All 39 tasks are ticked, 7.5 among them. Every
scenario of the 15 delta specs is held by a test, apart from the one
design 6 defers. A trial archive leaves `openspec validate --specs
--strict` passing 46 of 46. `sluice exec main` passes 37 of 37 tasks.

## The merge of #147 (task 7.5)

`a2911169` merges main at `b1f6a617`, and `f9381310` takes the change
back out of the archive so its specs can be applied over main's.

- **Both sides kept in the code.** #147's diff over `9a838ee1` matches
  what the merge brings into the branch, apart from this change's own
  lines in context and the three fits task 7.5 names:
  - a base place's `WaitingOn` gives main's `open_until` and `closed`
    as none;
  - `settle --every` reads its bound by main's `read_span`;
  - the conformance suite's counting terminal passes on main's `sends`.

  No line of #147 is lost. No nudge, lease or terminal's watch comes
  back, and no core code names a bolt, a unit or a depth.
- **The clause list.** `clauses.rs` reads blueprints at `9c060ab`, as
  main does. It names 202 to 206, 208, S109 to S112 and S118 built.
  193, 209 and S119 are still named unbuilt, by their own changes.
- **The specs.** #147 and this change both modify only clause-coverage's
  two requirements. The delta carries both: it names `9c060ab`, and it
  names 198, S104 and S105 built beside this change's clauses. Every
  other requirement #147 added or changed is untouched by this change's
  delta.

## What holds

### Tasks and their checks

- `openspec validate every-place-waiting-on-the-merge-queue-is-settled-at-every-depth --strict`:
  valid.
- The change archived over a copy of `openspec/` (openspec 1.14.3, the
  devenv's): +34, ~12, −8, →1, with `base-places` new. After it,
  `openspec validate --specs --strict` passes 46 of 46. Each archived
  spec matches the merge's `openspec/specs` at `a2911169` requirement
  for requirement, body for body. Only the order of requirements differs,
  in command-line, kinds-profile, place-status, seam-contracts and
  seam-fakes. Against main's specs, every difference is one of the
  delta's own added, modified, removed or renamed requirements.
- `sluice exec main` at `f9381310` passes 37 of 37 tasks, and sluice
  recorded the result under its key for `f938131`:
  - `flume:tools`: 130 of 130;
  - `plugin-herdr:test`: 27 of 27;
  - `root:specs`: 45 of 45, before the archive;
  - `root:changes`: 2 of 2;
  - `conformance:coverage`: every clause of Parts A and B, I1 to I8,
    and S1 to S119 at `9c060ab` cited by the suite's 794 scenarios, or
    named unbuilt by its change.

### Requirements and scenarios

Since the report at `70ec2fc9`, the only code changes are #147's,
merged in, and the three fits above. That report's account of each spec
still stands:

- **settling**: `engine/settling.rs`, S109 and S118 by name among them.
- **base-places**: `engine/base_places.rs` holds 20 of the 21
  scenarios, S110 to S112 by name among them. The one it lacks is
  "Points given to a base place", which design 6 defers to the change
  that builds points (165 to 168). flume still builds no points: neither
  the core nor the command line reads any, so no launch can reach that
  refusal.
- **command-line**, **place-status**, **end-delivery**,
  **stream-terminals**, **stage-runs**, **seam-fakes**: in
  `crates/flume/tests/commands.rs`, `engine/answers.rs`,
  `engine/end_delivery.rs`, `engine/stream_terminals.rs`,
  `engine/starts.rs` and `crates/flume-fakes/tests/fakes.rs`.
- **seam-contracts**: `seams/merge_queue.rs`, `seams/terminal.rs`,
  `seams/store.rs` and `round_trip.rs`.
- **merge-queue-profile**, **place-profile**, **kinds-profile**,
  **terminal-profiles**: in `tests/profiles/tools/merge_queue_sluice.rs`,
  `tests/profiles/place_git_worktree.rs`, `tests/profiles/kinds_weir.rs`,
  and `tests/profiles/tools/terminal_herdr.rs` with the herdr profile's
  unit test.
- **herdr-plugin**: `plugins/herdr/tests/run.sh` and
  `tests/install.sh`.
- **clause-coverage**: `clauses.rs` at `9c060ab`, as above.

### The rules

- **core.53**: the core tells a base place by
  `kind.lands && kind.stages.is_empty()` alone.
- **core.13**: `Engine::attended` (`settle.rs`) reads the host's watch
  alone. No terminal profile or plugin settles. #147's keep-open path
  reads whether a terminal hands words (`sends`), and settles nothing
  of its own.
- **core.19**: the change cites 198, 202 to 206 and 208 where it
  touches them. Record 0051 is accepted. Its amendment, that no
  terminal settles, stays proposed until the user sees it as a record,
  as the record says.
