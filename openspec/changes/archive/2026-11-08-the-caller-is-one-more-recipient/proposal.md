# The caller is one more recipient

## Why

The bolt `a-tidied-places-end-stays-readable` keeps a tidied place's
document readable for the runner, which carries each place's status
back to crew. Its change `a-tidied-places-end-waits-for-its-caller`
built a caller apart from the recipients, `flume launch --caller`, the
end of the place's work pending for that name alone, by a rule of its
own. The user answered agentplot-design's card on 2026-11-08: "Runner
as one more recipient now; one shared inbox in weir next." Record 0060
is rewritten to that answer, and the statement's amendment with it, at
agentplot/blueprints `design/flume-a-tidied-places-end-waits-for-its-caller`
at 479a6ab (43, the runner's row, S121).

## What Changes

- **No caller (7).** A launch names no caller and `flume launch` takes
  no `--caller`. A reader names itself a recipient beside the others its
  launch names; the runner names itself beside the role's address it
  names, so no default is dropped that the address had not dropped.
- **A recipient no pane holds collects its ends (43, 62).** It is sent
  no notice, as a recipient whose session is gone is not, and each end
  stays pending for it, the tidied document with it, until it collects
  the end by `flume inbox` or a wait.
- **A drop or a done is an end no notice carries (43, 52, 114, 132).**
  Its end, `over/dropped` or `over/done`, is recorded as before. No
  notice is sent of it and no role is woken for it. It stands pending
  for each recipient of the place that is no role's address, and for no
  one where every recipient is one, so no tidied document is kept for a
  role alone.
- **`flume inbox <place>` stays**, so a reader collects only what it
  read under that name.
- Rule core.25 and its mirror on the site, and the site's commands,
  state and callers pages, say it.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `seam-contracts`: the held state holds no caller; a drop's or a
  done's end is pending for the place's recipients that are no role's
  address, and no notice carries it.
- `launching`: a launch names no caller; a recipient it names may be a
  name no pane holds, beside a role's address.
- `end-delivery`: a recipient no pane holds collects its ends; a drop
  or a done is an end no notice carries; the tidied document stays for
  any recipient an end is pending for.
- `command-line`: `flume launch` takes no `--caller`; `flume inbox`
  takes a place.

## Impact

- `crates/flume-seams`: `held.rs` (`PlaceRecord::caller` gone,
  `EndHeld::recipients` for an over end, `EndHeld::noticed`, the
  caller's owing gone), `launch.rs` (`Launch::caller` gone),
  `round_trip.rs`.
- `crates/flume-core`: `ends.rs` (`deliver_ends` skips an end no
  notice carries), `wake.rs`, `wait.rs`, `status.rs`, `answer.rs`
  (`PlaceStatus::caller` gone, `EndStatus::of`), `launch.rs`.
- `crates/flume`: `commands/mod.rs` (`--caller` gone),
  `commands/print.rs`, `tests/commands.rs`.
- `crates/flume-conformance`: the launching, end-delivery and answers
  scenarios.
- `docs/`: record 0060 rewritten, proposed; rule core.25 and its
  details, mirrored on the site's rules page. The site's
  `reference/commands.md`, `reference/state.md` and
  `guides/callers-and-holds.md`.
- `caller` never reached main, so the JSON form main gives loses no
  field (61). No new crate, plugin, site part, dependency or tool.
  Boomdrive's runner naming itself a recipient is its own change,
  `the-runner-carries-each-places-end`.
