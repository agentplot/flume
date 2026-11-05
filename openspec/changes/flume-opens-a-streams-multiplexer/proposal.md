# flume opens a stream's multiplexer

## Why

Record 0031 runs each stream's agents in a session of the terminal of
their own, herdr's session of the stream's name under herdr. To see a
stream's agents a person opens a window of their own terminal program,
attaches a herdr client to that session in it, and titles the window so
their window manager files it. Each person keeps a launcher of that for
each stream, and each new multiplexer would be a new launcher for every
stream on every host.

The user, on 2026-11-05: "rather than maintaining aliases for launching
my multiplexers, I could have a command that uses Flume to open the
multiplexer for a given stream. I guess it would be at the stream
level, right? This way as I add new multiplexers like Rex or TUIOS, I
wouldn't have to keep maintaining custom scripts." They asked the same
day to rush it.

flume knows the host's terminal, each stream's session and the herdr it
drives. It does not know the program that opens a window on the host's
display, or the title a stream's window wants. Record 0037, proposed,
decides both, and the statement's amendment that states them is on the
agentplot/blueprints branch `design/flume-a-stream-opens-in-a-window`
at 3cc35fa (105, 120, 143, 186 to 189, B.5, B.8, B.9, C.5, C.7, C.8,
S90). Nothing is built until the user accepts record 0037 on their
review of this change and the amendment stands on blueprints main
(core.19).

## What Changes

- **`flume open [<stream>]` (186).** Opens a window on the stream's own
  session, `default` where none is named, and answers the stream, its
  session and the title, each with where it came from, the command and
  what the window did, printed or as JSON with `--json`. A stream the
  host does not declare is refused naming it. A second asking opens a
  second window.
- **The session is resolved as a start resolves it** (record 0031), by
  the same function: the stream's `session`, else its name, and the
  terminal's own for the default stream naming none.
- **A stream's title (143, 186).** `title` in `[streams.<stream>]`, else
  the stream's name, read from the managed layer alone. Status gives
  each stream's title with where it came from.
- **The terminal seam's attach (187).** Given the part of the terminal
  and the title, the command that shows the part to a person, with what
  it needs of its environment, the part started first where it does not
  run; or that the terminal showed it in a window of its own; or a
  refusal. herdr answers `<herdr> session attach <name>` by the full
  path of the herdr it runs, the session's server found or started
  headless as its open starts it. The printed command refuses.
- **A window seam (188, 189).** The eighth seam, bound once for the host
  by `[host] window`: given a command, its environment and a title, a
  window running the command under that title, or the command given for
  a person to run, or a refusal. Its profiles are `ghostty` and the
  printed command, which a host that names no window gets.
- **The ghostty profile (C.8).** On macOS `open -na Ghostty.app --env …
  --args --title=<title> -e <program> <arguments>`, elsewhere `ghostty
  --title=<title> -e …` started in a process group of its own and not waited
  on. Checked against Ghostty 1.4.0.
- **flume keeps nothing of a window** (core.48). It opens one only at a
  person's asking, and records, reads, finds again and closes nothing
  of it.
- **The conformance suite** holds the window seam's scenarios over its
  fake and its profiles, the terminal's attach over the fake, herdr and
  the printed command, the open's scenarios with S90, and its clause
  list read at the revision of blueprints main that states 186 to 189
  and S90.
- **The site** says how a person opens a stream's window: the commands
  and configuration references, the herdr guide and the agent-settings
  guide's streams, to any host's operator (core.35).

## Capabilities

### New Capabilities

- `stream-windows`: `flume open`, the stream's session and title, what
  the window did, and what refuses.
- `window-profiles`: the window seam through Ghostty and the printed
  command.

### Modified Capabilities

- `seam-contracts`: eight seams, the terminal seam's attach and the
  window seam's show with their types, and `ghostty` among the tools no
  code outside a profile names.
- `seam-fakes`: the fake terminal's attach and the fake window.
- `terminal-profiles`: herdr's attach, and the printed command's
  refusal.
- `streams`: a stream's title, and status's title of each stream.
- `command-line`: `open` among the commands.
- `clause-coverage`: the suite's list at the revision that states 186
  to 189 and S90.

## Impact

- agentplot/blueprints: the amendment on
  `design/flume-a-stream-opens-in-a-window` at 3cc35fa, which the build
  reads once it is on main (core.19). Clauses 184 and 185, S86 to S88
  and S89 stay with the changes that claim them.
- The change `each-stream-has-its-own-session-and-labels-and-the-host-a-local-layer`:
  this change reads its sessions, `StreamRoom::session`,
  `Herdr::socket_of` and `SESSION_HOME`, its host configuration's two
  layers and its `Stream::session`. This change is built after it
  lands, on main as it leaves them.
- `crates/flume-seams`: `Terminal::attach`, `Attached`, `ViewCommand`,
  a new `window.rs` with `Window`, `WindowShown`, and `Stream::title`,
  only added, a stream written before reading with none.
- `crates/flume-core`: a new `open.rs` with `Engine::open_stream`, the
  engine built with a window, `StreamStatus::title`.
- `crates/flume`: `profiles.rs`'s window seam and `window()`, a new
  `profiles/window/ghostty` and `profiles/window/printed`, herdr's and
  the printed terminal's attach, `host/config.rs`'s `title` and
  `[host] window`, `commands/mod.rs`'s `open`, `print.rs`, and their
  tests.
- `crates/flume-fakes`, `crates/flume-conformance`: the fake window,
  the fake terminal's attach, the window seam's scenarios, the open's
  scenarios citing S90, the no-tool words and the clause list.
- `site/`: the commands and configuration references, the herdr guide
  and the agent-settings guide.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency. Ghostty is not pinned: no gate task runs it, and its
  profile is held by tests against a stand-in. The herdr and Claude
  Code plugins are unchanged.
