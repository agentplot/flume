# Verify: flume-opens-a-streams-multiplexer

Read on 2026-11-05 at `3d31bb4`, the branch
`change/flume-opens-a-streams-multiplexer`, against the change's
proposal, design, twelve delta specs and tasks.

## Verdict

**Falls short; not archived.** Every task is ticked and the gate passes
as the tasks hold it, but one scenario's THEN is not built:
`stream-terminals`' "A place in two terminals" asks that the tidy's
answer name both terminals, and the answer has no field that names one.
A second scenario, `seam-fakes`' "A scripted window", is run by no test
in the way its WHEN gives it. Five more scenarios have a THEN that no
test asserts. The change goes back to its code stage for these.

## Tasks

All 25 tasks of groups 1 to 8 are ticked. `openspec validate
flume-opens-a-streams-multiplexer --strict` passes. Record 0037 is
`accepted` and quotes the user's words of 2026-11-05. core.39, core.48
and core.49 in `docs/architecture/core.md` are mirrored word for word
in `site/src/content/docs/architecture/rules.md`. `git grep
'view-and-terminal[-]app' -- docs openspec` finds nothing (1.4).

## Checks

`sluice exec main` at `3d31bb4`: 35 of 36 tasks passed, among them
seams, core, fakes, conformance (test and coverage), flume:test, the
plugins, site:prose, site:build, root:format, root:adrs, root:changes
and root:specs. `flume:tools` failed with 76 passed and 10 failed. Each
of the 10 panicked on its socket's path alone, as the tasks record:
`sluice exec`'s scratch folder puts a named herdr session's socket at
117 to 124 bytes, past the host's 103. These are the 10 named-session
tests that tasks 6.3, 6.5, 6.6 and 8.2 hold by a direct run on the
operator's ruling of 2026-11-05. The same tests run directly in the
devenv shell at `3d31bb4`: see "The direct run" below.

## Scenarios

Each scenario of the twelve delta specs was matched to the test that
holds it, and the test's assertions were read against each THEN. A
scenario not listed under "What fails" or "What is held only in part"
is held in full by the test named.

- **stream-terminals**, `crates/flume-conformance/src/engine/stream_terminals.rs`:
  two streams under two terminals, a stream naming none, a multiplexer
  changed while a change runs, a start written before (with
  `print.rs`'s unit test giving the host's terminal), a caller under a
  stream's terminal, one stream with no watch, a stream naming `rex`, a
  stream changed to `rex` while it waits, the terminals in status.
- **stream-views**, `crates/flume-conformance/src/engine/open.rs`: every
  scenario, `s90_a_streams_view` beside them. `Engine::open_stream`
  (`crates/flume-core/src/open.rs`) takes no lock, writes no held state
  and settles nothing, and `two_askings` asserts it.
- **window-profiles**, `crates/flume/tests/profiles/window_ghostty.rs`,
  `window_terminal.rs`, `window_printed.rs`, `crates/flume/tests/commands.rs`
  and `crates/flume/src/host/config.rs`: every scenario.
- **terminal-profiles**, `crates/flume/tests/profiles/tools/terminal_herdr.rs`:
  herdr's five scenarios.
- **seam-contracts**: each seam's operations, no tool named outside a
  profile (`ghostty` and `osascript` among the words, and none found
  outside a `profiles` directory), named terminals and windows, the
  round trip in `crates/flume-seams/src/round_trip.rs`.
- **seam-fakes**: a scripted attach and a scripted display,
  `crates/flume-fakes/tests/fakes.rs`.
- **streams**: the word written before, a view no word names, a title
  that is no text (`config.rs`), and the display in status
  (`engine/streams.rs`).
- **host-configuration**: a stream's multiplexer set locally and a
  stream's view set locally (`config.rs`). `LOCAL_KEYS` gains
  `streams.*.multiplexer` alone.
- **launching**: a stream's multiplexer flume does not carry
  (`engine/launching.rs`), and through the binary in `commands.rs`.
- **settling**: a watched host with one stream unwatched
  (`engine/settling.rs`).
- **command-line**: open under the printed command and of an undeclared
  stream (`commands.rs`), and the three through the binary
  (`crates/flume/tests/profiles/tools/open.rs`).
- **clause-coverage**: `crates/flume-conformance/src/clauses.rs` reads
  6188a10, as design 16 says. Its Part A, Part B, S1 to S91 and its
  unbuilt list match the spec's list entry for entry, read by hand.
  conformance:coverage passes.

## What fails

1. **stream-terminals, "A place in two terminals": the tidy's answer
   does not name the terminals.** The THEN: "its tidy asks tidy of
   `other` and of `fake`, each once, and the tidy's answer names both".
   `TerminalTidied` (`crates/flume-core/src/answer.rs:847`) holds only
   `place`, `closed` and `kept`. The test
   (`a_place_in_two_terminals`, `stream_terminals.rs:435`) asserts one
   tidy of each terminal. It then asserts two entries whose `closed`
   lists are identical, "the tab pair--one in trial" and "the
   workspace repository main in trial", so nothing in the answer says
   which terminal closed or kept what. Either `TerminalTidied` gains
   the terminal's profile, which only adds a field (core.14), printed
   in the person's form and asserted in the test, or the spec's THEN
   changes through this change.

2. **seam-fakes, "A scripted window": no test asks the core.** The
   WHEN scripts the fake window to answer printed with `attach work`
   "and asks the core to open `work`". The THEN: "the core's answer
   gives that line, and the fake window's log names one show under the
   stream's title". `a_scripted_window` (`fakes.rs:1179`) calls
   `window.show` directly with no core. The core's printed line is
   asserted in `engine/open.rs` (`a_command_that_needs_a_variable`,
   `s90_a_streams_view`), but there the window's log is not read.
   No one test holds both THENs through the core.

## What is held only in part

Each is a THEN that no test asserts. The behaviour looks right in the
code; the scenario is still not shown met.

3. **streams, "The keys and none"** (`config.rs:2109`): office's
   title `Office Herdr` from `stream`, and personal's title `personal`
   from `stream-name`, are not asserted. The test reads the raw
   `title` alone (`Some(..)` and `None`). `engine/streams.rs` asserts
   the title's source over the fake host, not from this TOML.
4. **terminal-profiles, "Attach under the printed command"**
   (`terminal_printed.rs:138`): "and runs nothing" is not asserted.
5. **seam-fakes, "A scripted title"** (`fakes.rs:1215`): status is
   never asked. The THEN is status giving `work`'s title `Work Agents`
   from `stream`; the test reads `host.streams()` alone.
6. **seam-contracts, "Show over the fake"**
   (`crates/flume-conformance/src/seams/window.rs`): "logging two shows
   with the command and the title" is asserted nowhere. The scenarios
   read `FakeWindow::windows()`, a list apart from the log, and
   `fakes.rs` logs one printed show. **"Attach over the fake"**
   (`seams/terminal.rs:215`) reads no log and checks its commands only
   as distinct and non-empty; `fakes.rs:1136` asserts both exactly, so
   that one is held across the two files.
7. **stream-views, "A command that needs a variable"**
   (`engine/open.rs:505`): "in its JSON form and the person's" is
   asserted in the JSON alone. The person's form is held only across
   `print.rs`'s unit test and `tools/open.rs:322`. Neither runs this
   scenario's answer through the person's form.

## Observations

None of these fails a THEN; each is for the code stage to weigh.

- **The printed window quotes `=` in its program word.**
  `crates/flume/src/profiles/window/printed/mod.rs:27` quotes the
  program by `shell::quote`, whose set leaves `=` out, where the
  window-profiles requirement gives `-_./:=@%+,`. It reads as
  deliberate, since after `NAME=<set>` a shell would read `a=b` as
  another assignment. No test holds it, and neither the spec nor the
  design says so.
- **`<name>=<set>` pasted into a POSIX shell.** A shell reads
  `XDG_CONFIG_HOME=<set> /bin/herdr …` as an empty assignment, `<set`
  and `> /bin/herdr`. Where a file `set` stands in the folder and the
  program is writable, the program is truncated. The form is core.30's
  and the spec's, not this build's own; the person's line may want
  words that no shell reads as a redirection.
- **`window_ghostty.rs:232`** sets `HERDR_ENV` in the test process and
  never clears it, so it leaks into the rest of the `profiles` test
  binary.
- **The ten-second bounds** of the ghostty and terminal profiles are
  held by no test; every test sets two seconds.
- **`run_bounded`** (`crates/flume/src/host/process.rs`) joins the
  error reader with no bound once the child exits, so a grandchild
  holding the pipe would hold `show` past its bound.
- **The clause list's tests** assert its counts, a sample of members
  and the owner of 184, 185, S88 and S89, not the list entry for entry.
  A swap that kept the counts would pass. The list itself matches the
  spec today.

## The direct run

`devenv shell -- cargo test -p flume --features tools --test tools --
open:: attach herdrs_app own_session s82 two_sessions
the_herdr_profile_passes`, run at `3d31bb4` on 2026-11-05 at 21:41
UTC: 11 passed, 0 failed, in 154.39 s. These are the 10 that failed
under `sluice exec` on their sockets, and `a_name_herdr_refuses`. The
tools layer holds as the operator ruled; the landing's gate stays its
proof.
