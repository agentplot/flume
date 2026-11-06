# flume list shows each place and stream on one line

## Why

The user, on 2026-11-05, after running `flume status` with no place on
deneb, under flume 0.9.1 and 53 places: "It was extremely slow and
returns a bunch of errors too. It's completely unusable." And: "We need
a short name because I need a short list because I don't know what's
available to open".

Status with no place reads every fact of every place where it lives
(58): the change at each head, the merge queue's record of each branch,
each agent's state in the terminal. On deneb it took 75 seconds and
printed 984 lines. Eleven places' entries repeated sluice's refusal to
read old policies. Four landed places printed their whole teardown
error, `sluice release` failing to read the host's sluice
configuration (`unknown field caches`). `flume status --line` took 2.6
seconds, still reading each live place's change, naming no repository
and saying nothing of the streams, which are what `flume open` takes
(186).

Record 0049, proposed, decides it. The statement's amendment is on
agentplot/blueprints' branch `design/flume-a-list-of-places-and-streams`
at be85a32, committed for the user's word (57, 62, 64 to 66, 68, 70,
107, 111, 130, 186, 200, 201, C.5, section 9, S108; core.19). The code
waits for the user's read.

## What Changes

- **`flume list` (200).** One line per place on the host, a tidied
  place whose document stays among them (62), under its repository:
  `<kind>--<name> · <stage> · <state>`, the state in the words its line
  gives it (56, 139). A place that waits adds whom it waits on: `waits
  on its caller` (130), `waits on the host` (70) or `waits on the merge
  queue` (111). A place over that still stands adds what keeps it, in a
  word: `kept by <step>` for the step its teardown failed at, `kept
  holding work`, `kept holding a lock`, `kept while its agent runs` or
  `kept unread` (64 to 66, 68). Then one line per stream the host
  declares: `<stream> · session <name> · runs`, or `not running`, `not
  read`, `no session` or `no profile for <name>` (107, 186, 190). The
  JSON form, `{"list":{"places":[…],"streams":[…]}}`, gives the same
  lines and their parts.
- **What it reads (200; core.52).** Each place's held state and, for a
  live agent, the asking mark its own record holds, as a place's line
  reads it; and each stream's terminal once, by runs. No worktree,
  head, deliverable, change or submission; no agent's state in its
  terminal. None of a place's long words, a stop-short's, a hold's, a
  failure's or a teardown's, in its print or its JSON. It settles,
  starts, sends and writes nothing (57), needs no repository and
  refuses nothing. It answers within a second on a host with fifty
  places.
- **The terminal seam's runs (201).** Given a part by its name or the
  terminal's own: that it runs or does not, with the name the terminal
  knows it by; or that the terminal holds no part, or could not read
  it, in its words. It starts nothing. herdr answers from one `herdr
  session list --json`, bounded at one second, `flume` its own session.
  The printed command holds no part. The fake terminal scripts it.
- **The conformance suite** holds the list's scenarios over the fakes,
  S108 among them, and the terminal seam's runs over the fake, and reads
  its clause list at the revision of blueprints main that lands the
  amendment.
- **The site** names `flume list` in its commands and its answer's
  fields in the state reference, to any host's operator (core.35).

## Capabilities

### New Capabilities

- `the-list`: `flume list`, a place's line and a stream's, what the list
  reads and leaves out, and its JSON form.

### Modified Capabilities

- `command-line`: `list` among the commands, needing no repository.
- `seam-contracts`: the terminal seam's runs, and the type of its
  answer.
- `seam-fakes`: the fake terminal scripts runs.
- `terminal-profiles`: herdr's runs and the printed command's.
- `clause-coverage`: the suite's list at the revision that states 200,
  201 and S108.

## Impact

- agentplot/blueprints: the amendment, to land on main before this
  change's code, which the clause list reads (core.19).
- `crates/flume-seams`: `SessionRuns` and `Terminal::runs`, only added.
- `crates/flume-core`: a new `list.rs` with `Engine::list`;
  `Answer::List` with `Listed`, `StreamListed` and `WaitsOn`; `line.rs`
  giving its state words and its stage to the list; nothing of
  `status.rs` changed.
- `crates/flume`: `commands/mod.rs`'s `list`, `print.rs`'s form of it,
  herdr's and the printed command's runs, and their tests.
- `crates/flume-fakes`, `crates/flume-conformance`: the fake terminal's
  runs, the seam's scenario, `engine/list.rs`, and the clause list.
- `site/`: the commands reference and the state reference.
- `flume status`, `status --line`, the plugins and the MCP server are
  unchanged. No new crate, plugin or site part, so no new moon project,
  and no new dependency.
- The change `where-an-agent-opens-is-given-by-kind-and-by-stage`, in
  flight, has a stream name sessions for its kinds and stages, which
  `flume open --session` shows. Whichever of the two lands second gives
  each such session a line of its own under its stream (design 9).
