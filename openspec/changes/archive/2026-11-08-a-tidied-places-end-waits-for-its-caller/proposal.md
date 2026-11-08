# A tidied place's end waits for its caller

## Why

The runner launches each place its control plane asks for and carries
the place's status back on every pass, and crew reads a place's end
only from that report (crew record 0056). Once a place is tidied, the
runner can read it only from the document flume keeps while an end of
it stands pending (62). Nothing of a place ever stood pending for the
runner, so that document was gone before the runner's next pass.

One settle reads a landing and tidies the place: the submission's step
makes the work over, the next step tidies it, delivers its ends and
keeps the document only for the recipients still owed one
(`settle_place`, `tidy_place`, `keep_for_ends`). The runner names the
work's role its recipient and runs `flume` with `FLUME_RECIPIENT`
unset, so the landing's notice reaches the role's run and the document
goes inside the settle that read the landing. Fix agentplot/485 went
this way on 2026-11-08 between two passes a minute apart, and crew read
it as submitted. A drop records no end, so a dropped place's document
goes at its tidy whatever its recipients.

The answer to the runner's question of 2026-11-08 was option 1: flume
keeps a launched place's tidied document for its caller until the
caller collects it (43, 62). Record 0060 decides how, and the statement
says it at agentplot/blueprints `design/flume-a-tidied-places-end-waits-for-its-caller`,
b08a773 (7, 8, 43, 56, 62, the runner's row, S121).

## What Changes

- **A launch may name its caller (7, 8).** `flume launch --caller
  <name>` keeps the name with the place, fixed by the launch and given
  by status. It is no owner (69), changes none of the place's
  recipients, and a launch that names none has none.
- **The end of a place's work is its caller's (43).** Its landing, and
  its drop or its done, stand pending for the caller until the caller
  collects them by `flume inbox` or a wait as itself. No notice is sent
  to the caller, a notice to a recipient of the same name does not count
  for it, and the caller's end wakes no role.
- **A drop or a done is an end (43, 52, 114).** It is recorded as an end
  keyed `over/dropped` or `over/done`, for the caller alone: the place's
  recipients are sent no notice of it and no role is woken for it.
- **The tidied document stays for the caller (62).** The tidy keeps a
  place's document while an end of it stands pending for a recipient or
  for its caller, and status gives the caller among those it waits on.
- **An inbox may name a place (43).** `flume inbox <place>` collects the
  recipient's ends of the documents `flume status <place>` reads under
  that name alone, and gives that place's waiting entry alone, so a
  caller collects what it has read and nothing tidied since.
- **Rule core.25** carries the caller's end (record 0060, proposed); the
  site's commands, state and callers pages say it.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `seam-contracts`: the held state holds a place's caller and an end of
  its work by a drop or a done, with its key and whom it is pending for.
- `launching`: a launch names its caller, kept and given by status.
- `end-delivery`: the end of a place's work waits for its caller, a drop
  or a done is an end for the caller alone, the tidied document stays
  for it, and an inbox may name a place.
- `command-line`: `flume launch --caller` and `flume inbox [<place>]`.

## Impact

- `crates/flume-seams`: `held.rs` (`PlaceRecord::caller`, `EndOf::Over`,
  `EndKey`'s `over/<dropped|done>`, whom an end is pending for),
  `launch.rs` (`Launch::caller`), `round_trip.rs`.
- `crates/flume-core`: `held.rs` (`record_ends`), `ends.rs`
  (`what_ended`, `keep_for_ends`), `wake.rs` (`wake_roles`), `wait.rs`
  (`inbox` by place, the caller's pending), `status.rs` (the documents
  read under a name, the caller), `answer.rs` (`PlaceStatus::caller`,
  `EndStatus::of`), `launch.rs`.
- `crates/flume`: `commands/mod.rs` (`--caller`, `inbox <place>`),
  `commands/print.rs` (the caller in the person's form), tests in
  `tests/commands.rs`.
- `crates/flume-conformance`: the launching and end-delivery scenarios.
- `docs/`: record 0060, proposed; rule core.25 and its details, mirrored
  in `site/src/content/docs/architecture/rules.md`. The site's
  `reference/commands.md`, `reference/state.md` and
  `guides/callers-and-holds.md`.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency or tool. The JSON form gains fields and values and loses
  none (61). Boomdrive's runner passing `--caller` and collecting is its
  own change, `the-runner-carries-each-places-end`, which waits on this
  one.
