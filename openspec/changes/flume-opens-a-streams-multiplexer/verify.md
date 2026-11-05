# Verify: flume-opens-a-streams-multiplexer

Read on 2026-11-05 at `b618a99`, the branch
`change/flume-opens-a-streams-multiplexer`, against the change's
proposal, design, twelve delta specs and tasks, with main at `c9d58df`,
which origin's main still is.

## Verdict

**Falls short; not archived.** Every task is ticked, every scenario of
the twelve delta specs is held by a test that asserts its THEN, and an
archive on a copy of the tree leaves `openspec validate --specs
--strict` passing 41 of 41. Group 11's mend holds: under the place's
short scratch every named-session test ran and passed. The gate does
not pass. Under `sluice exec main`, `flume:tools` failed one test,
`terminal_herdr::a_watchs_setup_is_given_by_status_while_its_lease_is_fresh`.
The race behind it is in flume's herdr plugin's watch, and sluice now
holds that fail under `flume:tools`' key for this content. A submission
of this content would read the check flaky, as #108 did.

## What fails

`sluice exec main` at `b618a99`, run on 2026-11-05 from 23:24 UTC: of
its 37 checks, 36 passed and `flume:tools` failed, 92 of its 93 tests
passing in 261.96 s (moon: 37 completed, 1 failed). sluice recorded all
37 results under their keys. The failing assertion is the test's last,
`terminal_herdr.rs:2757`. After `drop(watch)`,
`flume status --json` still gave the host's terminal setup as the two
lines the watch wrote, where the test expects none:

```text
left: Array [String("herdr's configuration, …/flume-test-setup-63379-58/herdr.toml, binds no key to flume's places: …"), String("herdr's configuration, …/herdr.toml, shows no row of flume's in the sidebar: …")]
right: Array []
```

The test and the watch are main's (`f3172a17`), and this change leaves
both as main has them. The test passed in group 11's run and in #108's.
It fails when the timing falls wrong, as it did under this run's load.

**The cause, read from the code.** `plugins/herdr/bin/watch.sh` writes
its lease and then `setup.json` (`write_lease`, `setup said`, lines 103
to 106). It starts its renewer, and only then installs `trap 'stop; exit
0' INT TERM` (line 133). The test reads `setup.json` beside a lease as
the watch being up. It checks the words and runs one `flume status`,
then sends SIGTERM. A SIGTERM that comes before line 133 ends the shell
by its default action, so `stop` never runs and the lease is never
removed. The herdr profile's `watched_servers`
(`crates/flume/src/profiles/terminal/herdr/mod.rs`) reads a lease as
fresh by its renewal time alone, for `LEASE_TERM`, a minute, so the
dead watch's setup stays in status. On a host, the same window leaves a
stopped watch read as live for up to a minute.

**What would mend it**, for the code stage to weigh: have the watch
install its trap before it writes the lease, so that every TERM after
the lease is visible runs `stop`. `/plugins/herdr/**` is among
`flume:tools`' inputs (`crates/flume/moon.yml`), so the mend also gives
the check a new key. The fail this run recorded then reaches no
landing. `plugin-herdr`'s own tasks hold the watch under sh and dash.

## Tasks

All 36 tasks of groups 1 to 11 are ticked, and `openspec validate
flume-opens-a-streams-multiplexer --strict` passes. Record 0037 is
`accepted` and quotes the user's words of 2026-11-05. Record 0047 is
`proposed`, as the code stage wrote it, and waits on the user's yes at
the change's review. The rules core.1, core.13, core.39, core.48, core.49
and core.51 in `docs/architecture/core.md` are mirrored word for word
in `site/src/content/docs/architecture/rules.md`. `git grep
'view-and-terminal[-]app' -- docs openspec` finds nothing (1.4).

Group 11, read against its tasks:

- **11.1.** `b2fb0a6` reverts `90373ac` whole. The tree at `b2fb0a6`
  equals the tree at `e5c596d`, and `openspec/specs` at `b618a99`
  equals both.
- **11.2.** `could_not_run` in `crates/flume/tests/profiles/tools/main.rs`
  writes its words to stderr and exits 77. `SessionHome::new` links the
  session home in the first of two folders, the temporary folder and
  `/tmp`, that leaves every named session's socket room and that the
  run may write. Where neither does, it ends the run naming each
  socket's bytes. `TestServer`'s short link and the open's `Host::new`
  do the same where no link can be made. The diff from `b2fb0a6`
  touches these setups alone and no assertion of any test.
  `.config/sluice.toml`'s `[tests]` gains `not_run = [77]`. Record 0047
  and core.51 cite sluice's clauses 19, 35, 56 and 114 to 116 and its
  record 0074.
- **11.3.** Holds for what it names: under the place's scratch, every
  named-session test ran and passed. The gate as a whole fails, as
  "What fails" says.

## The archive

On a copy of the tree's `openspec/`, `openspec archive
flume-opens-a-streams-multiplexer --yes` applied 32 requirements added
and 7 modified, and `openspec validate --specs --strict`, the command
`root:specs` runs, passed 41 of 41. Nothing is archived on the branch.

## Checks

The other 36 checks of `sluice exec main` at `b618a99` passed. Among
them are seams, core, fakes and conformance (lint, test and coverage),
flume:format, flume:lint and flume:test, the plugins' tasks, site:prose,
site:build, root:format, root:adrs, root:changes, root:specs and
root:hooks. `flume:tools` ran all 93 of its tests, the 11 that start a
named herdr session among them, and 92 passed.

## Scenarios

The reports at `70dc992` and `6f92dc8` matched each scenario to the
test that holds it and read each test's assertions against its THEN.
The deltas, the proposal and the design have not changed since
`6f92dc8`. The only code change is group 11's, to the setups of the
tools tests that start a named herdr session, and it changes no
assertion. That reading stands whole:

- **stream-terminals**: `a_place_in_two_terminals` asserts one tidy of
  `other` and one of `fake`, and the answer's terminals as
  `(Some("other"), …)` and `(Some("fake"), …)`, each with what it closed.
- **host-configuration**: the modified requirement "The local layer
  sets only C.7's keys" keeps main's four scenarios unchanged. It adds
  "A stream's multiplexer set locally", which `config.rs`'s
  `a_streams_multiplexer_set_locally` holds. The added requirement "A
  stream's view, terminal app and title are the managed layer's" is
  held by `a_streams_view_set_locally` and
  `a_terminal_app_set_by_the_local_layer`. `LOCAL_KEYS` gains
  `streams.*.multiplexer` and nothing else.
- **clause-coverage**: `clauses.rs` reads `6188a10`, a later revision of
  blueprints main than `36cfd06`, which the scenario allows. Part A
  holds 180 to 186 and 190, Part B holds 187 to 189, the scenarios run
  S1 to S91, and the unbuilt list is as the delta's scenario gives it.
- **terminal-profiles and stream-views through herdr**: the 11 tests
  that start a named session now run under the gate itself, since the
  place's scratch leaves their sockets room. The earlier reports held
  them by a direct run, on the operator's ruling.

## Observations

None of these fails a THEN or a requirement. Each is left for a later
change to weigh.

- **The proposal's Impact names neither `.config/sluice.toml` nor
  record 0047 and core.51.** Group 11 and the tasks' preamble say them;
  the proposal and the design do not.
- **One test's exit ends the whole tools binary.** A test running beside
  the one that exits 77 is cut off with no outcome. Record 0047 weighs
  this and accepts it, and sluice ends the run's process group.
- **The printed window quotes `=` in its program word.**
  `crates/flume/src/profiles/window/printed/mod.rs` quotes the program
  with `shell::quote`, whose set leaves `=` out, where the
  window-profiles requirement gives `-_./:=@%+,`. Its arguments go
  through `shell::argument`, which keeps `=` plain. No test holds a
  program word with `=`.
- **`<name>=<set>` pasted into a POSIX shell** reads as an empty
  assignment, `<set` and `> <program>`. The form is core.30's and the
  spec's, not this build's own; a change to it starts in the statement.
- **`window_ghostty.rs:232`** sets `HERDR_ENV` in the test process and
  never clears it, so it leaks into the rest of the `profiles` test
  binary.
- **The ten-second bounds** of the ghostty and terminal profiles
  (`BOUND` in each `mod.rs`) are held by no test, since every test sets
  two seconds.
- **`run_bounded`** (`crates/flume/src/host/process.rs`) joins the
  error reader with no bound once the child exits. A grandchild that
  holds the pipe would keep `show` running past its bound.
- **The clause list's tests** assert the clauses 1 to 190, skipping 100
  and 150 between the parts, then each part's count and a sample of its
  members, then the unbuilt list's count with every built clause absent
  from it. They do not check each part, or each unbuilt clause's owner,
  entry for entry. The list itself matches the spec.
