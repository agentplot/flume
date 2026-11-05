---
number: 37
title: A stream names its multiplexer, its view and its terminal app, the host's where it names none, and a person opens its agents in that view
status: accepted
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# A stream names its multiplexer, its view and its terminal app, the host's where it names none, and a person opens its agents in that view

Status: accepted by the user on 2026-11-05 in their Plannotator Inbox, on the change `flume-opens-a-streams-multiplexer`. They accepted a window flume opens titled as the stream declares, "Yes, accept it", and the same day, having read the research on how each multiplexer splits into a backend and the ways a person views it, ruled the three settings and their override, "Yes, amend it to multiplexer, view and terminal app, per Mac with a per-stream override", and the scope, "One change for all of it". They asked for it that morning and asked to rush it: "rather than maintaining aliases for launching my multiplexers, I could have a command that uses Flume to open the multiplexer for a given stream. I guess it would be at the stream level, right? This way as I add new multiplexers like Rex or TUIOS, I wouldn't have to keep maintaining custom scripts." Their words that led to the three settings: "Can this be set per stream as well? I would like to be able to test different setups using different streams to confirm that it all works." and "theoretically they all could have a backend CLI that you use as the multiplexer. Some of them could use only a native app, while others could be a native app or a terminal like Ghostty." It is stated on agentplot/blueprints main at 36cfd06, which supersedes 186 to 189 and S90 as main stated them at d680e35.

Deciders: the user, who asked for one command, at the stream's level, that opens the stream's multiplexer, so that a new multiplexer needs no new script; who ruled the three settings, `multiplexer`, `view` and `terminal_app`, each the host's with a stream's over it; and who ruled that one change builds the stream's multiplexer and its view together. The brief that carried their words gave the window's title as the stream's own, declared with the stream, so a window manager can file it. The writing agent's: the window as a seam of its own; the terminal seam's attach given the view; the command's name and form; the title's default; which of the keys the local layer sets; `[host] terminal` read as `[host] multiplexer`; a start recording its terminal and a recipient found in the terminal that names it; flume run attended while any terminal the host binds keeps no watch; what a name no profile carries refuses; Terminal.app driven by `osascript`; and that flume keeps nothing of a window.
Sources: `agentplot/blueprints:design/flume/requirements.md` on main at 36cfd06, clauses 4, 7, 43, 56, 105, 107, 118 to 120, 137, 143, 186 to 190, B.5, B.8, B.9, C.5, C.7, C.8, S90 and S91; `design/research/multiplexers-2026-11-05.md`, read on the host on 2026-11-05; records 0018, 0026, 0031 and 0032; rules core.1, core.2, core.13, core.14, core.17, core.29, core.30, core.35 and core.44; `crates/flume-seams/src/terminal.rs` (`Terminal`, `Attached`), `crates/flume-seams/src/window.rs` (`Window`), `crates/flume-seams/src/held.rs` (`Start`), `crates/flume-seams/src/resolver.rs` (`Resolver`), `crates/flume/src/profiles.rs` (`terminal`), `crates/flume-core/src/room.rs` (`StreamRoom::session`), `crates/flume-core/src/settle.rs` (`settle_attended`) and `crates/flume/src/profiles/terminal/herdr/mod.rs` (`socket_of`); Ghostty 1.4.0's `+help` and its `title` option's documentation; Terminal 2.15's scripting dictionary on macOS 27, its `do script` and a tab's `custom title`; herdr 0.9.5's `session attach`.

## Context and problem statement

Record 0031 runs each stream's agents in a session of the terminal of their own: the herdr session the host names for the stream, else the stream's name, and flume's own for the default stream. A person sees a stream's agents by attaching a herdr client to that session in a window of their own terminal program, titled so their window manager files it, as `<Row> Herdr`. Each person keeps a launcher for each stream, and each new multiplexer, Rex or TUIOS, would be a new launcher for every stream on every host.

flume knows the host's terminal, `[host] terminal`, one for every stream, each stream's session and the herdr flume drives. It knows neither the program that opens a window on the host's display nor the title a stream's window wants. And a multiplexer is two things, as read on the host on 2026-11-05: a backend the agents run in, and the ways a person views it. herdr 0.9.5 is a backend with a terminal client and no app. The Herdr app runs on herdr-core, a backend apart with sessions of its own. Rex has a server and a command line but no terminal client, and its app shows a session. cmux.app holds its own terminals, while cmux's server is a separate one with a terminal client. TUIOS has a daemon and a terminal client. So the backend, the view and the terminal app a terminal client runs in are three choices, and a person trying one wants it on one stream while the others run as they did.

## Considered options

For how a person opens a stream's agents:

1. Keep a launcher per person, per stream and per multiplexer.
2. `flume open <stream>` shows the stream's session in its view: a window that a window seam of its own opens under the title declared with the stream, running the command the terminal seam's attach gives; or the terminal's own app, which attach tells to show the session.
3. The same command, the window opened from a command line the host's configuration writes out with places for the title and the view's command, filled by flume.
4. The terminal profile opens the window itself, the herdr profile running Ghostty.
5. `flume open <stream>` gives only the view's command, and the person's own launcher opens and titles the window.

For where the choices are made:

6. One window program for the host, `[host] window`, and the terminal once for the host.
7. Three settings, `multiplexer`, `view` and `terminal_app`, each in `[host]` as the host's and in `[streams.<stream>]` as the stream's over it.
8. The three settings on the streams alone, with no host's default.

## Decision outcome

Options 2 and 7.

1. **The command.** `flume open [<stream>]`, `default` where none is named. A stream the host does not declare is refused naming it. The answer, printed or as its JSON with `--json`, gives the stream, its multiplexer, its session, its view, its title and its terminal app, each with where it came from, the command and what the window did (186; core.14).
2. **The three settings (186, 190, C.7).** Each is a key of `[host]`, the host's, and of `[streams.<stream>]`, the stream's over it.
   - `multiplexer`: the terminal profile a stream's agents run in. herdr is the one built, and the printed command stands where neither names one. herdr-core, Rex, cmux and TUIOS each become a name it takes once a profile is written for it.
   - `view`: `terminal`, a window a terminal app opens running the multiplexer's attach command under the stream's title; or `app`, the multiplexer's own app told to show the stream's session, which titles its own window. `terminal` stands where neither names one.
   - `terminal_app`: `ghostty`, or `terminal` for Terminal.app, driven by AppleScript with a custom title. The printed command stands where neither names one: no window, and the command given for the person to run. macOS has no setting for a default terminal, and a `.command` file it opens in Terminal carries no title, so flume picks no terminal app of its own.
3. **The stream's terminal (190).** A stage's agent opens in its stream's multiplexer, read at each start as its session is (record 0031), and recorded with the start with where it came from. Every later call on that agent goes to the terminal its start recorded, and a place is tidied in each terminal its starts recorded, so an agent stays where it opened whatever the configuration names by then. A start written before records none and is the host's. A launch's settings, a kind and a repository name no multiplexer: the stream is where a person tries a setup.
4. **A recipient's terminal (7, 43).** The calling command's session is the one the first terminal the host binds that names one gives, the host's own first and then each stream's by the stream's name. A notice to a recipient, and its state, go to the first terminal in that order that holds a session by its name. A recipient is held as before, by its name.
5. **Attended (119).** flume runs attended while any terminal the host binds, its own or a stream's, keeps no watch, and every command that acts then settles every place, as an attended host does.
6. **The session.** The stream's own part of its terminal, resolved as a start resolves it, by the same function, so a view and a stage's agent never disagree on where the stream runs (107).
7. **The title.** `title` in `[streams.<stream>]`, else the stream's name. flume adds nothing to it and builds it from nothing else. It titles a window in the `terminal` view; in the `app` view the app titles its own window (186).
8. **The terminal's attach (187).** Given the session and the view: for `terminal`, the command that shows it, with what the command needs of its environment, the session started first where it does not run; for `app`, the running app told to show the session, started first where it does not run; or a refusal. Under herdr the `terminal` view is `<herdr> session attach <name>`, by the full path of the herdr the profile runs, the session's server found or started headless as the profile's open starts it, and `XDG_CONFIG_HOME` in the command's environment where the profile's calls run under one. herdr refuses the `app` view: herdr 0.9.5 has no app of its own. The printed command refuses both: it holds no session.
9. **The window seam (188, 189).** Given a command, its environment and a title, a window running the command under that title, or the command given for a person to run, or a refusal. It is bound for each stream by `terminal_app`, the host's where the stream names none. The ghostty profile opens `open -na Ghostty.app --args --title=<title> -e …` on macOS and `ghostty --title=<title> -e …` elsewhere. The terminal profile has `osascript` tell Terminal.app to `do script` the command's line in a new window and set that tab's custom title, the line and the title passed as its arguments, and refuses elsewhere. The printed command gives the command.
10. **The window is the person's.** flume keeps, reads, finds again and closes nothing of a window. It opens one only at a person's asking, never at a launch, a settle or a start. A second asking opens a second window, and changes nothing flume holds (core.17). On macOS the display starts the window's process. Elsewhere flume starts Ghostty in a process group of its own and does not wait on it, as the herdr profile starts a session's server. Neither the window nor the view in it is a process of flume's (120).
11. **The layers (record 0032).** A stream's `multiplexer` is a key the local layer sets, as its `session` is: where its agents run, which a person or a control plane tries from hour to hour. A stream's `view`, `terminal_app` and `title` arrange the host's display and are the managed layer's alone, as every `[host]` key is.
12. **Names no profile carries.** `[host] terminal`, the word a host wrote before, is read as `[host] multiplexer`, and a host giving both is refused naming both. The host's own multiplexer naming no profile refuses every command, as `[host] terminal` does today. A stream's naming none refuses that stream's launches, its starts and its opens, naming it, and nothing else, and status names it with the stream. A `terminal_app` naming none refuses that stream's open alone. A `view` but `terminal` or `app` refuses the configuration, as any value its key refuses.

### Why not the others

Option 1 is what the user asked to end. Option 3 needs no code for a new window program, but it moves the knowledge of how to open one into every host's file, as a line with places to fill, which is the launcher again. It also leaves flume running a program no profile vouches for, where record 0026 keeps a program's entry in flume's code, checked against its release. Option 4 breaks core.2, a profile reaching a tool another seam's profile could own, and makes every pair of multiplexer and window program a profile of its own. Option 5 leaves the title and the window to the person's launcher, and so leaves a launcher per stream. Option 6 binds one terminal app and one terminal for the host, so a person tries a setup only by changing every stream's, and has no place for a multiplexer shown only by its own app. Option 8 makes a host with one setup name it on every stream.

The writing agent's choices, and the paths not taken:

- A launch's settings naming a multiplexer, as `--agent session=` names a session, would split one stream's agents across terminals by a caller's word, where the user asked for the stream.
- A recipient held with its terminal's name changes every recipient held, and gives a role's address no terminal, where asking the terminals in a fixed order finds the one that names it.
- flume run attended only for the places whose terminal keeps no watch needs each command to know a place's terminal before it settles, where settling every place is what an attended host does today and is safe to repeat (core.17).
- A stream's unknown multiplexer refusing the whole configuration would stop every stream for one stream's trial.
- All three settings in the local layer, or none: a stream's multiplexer is where its agents run, as its session is, while its view and its terminal app arrange the display, as its title does.
- `[host] terminal` refused would stop every host written before.
- Terminal.app opened by a `.command` file carries no title. AppleScript given the command and the title inside its own text would quote them there, where `osascript`'s arguments carry them as written.
- A title passed to an app that titles its own window.

`flume open <stream>` was chosen over `flume stream open <stream>`, which would begin a group of stream commands with one member. A later `flume open` of a place's own tab can take a flag. The name is cheap to change before it lands.

### Consequences

- The statement's 7, 105, 107, 118, 119, 143, 186 to 189, the vocabulary, C.5, C.7, C.8, section 9 and S90 are amended, and 190 and S91 added, on blueprints main at 36cfd06 (core.19). They supersede 186 to 189 and S90 as main stated them at d680e35, and the build reads the statement there.
- Rule core.48 says the view, core.49 is drawn for the stream's terminal, and core.1 names Terminal.app and `osascript` among the tools code outside a profile names none of. core.13's details name the window and the terminal's server as processes that are not flume's.
- The terminal seam gains attach given a view. A new seam, the window, gains its interface, its fake and its scenarios, and its profiles are ghostty, Terminal.app and the printed command. The resolver gives a terminal and a window by a profile's name, and a start records its terminal.
- A host that names no terminal app gets the command. `flume open madswan` then prints the line a person's own launcher ran, and a launcher of one line can run it.
- A new multiplexer is one terminal profile with attach for the views it offers, and a new terminal app one window profile, each written once for every host and tried on one stream. herdr-core, Rex, cmux and TUIOS have none yet; the research names what each offers and what was not confirmed.
- Ghostty and Terminal.app are not pinned by the tree: they are programs of the host's display, which no gate task runs. Their profiles are held by tests of the command lines they build, run against stand-ins for `open`, `ghostty` and `osascript`, and their flags are checked against Ghostty 1.4.0 and Terminal 2.15.
- The change `flume-opens-a-streams-multiplexer` builds it, all in one change as the user ruled, on main as `each-stream-has-its-own-session-and-labels-and-the-host-a-local-layer` left it, landed at 70afbf8.
