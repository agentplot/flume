# Verify: flume-opens-a-streams-multiplexer

Read on 2026-11-05 at `70dc992`, the branch
`change/flume-opens-a-streams-multiplexer`, against the change's
proposal, design, twelve delta specs and tasks.

## Verdict

**Falls short; not archived.** Every task is ticked, every scenario of
the twelve delta specs is held by a test that asserts its THEN, and the
gate passes at `70dc992` as the tasks hold it, `flume:tools`'s
named-session tests by a direct run as the operator ruled on
2026-11-05. But the archive this change asks for fails the gate:
`host-configuration`'s modified requirement, once synced, is past the
length `openspec validate --specs --strict` takes, so `root:specs`
fails on the archived specs. The change goes back to its code stage for
it. See "What fails".

## Tasks

All 30 tasks of groups 1 to 9 are ticked. `openspec validate
flume-opens-a-streams-multiplexer --strict` passes. Record 0037 is
`accepted` and quotes the user's words of 2026-11-05. core.39, core.48
and core.49 in `docs/architecture/core.md` are mirrored word for word
in `site/src/content/docs/architecture/rules.md`. `git grep
'view-and-terminal[-]app' -- docs openspec` finds nothing (1.4).

## Checks

`sluice exec main` at `70dc992`, run on 2026-11-05 against main at
`c9d58df`: 35 of 36 tasks passed, among them seams, core, fakes,
conformance (lint, test and coverage), flume:lint, flume:test, the
plugins, site:prose, site:build, root:format, root:adrs, root:changes,
root:specs (37 of 37) and root:hooks. `flume:tools` passed 76 and failed
10. Each of the 10 panicked on its socket's path alone: `sluice exec`'s
scratch folder puts a named herdr session's socket at 117 to 124 bytes,
past this host's 103. These are the 10 named-session tests that tasks
6.3, 6.5, 6.6, 8.2 and 9.4 hold by a direct run on the operator's ruling
of 2026-11-05; that run is under "The direct run" below.

## Scenarios

Each scenario of the twelve delta specs was matched to the test that
holds it, and the test's assertions were read against each THEN.

- **stream-terminals**, `crates/flume-conformance/src/engine/stream_terminals.rs`:
  two streams under two terminals, a stream naming none, a multiplexer
  changed while a change runs, a start written before (with
  `print.rs`'s unit test giving the host's terminal), a caller under a
  stream's terminal, one stream with no watch, a stream naming `rex`, a
  stream changed to `rex` while it waits, the terminals in status. "A
  place in two terminals" (`a_place_in_two_terminals`) asserts one tidy
  of `other` and one of `fake`, and the answer's `TerminalTidied`
  entries as `(Some("other"), …)` and `(Some("fake"), …)`, each
  terminal named by its `profile` with what it closed (137, 190).
  `print.rs`'s `each_terminal_a_tidy_closed_in` holds the person's
  form, and conformance's field list holds `tidy.terminal[].profile` in
  the JSON of `acted`, `settled` and `tidied`.
- **stream-views**, `crates/flume-conformance/src/engine/open.rs`: every
  scenario, `s90_a_streams_view` beside them. `Engine::open_stream`
  (`crates/flume-core/src/open.rs`) takes no lock, writes no held state
  and settles nothing, and `two_askings` asserts it. "A command that
  needs a variable" is held in JSON by `a_command_that_needs_a_variable`
  and in the person's form by `print.rs`'s test of the same name, which
  prints that scenario's own answer, `a_variables_open`, shared from
  conformance: `XDG_CONFIG_HOME` named, `/scratch/home` nowhere, the
  printed line last.
- **window-profiles**, `crates/flume/tests/profiles/window_ghostty.rs`,
  `window_terminal.rs`, `window_printed.rs`, `crates/flume/tests/commands.rs`
  and `crates/flume/src/host/config.rs`: every scenario.
- **terminal-profiles**, `crates/flume/tests/profiles/tools/terminal_herdr.rs`:
  herdr's five scenarios. `terminal_printed.rs`: the printed command's
  refusal of both views, and
  `attach_under_the_printed_command_runs_nothing`, through the core's
  open for a window and for the app, each refused in the printed
  command's words, the terminal app's stand-in `osascript` never run,
  the fake window showing nothing and the printed command's state
  folder unchanged.
- **seam-contracts**: each seam's operations, no tool named outside a
  profile (`ghostty` and `osascript` among the words, and none found
  outside a `profiles` directory), named terminals and windows, the
  round trip in `crates/flume-seams/src/round_trip.rs`. "Show over the
  fake" is held by conformance's `tests/seams.rs` `show_over_the_fake`:
  opened twice, the log naming `show /bin/fake-terminal attach work
  titled Work Agents` twice, then the printed line and the refusal.
  "Attach over the fake" is held by `seams/terminal.rs` with
  `crates/flume-fakes/tests/fakes.rs`'s `a_scripted_attach`, which
  asserts the commands and the log exactly.
- **seam-fakes**: a scripted attach and a scripted display,
  `crates/flume-fakes/tests/fakes.rs`. "A scripted window" by
  `engine/open.rs`'s `a_scripted_window`: the fake window scripted to
  print `attach work`, the core's open answering that line, and the
  fake window's log one show `titled Work Agents`, no window opened.
  "A scripted title" by `engine/streams.rs`'s `a_scripted_title`:
  status gives `work`'s title `Work Agents` from `stream`.
- **streams**: the word written before, a view no word names, a title
  that is no text (`config.rs`), and the display in status
  (`engine/streams.rs`). "The keys and none" by `config.rs`'s
  `the_keys_and_none`, each stream's three settings with where each
  came from, and through the binary by `commands.rs`'s
  `the_keys_and_none_in_status`: office's title `Office Agents` from
  `stream` and personal's `personal` from `stream-name`. Both name the
  profiles by stand-ins of the spec's `herdr` and `ghostty`, `mux` and
  `app-window` in the unit test and `printed` through the binary, since
  herdr needs the tools layer; what each reads and where from is the
  spec's.
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
  unbuilt list match the spec's list entry for entry.
  conformance:coverage passes.

## What fails

1. **The archived `host-configuration` spec fails `root:specs`.**
   `openspec archive flume-opens-a-streams-multiplexer --yes`, run at
   `bf232f6`, synced 31 requirements added and 7 modified. Then
   `openspec validate --specs --strict`, the command `root:specs` runs,
   passed 39 of 40 and failed `spec/host-configuration`: "requirements[1]:
   Requirement text is very long (>500 characters)". The requirement is
   "The local layer sets only C.7's keys", whose text this change's delta
   (`specs/host-configuration/spec.md`) takes from 407 characters to 506
   by adding `multiplexer` and "a stream's `view`, `terminal_app` and
   `title` and every key of `[host]` among them". `root:changes` does not
   catch it, since `openspec validate --changes --strict` does not hold a
   delta's requirement to that length. The archive was undone and nothing
   of it committed. The delta's requirement needs to come under 500
   characters, by shorter words or split into two requirements, each
   with its scenarios. It holds once `openspec archive` of the change
   leaves `openspec validate --specs --strict` passing, read on a copy
   of the tree or with the archive undone after.

## Observations

None of these fails a THEN or a requirement; each stands for a later
change to weigh.

- **The branch conflicts with main at `c9d58df`.** `sluice exec`
  names 18 files that would conflict, `clauses.rs`, `answer.rs`,
  `print.rs`, the fakes' host and terminal, `terminal_herdr.rs`,
  `docs/architecture/core.md` and three site pages among them, since
  #102, #104, #105 and #106 landed after this branch's merge of
  `70afbf8`. Submitted as it stands, it would be read conflicting and
  go back with the merge queue's words (27, 112); the code stage this
  change goes back to can merge main beside "What fails". Main changes
  16 of `openspec/specs/`'s files since `70afbf8` but not
  `host-configuration`, so the length is this change's alone. Main's
  clause list reads
  `6188a10` too and names 186 to 190, S90 and S91 unbuilt by this
  change, so the merge takes main's list with those entries built.
- **The printed window quotes `=` in its program word.**
  `crates/flume/src/profiles/window/printed/mod.rs` quotes the program
  by `shell::quote`, whose set leaves `=` out, where the window-profiles
  requirement gives `-_./:=@%+,`; its arguments go through
  `shell::argument`, which keeps `=` plain. Quoting a program word that
  holds `=` keeps a shell from reading it as an assignment after
  `NAME=<set>`, and quotes nothing the requirement needs plain in any
  scenario; no test holds a program word with `=`.
- **`<name>=<set>` pasted into a POSIX shell.** A shell reads
  `XDG_CONFIG_HOME=<set> /bin/herdr …` as an empty assignment, `<set`
  and `> /bin/herdr`: where a file `set` stands in the folder and the
  program is writable, the program is truncated. The form is core.30's
  and the spec's, not this build's own; the person's line may want
  words no shell reads as a redirection, which would be a change to
  core.30's form in the statement first.
- **`window_ghostty.rs:232`** sets `HERDR_ENV` in the test process and
  never clears it, so it leaks into the rest of the `profiles` test
  binary.
- **The ten-second bounds** of the ghostty and terminal profiles
  (`BOUND` in each `mod.rs`) are held by no test; every test sets two
  seconds.
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
the_herdr_profile_passes`, run at `70dc992` on 2026-11-05 from 22:23
UTC: 11 passed, 0 failed, in 161.50 s. These are the 10 that failed
under `sluice exec` on their sockets, and `a_name_herdr_refuses`. The
tools layer holds as the operator ruled; the landing's gate stays its
proof.
