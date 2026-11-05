# Verify: flume-opens-a-streams-multiplexer

Read on 2026-11-05 at `f2077ab`, the branch
`change/flume-opens-a-streams-multiplexer`, against the change's
proposal, design, twelve delta specs and tasks, with main at `c9d58df`,
which origin's main still is.

## Verdict

**Holds; archived.** Every task is ticked, every scenario of the twelve
delta specs is held by a test that asserts its THEN, and `sluice exec
main` passes all 37 of its checks, `flume:tools` passing 93 of 93. On
the branch, `openspec archive flume-opens-a-streams-multiplexer --yes`
applies 32 requirements added and 7 modified, and `openspec validate
--specs --strict` then passes 41 of 41.

## Tasks

All 37 tasks of groups 1 to 12 are ticked, and `openspec validate
flume-opens-a-streams-multiplexer --strict` passes. Record 0037 is
`accepted` and quotes the user's words of 2026-11-05. Record 0047 is
`proposed`, as the code stage wrote it, and waits on the user's yes at
the change's review. The rules core.1, core.13, core.39, core.48, core.49
and core.51 in `docs/architecture/core.md` are mirrored word for word
in `site/src/content/docs/architecture/rules.md`. `git grep
'view-and-terminal[-]app' -- docs openspec` finds nothing (1.4).

Groups 1 to 11 are as the reports at `70dc992`, `6f92dc8` and `4bfba81`
read them. Since `b618a99`, the branch changes `plugins/herdr/bin/watch.sh`,
`plugins/herdr/tests/run.sh` and the tasks, and nothing else. No crate,
no doc, no site page and no delta has changed.

Group 12, read against its task:

- **12.1.** `089d423` moves `stop` and the watch's `INT`, `TERM` and
  `EXIT` traps ahead of `write_lease`. They are set after the watch takes
  the lock and finds no lease of its own, and before it writes its lease.
  `stop` ends the renewer only where `renewer` is set. It takes the lock
  only where `holds_lock` says the watch does not hold it already. It
  removes the watch's `lease.tmp.$$` and, where the lease's holder is the
  watch, the lease. The renewer's subshell runs `trap - TERM EXIT` first,
  since its `$$` is the watch's and a `stop` of its own would take the
  watch's lease. So a SIGTERM at any moment after the lease can be read
  runs `stop`. A watch ended while it holds the lock gives the lock back.
  `tests/run.sh`'s new test makes `HERDR_CONFIG_PATH` a fifo, so the
  watch blocks in `setup said` once its lease is written. The test sends
  SIGTERM, opens the fifo for writing so the read ends, and asserts the
  lease is gone within ten seconds. On a copy of the plugin with the
  watch as it stood at `b618a99`, that test fails under the system's sh
  and under dash, "the ended watch left its lease". The copy's manifest
  test also failed, as the copy held no `Cargo.toml`; no other test
  failed. `crates/flume/moon.yml` names `/plugins/herdr/**/*` among
  `flume:tools`' inputs, so the check takes a new key.

## Checks

`sluice exec main` at `f2077ab`, run on 2026-11-05 from 23:47 UTC,
checked the branch against main at `c9d58df`. moon completed 38 tasks in
5 min 14 s. sluice said "passed: 37 tasks" and "recorded 1 result under
its key for f2077ab", and exited 0.

- `flume:tools` ran its 93 tests in 290.99 s and passed 93, among them
  `terminal_herdr::a_watchs_setup_is_given_by_status_while_its_lease_is_fresh`
  and the 11 that start a named herdr session.
- `plugin-herdr:test` passed all 34 tests under the system's sh and all
  34 under dash, the new test among them. `plugin-herdr:manifest`,
  `plugin-herdr:lint` (shellcheck) and `plugin-herdr:install` passed.
- seams, core, fakes and conformance passed their format, lint and test
  tasks, and `conformance:coverage` passed. `flume:format`, `flume:lint`
  and `flume:test` passed. The Claude Code plugin's tasks passed.
  `site:prose` and `site:build` passed. `root:format`, `root:adrs`,
  `root:changes`, `root:specs` (38 of 38 before the archive),
  `root:hooks`, `root:layers`, `root:toolchain` and `root:marketplace`
  passed.

## The archive

On the branch, `openspec archive flume-opens-a-streams-multiplexer
--yes` moves the change, this report with it, to
`openspec/changes/archive/2026-11-05-flume-opens-a-streams-multiplexer`.
It adds the specs stream-terminals, stream-views and window-profiles. It
syncs streams, terminal-profiles, seam-contracts, seam-fakes,
command-line, launching, settling, host-configuration and
clause-coverage. That is 32 requirements added and 7 modified, and
`openspec validate --specs --strict`, the command `root:specs` runs,
passes 41 of 41. The same archive on a copy of the tree at `f2077ab`
gave the same counts before it was run here.

## Scenarios

The reports at `70dc992` and `6f92dc8` matched each scenario to the
test that holds it and read each test's assertions against its THEN.
The deltas, the proposal and the design have not changed since
`6f92dc8`. Group 11's code changes the setups of the tools tests that
start a named herdr session and changes no assertion. Group 12 changes
the herdr plugin's watch and adds a plugin test. That reading stands
whole:

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
  that start a named session ran and passed under the gate itself.

## Observations

None of these fails a THEN or a requirement. Each is left for a later
change to weigh.

- **The proposal's Impact names neither `.config/sluice.toml`, record
  0047 and core.51, nor the herdr plugin's watch.** Groups 11 and 12 and
  the tasks' preamble say them; the proposal and the design do not.
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
- **The watch's renewer resets `TERM` and `EXIT` but not `INT`.** A
  POSIX shell resets caught traps in a subshell and an asynchronous list
  ignores `INT` where job control is off. So the renewer never runs the
  watch's `stop`, and the reset is a guard the shells do not need.
