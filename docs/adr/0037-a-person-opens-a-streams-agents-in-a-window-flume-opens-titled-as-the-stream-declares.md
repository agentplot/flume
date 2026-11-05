---
number: 37
title: A person opens a stream's agents in a window flume opens, titled as the stream declares
status: proposed
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# A person opens a stream's agents in a window flume opens, titled as the stream declares

Status: proposed. The user asked for it on 2026-11-05 and asked the same day to rush it: "rather than maintaining aliases for launching my multiplexers, I could have a command that uses Flume to open the multiplexer for a given stream. I guess it would be at the stream level, right? This way as I add new multiplexers like Rex or TUIOS, I wouldn't have to keep maintaining custom scripts." It is stated on the agentplot/blueprints branch `design/flume-a-stream-opens-in-a-window` at b557a2c, proposed until the user's yes. The record waits on the user's yes once they have seen it as a record, on their review of the change `flume-opens-a-streams-multiplexer`.

Deciders: the user, who asked for one command, at the stream's level, that opens the stream's multiplexer, so that a new multiplexer needs no new script. The brief that carried their words gave the window's title as the stream's own, declared with the stream, so a window manager can file it. The writing agent's: the window as a seam of its own, bound once for the host; the terminal seam's attach; the command's name and form; the title's default; both keys in the managed layer alone; what the printed command gives; and that flume keeps nothing of a window.
Sources: `agentplot/blueprints:design/flume/requirements.md` at b557a2c, clauses 105, 107, 120, 143 and 186 to 189, B.5, B.8, B.9, C.5, C.7, C.8 and S90; records 0018, 0026, 0031 and 0032; rules core.1, core.2, core.13, core.14, core.17, core.29, core.30, core.35; `crates/flume-seams/src/terminal.rs` (`Terminal`), `crates/flume/src/profiles.rs` (`terminal`), and, on the change `each-stream-has-its-own-session-and-labels-and-the-host-a-local-layer`, `crates/flume-core/src/room.rs` (`StreamRoom::session`) and `crates/flume/src/profiles/terminal/herdr/mod.rs` (`socket_of`, `start_session`); Ghostty 1.4.0's `+help` and its `title` option's documentation; herdr 0.9.5's `session attach`.

## Context and problem statement

Record 0031 runs each stream's agents in a herdr session of their own, the one the host names for the stream, else the stream's name, and flume's own for the default stream. A person sees a stream's agents by attaching a herdr client to that session in a window of their own terminal program. Today each person keeps a launcher of their own for each stream: it opens a window of their terminal program, runs `herdr --session <name>` in it and titles the window so their window manager files it, as `<Row> Herdr`. Each new multiplexer, Rex or TUIOS, would be a new launcher for every stream on every host.

flume already knows the host's terminal (`[host] terminal`), each stream's session and the herdr flume drives. It knows neither the program that opens a window on the host's display nor the title a stream's window wants.

## Considered options

1. Keep a launcher per person, per stream and per multiplexer.
2. `flume open <stream>` opens a window on the stream's session through a window seam of its own, bound once for the host, its title declared with the stream; the terminal seam's attach gives the command the window runs.
3. The same command, the window opened from a command line the host's configuration writes out with places for the title and the view's command, filled by flume.
4. The terminal profile opens the window itself, the herdr profile running Ghostty.
5. `flume open <stream>` gives only the view's command, and the person's own launcher opens and titles the window.

## Decision outcome

Option 2.

1. **The command.** `flume open [<stream>]`, `default` where none is named. A stream the host does not declare is refused naming it. The answer, printed or as its JSON with `--json`, gives the stream, its session and where it came from, the title and where it came from, the command and what the window did (186; core.14).
2. **The session.** The stream's own part of the terminal, resolved as a start resolves it from the stream (record 0031): the stream's `session`, else its name, and the terminal's own for the default stream naming none. It is the same resolution, called by both, so a window and a stage's agent never disagree on where the stream runs (107).
3. **The title.** `title` in `[streams.<stream>]`, else the stream's name. flume adds nothing to it and builds it from nothing else: the window manager's naming is the host's, written in the host's configuration (186).
4. **The terminal's attach (187).** Given the session and the title, the terminal answers the command that shows it, with what the command needs of its environment, the session started first where it does not run; or that it showed the part in a window of its own, for a terminal that draws its own windows; or a refusal. Under herdr it is `<herdr> session attach <name>`, by the full path of the herdr the profile runs, the session's server found or started headless as the profile's open starts it, and `XDG_CONFIG_HOME` in the command's environment where the profile's calls run under one. The printed command refuses: it holds no session.
5. **The window seam (188, 189).** Given a command, its environment and a title, a window running the command under that title, or the command given for a person to run, or a refusal. It is bound once for the host by `[host] window`. The ghostty profile opens `open -na Ghostty.app --args --title=<title> -e …` on macOS and `ghostty --title=<title> -e …` elsewhere. A host that names no window gets the printed command, which gives the command for the person to run. A name no profile carries refuses `flume open` alone.
6. **The window is the person's.** flume keeps, reads, finds again and closes nothing of a window. It opens one only at a person's asking, never at a launch, a settle or a start. A second asking opens a second window, and changes nothing flume holds (core.17). On macOS the display starts the window's process. Elsewhere flume starts it in a session of its own and does not wait on it, as the herdr profile starts a session's server. Neither the window nor the view in it is a process of flume's (120).
7. **The managed layer alone.** A stream's `title` and `[host] window` arrange the host's display, not how its work runs from hour to hour, so the local layer does not set them (record 0032).

### Why not the others

Option 1 is what the user asked to end. Option 3 needs no code for a new window program, but it moves the knowledge of how to open one into every host's file, as a line with places to fill, which is the launcher again. It also leaves flume running a program no profile vouches for, where record 0026 keeps a program's entry in flume's code, checked against its release. Option 4 breaks core.2, a profile reaching a tool another seam's profile could own, and makes every pair of multiplexer and window program a profile of its own. Option 5 leaves the title and the window to the person's launcher, and so leaves a launcher per stream.

`flume open <stream>` was chosen over `flume stream open <stream>`, which would begin a group of stream commands with one member. A later `flume open` of a place's own tab can take a flag. The name is cheap to change before it lands.

### Consequences

- The statement gains 186 to 189, B.8 the window, with the binding moved to B.9, C.8 the window's profiles and S90. It amends the preamble's seams, the vocabulary, 105, 143, C.5 and C.7, and section 9's command line and profiles (core.19). The amendments are on the blueprints branch until the user accepts them, and the change is built once they stand on blueprints main.
- Rule core.48 is drawn from this record. core.1 names the window program among the tools code outside a profile names none of, and core.13's details name the window and the terminal's server as processes that are not flume's.
- The terminal seam gains attach. A new seam, the window, gains its interface, its fake and its scenarios. Its profiles are ghostty and the printed command.
- A host that names no window gets the command. `flume open madswan` then prints the line a person's own launcher ran, and a launcher of one line can run it.
- A new multiplexer is one terminal profile with its attach, and a new window program one window profile, each written once for every host.
- Ghostty is not pinned by the tree: it is a program of the host's display, which no gate task runs. The ghostty profile is held by tests of the command line it builds, run against a stand-in for `open` and `ghostty`, and its flags are checked against Ghostty 1.4.0's documentation. A Ghostty that changes `title` or `-e` breaks the window and nothing else.
- The change `flume-opens-a-streams-multiplexer` builds it, after the change `each-stream-has-its-own-session-and-labels-and-the-host-a-local-layer` lands, since it reads that change's sessions.
