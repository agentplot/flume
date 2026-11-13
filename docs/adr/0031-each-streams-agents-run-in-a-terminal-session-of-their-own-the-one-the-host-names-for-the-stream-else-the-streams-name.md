---
number: 31
title: Each stream's agents run in a terminal session of their own, the one the host names for the stream, else the stream's name
status: accepted
date: 2026-11-03
decision-makers:
- Chuck Swanberg
---

# Each stream's agents run in a terminal session of their own, the one the host names for the stream, else the stream's name

Status: accepted by the user on 2026-11-07, on their card. The user decided it on 2026-11-03 on their Pending You card (req_7e8ef2d0): "Yes: a session per stream, its name by default." It is stated on boomdrivedev/blueprints main at 7367604, which amends record 0020.

Deciders: the user, who chose a session per stream, named by the stream's name unless the host names another. The writing agent's: that the host's default stream keeps flume's own session unless its own table names one, that a launch names a session for its place by its settings alone, that the session is read at each start, how status names it, and how the herdr profile keeps a place's tabs and its base's workspaces across sessions.
Sources: `boomdrivedev/blueprints:design/flume/requirements.md` at 7367604, clauses 32, 107 and 143, C.5's herdr row, C.7 and S82; records 0018 and 0020; rules core.1, core.18, core.29, core.30; `crates/flume/src/profiles/terminal/herdr/mod.rs` (`SESSION`, `Server`, `Herdr::socket`, `start_session`, the workspace and tab records), `crates/flume-seams/src/terminal.rs` (`Terminal::open`'s `beside`), `crates/flume-core/src/settings.rs` (`placeless`).

## Context and problem statement

Under herdr every place's agents open in one session, flume's own, `flume`: one workspace per repository and base, one tab per place (record 0018). Record 0020 runs a host's work in streams, `work` and `personal` among them, each with its own bound, priority and pool. Every stream's agents still open in the one session. A person who wants to see only the work stream's agents opens flume's session and finds every stream's there, and a person who keeps a herdr session per kind of work can put flume's agents in none of them.

C.5's herdr row already said a pane opens in "the herdr session the launch's settings name, flume's own otherwise". No setting names one: `--agent` gives `program`, `model`, `instructions`, `beside` and `mode`, and the profile opens every pane in `flume`.

## Considered options

1. Keep flume's own session for every place.
2. A session per stream: the one the host's configuration names for the stream, else the stream's name; a launch may name another for its place; the host's default stream keeps flume's own.
3. A workspace per stream inside flume's own session.
4. A session per repository.

## Decision outcome

Option 2.

1. **Where an agent runs (107).** Where the launch's settings and the stage's declaration name no place in the terminal, a stage's agent runs in its place's stream's own session: the one the host's configuration names for the stream by `session` in `[streams.<stream>]`, else the session named by the stream's name. A place in the host's default stream runs in flume's own session, unless `[streams.default]` names one. A split beside a pane the settings name stays beside that pane, wherever that pane is.
2. **The launch's session.** A launch names a session for every stage or for one by `--agent [<stage>.]session=<name>`, its settings' `session`. It comes before the stream's. Only the launch's layer gives it: a kind or a repository names nothing a host owns (158), and the host names a stream's session in the stream's table, not in its tables for a kind. A stage's `beside` comes before any session.
3. **Read at each start.** The stream's session is read from the host's configuration as it stands at each start, as the rest of the host's configuration is (32), so a change to it takes the next start. An agent already started stays where it opened.
4. **Status.** Each agent's session, with where it came from: the launch, the stream's `session`, the stream's name, or flume's own. Each stream gives its session the same way (56).
5. **Under herdr.** The session is herdr's session of that name, its server started headless where none runs, as flume's own is today. `herdr --session work` shows every agent of the work stream. Within each session the layout is record 0018's: one workspace per repository and base, labelled `<repository> <base>`, one tab per place. A base whose places run in two streams has a workspace in each stream's session. The profile records each workspace and tab with its server, and a place's tidy closes its tab in every session it opened one in, and each of its base's workspaces once it holds nothing of a live place and nothing flume did not make (137, 138). A name the pinned herdr refuses for a session refuses the open in herdr's words (core.29).
6. **The printed command** prints its command as before. A session names nothing there.

### Why not the others

Option 1 is what the user asked to change. Option 3 keeps every stream in one herdr server, so a person still opens all of them to see one. It also breaks the workspace per repository and base, which the bolts crew runs are kept in. Option 4 splits one stream's agents across sessions and puts two streams' agents of one repository in one session, where the user asked for the stream's agents together.

### Consequences

- Clauses 32, 107 and 143, C.5's herdr row and C.7 are amended, and S82 added, on blueprints main at 7367604 (core.19).
- Rule core.44 is drawn from this record.
- The terminal seam's open takes where the agent runs, a pane to open beside, a session by its name, or the terminal's own, in place of the pane alone.
- A host that declares no stream runs every place in `default`, so its agents stay in flume's own session as before. A host that declares streams finds each stream's next starts in the stream's session. An agent running when the change lands stays where it opened, and its place's tab there is closed at the place's tidy.
- Each session's server runs the herdr plugin's watch, which settles every place. Settles are safe to repeat (core.17), so several watches cost more settles and change nothing.
- The change `each-stream-has-its-own-session-and-labels-and-the-host-a-local-layer` builds it.
