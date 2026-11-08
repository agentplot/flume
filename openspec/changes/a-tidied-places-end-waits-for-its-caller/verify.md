# Verify: a-tidied-places-end-waits-for-its-caller

Verified on 2026-11-08 at `e828e2f` on
`change/a-tidied-places-end-waits-for-its-caller`, cut from
`bolt/a-tidied-places-end-stays-readable` at `81d5722`.

## The whole checks

`devenv shell -- sluice exec bolt/a-tidied-places-end-stays-readable`,
run once, passed: sluice ran the checks `e828e2f` affects against the
base (`81d5722`), the bolt's `#check-fast`. Moon completed 24 tasks in
56s, and sluice recorded 23 results passed under their keys. It exited
0.

| project | tasks passed |
|---|---|
| `seams` | format, test |
| `core` | format, lint, test |
| `fakes` | lint, test |
| `conformance` | format, lint, test, coverage |
| `flume` | format, lint, test |
| `plugin-claude` | test |
| `nix` | typecheck, test |
| `site` | install, prose, build |
| `root` | format, adrs, changes |

Moon warned that it could not write its shared cache's blobs or the
outputs archive under the sandbox. Those are cache writes only, and no
task failed on them. `conformance:coverage` reports every clause of the
suite's list at 9c060ab cited. As the tasks say, S121 is left to the
chore that adopts the revision that holds it.

`openspec validate a-tidied-places-end-waits-for-its-caller --strict`:
valid.

## Tasks

All nine tasks, 1.1 to 4.1, are ticked done. Each task's named test
ran in the gate and passed:

- 1.1 `round_trip::a_held_state_with_a_caller` (seams:test)
- 1.2 `round_trip::every_ends_key_round_trips` and
  `round_trip::a_key_in_another_form_is_refused` (seams:test)
- 1.3 `round_trip::the_end_of_the_work_is_owed_its_caller` (seams:test)
- 2.1 to 2.3 `the_engine_passes_its_scenarios` (conformance:test,
  `tests/engine.rs`)
- 3.1 `a_callers_place_collected_by_its_inbox` and
  `the_helps_of_needs_hold_retry_tell_and_launch` (flume:test)
- 3.2 `commands::print::tests::a_places_caller` (flume:test)
- 4.1 `site:prose` and `site:build`

## Requirements and scenarios

### seam-contracts

- **The types of a caller and of the end of a place's work.**
  `PlaceRecord::caller` and `Launch::caller` are
  `Option<String>`, skipped where none.
  `EndOf::Over { over }` serializes `{"of":"over","over":"dropped"}`,
  and `EndKey::Over` reads `over/dropped` and `over/done` back from the
  token alone. `over/landed`, `over/`, `over/Dropped` and
  `over/dropped/1` are refused naming `END_KEY_FORMS`, which names the
  new form (`crates/flume-seams/src/held.rs`).
  - *A caller and a drop round-trip*: `a_held_state_with_a_caller` and
    `every_ends_key_round_trips` (the drop's end held, round-tripped,
    with no recipient), and `a_key_in_another_form_is_refused`. Met.
  - *State written before*: `a_held_state_with_a_caller` reads a state
    with no `caller` as none and writes none. Met.
- **Whom the end of a place's work is pending for.**
  `EndHeld::for_caller`, `pending_for_caller` (only an `Inbox` or a
  `Wait` delivery to the caller's name counts), `owed` and
  `pending_to(recipients, caller, recipient)`. `HeldState::pending_for`
  reads `owed`. `pending` stays what a notice and a wake are owed.
  - *The end of the work owed its caller*: the construct and the failure
    are pending for `ops`, the landing for `ops` and `boomdrive`, and
    the drop for `boomdrive` alone. Met.
  - *A caller that is also a recipient*: after a notice to `ops` the
    landing is still owed to `ops`, and once an inbox delivery is
    recorded it is owed to no one. Met.

### launching

- **A launch may name its caller.** The core keeps `Launch::caller` on
  the record (`crates/flume-core/src/launch.rs`). An empty caller is
  refused before anything is made, and status gives `PlaceStatus::caller`.
  - *A caller named*: `a_caller_named` checks the recipient
    `madswan/planner` from the launch, the caller `boomdrive`, and the
    caller unchanged after a hold and a retry. Met.
  - *An empty caller*: `an_empty_caller` checks a refusal that names the
    caller and that nothing was made. Met.
- **A caller is no recipient and no owner.** A wake's launch names no
  caller (`wake.rs`).
  - *No caller named*: `no_caller_named` gives the recipient `ops` from
    `FLUME_RECIPIENT` and no caller. Met.
  - *A woken run has no caller*: `a_woken_run_has_no_caller` checks that
    the run of `east/ops` has none, though the place whose end woke it
    names `boomdrive`. Met.

### end-delivery

- **The end of a place's work waits for its caller.** `deliver_ends`
  (`ends.rs`), `wake_roles` and `pending_addresses` (`wake.rs`) read
  `pending(&recipients)` alone, so no notice goes to the caller and no
  role wakes for its end. The inbox and the wait read `pending_to` with
  the caller (`wait.rs`). `keep_for_ends` and `settle_tidied` read
  `HeldState::pending_for`, which names the caller.
  - *A landing collected by its caller*: `a_landing_collected_by_its_caller`.
    Met.
  - *A caller that is also a recipient*: `a_caller_that_is_also_a_recipient`.
    Met.
  - *A wait as the caller collects*: `a_wait_as_the_caller_collects`
    shows the deliveries `ops` by notice and `boomdrive` by wait, and
    the document removed at the next settle. Met.
  - *A caller written as an address wakes nothing*:
    `a_caller_written_as_an_address`. Met.
  - *No caller named*: `a_landing_with_no_caller`. Met.
- **A drop or a done is an end for the caller alone.** `record_ends`
  (`crates/flume-core/src/held.rs`) records `EndOf::Over` once, in the
  write whose phase is `Over` or `Gone` with `Dropped` or `Done`.
  `what_ended` words it `dropped` or `done`.
  - *A drop*: `a_drop` (recipient `east/planner`, declared with a run).
    Met.
  - *A done*: `a_done` (a kind that does not land). Met.
  - *A drop with no caller*: `a_drop_with_no_caller`. Met.
- **The inbox collects what is pending (modified).**
  `Engine::inbox(recipient, place)` reads the name's documents through
  `documents_of`, the one lookup that status shares. It reads them
  before its settle, collects only their ends, and gives only their
  entries of `waiting`. It is refused, marking nothing, where nothing
  stands under the name.
  - *Collected after a restart*: the existing `collected_after_a_restart`,
    now calling `inbox(.., None)`. Met.
  - *An inbox naming a place*: `an_inbox_naming_a_place`, which also
    names the second place's tidied document as `<location>@<at>`. Met.
  - *A name under which nothing stands*: `a_name_under_which_nothing_stands`.
    Met.
- **A tidied place's document stays while an end is pending
  (modified).**
  - *A landing with its recipient away*, *Dropped while pending* and
    *Launched again*: the existing scenarios still pass. Met.
  - *Kept for its caller*: `kept_for_its_caller`, where the tidied phase
    is pending for `boomdrive` and status gives the caller. Met.
- **An end's recipients (modified).** `EndHeld::recipients` gives an
  over end none. The existing scenarios, *A stop-short that named a
  role* and *A stall of a question for another stream*, still pass.
  Met.

### command-line

- **launch takes a caller and inbox a place.** `flume launch --caller
  <NAME>` and `flume inbox [PLACE]` are in
  `crates/flume/src/commands/mod.rs`. The launch's JSON form carries
  `caller` (`a_launch` round-trips it).
  - *A caller named and its place's end collected*:
    `a_callers_place_collected_by_its_inbox`. Met.
  - *The helps*: `the_helps_of_needs_hold_retry_tell_and_launch` covers
    `--caller <NAME>`, that the end of the place's work waits for it, and
    `inbox`'s `[PLACE]`. Met.

## Beside the specs

- The person's form prints `  caller: <name>` after the recipients'
  line, and an over end as any end's line (`a_places_caller`).
- The JSON form only gains fields and values: `caller` on a place, and
  an end's `over` in the conformance suite's `FIELDS` (61; core.14).
- The site's commands, state and callers pages cover `--caller`,
  `flume inbox [<place>]`, `over/<dropped|done>` and the tidied document
  kept for the caller, written to any host's operator (core.35).
  `site:prose` passed.
- Rule core.25 and its mirror on the site's rules page carry record
  0060, which stays proposed until the operator's yes.
- The inbox's limit to the named place's entries of `waiting` is
  built. No scenario of the change states it on its own, and this report
  counts that as no shortfall.

## Outcome

The whole checks pass and the work holds every requirement and
scenario of the change. The change is archived.
