# Tasks

One agent builds this change, task by task in order, reading the
proposal, the design and the specs first. The groups are headings for
reading. Each task names the narrowest check that shows it done, run as
written; none runs the repository's whole checks, which the verify stage
runs once after every task is done. Every git call in a test runs under
the scratch environment `crates/flume/tests/common` gives, and tests
reach no real account, no repository of the user's, no person's queue
and no network.

Record 0056 is accepted, on the user's card of 2026-11-07, and the
statement carries it in 1, 203 and I6 on agentplot/blueprints' main at
24d36f8. The statement's amendment this design needs, 210 and S120
among it, is on blueprints' branch
`design/flume-a-base-place-launches-its-fix` at cffcbcc. Only task 6.2,
the clause list, needs either on the remote's main, and only task 7.1
needs weir's follow-up W2 (design, "Named follow-ups"); each is near
the end, so every task before it is built either way. A build that
finds what a task needs not landed stops short naming it, every task
before it done.

The engine's scenarios use the repository's kinds `train`, landing with
no stages, and `mend`, landing with one code stage delivering commits,
so nothing rests on flume's own kinds, and they name no tool (core.1).
No code of the core names a kind, a prefix or a depth to tell a base
place or its fix: it reads the kind's shape and the kind its kind names
(202; core.53). The forecast is asked only through the merge queue seam
(core.2), and the fix is launched only inside a settle (core.13).
Every scenario added cites its clauses as data (106).

## 1. The decision recorded

- [x] 1.1 Hold record 0056's amendment line as this change's construct wrote it, as design 14 words it, citing the blueprints branch at cffcbcc; amend it where the statement the build reads differs (core.19). True when `devenv shell -- sh check-adrs.sh` and `devenv shell -- treefmt --ci docs/adr/0056-a-base-place-forecast-or-found-to-conflict-with-its-base-or-whose-submission-failed-has-its-fix-launched-by-flume.md` pass.
- [x] 1.2 Hold core.53's and core.34's rules and details and core.6's and core.36's details in `docs/architecture/core.md`, and the mirror of core.53 and core.34 in `site/src/content/docs/architecture/rules.md`, as this change's construct wrote them to design 14 (core.19, core.35). True when `devenv shell -- treefmt --ci docs/architecture/core.md site/src/content/docs/architecture/rules.md` and `devenv shell -- sh -c 'cd site && vale src/content/docs/architecture/rules.md'` pass.

## 2. The seams and the fakes

- [x] 2.1 Give `flume-seams` `MergeQueue::forecast(repository, branch, base) -> Forecast` with `Forecast::{Clear, Conflict, CouldNotRead}` as design 3 has it, re-exported, with no default method, and have every implementor answer it: the fake clear at the base's revision until 2.3 scripts it, the sluice profile `CouldNotRead` with `not yet built` until 5.1, and every stand-in a test or a harness holds, `crates/flume/tests/commands.rs`'s among them, `CouldNotRead` naming the stand-in (210; core.2). True when `devenv shell -- cargo check --workspace --all-targets --all-features` passes.
- [x] 2.2 Give `flume-seams` design 11's held types: `Kind::fix` and `Kind::fix_limit`, `PlaceRecord::fixes_as` with `FixKind`, `PlaceRecord::fix_of` with `FixOf`, `HeldState::forecast` with `ForecastHeld`, `HeldState::fixes` with `BaseFix`, `FixCause` and `FixEnded`, and `RecipientsFrom::BasePlace`, each defaulting to none so a document written before reads unchanged; add the seam-contracts spec's round trips to `crates/flume-seams/src/round_trip.rs`, a held state written before this change among them (21, 61, 203). True when `devenv shell -- cargo test -p flume-seams --lib round_trip` and `devenv shell -- cargo check --workspace --all-targets --all-features` pass.
- [ ] 2.3 Have the fake merge queue answer forecast as the seam-fakes spec has it, by `answer_forecast(repository, branch, Forecast)` beside `answer_open`, logging each ask `forecast <branch> onto <base>`; add the spec's scenario to `crates/flume-fakes/tests/fakes.rs` as `the_fake_merge_queue_scripts_a_forecast` (106, 210). True when `devenv shell -- cargo test -p flume-fakes --test fakes the_fake_merge_queue_scripts_a_forecast` passes.
- [ ] 2.4 Give `MergeQueueHarness` a way to move a base with a change to a file a branch also changed, the fake's harness scripting its forecast so, and add the seam-contracts spec's "Clear, then in conflict" to `base_place_scenarios` in `crates/flume-conformance/src/seams/merge_queue.rs`, citing 210 (106, 210). True when `devenv shell -- cargo test -p flume-conformance --test seams the_fake_merge_queue_passes_the_base_place_scenarios` passes.

## 3. The core's fixes of a base place

- [ ] 3.1 Have `launch.rs` resolve a base place's fix kind at its launch as design 2 has it, before the merge queue's open, refusing the launch whole naming the field and why, and keep the identity and where it came from as `PlaceRecord::fixes_as`; add the base-places spec's "A base place whose kind names mend" and the two scenarios of "A base place's launch is refused for a fix kind flume could not launch" to `crates/flume-conformance/src/engine/base_places.rs` (4, 21, 23, 154; core.8). True when `devenv shell -- cargo test -p flume-conformance --test engine` passes.
- [ ] 3.2 Have the settle forecast a base place as design 4 has it, in `crates/flume-core/src/base.rs` beside the base place's other acts and called from the settle's steps, holding the forecast on the base place only where its outcome, head or base revision changed, and a forecast that could not be read launching nothing and failing no settle; add the scenarios of the spec's "A settle forecasts a base place waiting on its caller" and "A base place holds the newest forecast a settle read" to `base_places.rs` (46, 203, 210). True when `devenv shell -- cargo test -p flume-conformance --test engine` passes.
- [ ] 3.3 Have a forecast in conflict with the base itself, and a failed or conflicting submission in `submit.rs`'s base place arm of `verdict`, launch the fix as designs 5 and 7 have it: the fix written onto the base place by compare-and-set, then `Engine::make_place` called inside the settle with design 7's `Launch`, its name `<base place's name>-<n>`, its brief one of design 7's three first lines, a blank line and the merge queue's words whole, its `fix_of` and its recipients from `RecipientsFrom::BasePlace`; a forecast in conflict only with entries queued ahead launching nothing. Unit-test the three first lines beside the code that writes them, and add the scenarios of the spec's "A conflict with the base, or a failed submission, launches the fix", "A base place's fix is launched as its base place's" and "A fix's brief carries the merge queue's words", and S120's engine half as `s120_a_base_place_forecast_in_conflict_launches_its_fix`, to `base_places.rs` (43, 112, 203, S120; core.5, core.28, core.36). True when `devenv shell -- cargo test -p flume-core --lib` and `devenv shell -- cargo test -p flume-conformance --test engine` pass.
- [ ] 3.4 Hold design 6 and design 8's first item: one fix at a time, a failure acted on only while no fix of the base place's is live, each forecast by its head and base revision and each failure by its submission acted on once, a fix written with no place standing launched by the next settle, and a launch refused because the fix's place now stands taking that place as the fix, as `wake.rs` takes a role's run; add the scenarios of the spec's "A base place's fixes run one at a time" and "Each forecast and each failure launches one fix at most" to `base_places.rs` (3, 17, 74, 75, 203; core.17). True when `devenv shell -- cargo test -p flume-conformance --test engine` passes.
- [ ] 3.5 Hold design 9: a fix's own settle that reads its work over writes how it ended onto its base place's `fixes` by compare-and-set, landed with its commit or dropped, a base place whose document is gone taking nothing; a dropped fix leaving its base place waiting on its caller with words naming it; add the spec's "A dropped fix leaves its base place waiting on its caller" to `base_places.rs` (63, 75, 130, 203). True when `devenv shell -- cargo test -p flume-conformance --test engine` passes.
- [ ] 3.6 Hold design 8's bound and refused launch: each fix counted toward the kind's bound, `Defaults::send_backs` where the kind gives none, a refused launch among them; a conflict or a failure read with the bound spent launching nothing, the base place waiting on its caller with design 8's words, stalling and delivered as any wait on its caller; a refused launch held refused with its words and the base place waiting on its caller with them; add the spec's "A base place's fixes count toward its kind's bound", "A fix whose launch is refused counts and leaves its base place waiting" and "A declaration changed after the launch" to `base_places.rs` (21, 23, 43, 130, 203). True when `devenv shell -- cargo test -p flume-conformance --test engine` passes.
- [ ] 3.7 Keep a base place whose kind names no fix as today, never forecast and its failure waiting on its caller with the merge queue's words, and move the existing failed and conflicting submission scenarios and `s112_a_failed_base_place_takes_a_fix` in `base_places.rs` to a `train` naming no fix, as the spec's modified requirement has it (112, 203, S112). True when `devenv shell -- cargo test -p flume-conformance --test engine` passes.

## 4. Status

- [ ] 4.1 Give `PlaceStatus` design 10's `fix_kind`, `fix_limit`, `forecast`, `fixes` and `fix_of`, each left out where it does not apply, read from the held state with no forecast formed, and the recipients' source `base-place`; list each in `FIELDS` in `crates/flume-conformance/src/engine/answers.rs`, and add the place-status spec's JSON scenarios to the engine's status scenarios (56, 58, 61, 203; core.14, core.15). True when `devenv shell -- cargo test -p flume-conformance --test engine` passes.
- [ ] 4.2 Have `crates/flume/src/commands/print.rs` give design 10's person's form of a base place's fix kind, forecast and fixes and of a fix's base place, with a unit test of the place-status spec's "A fix landed, in the person's form" in its `tests` module (56). True when `devenv shell -- cargo test -p flume --lib commands::print` passes.
- [ ] 4.3 Name the new fields in `site/src/content/docs/reference/state.md`: a base place's fix kind, bound, forecast and fixes, and a fix's `fix_of` and its recipients' source, to any host's operator (56, 61; core.35). True when `devenv shell -- sh -c 'cd site && vale src/content/docs/reference/state.md'` passes.

## 5. The sluice profile's forecast

- [ ] 5.1 Have the sluice profile answer forecast by `sluice submit <base> --check --head <branch> --json` in the repository as design 3 and the merge-queue-profile spec have it, its answer read of the pinned sluice 0.71.0, passing `--check` to no submit; unit-test its reading of a clear answer, a conflict with and without entries ahead, and a refusal, the spec's "A conflict's words" among them (210, C.6; core.6). True when `devenv shell -- cargo test -p flume --lib profiles::merge_queue::sluice` passes.
- [ ] 5.2 Add the merge-queue-profile spec's "A base place's branch forecast over a scratch repository" to `crates/flume/tests/profiles/tools/merge_queue_sluice.rs` against the pinned sluice, and give its harness 2.4's move of a base so the seam's base place scenarios, "Clear, then in conflict" among them, run against sluice too (203, 210; sluice 64). True when `devenv shell -- cargo test -p flume --features tools --test tools merge_queue_sluice` passes.

## 6. The guide and the clause list

- [ ] 6.1 Say in `site/src/content/docs/guides/merge-queue.md`'s section on branches that land on branches that a kind naming its fix has the fix launched on its branch at a settle, on a forecast conflict with its base or a failed or conflicting submission, one at a time within its bound and named for the base place and its number, its submission still the reader's retry, and that a kind naming none waits on its caller as before, to any host's operator, naming no bolt and no caller of ours (23, 203; core.35). True when `devenv shell -- sh -c 'cd site && vale src/content/docs/guides/merge-queue.md'` passes.
- [ ] 6.2 Have `crates/flume-conformance/src/clauses.rs` read the revision of agentplot/blueprints main that holds 24d36f8 and the amendment at cffcbcc: 210 in Part B, S120 among the scenarios, 209 out of Part A and `RENAME` out of the unbuilt list since main took 209 out at b0df70b, and `REVISION` and its tests moved to match, as the clause-coverage spec has it (106; core.19). Where the remote's main holds neither, stop short naming it. True when `devenv shell -- cargo test -p flume-conformance --lib clauses` and `devenv shell -- cargo run -q -p flume-conformance --bin coverage` pass, 210 and S120 cited by this change's scenarios.

## 7. The kinds profile

- [ ] 7.1 Move `weir-kinds`' pin in `Cargo.toml` to the revision of weir's main that lands W2, and have `map_kind` in `crates/flume/src/profiles/kinds/weir/mod.rs` map weir's two fields of a kind with no stages onto `Kind::fix` and `Kind::fix_limit` in weir's spelling; add the kinds-profile spec's "A train naming mend over a scratch repository" to `crates/flume/tests/profiles/kinds_weir.rs`, and give `site/src/content/docs/guides/kinds.mdx`'s kind with no stages the kind of its fixes and its bound in weir's spelling (4, 23, 157; weir 52, 57, 89; core.4, core.22, core.35). Where W2 has not landed on weir's main, stop short naming it. True when `devenv shell -- cargo test -p flume --test profiles kinds_weir` and `devenv shell -- sh -c 'cd site && vale src/content/docs/guides/kinds.mdx'` pass.
