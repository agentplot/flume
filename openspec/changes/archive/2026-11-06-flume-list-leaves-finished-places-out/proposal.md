# flume list leaves out finished places

## Why

`flume list` gives a line for every place flume keeps (record 0049). On
deneb on 2026-11-06, 16 of its 67 lines were places that landed and were
tidied, or were dropped. The user asked for the list to see what is
there to open, and such a place has nothing left to open. Asked
"Should flume list leave finished places out?", the user answered on
2026-11-06: "Yes: leave out landed and dropped places unless --all, as
a follow-up fix after this lands" (flume list landed in 78ea62ea,
released in v0.13.0).

## What Changes

- **The list leaves out a place that landed or was dropped (63, 200).**
  Whatever keeps it, and whether it is being tidied or tidied, its line
  is given only where every place is asked. A place its caller declared
  done, a live one and one being made stay listed.
- **`flume list --all` gives every place,** each in the words the list
  gives it, a tidied place whose document stays among them (62).
- **The answer's form is unchanged (61).** `--all` decides which places
  it holds.
- **Clause 200 and S108 are amended** at blueprints e4fab7b, record
  0049 gains its amendment of 2026-11-06, accepted on the user's yes,
  and rule core.52 is amended with it (core.19).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `the-list`: a place that landed or was dropped is listed only where
  every place is asked.
- `command-line`: `flume list` takes `--all`, and its help names it.

## Impact

- `crates/flume-core`: `Engine::list` takes whether every place is
  asked; `landed_or_dropped`.
- `crates/flume`: `flume list --all`; the command line's tests.
- `crates/flume-conformance`: the list's scenarios ask for every place
  where they read a landed or dropped place, and one scenario reads each
  phase left out.
- `site/`: the commands and state references.
- `docs/`: record 0049's amendment, accepted; rule core.52.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency or tool.
