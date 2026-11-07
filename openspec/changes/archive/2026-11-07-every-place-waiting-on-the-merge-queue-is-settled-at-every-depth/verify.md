# Verify: every-place-waiting-on-the-merge-queue-is-settled-at-every-depth

Read at `c553fc11` on `change/every-place-waiting-on-the-merge-queue-is-settled-at-every-depth`,
against main at `b1f6a617` (#147), which the branch holds. main is
still at `b1f6a617` at origin.

## Verdict

**Holds: archived.** All 40 tasks are ticked, 7.6 among them. Every
scenario of the 15 delta specs is held by a test, apart from the one
design 6 defers. A trial archive leaves `openspec validate --specs
--strict` passing 46 of 46. `sluice exec main` passes 37 of 37 tasks,
`flume:tools` 130 of 130 among them, S105's test included.

## S105's test (task 7.6)

Main's gate failed `flume:tools` for submission #149 on
`open_sessions::s105_the_agents_account_spent_within_the_hour_and_a_retry_answers`.
`fa55ca85` fixes it, and `c553fc11` takes the change back out of the
archive, its specs main's at `b1f6a617`.

- **The fix.** Code's second start ran its script straight through, so
  its agent delivered and the place landed and was tidied between two of
  the test's reads. Its script now opens with the stand-in's
  `wait_for` act on a signal from `Walk::signal`. The test writes that
  signal only after it has read the live agent on the pool's second
  account, `stand-in-2`. The stand-in waits up to two minutes for the
  file, as its module says, and then goes on, so a test that never writes
  the signal still ends.
- **Nothing else moved.** Between the archive at `9ba5e5bb` and
  `fa55ca85`, the only change is that test's 11 lines. The scenario's
  assertions are unchanged: the account, the landing, the remote's
  `notes.md` and the second start's prompts. S105 is still cited (146,
  198, S105).
- **The specs.** `openspec/specs` at `c553fc11` is main's at
  `b1f6a617`, file for file. The trial archive's specs are identical to
  the archive's at `9ba5e5bb`.

## What holds

### Tasks and their checks

- `openspec validate every-place-waiting-on-the-merge-queue-is-settled-at-every-depth --strict`:
  valid.
- The change archived over a copy of `openspec/` (openspec 1.14.3, the
  devenv's): +34, ~12, −8, →1, with `base-places` new. After it,
  `openspec validate --specs --strict` passes 46 of 46. `diff -r`
  against `openspec/specs` at `9ba5e5bb` finds no difference.
- `sluice exec main` at `c553fc11` passes 37 of 37 tasks, and sluice
  recorded the result under its key for `c553fc1`:
  - `flume:tools`: 130 of 130, both of `open_sessions`' S105 tests among
    them;
  - `plugin-herdr:test`: 27 of 27;
  - `root:specs`: 45 of 45, before the archive;
  - `root:changes`: 2 of 2;
  - `conformance:coverage`: every clause of Parts A and B, I1 to I8,
    and S1 to S119 at `9c060ab` cited by the suite's 794 scenarios, or
    named unbuilt by its change.

### The merge of #147 (task 7.5)

`a2911169` merges main at `b1f6a617` with both sides kept, apart from
the three fits task 7.5 names:

- a base place's `WaitingOn` gives main's `open_until` and `closed` as
  none;
- `settle --every` reads its bound by main's `read_span`;
- the conformance suite's counting terminal passes on main's `sends`.

No line of #147 is lost. No nudge, lease or terminal's watch comes back,
and no core code names a bolt, a unit or a depth. `clauses.rs` reads
blueprints at `9c060ab`, as main does. It names 202 to 206, 208, S109
to S112 and S118 built. 193, 209 and S119 are still named unbuilt, by
their own changes. The clause-coverage delta names `9c060ab` and names
198, S104 and S105 built beside this change's clauses.

### Requirements and scenarios

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
- **core.51**: S105's test waits on a signal in its walk's own scratch.
  It neither skips nor returns early.
- **core.19**: the change cites 146, 198, 202 to 206 and 208 where it
  touches them. Record 0051 is accepted. Its amendment, that no
  terminal settles, stays proposed until the user sees it as a record,
  as the record says.
