# Tasks

Each group is one session's work for an agent that reads this change
(proposal, design, specs) and its group, and builds on main and on the
groups before it. Record 0037 is accepted and the statement's
amendment stands on agentplot/blueprints main at d680e35. Groups 1 and
2 are built on main as it stands. Groups 3 and 4 are built once
`each-stream-has-its-own-session-and-labels-and-the-host-a-local-layer`
has landed, on main as it leaves `room.rs`, `host.rs`, `config.rs` and
the herdr profile, merged into this branch first, its `Stream`, its
terminal seam and its fake terminal resolved beside group 2's. A
group's last task runs the checks it touched
through sluice. Every conformance scenario a group adds cites its
clauses as data (106). The engine's scenarios name no tool (core.1).
Tests reach no window of the host's display, no herdr session of the
host's and no configuration directory of the user's: a window program
in a test is a stand-in of the test's own that records its words, its
environment and its process group, and every host configuration,
session home and state directory a test names is a scratch one of the
test's own. A herdr session a test starts is named by one letter of its
test's own, `w` for the stream's, as the sessions' tests name theirs,
and a test whose socket's path leaves no room fails naming its bytes.

## 1. The decision recorded

- [x] 1.1 Add record 0037, proposed, "A person opens a stream's agents in a window flume opens, titled as the stream declares", quoting the user's words of 2026-11-05, with its deciders, sources (the statement at 3cc35fa on `design/flume-a-stream-opens-in-a-window`; records 0018, 0026, 0031, 0032), context, five considered options, outcome, why not the others and consequences (105, 120, 143, 186 to 189, C.5, C.7, C.8, S90; core.19). True when `sluice exec main -- moon run root:adrs root:format` passes.
- [x] 1.2 Draw core.48 in `docs/architecture/core.md` under "Settling, surfaces and state" with its details, have core.1 name the window program and its details `ghostty` and `Ghostty.app`, and have core.13's details name a session's server the terminal's profile starts and a window the window seam hands to the display as no process of flume's, each citing record 0037. Have `site/src/content/docs/architecture/rules.md` mirror core.1 and core.48 as written (core.19, core.35). True when `sluice exec main -- moon run root:format site:prose site:build` passes.
- [x] 1.3 Run the checks of the projects this group touched. True when `sluice exec main -- moon run root:format root:adrs root:changes site:prose site:build` passes.
- [x] 1.4 Set record 0037 accepted on the user's word of 2026-11-05, "Yes, accept it", citing the statement on agentplot/blueprints main at d680e35, and its build's order as the user ruled, "I want to build it now": groups 1 and 2 on main as it stands, groups 3 and 4 after the change they read lands. Have the proposal, the design and this list cite that revision, and write design 13, how a multiplexer's native app is added as a terminal profile whose attach answers that it showed the part (187; core.2, core.19). True when `sluice exec main -- moon run root:format root:adrs root:changes` passes.

## 2. The seams and their fakes

- [ ] 2.1 Give `flume-seams` design 2's `ViewCommand`, `Attached` and `Terminal::attach`, a new `window.rs` with design 4's `WindowShown` and `Window`, re-exported, and `Stream::title`, read with a default and skipped where none. Have every `Terminal` implement attach: the fake as design 9 has it, the printed command refusing as design 3 has it, and herdr refusing `not yet built` until 4.3. Add to `round_trip.rs` the seam-contracts spec's scenario of a view, a window and a stream (143, 187, 188; core.1, core.2, core.14). True when `sluice exec main -- moon run seams:test fakes:test conformance:test flume:test` passes, every existing test passing unchanged.
- [ ] 2.2 Give `flume-fakes` design 9's fake terminal attach, its log line and its scripts; a new `FakeWindow` in `window.rs`, logging each show and answering as scripted; and the fake host's scripted title. Add to `crates/flume-fakes/tests/fakes.rs` the seam-fakes spec's two scenarios (106, 143, 187, 188). True when `sluice exec main -- moon run fakes:test` passes.
- [ ] 2.3 Add to `flume-conformance` `seams/window.rs`, the window seam's scenarios generic over `Window` below a marker of its own, run against the fake, and to `seams/terminal.rs` attach's scenario over the fake, as the seam-contracts spec has them. Change "seven seams" to eight in `One interface per seam`'s test, and add `ghostty` to `no_tool.rs`'s words (106, 187 to 189; core.1). True when `sluice exec main -- moon run conformance:test` passes, the no-tool scenario finding no `ghostty` outside a `profiles` directory.
- [ ] 2.4 Run the checks of the projects this group touched. True when `sluice exec main -- moon run seams:format seams:lint seams:test fakes:format fakes:lint fakes:test conformance:format conformance:lint conformance:test flume:format flume:lint flume:test` passes.

## 3. The core's open and status

- [ ] 3.1 Add `crates/flume-core/src/open.rs` with `Engine::open_stream(stream, window)` as design 1 has it, the session from `StreamRoom::session` and the title from the stream's `title`, else its name; `Answer::Opened` with `StreamOpened`, `TitleHeld`, `TitleFrom`, `ViewCommandStatus` and `WindowOutcome`; and its refusals. It takes no lock, writes no held state and settles nothing (107, 186 to 188; core.14, core.17, core.30, core.48). True when `sluice exec main -- moon run core:test conformance:test` passes, with `engine/open.rs` holding the stream-windows spec's scenarios over the fakes, the one S90 cites named `s90_a_streams_window`.
- [ ] 3.2 Give `StreamStatus` its `title`, resolved as `open_stream` resolves it, in the JSON form, and have the person's form give it beside the stream's session (56, 143, 186; core.14). True when `sluice exec main -- moon run core:test conformance:test` passes, `engine/streams.rs` holding the streams spec's scenario of a title and none over the fake host.
- [ ] 3.3 Run the checks of the projects this group touched. True when `sluice exec main -- moon run core:format core:lint core:test conformance:format conformance:lint conformance:test` passes.

## 4. The profiles and the command

- [ ] 4.1 Have `crates/flume/src/host/config.rs` read `[streams.<stream>] title`, a text, into `Stream::title`, and `[host] window`, a name, into `HostConfig::window`, neither among `LOCAL_KEYS`, as design 7 has it (143, C.7; core.45). True when `sluice exec main -- moon run flume:test` passes, with `config.rs`'s unit tests holding the streams spec's three scenarios and the window-profiles spec's scenario of a window the local layer sets.
- [ ] 4.2 Add `profiles/window/ghostty` and `profiles/window/printed` as designs 5 and 6 have them, the seam's marker and module list in `profiles.rs`, `window()` and `window_profiles()`. Build the ghostty profile for a system it is given, `macos` or another, its default the host's, so both paths are held on every host that runs the gate (120, 188, 189, C.8; core.1, core.2, core.30). True when `sluice exec main -- moon run flume:test conformance:test` passes, with `crates/flume/tests/profiles/window_ghostty.rs` and `window_printed.rs` holding the window-profiles spec's scenarios over stand-ins for `open` and `ghostty`, and running the window seam's scenarios against both profiles.
- [ ] 4.3 Give the herdr profile its attach as design 3 has it, the server found or started by `socket_of`, the program's full path found as its calls find it, and `XDG_CONFIG_HOME` from `with_session_home` or the command's own (107, 187, C.5; core.29). True when `sluice exec main -- moon run flume:test flume:tools` passes, with `crates/flume/tests/profiles/tools/terminal_herdr.rs` holding the terminal-profiles spec's scenarios of herdr's attach under the pinned herdr and a scratch session home, and `terminal_printed.rs` the printed command's refusal.
- [ ] 4.4 Add `flume open [<stream>] [--json]` to `commands/mod.rs`, binding the window `[host] window` names for this command alone and needing no repository, and give `print.rs` the opened, printed and terminal-shown lines, the printed window's line on a line of its own, each variable by name alone (61, 105, 186; core.14, core.30). True when `sluice exec main -- moon run flume:test` passes, with `print.rs` unit tests of the three lines and `crates/flume/tests/commands.rs` holding the command-line spec's scenarios of open under the printed command and of an undeclared stream, and the window-profiles spec's scenario of an unknown window.
- [ ] 4.5 Add `crates/flume/tests/profiles/tools/open.rs`, listed in the tools' `main.rs`: design 10's S90 through the binary under the pinned herdr, the ghostty profile over a stand-in placed first on the binary's `PATH`, and the printed window on the same host naming none (186 to 189, S90). True when `sluice exec main -- moon run flume:tools` passes with the command-line spec's two scenarios through the binary, and every existing tools test passes unchanged.
- [ ] 4.6 Run the checks of the projects this group touched. True when `sluice exec main -- moon run flume:format flume:lint flume:test flume:tools conformance:test` passes.

## 5. The site

- [ ] 5.1 Write design 11 into `reference/commands.md`, `reference/configuration.md`, `guides/places-and-herdr.mdx`, `guides/agent-settings.md`, `reference/state.md` and `roadmap.md`: `flume open [<stream>]`, `[host] window` with its profiles, `[streams.<stream>] title`, a window manager filing a window by its title, the printed line where a host names no window, and status's title. Write it to any host's operator, naming Ghostty and herdr as products flume integrates with and no host, team or word of ours (core.35). True when `sluice exec main -- moon run site:prose site:build` passes.
- [ ] 5.2 Run the checks of the project this group touched. True when `sluice exec main -- moon run site:prose site:build root:format` passes.

## 6. Coverage and the gate

- [ ] 6.1 Have the suite's clause list read the statement at the revision of agentplot/blueprints main that states 186 to 189 and S90: Part A gaining 186, Part B 187 to 189, the scenarios running to S90, and every clause of that revision another change has not yet built named unbuilt by it, as the clause-coverage spec has it (core.19). True when `sluice exec main -- moon run conformance:test conformance:coverage` passes, 186 to 189 and S90 cited by this change's scenarios.
- [ ] 6.2 Run the checks of the projects this change touched. True when `sluice exec main -- moon run seams:format seams:lint seams:test core:format core:lint core:test fakes:format fakes:lint fakes:test conformance:format conformance:lint conformance:test conformance:coverage flume:format flume:lint flume:test flume:tools site:prose site:build root:format root:adrs root:changes root:specs` passes.
