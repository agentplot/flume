---
number: 48
title: A stream names the session its agents of a kind, or of one stage of a kind, open in, below the stream and beside its own
status: proposed
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# A stream names the session its agents of a kind, or of one stage of a kind, open in, below the stream and beside its own

Status: proposed. The need is the user's, on 2026-11-05: "I'm realizing that mixing construction sessions from Flume with research and design sessions makes it hard to find and keep track of the research and design sessions. We probably want to put those into different Herdr sessions. I don't know if that is possible, whether that has to be explicit at the stream level (so we need different streams), or whether we can do it below the stream level". Which part of the terminal and where the host says it are the construct stage's proposal, for the user to read: the code waits on that read. It stands proposed until the user's yes.

Deciders: the user, on the need; the construct stage of `where-an-agent-opens-is-given-by-kind-and-by-stage`, on 2026-11-05, on the design.
Sources: `agentplot/blueprints:design/flume/requirements.md` at cd8fd73, clauses 32, 70, 107, 143, 180 to 182, 186, 187, 190, C.5's herdr row and C.7, and its amendment at 72a6f12 on the branch `design/flume-where-an-agent-opens`, which adds 199, S106 and S107; records 0018, 0020, 0031, 0032, 0033 and 0037; rules core.1, core.29, core.44, core.46; the changes `an-agents-program-settings-come-from-flumes-layers`, `an-agents-effort-is-a-setting-of-flumes-layers`, `an-answer-reaches-the-open-session-within-an-hour` (`keep_open`), `every-question-a-place-asks-reaches-someone-who-answers-it` and `flume-opens-a-streams-multiplexer`; afterthought/swancloud's change `each-stream-opens-in-a-session-the-host-chooses-and-gets-its-labels` (its record 0053); `crates/flume-core/src/room.rs` (`StreamRoom::session`, `Queue::session_of`), `crates/flume-core/src/start.rs` (`OpenIn`), `crates/flume-core/src/settings.rs` (`placeless`, `sessionless`), `crates/flume-seams/src/terminal.rs` (`OpenIn`, `SessionFrom`, `SessionHeld`), `crates/flume/src/host/config.rs` (`streams_declared`, `LOCAL_KEYS`) and `crates/flume/src/profiles/terminal/herdr/mod.rs`.

## Context and problem statement

Record 0031 opens each stream's agents in a session of its terminal, the one the host names for the stream, else the stream's name. The fleet names each stream by its partition, so a partition's agents open in the partition's own herdr session. There they mix: a change's construct stage, which writes its design, a research place, a plan stage held for review, beside every change's code and verify agents, every fix and every chore. A person looking for the design work reads every workspace's tabs.

A place runs in exactly one stream (143), and its stages are one place: a change's construct, code and verify stages share its stream, its bound and its label. A stream of its own for research would split research from the rest, but not a change's design from its code. It would also split the partition's one stream, its bound, its pool and its planning label's share, which the fleet keeps one per partition on the user's words ("Put planning in a label of each work stream instead of its own stream").

Where an agent opens is resolved today from the launch's settings (a pane beside, or a session), then the stream (107; `Queue::session_of`). No host's word names a session by kind or by stage: every layer but the launch's has `session` taken away (`sessionless`).

## Considered options

For which part of the terminal:

1. A session of the stream's terminal: a herdr session of its own, a server of its own, under herdr.
2. A workspace within the stream's session, holding the research and design places of every repository and base.

For where the host says it:

3. A stream per sort of work.
4. One of clause 32's settings, `session`, given by the launch and by the host's agents tables for a repository and kind, a kind, a stream and every agent, as `keep_open` is.
5. A word of clause 32's layers naming a part, `design`, which each stream maps to a session, flume naming the session `<stream>-<part>` where the stream maps none.
6. The stream's own word for its kinds and stages: `session` in `[streams.<stream>.kinds.<kind>]` and in its `.stages.<stage>`.
7. A session per label.

## Decision outcome

Options 1 and 6.

1. **A session of the stream's terminal (190, 199).** A stream may name, for a kind and for one stage of a kind, the session of its terminal its agents of it open in. Under herdr that is herdr's session of that name, started headless where it does not run, laid out as every session is (record 0018): one workspace per repository and base, one tab per place. A person opens it to find the research and design agents together.
2. **The stream's own word (143, 199, C.7).** `session` in `[streams.<stream>.kinds.<kind>]`, and in `[streams.<stream>.kinds.<kind>.stages.<stage>]` for one stage of the kind. A kind is named as `[kinds.<kind>]` names it, by its name whatever source and version it resolves to. Nothing else is named there: no repository, label, stage of every kind or terminal. It is not one of clause 32's layered settings, and no agents table, kind or repository gives a session.
3. **The order (107).** A pane the settings name; then a session the launch's settings name, the stage's own first; then the session the stream names for the place's kind's stage, then for its kind; then the stream's own, the one it names, else its name, and flume's own for the default stream naming none.
4. **Read at each start (32, 190).** From the host's configuration as it stands, the local layer over the managed one. A change takes the next start, and an agent already started stays where it opened. A table for a kind or a stage the place does not run is ignored.
5. **What refuses (4).** A start that reads its stream's table for its kind or its stage holding any key but `session` and, for a kind, `stages`, is refused naming the key until the configuration is mended, as a label's table is (182). A session name the pinned herdr refuses refuses the open in herdr's words (core.29).
6. **The place keeps its stream (70, 143, 180).** Its bound, priority, pool, points, pace and its label's share are its stream's as before. Only where its agents open differs.
7. **The local layer (C.7; record 0032).** The local layer may set these `session` keys, each a key of its own, and the managed layer may pin them.
8. **Status (56).** Each agent's session with where it came from: `launch`, `stream-stage`, `stream-kind`, `stream`, `stream-name` or `own`, none beside a pane. Each stream gives its own session as before, and each session it names for its kinds and stages, with those kinds and stages.
9. **The view (186; record 0037).** `flume open [<stream>] [--session <name>]` shows the stream's own session where no session is named, and otherwise a session the stream names for a kind or a stage, in the stream's view and terminal app. Its window's title is the stream's for the stream's own session and the session's name for another. A session the stream names for nothing is refused naming the stream's sessions, before anything is started, so a mistyped name starts no session.
10. **The terminal profile (190; C.5).** Nothing new is asked of herdr. The start records the session and the pane by its id and server, every later call goes to that pane, a place has a tab in each session its agents opened in, its tidy closes each, and the base's count shows on each of its workspaces, as record 0031 built for a launch's session per stage. The printed command names no session.

### Why not the others

Option 2 puts the design work in the window a person already has open, but a workspace holding places of several repositories and bases breaks the layout of record 0018, which the base's count clue (139) and the tidy of a base's workspace rest on, and it needs a part within a part that the terminal seam does not have. The user asked for sessions.

Option 3 splits the partition's stream, which the user ruled out for planning, and still leaves a change's design with its code, since a place runs in one stream.

Option 4 gives a session for a kind in every stream at once: the host's agents tables for a kind and for a repository and kind name no stream, so two partitions' research would open in one session, or the host would write a session for every repository. Its layers for a stream and for every agent name no stage. And `[agents.streams.<stream>] session` would spell `[streams.<stream>] session` twice.

Option 5 adds a word and a mapping for one place, and its default is a session name flume builds, which core.44 rules out; the fleet would still declare each session on each host by name.

Option 7 gives a label a meaning beyond its share, which clause 180 and core.46 rule out, and a label is a kind's, so a change's construct and code would share it.

A table of every kind's stage, `[streams.<stream>.stages.<stage>]`, is not taken: a stage's name means nothing across kinds, and clause 32 names a stage only within its kind.

### Consequences

- Clauses 32, 107, 143, 186 and 190, C.5's herdr row and C.7 are amended, and 199, S106 and S107 added, at 72a6f12 on blueprints' branch `design/flume-where-an-agent-opens`, for the user's word (core.19).
- Rule core.44 is amended with this record.
- `SessionFrom` gains `stream-kind` and `stream-stage`; a start written before reads as it did.
- `flume open` gains `--session`, built after `flume-opens-a-streams-multiplexer` lands.
- A host that names no session for a kind or a stage runs as before. One that names `work-design` for research in `work` finds research's next starts there, and every agent running stays where it opened.
- Each session's server runs the herdr plugin's watch, as each stream's does (record 0031), and settles are safe to repeat (core.17).
- The change `where-an-agent-opens-is-given-by-kind-and-by-stage` builds it.
