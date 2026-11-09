# Verify: flumes-inbox-is-weirs-delivery-form

Checked on 2026-11-08 at `change/flumes-inbox-is-weirs-delivery-form`
c15fa366, cut from `bolt/flumes-ends-are-owed-until-confirmed` at
72107a2b, the bolt's head still. The check covers the change's proposal,
design, its eight specs (`deliveries`, `end-delivery`, `role-waking`,
`place-status`, `command-line`, `claude-code-plugin`, `store-profile`
and `seam-contracts`) and its tasks.

**The whole checks pass and the work holds, so the change is archived.**
`sluice exec bolt/flumes-ends-are-owed-until-confirmed` at c15fa366
passed every one of the 32 checks it ran, with none failed. All 21 tasks
are ticked. Every requirement is built, and each of the specs' 78
scenarios has a test that passes in the same run. A few scenarios'
tests leave one part of what they say to another test. Those are listed
under "Scenarios" and none of them sends the change back.

## Checks

`sluice exec bolt/flumes-ends-are-owed-until-confirmed` ran once, with
the worktree clean, as the repository's instructions name the gate for a
place on a bolt: the bolt's `check-fast` layer (`.config/sluice.toml`,
`[branches."bolt/*"]`), with `check-tools` left to the bolt's merge onto
main. It ran "the checks c15fa36 affects against
bolt/flumes-ends-are-owed-until-confirmed (72107a2)", recorded 32
results under their keys for c15fa36, and exited 0. moon: 33 tasks
completed, 32 checks passed, none failed.

Passed:

- `seams`, `core`, `fakes`, `conformance` and `flume`: format, lint and
  test. The crates ran 814 tests with none failed: `seams` 109, `core`
  49, `fakes` 65, `conformance` 25 and `flume` 566. Among them are
  `the_engine_passes_its_scenarios`,
  `the_fake_store_passes_the_store_scenarios`,
  `store_state_dir::the_store_profile_passes_the_store_scenarios` and
  `store_state_dir::the_hosts_deliveries_are_seen_by_another_process`.
- `conformance:coverage`: every clause of Parts A and B, I1 to I8 and S1
  to S120 at 31f9f1c is cited by the suite's 923 scenarios, or left to
  the change it names.
- `plugin-claude`: format, typecheck, validate and test.
- `plugin-herdr`: manifest and test (27 of 27 under sh and dash).
- `site`: install, build (19 pages), and prose with 0 errors, 0
  warnings and 109 suggestions.
- `nix`: format, lint, typecheck and test, "every case holds", which
  covers `nix/packages/git-crates.nix`'s four weir entries at 52b95528.
- `root`: format, changes (2 of 2 open changes valid), adrs (59 records)
  and versions.

The tasks the diff doesn't touch are not run by the gate: `root:specs`,
`root:hooks`, `root:toolchain`, `root:layers`, `root:marketplace`,
`plugin-herdr:lint` and `plugin-claude:engine-types`. The archive below
changes `openspec/specs/`, so `openspec validate --specs --strict` runs
on the archive's commit in their place.

moon warned that it could not write its shared cache's blobs or archive
task outputs under the sandbox. Neither is a check, and no check failed
because of them.

## Tasks

All 21 tasks of the seven groups are ticked, `[x]`, and none is open.
The gate run above covers the narrow check each one names:

- `root:adrs` and `root:format` for 1.1;
- `flume:test`'s `kinds_weir` and `nix:test` for 2.1 and 2.2;
- `seams:test`'s four `round_trip.rs` tests for 3.1 and 3.2;
- every crate's lint for 3.3;
- `core:test`'s two time tests for 4.1;
- `conformance:test` and `flume:test`'s store profile for 4.2;
- `conformance:test`'s `the_engine_passes_its_scenarios` for 5.1 to 5.6;
- `flume:test`'s `commands.rs`, the `--lib` tests of `host/config.rs` and
  `commands/print.rs`, `store_state_dir.rs` and `hooks_claude.rs` for
  6.1 to 6.5;
- `plugin-claude:test` for 6.5;
- `site:prose` for 7.1 and 7.2.

weir is at `52b95528a495100e34fca39109d563d8bf75673a` for `weir-kinds`,
`weir-delivery` and `weir-pack` in `Cargo.toml`, `Cargo.lock` and
`git-crates.nix`, and `git grep -n daacda3 -- Cargo.toml Cargo.lock`
finds nothing. `openspec validate flumes-inbox-is-weirs-delivery-form
--strict` gives the change valid.

c15fa366 also moves the herdr walkthrough
(`tests/profiles/tools/walkthroughs.rs`). The walkthroughs are
`check-tools`, so they are not run on a bolt. Their code builds under
`flume:lint`, and they run at the bolt's merge onto main.

## Requirements and scenarios

Each scenario has a test of its own, which passed in the run above:

- `deliveries` (3 requirements, 6 scenarios): the store seam's three new
  scenarios in `crates/flume-conformance/src/seams/store.rs`, run over
  the fakes' store and the state directory, weir's pack among them with
  every case passing; and `a_place_launched_twice` and
  `a_place_written_before_the_token` in `end_delivery.rs`.
- `end-delivery` (16 requirements, 46 scenarios): the 58 scenarios of
  `crates/flume-conformance/src/engine/end_delivery.rs`, each modified
  scenario reading as the spec now words it. The new ones are all there:
  "A wait that confirms", "Collected and confirmed", "Handed and stopped
  before acting", "A notice claimed and not recorded", "Confirmed
  twice", "A key of another recipient's", "A misspelt recipient",
  "Confirmed once expired" and "A live place upgraded". "A standing
  agent under herdr through the binary" is the herdr walkthrough's
  (`check-tools`).
- `role-waking` (2 requirements, 5 scenarios): `role_waking.rs`, with
  "A run that did not confirm" and "The next run is handed it" new and
  "The run starts" confirming on receipt.
- `place-status` (2 requirements, 3 scenarios): `place_status.rs`'s
  `a_stalled_place_with_a_pending_end`, `two_expired_ends` and
  `the_fields_kept`, with every new field's path in `answers.rs`'s
  `FIELDS`.
- `command-line` (2 requirements, 5 scenarios): the five tests in
  `crates/flume/tests/commands.rs` the tasks name.
- `claude-code-plugin` (1 requirement, 4 scenarios): `hooks_claude.rs`,
  `a_restarted_pane` reading the stop-short handed by `session_start`
  and confirmed. `session_inbox` confirms on receipt
  (`crates/flume-core/src/wait.rs`), and `hooks.json`'s description says
  so.
- `store-profile` (2 requirements, 5 scenarios):
  `the_hosts_deliveries_are_seen_by_another_process`, which checks mode
  0600, and the four `ends_expire` tests in `host/config.rs`. The real
  host gives the configured span (`host/mod.rs`).
- `seam-contracts` (2 requirements, 4 scenarios):
  `whom_each_end_is_pending_for` and the four new tests in
  `crates/flume-seams/src/round_trip.rs`.

The code matches what the specs name:

- the notice ends `; confirm: flume confirm <delivery key>` (`ends.rs`);
- `EndStatus` gives `delivery`, `expires_at`, `owed` (with `expired`
  and `known`) and `confirmed`;
- `HostStatus` gives `expired`;
- the inbox's `Collected` gives `confirmed` and `expired`;
- `flume confirm`, `flume wait --confirm` and `flume inbox --confirm`
  carry the helps the command-line spec asks for.

### Scenarios whose tests leave a part to another test

None of these is a defect in what flume does. Each part is built, and
most are asserted by another scenario in the same run:

- `place-status` "A stalled place with a pending end": the test asserts
  the move, since when, stalled, the stall's hand-over to `ops` with
  when, and pending for `madswan/planner` and the answerer. It does not
  assert `ops`'s confirmation with its time, or `madswan/planner` read
  as a name never confirmed under. "A live place upgraded" asserts a
  confirmation with its time, and "A misspelt recipient" asserts
  `known` false.
- `end-delivery` "Collected after a restart": the test asserts each
  end's key, but not its delivery key in the inbox's answer.
  `a_key_confirmed_through_the_binary` reads the delivery key from the
  inbox's JSON.
- `end-delivery` "Collected and confirmed": at the engine, the test
  asserts that nothing is pending afterwards, but not each collected
  end's `confirmed`. The binary's `a_readers_place_collected_by_its_inbox`
  asserts `confirmed` false, then true.
- The requirement that an expired end shows so in the inbox and a wait
  (`Collected::expired`, and `owed[].expired` on a wait's end) is built
  (`wait.rs`). No scenario asks for it there, and no test asserts it:
  "A misspelt recipient" and "Two expired ends" assert it in status
  alone.

A later change could tighten these tests. None of them leaves a clause
unbuilt, so the change is archived.
