# Verify: flume-opens-a-streams-multiplexer

Read on 2026-11-05 at `6f92dc8`, the branch
`change/flume-opens-a-streams-multiplexer`, against the change's
proposal, design, twelve delta specs and tasks, with main at `c9d58df`.

## Verdict

**Holds; archived.** Every task is ticked, every scenario of the twelve
delta specs is held by a test that asserts its THEN, the archive leaves
`openspec validate --specs --strict` passing, and the gate passes at
`6f92dc8` as the tasks hold it, `flume:tools`'s named-session tests by a
direct run as the operator ruled on 2026-11-05. What the report at
`70dc992` found is mended: the local layer's requirement is split in
two, each under openspec's 500 characters, and main at `c9d58df` is
merged.

## Tasks

All 33 tasks of groups 1 to 10 are ticked. `openspec validate
flume-opens-a-streams-multiplexer --strict` passes. Record 0037 is
`accepted` and quotes the user's words of 2026-11-05. core.1, core.13,
core.39, core.48 and core.49 in `docs/architecture/core.md` are mirrored
word for word in `site/src/content/docs/architecture/rules.md`. `git
grep 'view-and-terminal[-]app' -- docs openspec` finds nothing (1.4).

## The archive

On a copy of the tree's `openspec/`, `openspec archive
flume-opens-a-streams-multiplexer --yes` applied 32 requirements added
and 7 modified, and `openspec validate --specs --strict`, the command
`root:specs` runs, passed 41 of 41.

Of the seven requirements the change modifies, main changed two after
`70afbf8`, the revision the deltas were written on: clause-coverage's
"The suite knows the statement's clauses" and "A clause cited by nothing
fails the suite". The delta of each holds main's body and replaces only
the scenario main's change wrote for itself, so the archive drops
nothing main built. The other five stand on main as they stood at
`70afbf8`.

## Checks

`sluice exec main` at `6f92dc8`, run on 2026-11-05 against main at
`c9d58df`, which origin's main still is: 37 of 38 tasks passed, among
them seams, core, fakes, conformance (lint, test and coverage),
flume:format, flume:lint, flume:test, the plugins, site:prose,
site:build, root:format, root:adrs, root:changes, root:specs (38 of 38
before the archive) and root:hooks. `flume:tools` passed 83 and failed
10, each panicking on its socket's path alone: `sluice exec`'s scratch
folder puts a named herdr session's socket at 118 to 124 bytes, past
this host's 103. These are the 10 named-session tests that tasks 6.3,
6.5, 6.6, 8.2, 9.4 and 10.3 hold by a direct run on the operator's
ruling of 2026-11-05; that run is under "The direct run" below. No
conflict with main: the branch holds `c9d58df`.

## Scenarios

The report at `70dc992` matched each scenario to the test that holds it
and read the test's assertions against each THEN; that reading stands,
with what follows read again at `6f92dc8`.

- **The merge kept every test.** Each function the branch added over
  `70afbf8` before the merge is among those it adds over `c9d58df`
  after, but one test renamed for main's list: the unbuilt clauses' test
  now names records 0021 to 0029's and 183, since main built 0035's and
  0036's. The functions the merge adds beside them are main's usage
  window's, renamed `UsageWindow` (10.1).
- **stream-terminals**: `a_place_in_two_terminals` still asserts one
  tidy of `other` and one of `fake`, and the answer's terminals as
  `(Some("other"), …)` and `(Some("fake"), …)` with what each closed.
- **host-configuration**: the modified requirement "The local layer
  sets only C.7's keys" holds main's four scenarios unchanged and adds
  "A stream's multiplexer set locally", held by `config.rs`'s
  `a_streams_multiplexer_set_locally`: the local layer taken,
  `trial`'s multiplexer from `stream` in the local layer and the host's
  from the managed. The added requirement "A stream's view, terminal app
  and title are the managed layer's" is held by
  `a_streams_view_set_locally`, the layer not taken and
  `streams.work.view` named, and by `a_terminal_app_set_by_the_local_layer`,
  which names `streams.work.title` and `host.multiplexer` each, the
  latter with a stand-in profile's name where the scenario writes
  `herdr`, since the key is what is refused. `LOCAL_KEYS` gains
  `streams.*.multiplexer` alone.
- **clause-coverage**: `clauses.rs` reads `6188a10`, a later revision of
  blueprints main than `36cfd06`, as the scenario allows. Part A holds
  180 to 186 and 190, Part B 187 to 189, the scenarios S1 to S91, and
  the unbuilt list records 0021's, 0022's, 0024's plan stage's, 0025's,
  0026's, 0028 and 0029's and 183, as the delta's scenario lists them;
  184, 185 and S86 to S89 are named built, their changes having landed.

## Observations

None of these fails a THEN or a requirement; each stands for a later
change to weigh. Each was found at `70dc992` and holds at `6f92dc8`.

- **The printed window quotes `=` in its program word.**
  `crates/flume/src/profiles/window/printed/mod.rs` quotes the program
  by `shell::quote`, whose set leaves `=` out, where the window-profiles
  requirement gives `-_./:=@%+,`; its arguments go through
  `shell::argument`, which keeps `=` plain. No test holds a program word
  with `=`.
- **`<name>=<set>` pasted into a POSIX shell** reads as an empty
  assignment, `<set` and `> <program>`. The form is core.30's and the
  spec's, not this build's own; a change to it starts in the statement.
- **`window_ghostty.rs:232`** sets `HERDR_ENV` in the test process and
  never clears it, so it leaks into the rest of the `profiles` test
  binary.
- **The ten-second bounds** of the ghostty and terminal profiles
  (`BOUND` in each `mod.rs`) are held by no test; every test sets two
  seconds.
- **`run_bounded`** (`crates/flume/src/host/process.rs`) joins the
  error reader with no bound once the child exits, so a grandchild
  holding the pipe would hold `show` past its bound.
- **The clause list's tests** assert 1 to 190 but 100 and 150 between
  the parts, each part's count and a sample of its members, and the
  unbuilt list's count with every built clause absent from it, not each
  part or each unbuilt clause's owner entry for entry; two clauses
  swapped between the parts outside the sample, or two owners swapped,
  would pass. The list itself
  matches the spec.

## The direct run

`devenv shell -- cargo test -p flume --features tools --test tools --
open:: attach herdrs_app own_session s82 two_sessions
the_herdr_profile_passes`, run at `6f92dc8` on 2026-11-05 from 22:55
UTC: 11 passed, 0 failed, in 165.50 s. These are the 10 that failed
under `sluice exec` on their sockets, and `a_name_herdr_refuses`. The
tools layer holds as the operator ruled; the landing's gate stays its
proof.
