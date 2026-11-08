# A base place's fix kind is read from weir's format

## Why

Record 0056 has flume launch a base place's fix when the merge queue
forecasts its branch in conflict with its base or its submission fails,
the fix a place of the kind the base place's kind names, within the
kind's bound (203). The change a-conflicting-base-place-gets-its-fix-launched
built all of it over kinds built in Rust and left the kinds profile as
it stood (its task 7.1), so a kind in weir's format still names no fix
and its base place waits on its caller as before.

weir's format now names both. Record 0005 of weir's, the user's,
accepted on their card on 2026-11-07, "Yes, write it into both
formats", calls the field the kind's landing fix and has it name the
fix kind's whole identity:

> Wouldn't it include the fully qualified change name, including the
> version?

> Should 'fix' be the right term, or should it be something more
> related to the merge queue … since this is specifically for being
> kicked back from that?

A kind that lands with no stages gives `landing_fix`, an identity with
its version such as `flume/fix@1`, and `max_landing_fixes`, its bound.
weir-kinds reads them at weir's main daacda3, which landed
`bolt/weir-kinds-name-their-fix`, and refuses a bare name, an identity
with no version, and either field on a kind with stages. flume's
statement carries it in 23 at agentplot/blueprints 8eec6dd.

## What Changes

- **weir's crate at the revision that reads the landing fix (core.4,
  core.22).** weir-kinds' pin moves to weir's main at
  `daacda39e59469029b2841f889c6cbd8cd7839d9`. Between the pins only
  weir-kinds' reading of the two fields changed.
- **The kinds profile maps both (23; weir 52, 57).** A kind's
  `landing_fix` becomes the seam's kind of its fixes, spelled as weir's
  crate spells the identity, and `max_landing_fixes` becomes the seam's
  bound. Nothing else feeds either field. A bound past what flume counts
  is refused, naming the field. `fix` and `max_fixes` are sections, as
  weir returns them, and flume ignores them.
- **A base place launched under a kind in weir's format resolves its
  fix (21, 154, 203).** The core's resolution already takes a whole
  identity as a launch's name, at the version it names. This change
  proves it through the profile over a scratch repository: `train`
  naming `repo/mend@1` holds `repo/mend@1`, and naming `repo/mend@2`
  while the repository declares `mend` at version 1 refuses the launch
  before the merge queue opens anything.
- **The words follow.** The seam's `Kind::fix` and `Kind::fix_limit`
  say they hold the kind's landing fix as its reader gives it. The kinds
  guide, the configuration reference and the merge-queue guide give
  both fields in weir's spelling, to any host's operator (core.35).
  Record 0056's amendment, core.34 and the details of core.4 and core.34
  name the landing fix by its identity with its version, citing weir's
  record 0005, and the site's rules page mirrors core.34. This change's
  construct wrote those at 15d15448.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `kinds-profile`: a kind with no stages names its landing fix and its
  bound through weir's crate, and a base place's launch resolves it.

## Impact

- `Cargo.toml`, `Cargo.lock`: weir-kinds, and weir-capture beneath it,
  at weir's daacda3.
- `crates/flume`: `map_kind` in the kinds profile, its docs, and
  scenarios in `tests/profiles/kinds_weir.rs`.
- `crates/flume-seams`: the docs of `Kind::fix` and `Kind::fix_limit`.
  No type changes, so nothing held or answered changes shape (21, 61;
  core.14).
- `site/`: the kinds guide, the configuration reference, the
  merge-queue guide and the rules page.
- `docs/`: record 0056's amendment line, core.34, and the details of
  core.4 and core.34.
- No change to the core, the fakes or the conformance suite. The
  engine's base place scenarios run over kinds built in Rust, and the
  clause list stays at 31f9f1c: 8eec6dd adds and removes no clause.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency or tool.
- afterthought/crew: its follow-up C2, a bolt's kind naming its landing
  fix, can now be written in weir's spelling. It's crew's, named in
  record 0056.
