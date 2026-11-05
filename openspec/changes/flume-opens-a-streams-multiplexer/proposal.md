# flume opens a stream's multiplexer

## Why

Record 0031 runs each stream's agents in a session of the terminal of
their own, herdr's session of the stream's name under herdr. To see a
stream's agents a person opens a window of their own terminal program,
attaches a herdr client to that session in it, and titles the window so
their window manager files it. Each person keeps a launcher of that for
each stream, and each new multiplexer would be a new launcher for every
stream on every host. And the host binds one terminal for every
stream, so a person cannot try a multiplexer on one stream while the
others run as they did.

The user, on 2026-11-05: "rather than maintaining aliases for launching
my multiplexers, I could have a command that uses Flume to open the
multiplexer for a given stream. I guess it would be at the stream
level, right? This way as I add new multiplexers like Rex or TUIOS, I
wouldn't have to keep maintaining custom scripts." They asked the same
day to rush it, and, reading how each multiplexer splits into a backend
and the ways it is viewed (`design/research/multiplexers-2026-11-05.md`):
"Can this be set per stream as well? I would like to be able to test
different setups using different streams to confirm that it all
works." They ruled, "Yes, amend it to multiplexer, view and terminal
app, per Mac with a per-stream override", and "One change for all of
it".

Record 0037, accepted by the user on 2026-11-05 for the three settings
and their per-stream override, decides it. The statement's amendment
stands on agentplot/blueprints main at 36cfd06, superseding 186 to
189 and S90 as main stated them at d680e35 (7, 105, 107, 118, 119,
143, 186 to 190, C.5, C.7, C.8, S90, S91; core.19).

## What Changes

- **Three settings, the host's and a stream's over it (143, 186, 190,
  C.7).** `multiplexer`, the terminal profile a stream's agents run in,
  herdr or the printed command, `[host] terminal` read as the host's;
  `view`, `terminal` or `app`; and `terminal_app`, `ghostty` or
  `terminal` for Terminal.app, the printed command where none is named.
  A stream also declares its `title`. A stream's `multiplexer` is the
  local layer's to set as its `session` is; its `view`, `terminal_app`
  and `title` are the managed layer's.
- **Each stream's agents run in its multiplexer (190).** A start reads
  its stream's at each start and records it, and every later call on
  its agent goes to the terminal it recorded. A place is tidied in each
  terminal its agents opened in, a recipient is found in the terminal
  that names it, and flume runs attended while any terminal the host
  binds keeps no watch. A stream whose multiplexer names no profile
  refuses its launches, its starts and its opens alone. Status gives
  each stream's terminal, view, terminal app and title, and each
  agent's terminal, each with where it came from.
- **`flume open [<stream>]` (186).** Shows the stream's own session,
  `default`'s where none is named, in its view: a window its terminal
  app opens titled as the stream declares, or its multiplexer's own
  app. It answers the stream, its multiplexer, session, view, title and
  terminal app with where each came from, the command and what the
  window did, printed or as JSON with `--json`. A second asking opens a
  second window.
- **The terminal seam's attach (187).** Given a part of the terminal
  and a view: for a window, the command that shows the part, with what
  it needs of its environment, the part started first where it does not
  run; for the app, the app told to show it; or a refusal. herdr answers
  `<herdr> session attach <name>` by the full path of the herdr it runs,
  and refuses the app, having none of its own. The printed command
  refuses both.
- **The window seam (188, 189).** The eighth seam, bound per stream
  through the resolver by `terminal_app`: given a command, its
  environment and a title, a window running the command under that
  title, or the command given for a person to run, or a refusal. Its
  profiles are `ghostty`, `terminal` and the printed command.
- **Ghostty and Terminal.app (C.8).** Ghostty by `open -na Ghostty.app
  --env … --args --title=<title> -e <program> <arguments>` on macOS and
  `ghostty --title=<title> -e …` elsewhere, checked against 1.4.0.
  Terminal.app by `osascript` telling Terminal to `do script` the line
  and set that tab's custom title, checked against Terminal 2.15 on
  macOS 27, and refused elsewhere.
- **flume keeps nothing of a window** (core.48). It opens one only at a
  person's asking, and records, reads, finds again and closes nothing
  of it.
- **The conformance suite** holds the window seam's scenarios over its
  fake and its profiles, attach by view over the fake, herdr and the
  printed command, a stream's terminal over two fake terminals with
  S91, the open's scenarios with S90, and its clause list read at the
  revision of blueprints main the amendment lands at.
- **The site** says how a person names a stream's multiplexer, view and
  terminal app and opens its agents, to any host's operator (core.35).

## Capabilities

### New Capabilities

- `stream-terminals`: a stream's multiplexer, where its agents open
  and stay, a place's terminals, recipients, attended, refusals and
  status.
- `stream-views`: `flume open`, the stream's session, view and title,
  what the window or the app did, and what refuses.
- `window-profiles`: the window seam through Ghostty, Terminal.app and
  the printed command, bound per stream.

### Modified Capabilities

- `seam-contracts`: eight seams, attach by view and the window seam's
  show with their types, the resolver's terminals and windows by name,
  and `ghostty` and `osascript` among the tools no code outside a
  profile names.
- `seam-fakes`: the fake terminal's attach by view, the fake window,
  the fake host's display and the fake resolver's named terminals.
- `terminal-profiles`: herdr's attach by view, and the printed
  command's refusal.
- `streams`: a stream's title, and status's display of each stream.
- `host-configuration`: the local layer sets a stream's `multiplexer`.
- `launching`: a launch on a stream whose multiplexer names no profile
  is refused whole.
- `settling`: flume runs attended while any terminal the host binds
  keeps no watch.
- `command-line`: `open` among the commands.
- `clause-coverage`: the suite's list at the revision that states 186
  to 190, S90 and S91.

## Impact

- agentplot/blueprints: the amendment on main at 36cfd06, which the
  clause list reads (core.19).
  Clauses 184 and 185, S86 to S88 and S89 stay with the changes that
  claim them.
- The change `each-stream-has-its-own-session-and-labels-and-the-host-a-local-layer`,
  landed on main at 70afbf8: this change reads its sessions, its two
  layers and its herdr profile's `socket_of`, and is merged with it
  first.
- `crates/flume-seams`: `View`, `ViewHeld`, `NamedBy`, `ProfileHeld`,
  `StreamDisplay`, `Host::display_for`, `Stream`'s four keys,
  `Start::terminal`, `Resolver`'s terminals and windows,
  `Terminal::attach` by view, and decb865's `ViewCommand`, `Attached`,
  `Window` and `WindowShown`, each only added, a stream and a start
  written before reading with none.
- `crates/flume-core`: `terminal_of` and every call on an agent through
  it, a place's terminals for its clues and its tidy, the host's
  terminals for the watch, the caller's session and a recipient, the
  start's and the launch's refusals, a new `open.rs` with
  `Engine::open_stream`, and status's fields.
- `crates/flume`: `host/config.rs`'s keys, `profiles.rs`'s resolver of
  terminals and windows, `profiles/window/ghostty`, `terminal` and
  `printed`, herdr's and the printed terminal's attach,
  `commands/mod.rs`'s `open`, `print.rs`, and their tests.
- `crates/flume-fakes`, `crates/flume-conformance`: the fakes above,
  the window seam's scenarios, the stream-terminals and open scenarios
  citing S90 and S91, the no-tool words and the clause list.
- `site/`: the commands and configuration references, the herdr guide,
  the agent-settings guide, the state reference and the roadmap.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency. Ghostty and Terminal.app are not pinned: no gate task
  runs them, and their profiles are held by tests against stand-ins.
  The herdr and Claude Code plugins are unchanged.
