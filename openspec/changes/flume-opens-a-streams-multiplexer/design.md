# Design

## Context

flume's main at 70afbf8 holds the change
`each-stream-has-its-own-session-and-labels-and-the-host-a-local-layer`
(record 0031), landed as #98. Each stream's agents open in a session of
their own: `StreamRoom::session(name)` in `crates/flume-core/src/room.rs`
resolves the stream's `session`, else its name, and for `default`
naming none the terminal's own, as a `SessionHeld` with its
`SessionFrom`, and each `Start` in `crates/flume-seams/src/held.rs`
keeps the `session` its agent opened in. The herdr profile reaches a
named session's server by `socket_of(&Server::Session(name))`, found
by `herdr session list --json` and started headless where none runs,
and runs every call with `XDG_CONFIG_HOME` set where
`with_session_home` gave a home. The host's configuration is read in
two layers, `LOCAL_KEYS` in `crates/flume/src/host/config.rs` naming
what the local one sets.

The terminal is bound once for the host. `profiles::terminal` in
`crates/flume/src/profiles.rs` builds the herdr profile or the printed
command from `[host] terminal`, and `Engine` in
`crates/flume-core/src/lib.rs` holds it as `terminal`. About thirty
calls in the core reach it: open in `start.rs`; send, state, screen,
answer_trust and close in `acts.rs`, `deliver.rs`, `end.rs`,
`reading.rs`, `spent.rs`, `status.rs`, `submit.rs`, `tidy.rs`,
`uncarried.rs` and `unsettled.rs`; send_to and session_state in
`ends.rs`; tidy in `tidy.rs`; show and shown_at in `settle.rs` and
`clues.rs`; kept and watch in `room.rs`; watch in `settle.rs`'s
`settle_attended` and `wait.rs`; and session, the caller's, in
`launch.rs` and `wait.rs`. `Resolver` in
`crates/flume-seams/src/resolver.rs` gives the place, change,
merge-queue and agent profiles by name; `BuiltIn` in `profiles.rs` and
`FakeResolver` in `crates/flume-fakes/src/resolver.rs` implement it.

This branch holds, at decb865, the terminal seam's `ViewCommand`,
`Attached` and `Terminal::attach(session, title)`, the window seam's
`Window` and `WindowShown` in `crates/flume-seams/src/window.rs`,
`Stream::title`, the fake terminal's attach, the printed command's
refusal and herdr's `not yet built`. Its place holds, staged and
uncommitted, `FakeWindow` in `crates/flume-fakes/src/window.rs`,
`FakeHost::set_title` and their tests in
`crates/flume-fakes/tests/fakes.rs`.

A person who wants a stream's agents today runs a launcher of their own:
it opens a window of their terminal program, `herdr --session <name>`
in it, titled so their window manager files it. Ghostty 1.4.0 forces a
window's title by `--title`, ignoring the titles a program inside sets,
and runs a command by `-e`. On macOS its command line starts no window,
and `open -na Ghostty.app --args …` does, `open --env` giving the new
process a variable. Terminal 2.15 on macOS 27 answers AppleScript's
`do script` with the new window's tab, whose `custom title` shows in
the window's title beside what the person's Terminal profile adds.
herdr 0.9.5 attaches a client to a named session by `herdr session
attach <name>`. How each multiplexer splits into a backend and its
views is `design/research/multiplexers-2026-11-05.md`.

Record 0037, accepted by the user on 2026-11-05 for the three settings
and their per-stream override, decides it. The statement's amendment
stands on agentplot/blueprints main at 36cfd06: 7, 105, 107, 118, 119,
143, 186 to 189, the vocabulary, C.5, C.7, C.8, section 9 and S90
amended, and 190 and S91 added, and the build reads it there (core.19).
Clauses 184 and 185, S86 to S88 and S89 are claimed by
`a-pool-chooses-the-seat-with-the-most-quota-left` and
`a-places-readiness-is-one-design-across-the-tools`.

The rules this design relies on: core.1 and core.2 (the core names no
terminal and no window program, and one interface per seam, a profile
calling no other), core.13 (no process of flume's outlives its
command), core.14 (the JSON form only gains fields), core.17 (every act
safe to repeat), core.19 (the clauses cited), core.21 and core.22 (no
new project; Ghostty and Terminal.app run by no gate task), core.29
(every herdr call bounded), core.30 (no variable's value in an answer),
core.35 (the site's neutral voice), core.44 and core.45 (a stream's
session, and the two layers, as main has them), core.48 (the view) and
core.49 (the stream's terminal), both drawn with record 0037. The one
rules page is `docs/architecture/core.md`. Where flume has no rule, the
kit standard's code profile binds (kit:specs.3, kit:testing.5).

## Goals / Non-Goals

**Goals:**

- Each stream names its multiplexer, its view and its terminal app,
  the host's where it names none, so a person tries a setup on one
  stream while the others run as they did (143, 186, 190).
- A stage's agent opens in its stream's multiplexer and stays there for
  as long as it lives (190, S91).
- `flume open [<stream>]` shows the stream's session in its view: a
  window titled as the stream declares, or the multiplexer's own app
  (186, S90).
- A new multiplexer is one terminal profile with attach for the views
  it offers, and a new terminal app one window profile (187, 188).

**Non-Goals:**

- A terminal profile for herdr-core, Rex, cmux or TUIOS. Each is a
  change of its own, checked against its release; the research says
  what each offers and what was not confirmed.
- A multiplexer named by a launch's settings, a kind, a repository or a
  label. The stream is where a setup is tried (record 0037).
- A window on a place's own tab or an agent's pane, or a `--view` flag
  for one asking. The herdr plugin's key already opens a place's pane.
- Finding, raising or closing a window already open (core.48).
- A key of the herdr plugin or a command of the Claude Code plugin that
  opens a stream's view.

## Decisions

### 1. The settings

`[host]` takes `multiplexer`, `view` and `terminal_app`, and
`[streams.<stream>]` takes the same three and `title`
(`crates/flume/src/host/config.rs`):

- **`multiplexer`**, a name: the terminal profile. `[host] terminal`, a
  host's word written before, is read as `[host] multiplexer`; a
  `[host]` giving both is refused, `[host] gives both terminal and
  multiplexer; terminal is the word written before multiplexer, so give
  one`. Neither naming one is the printed command, as today.
- **`view`**: `terminal` or `app`, any other value refused as a value
  its key refuses, `[streams.work] view is "tab", where a view is
  terminal or app`. Neither naming one is `terminal`.
- **`terminal_app`**, a name: the window's profile. Neither naming one
  is the printed window.
- **`title`**, a text, on streams alone.

`LOCAL_KEYS` gains `streams.*.multiplexer` alone. A local layer setting
a stream's `view`, `terminal_app` or `title`, or any `[host]` key, is
not taken, the managed layer read alone and status naming the key, as
for every key the local layer may not set (record 0032).

The host gives them as one fact. `flume-seams/src/host.rs` gains:

```rust
/// Where a stream's multiplexer, view or terminal app came from: its
/// stream's key, the host's, or neither, the default standing (186,
/// 190, C.7).
pub enum NamedBy { Stream, Host, Neither }

/// A profile's name and where it came from; the printed command's name
/// where neither names one (105, 190).
pub struct ProfileHeld { pub profile: String, pub from: NamedBy }

/// How a person is shown a stream's agents (186).
pub enum View { Terminal, App }

pub struct ViewHeld { pub view: View, pub from: NamedBy }

/// A stream's terminal and how a person views it (143, 186, 190).
pub struct StreamDisplay {
    pub multiplexer: ProfileHeld,
    pub view: ViewHeld,
    pub terminal_app: ProfileHeld,
}
```

`Stream` gains `multiplexer: Option<String>`, `view: Option<View>` and
`terminal_app: Option<String>` beside decb865's `title`, each read with
a default and skipped where none, so a stream written before reads with
none. `Host` gains `display_for(stream) -> StreamDisplay`, resolved as
`pool_for` resolves a pool: the stream's key, else the host's, else the
default, which the host's reader names, `printed` for both profiles,
so the core names neither (core.1). `NamedBy` and `View` serialize in
kebab case: `stream`, `host`, `neither`, `terminal`, `app`.

### 2. The resolver gives a terminal and a window by name

`Resolver` gains:

```rust
/// The terminal profile named `profile`, none where it carries none
/// (105, 190).
fn terminal(&self, profile: &str) -> Option<Arc<dyn Terminal>>;
fn terminal_profiles(&self) -> Vec<String>;
/// The window profile named `profile`, none where it carries none
/// (105, 188).
fn window(&self, profile: &str) -> Option<Arc<dyn Window>>;
fn window_profiles(&self) -> Vec<String>;
```

`BuiltIn` builds each terminal profile at most once for a command, the
one `[host] multiplexer` names being the very `Arc` the engine is built
with, so the host's herdr is one instance whatever reaches it.
`Engine::new` keeps its `terminal`, the host's own. `profiles::terminal`
builds a profile by name as today; `terminal_profiles()` and a new
`window_profiles()` list them for refusals and `--help`.

### 3. A start opens in its stream's terminal and records it

`claim` in `start.rs` reads `host.display_for(stream).multiplexer` at
each start, beside the session, and records it on the start:

```rust
// flume-seams/src/held.rs, on Start:
/// The terminal its agent opened in, with where it came from; none in
/// a start written before, which is the host's (56, 190).
#[serde(default, skip_serializing_if = "Option::is_none")]
pub terminal: Option<ProfileHeld>,
```

A name the resolver carries no profile for refuses the start as a
setting does, `stage.refused_setting`, `the stream trial names the
multiplexer rex, and no terminal profile is named rex; the terminal
profiles there are: herdr, printed`. `make_place` in `launch.rs`
refuses a launch on that stream the same way before it makes anything
(4; core.8). Nothing else on the host is refused for it.

The engine gains `terminal_of(&Start) -> Result<Arc<dyn Terminal>,
String>`: the start's recorded profile through the resolver, the
host's where it records none. Every call the core makes on an agent
goes through it: send, state, screen, answer_trust and close, in each
file the context names. A recorded profile the resolver no longer
carries reads the agent's state unknown and refuses each call, naming
the profile, which a release that drops a profile would cause and no
configuration can.

The session is resolved as `StreamRoom::session` resolves it today;
"the terminal's own" for a default stream naming none is the stream's
terminal's own, which for herdr stays `flume`.

### 4. A place's terminals

- **Clues.** show and shown_at go to the terminal of the place's live
  start, else its newest start, else its stream's as the configuration
  stands.
- **Tidy.** `tidy(place)` runs on each distinct terminal the place's
  starts recorded, the host's for a start recording none, and on its
  stream's as the configuration stands where it has no start; each
  answer is one `TerminalTidied`, as the list holds today (137). The
  place's held state keeps `terminals`, each terminal a start of its
  opened an agent in, once, so a stage started again elsewhere leaves
  its first terminal in the list; a place written before keeps none.
- **The terminals the host binds.** The host's own, then each declared
  stream's that a profile carries, by the stream's name, each once.
  `kept()` and `watch()` are read of each. `HostStatus` keeps
  `terminal_kept` and `watch` as the host's own and gains `terminals`,
  each `{profile, kept, watch}` (core.14).
- **Attended (119).** `settle_attended` settles every place where any
  terminal the host binds answers `Watch::None`, and `wait` drives a
  settle on the same test. Settling every place is what an attended
  host does today and is safe to repeat (core.17).
- **The caller's session.** `recipient()` asks each terminal the host
  binds for `session()`, in that order, and takes the first named.
- **A recipient.** `ends.rs` asks each terminal the host binds, in
  that order, for `session_state(recipient)` and sends by `send_to` to
  the first that holds it; a `ToSession::NotFound` from every terminal
  is not found, as from one today (43, 95). Recipients stay held as
  names.

### 5. Status

- Each stream's `StreamStatus` gains `multiplexer: ProfileHeld`,
  `view: ViewHeld`, `terminal_app: ProfileHeld` and `title: TitleHeld`,
  and its `refused` says where its multiplexer or terminal app names no
  profile, the terminal app's refusal also given alone as
  `view_refused`, since it refuses the stream's open and none of its
  stages. The person's form gives `multiplexer herdr (the host's),
  view terminal (the host's), terminal app ghostty (the stream's),
  title "Work Herdr" (the stream's title)` beside its session, and
  says "it starts no stage" of the rest of a refusal and "it opens no
  view" of the terminal app's.
- Each start's status gives its `terminal` beside its session, with
  where it came from, none for a start written before, which the
  person's form gives as the host's.
- `HostStatus::terminals` as decision 4 says (56, 143, 190; core.14).

### 6. The terminal seam's attach

`Terminal::attach` takes the view in place of the title, which a
terminal's command does not carry and an app does not take:

```rust
/// attach: for a window, the command that shows the part, the part
/// started first where it did not run; for the terminal's own app, the
/// app told to show it; or a refusal where the terminal holds no part
/// a person can be shown or offers no such view (187).
pub enum Attached {
    Command { command: ViewCommand },
    Shown,
    Refused { reason: String },
}

/// The part named `session`, the terminal's own where none, shown to a
/// person in `view` (187).
fn attach(&self, session: Option<&str>, view: View) -> Attached;
```

`View::Terminal` answers `Command` or `Refused`, `View::App` `Shown`
or `Refused`; the conformance suite holds every profile to it, and the
core refuses any other answer naming the terminal. `ViewCommand` is
decb865's: its program by its full path, its arguments, and its
environment with values, read with a default and skipped where empty.
The core turns it into the names alone before anything prints it.

### 7. herdr's and the printed command's attach

`Herdr::attach` (`crates/flume/src/profiles/terminal/herdr/mod.rs`):

- **The app view** refuses before anything else: `herdr 0.9.5 has no
  app of its own; the Herdr app runs on herdr-core, a backend apart
  with sessions of its own`.
- **The server.** For a name, `socket_of(&Server::Session(name))`, so
  a session not yet running is started headless as open starts it,
  with the caller's `HERDR_*` stripped and the session home set. For
  none, the profile's own server, `Server::Session(SESSION)`, `flume`.
  A profile whose own server is `Server::Socket`, as a test's is,
  refuses its own part: herdr attaches a client by a session's name. A
  session herdr refuses, or one that does not start within the bound,
  refuses in herdr's words (core.29).
- **The program.** The full path of the herdr the profile runs: its
  `program` where it is absolute, else the first executable of that
  name on the `PATH` the command runs under. None found refuses naming
  it. The window then runs the same release whatever its own `PATH`.
- **The arguments.** `session attach <name>`, `flume` for the own.
- **The environment.** `XDG_CONFIG_HOME` where the profile's calls run
  under one: the home `with_session_home` gave, else the command's own
  `XDG_CONFIG_HOME` where it is set. Nothing otherwise, never a
  `HERDR_*`.

The printed command's attach refuses both views, as decb865 has it:
`the printed command holds no session a person can be shown; its
agents run in the commands it printed`.

### 8. The core's open

`flume open [<stream>] [--json]` calls `Engine::open_stream(stream)` in
a new `crates/flume-core/src/open.rs`, `default` where none is named.
In order:

1. The stream, as `Host::streams` gives them, `default` always among
   them; one it does not declare refuses, `the host declares no stream
   <name>; its streams are: …` (4, 186).
2. `host.display_for(stream)`. Its multiplexer through
   `resolver.terminal`, refused naming the profiles where none.
3. The session from `StreamRoom::session(name)`, the claim's own
   resolution (107).
4. For `View::Terminal`: the title, the stream's `title` from `stream`,
   else its name from `stream-name`, as a `TitleHeld`; its terminal app
   through `resolver.window`, refused naming the window profiles where
   none, before attach starts any session; `attach(session,
   View::Terminal)`; then `show(&command, &title)`. A refusal of either
   refuses in its words.
5. For `View::App`: `attach(session, View::App)`, its `Shown` the
   answer and its refusal refused in its words. No title is given and
   the terminal app is not read.

It takes no lock, writes no held state and settles nothing (core.48);
a second call opens a second window (core.17).

```rust
pub struct StreamOpened {
    pub stream: String,
    pub multiplexer: ProfileHeld,
    pub session: SessionHeld,
    pub view: ViewHeld,
    /// None for the app's view, whose app titles its own window.
    pub title: Option<TitleHeld>,
    /// None for the app's view.
    pub terminal_app: Option<ProfileHeld>,
    /// None for the app's view.
    pub command: Option<ViewCommandStatus>,
    pub window: WindowOutcome,
}

pub struct TitleHeld { pub title: String, pub from: TitleFrom }
pub enum TitleFrom { Stream, StreamName }

/// The command by its program, its arguments and its variables'
/// names, never their values (core.30).
pub struct ViewCommandStatus {
    pub program: String,
    pub arguments: Vec<String>,
    pub environment: Vec<String>,
}

pub enum WindowOutcome {
    /// The terminal app opened a window.
    Opened,
    /// No window: the line for a person to run.
    Printed { line: String },
    /// The multiplexer's own app showed the session.
    App,
}
```

The person's form says, for instance, `opened a ghostty window titled
"Work Herdr" (the stream's title) on herdr's session work (the stream's
name)`, `herdr's app showed the session work`, and for the printed
window `run: <line>` on a line of its own, so a person's own launcher
can run what `flume open work` prints.

### 9. The window seam

`flume-seams/src/window.rs` is decb865's: `WindowShown` with `Opened`,
`Printed { line }` and `Refused { reason }`, and `Window::show(command,
title)`, its guarantee one window at most at each asking, nothing run
in it but the command, and an answer within a bound the profile states
(188, 189). It is bound per stream through the resolver by
`terminal_app`. `profiles.rs` gains the seam's marker and module list,
`pub mod window { pub mod ghostty; pub mod terminal; pub mod printed; }`,
and `window(name)`: the printed window for `printed`, ghostty for
`ghostty`, Terminal.app for `terminal`, none otherwise.

### 10. The ghostty window

`crates/flume/src/profiles/window/ghostty/mod.rs`, `NAME = "ghostty"`:

- **On macOS:** `open -na Ghostty.app`, `--env <name>=<value>` for each
  variable of the command's environment in order, then `--args
  --title=<title> -e <program> <arguments…>`, each a word of its own and
  nothing passed through a shell. `open` is bounded at ten seconds and
  waited on. Its exit 0 is `Opened`; any other exit, or none within the
  bound, is `Refused` with its standard error's words, `Unable to find
  application named 'Ghostty.app'` among them.
- **Elsewhere:** `ghostty --title=<title> -e <program> <arguments…>`,
  started in a process group of its own, as the herdr profile starts a
  session's server, its input, output and errors closed, under the host
  user's login environment, `host::login_environment()`, with the
  command's environment over it and nothing else. It is not waited on:
  a start that fails to spawn is `Refused`, and any other is `Opened`.
- `Ghostty::new()` takes the host's system; `for_system(System::MacOs)`
  and `for_system(System::Other)` build either path, so a test on any
  host holds both. `with_programs(open, ghostty)` names a test's
  stand-ins, and `with_login(variables)` its login environment.

Checked against Ghostty 1.4.0 (C.8).

### 11. The Terminal.app window

`crates/flume/src/profiles/window/terminal/mod.rs`, `NAME = "terminal"`:

- **On macOS:** `osascript` with the script, each line an `-e` of its
  own, and the command's line and the title as its two arguments:

  ```applescript
  on run argv
    tell application "Terminal"
      set opened to do script (item 1 of argv)
      set custom title of opened to (item 2 of argv)
      set title displays custom title of opened to true
      activate
    end tell
  end run
  ```

  The line is `exec /usr/bin/env <name>=<value> … <program>
  <arguments…>`, `/usr/bin/env` and its variables left out where the
  command needs none, each word quoted for a POSIX shell where it holds
  anything but letters, digits and `-_./:=@%+,`. `exec` has the attach
  take the place of Terminal's shell. `osascript` is bounded at ten
  seconds and waited on; exit 0 is `Opened`, and any other exit, or
  none within the bound, `Refused` with its standard error's words,
  `Not authorized to send Apple events to Terminal` among them.
- **Elsewhere:** `Refused`, `Terminal.app runs only on macOS`.
- `for_system` and `with_program(osascript)` as the ghostty profile
  has them.

Checked against Terminal 2.15 on macOS 27 (C.8). The window's title is
the custom title beside what the person's Terminal profile adds there.

### 12. The printed window

`crates/flume/src/profiles/window/printed/mod.rs`, `NAME = "printed"`,
the window a stream and a host naming none get. `show` answers
`Printed` with the line: each variable of the command's environment as
`<name>=<set>`, then the program and its arguments, quoted as decision
11 quotes them. It runs nothing. A command with no environment prints
as a person runs it, `/nix/store/…/bin/herdr session attach work`
(core.30).

### 13. The fakes

- **The fake terminal.** `attach(session, view)` answers, by default,
  `Command` of `fake-terminal attach <session>` or `attach own` with no
  environment for `View::Terminal`, and `Shown` for `View::App`. A
  script sets either view's answer. Each asking is logged `attach
  <session> for terminal` or `for app`.
- **The fake window**, `FakeWindow` in
  `crates/flume-fakes/src/window.rs`, logs each show with its command
  and title and answers `Opened`, `Printed` or `Refused` as scripted,
  keeping the windows it opened. It reads no display.
- **The fake host.** `set_title(stream, title)` and
  `set_display(stream, multiplexer, view, terminal_app)` script a
  stream's keys; `set_host_display(multiplexer, view, terminal_app)`
  the host's; `display_for` resolves them as decision 1 says, `fake`
  and `fake-window` standing where neither names one.
- **The fake resolver** holds terminals and windows by name:
  `fake`, the engine's own, and any a test adds by
  `with_terminal(name, FakeTerminal)` and `with_window(name,
  FakeWindow)`, each with a log of its own.

### 14. How it is held

- `flume-seams/src/round_trip.rs`: `ViewCommand`, `Attached`,
  `WindowShown`, `View`, `ViewHeld`, `ProfileHeld`, `StreamDisplay`, a
  `Stream` with its four keys and one written before reading with none,
  and a `Start` with its terminal and one written before with none.
- `flume-fakes`: the fake terminal's attach by view, the fake window,
  the fake host's display and the fake resolver's named terminals, in
  `crates/flume-fakes/tests/fakes.rs`.
- `flume-conformance/src/seams/window.rs`: the window seam's scenarios,
  run against the fake and, through `Seam scenarios run against any
  profile`, against the ghostty and terminal profiles over their
  stand-ins and the printed one. `seams/terminal.rs` gains attach's
  scenarios by view over the fake, and the printed terminal's refusal.
- `flume-conformance/src/engine/stream_terminals.rs`: the
  stream-terminals spec's scenarios over two fake terminals, the S91
  one named `s91_a_stream_under_its_own_terminal`.
- `flume-conformance/src/engine/open.rs`: the stream-views spec's
  scenarios over the fakes, the S90 one named `s90_a_streams_view`.
- `flume-conformance/src/no_tool.rs`: `ghostty` and `osascript` among
  the words (core.1).
- `crates/flume/src/host/config.rs`'s unit tests: the four keys,
  `[host] terminal` read as `multiplexer` and both refused, a bad view
  refused, and the local layer taking a stream's `multiplexer` and not
  its `view`, `terminal_app` or `title`.
- `crates/flume/src/commands/print.rs`'s unit tests: the opened,
  printed and app lines, and a stream's and a start's terminal.
- `crates/flume/tests/profiles/window_ghostty.rs`,
  `window_terminal.rs` and `window_printed.rs` (check-fast): each
  profile's words and environment over stand-ins for `open`, `ghostty`
  and `osascript`, both systems, a stand-in that exits non-zero refused
  in its words, one that never exits refused at the bound, and the
  printed line.
- `crates/flume/tests/commands.rs` (check-fast): `flume open` under the
  printed command refused, an undeclared stream refused, a stream whose
  `terminal_app` names no profile refusing its open alone, a stream
  whose `multiplexer` names `rex` refusing a launch on it while a
  launch on another stream runs, and `--json`'s shape.
- `crates/flume/tests/profiles/tools/terminal_herdr.rs` (check-tools):
  attach under the pinned herdr with a scratch session home and
  one-letter session names: a session not running started headless and
  listed running, the command naming the herdr's full path and `session
  attach w` with `XDG_CONFIG_HOME` the scratch home, a client it runs
  attaching to that server, the own session of a socket server refused,
  and the app view refused.
- `crates/flume/tests/profiles/tools/open.rs` (check-tools): S90
  through the binary under the pinned herdr, the ghostty profile over a
  stand-in `open` or `ghostty` and the terminal profile over a stand-in
  `osascript` first on the binary's `PATH`, Terminal.app held by its
  refusal on a host that is not macOS; and S91's binary part, a stream
  under the printed command beside one under herdr, status giving each
  agent's terminal.

### 15. The site

Every page speaks to any host's operator (core.35).

- `reference/commands.md`: `flume open [<stream>]`, what it answers and
  what refuses.
- `reference/configuration.md`: `[host] multiplexer`, `terminal` read
  as it, `view` and `terminal_app`, their values and defaults, and a
  stream's four keys, which layer sets each.
- `guides/places-and-herdr.mdx`: a stream's view, a window manager
  filing a window by its title, the printed line where no terminal app
  is named, herdr's lack of an app, and trying a multiplexer on one
  stream.
- `guides/agent-settings.md`, under "Streams": a stream's multiplexer,
  view, terminal app and title beside its session.
- `reference/state.md`: status's fields of decision 5.
- `roadmap.md`: the stream's terminal and view built, and profiles for
  herdr-core, Rex, cmux and TUIOS not yet written.

### 16. The clause list

`clauses.rs` reads the statement at the revision of blueprints main
the amendment lands at: Part A gains 186 and 190, Part B 187 to 189,
and the scenarios run to S91. A clause of that revision another change
builds and has not built, 184, 185 and S86 to S89 among them where they
stand there unbuilt, stays named unbuilt by that change. 186 to 190, S90
and S91 are cited by this change's scenarios.

## Risks / Trade-offs

- **Ghostty and Terminal.app are not pinned.** A release that changes
  their words breaks the window and nothing else, and the gate does not
  see it, since no gate task opens a window. The stand-ins hold the
  words flume passes, and C.8 names the releases they were checked
  against.
- **`open` and `osascript` answer once the display has the request.** A
  Ghostty that starts and fails at once shows its error in its own
  window, not in flume's answer.
- **Terminal.app's first asking.** macOS asks the person to let the
  program running flume control Terminal; an asking the person has not
  allowed within the bound is refused in `osascript`'s words, and the
  next succeeds once they allow it.
- **Terminal.app's title.** The person's Terminal profile may add the
  process or the size beside the custom title, so a window manager
  filing by the whole title should match the stream's title within it.
- **Terminal.app's line.** `do script` types the command's line into
  the window's shell, so a variable's value it needs shows there, the
  sessions' home under a test's herdr among them; the answer names the
  variable alone (core.30).
- **A window per asking.** A launcher bound to a key opens a window each
  press; finding the one already open is the window manager's (core.48).
- **The session started for the window.** A stream whose session is not
  running is started headless at `open`, before any of its places has
  an agent. It stays as every session's server stays (C.5).
- **Attended by one stream.** A host with one stream under the printed
  command settles every place at each acting command, as a host under
  the printed command does today; that stream's trial costs the others
  nothing but those settles.
- **One place in two terminals.** A stream's multiplexer changed while
  a place runs leaves its live agent where it opened and opens the next
  stage in the new one, so the place's agents are found in two
  terminals until it is tidied in both.

## Migration Plan

- A host whose configuration gives `[host] terminal` runs as before,
  read as `[host] multiplexer`. A host that names no view, no terminal
  app and no title gets the printed command from `flume open`, and
  status gives each stream's title as its name.
- A host's launcher becomes `flume open <stream>`. The title it gave the
  window is declared once as the stream's `title`, and `[host]
  terminal_app = "ghostty"` names the window program.
- A stream written before reads with none of the four keys, and a start
  written before records no terminal, read as the host's.

## Open Questions

None. The settings and their override are the user's, accepted on
2026-11-05; the rest is record 0037's.
