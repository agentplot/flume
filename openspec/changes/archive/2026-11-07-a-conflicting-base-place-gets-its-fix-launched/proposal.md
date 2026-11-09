# A conflicting base place gets its fix launched

## Why

On 2026-11-07 two of this repository's proven bolts,
`a-fix-round-that-finds-work-asks` and
`flume-codes-a-change-with-one-agent`, were forecast to conflict with
main for 9h 46m and 2h 14m while nothing launched the fix that takes
main in. The user, relayed by agentplot-operator the same day:

> Why couldn't the fix have been launched? That is my fucking point and
> you know that

> After we rework Boomdrive and Flume, the fixes should be automatic
> because Flume will settle. At least according to the diagrams, it
> should all be in the Rust code. It should detect and launch a new fix

> Just make sure it's in the fucking design because it should be.

Record 0056 decides it, accepted by the user on their card on
2026-11-07, and the statement carries it in 1, 203 and I6, accepted at
agentplot/blueprints 24d36f8 and on its main at 31f9f1c. Today a base
place, a bolt under
record 0051, waits on its caller after a failed or conflicting
submission (`submit.rs`, the base place's arm of the outcome), and
nothing forecasts a waiting base place against its base at all, so a
bolt that main moved past is found only when its caller submits it.
A place with stages already goes back to its fix round on its own (112,
131). A base place has no stage to go back to.

## What Changes

- **A base place's kind names its fix (23).** A kind that lands with no
  stages may name the kind its fixes run as, and how many fixes it runs
  before its place waits on its caller. flume reads both through
  weir's crate (core.4), which carries them once weir's unit
  a-kind-with-no-stages-names-its-fix (W2) lands, and flume's follow-up
  unit a-base-places-fix-kind-is-read-from-weirs-format maps them after
  it; until then a kind in weir's format names no fix, and its base
  place waits on its caller as before (203). This change builds every
  part but that mapping over kinds built in Rust. A base place's launch
  resolves the kind named as a launch's
  name resolves (154), and the launch is refused whole where it resolves
  to no kind flume could launch on the branch for it (4; core.8). The
  base place keeps the identity it resolved (21).
- **A settle forecasts a waiting base place (203, 210).** The merge
  queue seam gains forecast: the conflicts a branch's head would meet
  were it submitted to its base now, placed after every entry already
  submitted, creating no entry. A live base place whose kind names a fix,
  with no place on its branch being made or live and no submission open,
  is forecast at each settle. sluice answers it by `sluice submit <base>
  --check --head <branch> --json` in the repository (C.6; sluice 64).
- **A conflict with the base, or a failed or conflicting submission,
  launches the fix (203).** The settle that reads it launches a place of
  the fix kind on the base place's branch, named `<name>-<n>` for the
  base place's name and the fix's number, in its stream, at its
  priority, with its recipients and named intent, its brief the merge
  queue's words under one line saying what the fix is for. For a
  conflict, that line asks the fix to take the base's revision in by a
  merge (112). A forecast that conflicts only with entries queued ahead
  launches nothing, and is forecast again at the next settle.
- **One at a time, once each, within the bound (203).** While a fix is
  live it is a place on the branch, so the base place waits on it and
  nothing is forecast or launched. flume acts on each forecast, by its
  head and the base's revision, and on each submission once. Each fix
  counts toward the kind's bound, a refused launch among them, past
  which the base place waits on its caller, said and delivered (130,
  43).
- **The submission stays the caller's (203; core.6).** A base place is
  still submitted only on its caller's retry. A fix that landed leaves
  it forecast again, and waiting on its caller where the forecast is
  clear.
- **Status shows it (56).** A base place gives the kind of its fixes,
  the newest forecast a settle read and each fix launched for it, why
  and how it ended. A fix gives the base place it was launched for, and
  its recipients as its base place's. The JSON form only gains fields
  (61; core.14).
- **The statement and the rules follow.** The statement's amendment,
  worded in design 13, is on agentplot/blueprints' main at 31f9f1c: 23,
  56, 157, 165, 203, the new Part B clause 210, C.6, S112 and the new
  S120, with weir 52, 57, 89 and C.4, under record 0056's amendment,
  settled by agentplot-design as mechanism within the decision the user
  accepted. Rules core.53 and core.34 and the details of core.6,
  core.34, core.36 and core.53 are amended, mirrored on the site's rules
  page, and record 0056 gains that amendment, naming what this design
  decides beyond it; this change's construct wrote each, and its build
  holds them.

## Capabilities

### New Capabilities

None. The behaviour belongs to `base-places`.

### Modified Capabilities

- `base-places`: the kind of a base place's fixes resolved at its
  launch; the forecast at each settle; the fix launched on a conflict
  or a failure, one at a time, once each, within the bound; a base place
  whose kind names no fix waits on its caller as before.
- `place-status`: a base place's fix kind, forecast and fixes, and a
  fix's base place.
- `seam-contracts`: the merge queue's forecast and its types, and the
  held types of a base place's fixes.
- `seam-fakes`: the fake merge queue scripts a forecast.
- `merge-queue-profile`: sluice forecasts a base place's branch.
- `clause-coverage`: the suite reads the statement's revision holding
  210 and S120.

## Impact

- `crates/flume-seams`: `MergeQueue::forecast` with `Forecast`;
  `Kind::fix` and `Kind::fix_limit`; `PlaceRecord::fixes_as` and
  `PlaceRecord::fix_of`; `HeldState::forecast` and `HeldState::fixes`;
  `RecipientsFrom::BasePlace`; round trips.
- `crates/flume-core`: the base place's launch resolving its fix kind;
  the settle's forecast and fix launch, by `make_place`, inside the
  settle; the failure's fix in `submit.rs`; a fix's end written onto its
  base place; status's new fields.
- `crates/flume-fakes`: the fake merge queue's forecast.
- `crates/flume-conformance`: base places' scenarios, S112 as amended
  and S120, the seam's forecast scenarios, and the clause list.
- `crates/flume`: the sluice profile's forecast; status in the person's
  form; tests over scratch repositories and against the pinned sluice.
- `site/`: the merge-queue guide's base place failure, the state
  reference, and the rules page.
- `docs/`: record 0056's amendment; rules core.53 and core.34, and the
  details of core.6, core.34, core.36 and core.53.
- agentplot/blueprints: main at 31f9f1c, holding 1, 203 and I6,
  accepted at 24d36f8, and flume 23, 56, 157, 165, 203, 210, C.6, S112
  and S120 with weir 52, 57, 89 and C.4. The clause list reads that
  revision.
- agentplot/weir: W2, the two fields of a kind with no stages, queued
  as weir's unit a-kind-with-no-stages-names-its-fix; then flume's
  follow-up unit a-base-places-fix-kind-is-read-from-weirs-format moves
  weir-kinds' pin to the revision that lands it and maps both in the
  kinds profile.
- afterthought/crew: follow-up C2, its bolt kind naming its fix, and its
  own bolt-against-main forecast and take-main fix retired.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency or tool. The pinned sluice, 0.71.0, already forecasts a
  named head.
