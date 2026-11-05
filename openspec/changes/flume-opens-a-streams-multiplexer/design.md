# Design

## Context

flume's main at d81d258 binds the terminal once for the host from
`[host] terminal` (`profiles::terminal` in `crates/flume/src/profiles.rs`):
the herdr profile, or the printed command where the host names none.
`Terminal` (`crates/flume-seams/src/terminal.rs`) opens, sends to,
reads, closes, tidies and shows agents, and names the calling
command's session. Nothing in it shows a person a part of the terminal.

The change `each-stream-has-its-own-session-and-labels-and-the-host-a-local-layer`
(record 0031), built on its branch and not yet landed, runs each
stream's agents in a session of their own. `StreamRoom::session(name)`
in `crates/flume-core/src/room.rs` resolves a stream's session: its
`session`, else its name, and for `default` naming none the terminal's
own, as a `SessionHeld` with its `SessionFrom`. `Stream` in
`crates/flume-seams/src/host.rs` gains `session`. The herdr profile
reaches a named session's server by `socket_of(&Server::Session(name))`,
found by `herdr session list --json` and started headless by `herdr
--session <name> server` in a process group of its own where none runs,
and runs every call with `XDG_CONFIG_HOME`, `SESSION_HOME`, set where
`with_session_home` gave a home. The host's configuration is read in two
layers, and `[streams.<stream>]` takes `agents`, `priority`, `pool`,
`session` and `labels`, refusing any other key. This change is built
after that one lands, on main as it leaves them.

A person who wants a stream's agents today runs a launcher of their own:
it opens a window of their terminal program, `herdr --session <name>`
in it, titled so their window manager files it. Ghostty 1.4.0 forces a
window's title by `--title`, ignoring the titles a program inside sets,
and runs a command by `-e`. On macOS its command line starts no window,
and `open -na Ghostty.app --args …` does, `open --env` giving the new
process a variable. herdr 0.9.5 attaches a client to a named session by
`herdr session attach <name>`, finding it under its sessions' home.

The statement's amendment is on the agentplot/blueprints branch
`design/flume-a-stream-opens-in-a-window` at 3cc35fa: 186 in Part A,
attach (187) in B.5, the window as B.8 (188, 189) with the binding
moved to B.9, 105, 143, C.5's attach row, C.7's `title` and `[host]
window`, C.8 the window's profiles, section 9's command line and S90.
Record 0037 decides it, proposed until the user's review of this change
accepts it. The build reads the statement at the revision of blueprints
main that states it (core.19). Clauses 184 and 185, S86 to S88 and S89
are claimed by `a-pool-chooses-the-seat-with-the-most-quota-left` and
`a-places-readiness-is-one-design-across-the-tools`.

The rules this design relies on: core.1 and core.2 (the core names no
terminal and no window program, and one interface per seam, a profile
calling no other), core.13 (no process of flume's outlives its command;
its details name the terminal's server and the window as not flume's),
core.14 (the JSON form only gains fields), core.17 (every act safe to
repeat), core.19 (the clauses cited), core.21 and core.22 (no new
project; Ghostty is run by no gate task), core.29 (every herdr call
bounded and read from the pinned herdr), core.30 (no variable's value in
an answer), core.35 (the site's neutral voice), and core.48, drawn with
record 0037. The one rules page is `docs/architecture/core.md`. Where
flume has no rule, the kit standard's code profile binds (kit:specs.3,
kit:testing.5).

## Goals / Non-Goals

**Goals:**

- `flume open [<stream>]` opens a window on the stream's session, titled
  as the stream declares or by its name (186, S90).
- A new multiplexer is a terminal profile with its attach, and a new
  window program a window profile, each written once for every host
  (187, 188).
- A host that names no window is given the command to run (105, 188).

**Non-Goals:**

- A terminal profile for Rex or TUIOS. The seam admits both: a
  multiplexer that a window program runs answers attach with its
  command, and one that draws its own windows answers that it showed
  the part. Each is a change of its own, checked against its release.
- A window on a place's own tab or an agent's pane. The herdr plugin's
  key already opens a place's pane. A later `flume open` of a place can
  take a flag.
- Finding, raising or closing a window already open. flume keeps
  nothing of a window (core.48), and a window manager files it by its
  title.
- A key of the herdr plugin or a command of the Claude Code plugin that
  opens a stream's window.

## Decisions

### 1. The command and the core

`flume open [<stream>] [--json]` calls `Engine::open_stream(stream,
window)` in a new `crates/flume-core/src/open.rs`, `default` where the
command names no stream. The window is passed at the call rather than
held by the engine. Only `open` binds it, so a host whose `[host]
window` names no profile refuses `open` alone and every other command
runs as before (record 0037, item 5).

`open_stream`, in order:

1. The stream: the host's streams as `Host::streams` gives them, with
   `default` always among them. A stream it does not declare refuses,
   `the host declares no stream <name>; its streams are: …` (4, 186).
2. The session: `StreamRoom::session(name)` over the stream's
   `session` as the configuration stands now, the same resolution the
   claim uses for a stage, so a window and a stage's agent never
   disagree (107). It is a `SessionHeld`: a name from `stream` or
   `stream-name`, or none from `own`.
3. The title: the stream's `title`, from `stream`, else its name, from
   `stream-name`, as a `TitleHeld` (186).
4. `terminal.attach(session.name.as_deref(), &title)`. `Refused`
   refuses with the terminal's words. `Shown` answers with no command
   and asks nothing of the window.
5. `window.show(&command, &title)`. `Refused` refuses with the window's
   words.

It takes no lock, writes no held state and settles nothing (core.48).
A second call opens a second window, and changes nothing flume holds,
so it is safe to repeat (core.17).

The answer is a new `Answer::Opened`:

```rust
pub struct StreamOpened {
    pub stream: String,
    pub session: SessionHeld,
    pub title: TitleHeld,
    /// None where the terminal showed the part in its own window.
    pub command: Option<ViewCommandStatus>,
    pub window: WindowOutcome,
}

pub struct TitleHeld { pub title: String, pub from: TitleFrom }
pub enum TitleFrom { Stream, StreamName }

/// The command as the answer gives it: its program, its arguments, and
/// the names of the variables it needs, never their values (core.30).
pub struct ViewCommandStatus {
    pub program: String,
    pub arguments: Vec<String>,
    pub environment: Vec<String>,
}

pub enum WindowOutcome {
    /// The window seam opened a window.
    Opened,
    /// No window: the line for a person to run, as the profile gave it.
    Printed { line: String },
    /// The terminal showed the part in a window of its own.
    Terminal,
}
```

`TitleFrom` serializes as `stream` and `stream-name`, as `SessionFrom`
does. The person's form says, for instance, `opened a window titled
"Work Herdr" (the stream's title) on the session work (the stream's
name)`, and for the printed window, `run: <line>` on a line of its
own, so `flume open work` can be pasted or read by a person's own
launcher.

### 2. The terminal seam's attach

`flume-seams/src/terminal.rs` gains:

```rust
/// The command a person runs to see a part of the terminal: its program
/// by its full path, its arguments, and the variables it needs of its
/// environment with their values, which the window passes on and no
/// answer gives (187; core.30).
pub struct ViewCommand {
    pub program: String,
    pub arguments: Vec<String>,
    pub environment: BTreeMap<String, String>,
}

/// attach: the command that shows the part, the part started first
/// where it did not run; the part shown in a window the terminal draws
/// itself; or a refusal (187).
pub enum Attached {
    Command { command: ViewCommand },
    Shown,
    Refused { reason: String },
}

/// The part named `session`, the terminal's own where none, shown to a
/// person under `title` (187).
fn attach(&self, session: Option<&str>, title: &str) -> Attached;
```

`environment` serializes with a default and is skipped where empty,
for the fakes' round trip. The core turns it into the names alone
before anything prints it.

### 3. herdr's attach

`Herdr::attach` (`crates/flume/src/profiles/terminal/herdr/mod.rs`):

- **The server.** For a name, `socket_of(&Server::Session(name))`, so
  a stream's session not yet running is started headless as open
  starts it, with the caller's `HERDR_*` stripped and the session home
  set, before any window attaches. For none, the profile's own server:
  `Server::Session(SESSION)` on the host, `flume`. A profile whose own
  server is `Server::Socket`, as a test's is, refuses its own part:
  herdr attaches a client by a session's name, and that server has
  none. A session herdr refuses, or one that does not start within the
  bound, refuses in herdr's words, as open does (core.29).
- **The program.** The full path of the herdr the profile runs: its
  `program` where it is absolute, else the first executable of that
  name on the `PATH` the command runs under, the one every call of the
  profile's runs. None found refuses naming it. The window then runs the
  same release whatever its own `PATH` holds.
- **The arguments.** `session attach <name>`, `flume` for the own
  session.
- **The environment.** `XDG_CONFIG_HOME`, herdr 0.9.5's sessions' home,
  where the profile's calls run under one: the home `with_session_home`
  gave, else the command's own `XDG_CONFIG_HOME` where it is set.
  Nothing otherwise, and never a `HERDR_*`.

The title is not passed to herdr: the window gives it, and Ghostty
holds it whatever herdr sets.

The printed command's `attach` refuses: `the printed command holds no
session a person can be shown; its agents run in the commands it
printed` (187).

### 4. The window seam

A new `flume-seams/src/window.rs`, re-exported from the crate:

```rust
/// show: a window running the command under the title, the command for
/// a person to run where the host shows no window, or a refusal (188).
pub enum WindowShown {
    Opened,
    Printed { line: String },
    Refused { reason: String },
}

/// The window seam (B.8), bound once for the host. The guarantee: show
/// opens at most one window at each asking, runs nothing in it but the
/// command it is given, and answers within a bound the profile states
/// (189).
pub trait Window: Send + Sync {
    fn show(&self, command: &ViewCommand, title: &str) -> WindowShown;
}
```

`profiles.rs` gains the seam's marker and module list,
`pub mod window { pub mod ghostty; pub mod printed; }`, and
`window(name: Option<&str>) -> Result<Arc<dyn Window>, String>`: the
printed command where none is named, ghostty for `ghostty`, and
otherwise a refusal naming the profiles, as `terminal()` refuses an
unknown terminal. `window_profiles()` lists them for the refusal and
for `--help`.

### 5. The ghostty profile

`crates/flume/src/profiles/window/ghostty/mod.rs`, `NAME = "ghostty"`:

- **On macOS:** `open -na Ghostty.app`, `--env <name>=<value>` for each
  variable of the command's environment in order, then `--args
  --title=<title> -e <program> <arguments…>`, each a word of its own and
  nothing passed through a shell. `open` runs under a bound of ten
  seconds and is waited on. Its exit 0 is `Opened`. Any other exit, or
  no answer within the bound, is `Refused` with its standard error's
  words, `Unable to find application named 'Ghostty.app'` among them.
  The display starts Ghostty under its own environment with the
  variables given, so nothing of the calling command's, its `HERDR_*`
  among it, reaches the window.
- **Elsewhere:** `ghostty --title=<title> -e <program> <arguments…>`,
  started in a process group of its own, as the herdr profile starts a
  session's server, with its input, output and errors closed, under the
  host user's login environment, `host::login_environment()`, with the
  command's environment over it and nothing else. It is not waited on:
  a start that fails to spawn is `Refused` with the error, and any
  other is `Opened`.
- `Ghostty::new()` runs `open` and `ghostty` by name. A test builds it
  with `with_programs(open, ghostty)` naming stand-ins of its own, and
  `with_login(variables)` as `Herdr::with_login` does.

The profile's flags are checked against Ghostty 1.4.0 (C.8). Ghostty is
not pinned by the tree: it is a program of the host's display, and no
gate task runs it (core.22). The profile is held by its tests' stand-in
for `open` and `ghostty`, which records the words and the environment
it was run with.

### 6. The printed window

`crates/flume/src/profiles/window/printed/mod.rs`, `NAME = "printed"`,
the window a host that names none gets. `show` answers `Printed` with
the line: each variable of the command's environment as `<name>=<set>`,
then the program and its arguments, each quoted for a POSIX shell where
it holds anything but letters, digits and `-_./:=@%+,`. It runs
nothing. A command with no environment, herdr's on a host, prints as a
person runs it: `/nix/store/…/bin/herdr session attach work` (core.30).

### 7. The host's configuration

`crates/flume/src/host/config.rs`:

- `[streams.<stream>]` takes `title`, a text, kept in `Stream::title`,
  none where it gives none; any other key is refused as today.
- `[host]` takes `window`, a name, kept in `HostConfig::window`.
- Neither is a key of `LOCAL_KEYS`, so a local layer that sets
  `streams.<stream>.title` or `host.window` is not taken, the managed
  layer read alone and status naming the key, as for every key the local
  layer may not set (record 0032; record 0037, item 7).
- `Stream` in `flume-seams/src/host.rs` gains `title: Option<String>`,
  read with a default and skipped where none, so a stream written before
  reads with none.

### 8. Status

`StreamStatus` gains `title: TitleHeld`, resolved as `open_stream`
resolves it, so status gives each stream's title with where it came
from in the JSON form, and in the person's form as `title "Work Herdr"
(the stream's title)` beside its session (56, 143; core.14).

### 9. The fakes

- The fake terminal's `attach` answers a scripted `Attached`, by
  default `Command` of `fake-terminal attach <session>` or `attach own`
  with no environment, and logs `attach <session> titled <title>` on a
  line of its own. A script makes it answer `Shown` or `Refused`.
- A new fake window, `FakeWindow` in `crates/flume-fakes/src/window.rs`,
  logs each `show` with its command and title, and answers `Opened`,
  `Printed` or `Refused` as scripted. It reads no display.
- The fake host's streams take a scripted `title`.

### 10. How it is held

- `flume-seams/src/round_trip.rs`: `ViewCommand`, `Attached`,
  `WindowShown`, `Stream::title`, and a stream written before reading
  with none.
- `flume-fakes`: the fake terminal's attach and log, the fake window,
  and their tests in `crates/flume-fakes/tests/fakes.rs`.
- `flume-conformance/src/seams/window.rs`: the window seam's scenarios,
  run against the fake, and through `Seam scenarios run against any
  profile`, against the ghostty profile over its stand-in and the
  printed one. `seams/terminal.rs` gains attach's scenarios over the
  fake, and the printed terminal's refusal.
- `flume-conformance/src/engine/open.rs`: the stream-windows spec's
  scenarios over the fakes, S90 among them as `s90_a_streams_window`.
- `flume-conformance/src/no_tool.rs`: `ghostty` among the words (core.1).
- `crates/flume/src/host/config.rs`'s unit tests: `title`, `window`, and
  a local layer setting either not taken.
- `crates/flume/src/commands/print.rs`'s unit tests: the opened, printed
  and terminal-shown lines.
- `crates/flume/tests/profiles/window_ghostty.rs` and
  `window_printed.rs` (check-fast): the profile's words and environment
  over a stand-in for `open` and `ghostty`, the refusal of a stand-in
  that exits non-zero, and the printed line.
- `crates/flume/tests/commands.rs` (check-fast): `flume open` refused
  under the printed terminal, an undeclared stream refused, an unknown
  `[host] window` refusing `open` alone, and `--json`'s shape.
- `crates/flume/tests/profiles/tools/terminal_herdr.rs` (check-tools):
  attach under the pinned herdr with a scratch session home, one-letter
  session names as the sessions' tests have them: a session not running
  started headless and listed running, the command naming the herdr's
  full path and `session attach w` with `XDG_CONFIG_HOME` the scratch
  home, a client the command runs attaching to that server, and the own
  session of a socket server refused.
- `crates/flume/tests/profiles/tools/open.rs` (check-tools): S90 through
  the binary, `flume open w` under herdr with the ghostty profile over
  a stand-in, then an agent of a fix launched on `w` opening in the
  session the window attached to, a second open recording a second
  window, `flume open` naming no stream attaching to the own session,
  and the printed window's line under a host that names none.

### 11. The site

Every page speaks to any host's operator (core.35).

- `reference/commands.md`: `flume open [<stream>]`, what it answers and
  what refuses.
- `reference/configuration.md`: `[host] window`, its profiles and its
  default, and `[streams.<stream>] title`, each the managed layer's.
- `guides/places-and-herdr.mdx`: opening a stream's session in a
  window, its title for a window manager, and the printed command where
  the host names no window.
- `guides/agent-settings.md`, under "Streams": a stream's title beside
  its session.
- `reference/state.md`: status's `title` of each stream.
- `roadmap.md`: the window built, and terminals for other multiplexers
  not yet written.

### 12. The clause list

`clauses.rs` reads the statement at the revision of blueprints main
that states 186 to 189 and S90: Part A gains 186, Part B 187 to 189, and
the scenarios run to S90. A clause of that revision another change
builds and has not built, 184, 185 and S86 to S89 among them where they
stand there unbuilt, stays named unbuilt by that change. 186 to 189 and
S90 are cited by this change's scenarios.

## Risks / Trade-offs

- **Ghostty is not pinned.** A release that changes `--title` or `-e`,
  or how macOS starts it, breaks the window and nothing else, and the
  gate does not see it, since no gate task opens a window. The stand-in
  holds the words flume passes, and C.8 names the release they were
  checked against.
- **`open` on macOS answers once the display has the request.** A
  Ghostty that starts and fails at once, a bad option among them, shows
  its own error in its window, not in flume's answer.
- **A window per asking.** A launcher bound to a key opens a window
  each press. A window manager that files windows by title stacks them;
  finding the one already open is the window manager's, and flume keeps
  nothing to find it by (core.48).
- **The session started for the window.** A stream whose session is not
  running is started headless at `open`, so herdr's server runs before
  any of the stream's places has an agent. It runs as open would have
  started it, and stays as every session's server stays (C.5).
- **A sessions' home set only for flume.** A command run with an
  `XDG_CONFIG_HOME` of its own passes it to the window, so the window's
  herdr finds the session flume started. The printed line names it
  `<set>`, so a person running it by hand runs it under the same home.

## Migration Plan

- A host that names no window and no title gets the printed command
  from `flume open`, and status gives each stream's title as its name.
  Nothing else moves.
- A host's launcher becomes `flume open <stream>`. The title it gave the
  window is declared once as the stream's `title`, and `[host] window =
  "ghostty"` names the window program.
- A stream written before reads with no title. Held state is unchanged:
  `open` writes none.

## Open Questions

None. The command's name, the window seam and the keys' layer are
record 0037's, proposed until the user's review of this change.
