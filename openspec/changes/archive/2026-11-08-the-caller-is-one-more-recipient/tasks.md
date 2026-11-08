# Tasks

The work is on `fix/a-tidied-places-end-stays-readable--the-caller-is-one-more-recipient`,
cut from `bolt/a-tidied-places-end-stays-readable`. The statement is
read at agentplot/blueprints `design/flume-a-tidied-places-end-waits-for-its-caller`,
at 479a6ab, over b08a773 (43, the runner's row, S121). Record 0060, rule core.25 and
its mirror on the site's rules page are in the first commit. The
suite's list of clauses stays at 9c060ab, so no scenario cites S121.

A fix has no verify stage, so its code stage runs the gate's checks once
at the end, `sluice exec bolt/a-tidied-places-end-stays-readable`, and
archives the change. Each task names the narrowest check that shows it
done. Where a task changes a type other crates match on, its `cargo
check` keeps the workspace building, so each commit passes the
pre-commit hook's clippy.

## 1. The held state and the core

- [x] 1.1 Take `caller` out of `Launch` (`crates/flume-seams/src/launch.rs`) and `PlaceRecord` (`held.rs`), and every literal of them in the tree; take out `EndHeld::for_caller`, `pending_for_caller` and `owed`, give `pending_to` the recipients alone, and have `HeldState::pending_for` read `pending`; have `EndHeld::recipients` give an over end the place's recipients that are no address, each once, and add `EndHeld::noticed`, false for an over end alone (43, 132). In `round_trip.rs`, replace `a_held_state_with_a_caller` by `a_held_state_with_a_caller_reads_without_it` and `the_end_of_the_work_is_owed_its_caller` by `whom_each_end_is_pending_for`, over the seam-contracts spec's "State written with a caller" and "Whom each end is pending for". Check: `devenv shell -- cargo test -p flume-seams` and `devenv shell -- cargo check --workspace --all-targets`.
- [x] 1.2 In `crates/flume-core`: take out the empty caller's refusal and the caller kept in `launch.rs`, a wake's `caller` in `wake.rs`, `PlaceStatus::caller` in `answer.rs` and `status.rs`, and the caller in the wait and the inbox in `wait.rs`; give `EndStatus::of` the recipients alone; have `deliver_ends` in `ends.rs` skip an end not `noticed`, and `wake_roles` read its addresses from `HeldState::pending_for` (43, 52, 62, 114, 132; core.11, core.25, core.28). Check: `devenv shell -- cargo test -p flume-core` and `devenv shell -- cargo check --workspace --all-targets`.

## 2. The conformance suite

- [x] 2.1 In `crates/flume-conformance/src/engine/launching.rs`, take out the four caller scenarios and add the launching spec's "A role's address and a reader beside it"; in `end_delivery.rs`, replace the caller scenarios by the end-delivery spec's "A landing for a recipient no pane holds", "Kept for a recipient no pane holds", "A drop for a role and a reader", "A drop for a role alone", "A drop for a session in a pane" and "A done", and move "An inbox naming a place" and "A name under which nothing stands" onto a recipient no pane holds; have `place_status.rs`'s "a place no longer held is not answered for" collect its drop before it reads status, its recipient owed the drop's end; in `answers.rs`, take the caller out of the full place and its paths out of `FIELDS`, keeping the over end's (7, 43, 52, 61, 62, 114, 132). Check: `devenv shell -- cargo test -p flume-conformance --test engine`.

## 3. The command line

- [x] 3.1 Take `--caller` out of `flume launch` in `crates/flume/src/commands/mod.rs`, and the `caller:` line and `a_places_caller` out of `commands/print.rs`, adding `a_drops_end_line` over an over end's line pending for a recipient; in `tests/commands.rs`, replace `a_callers_place_collected_by_its_inbox` by `a_readers_place_collected_by_its_inbox` and have `the_helps_of_needs_hold_retry_tell_and_launch` say `launch` names no `--caller` and refuses it, over the command-line spec's "A reader's place collected" and "The helps" (7, 43, 61; core.35). Check: `devenv shell -- cargo test -p flume --test commands a_readers_place_collected_by_its_inbox`, `devenv shell -- cargo test -p flume --test commands the_helps_of_needs_hold_retry_tell_and_launch` and `devenv shell -- cargo test -p flume --lib a_drops_end_line`.

## 4. The site

- [x] 4.1 In `site/src/content/docs/reference/commands.md`, take out `--caller`; in `reference/state.md`, take out `caller` and give a drop's or a done's end pending for the recipients that are no role's address; in `guides/callers-and-holds.md`, say how a reader names itself a recipient no pane holds, reads a tidied place's status and collects it by naming the place, and that a drop or a done is an end no notice carries, all to any host's operator (7, 43, 56, 62; core.35). Check: `sluice exec bolt/a-tidied-places-end-stays-readable -- moon run site:prose`.

## 5. The gate and the archive

- [x] 5.1 Run the gate's checks once, `sluice exec bolt/a-tidied-places-end-stays-readable`, then archive the change, `devenv shell -- openspec archive the-caller-is-one-more-recipient --yes`, and check the merged specs with `sluice exec bolt/a-tidied-places-end-stays-readable -- moon run root:specs`.
