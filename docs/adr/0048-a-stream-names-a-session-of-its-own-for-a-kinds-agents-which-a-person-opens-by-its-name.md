---
number: 48
title: A stream names a session of its own for a kind's agents, which a person opens by its name
status: accepted
date: 2026-11-06
decision-makers:
- Chuck Swanberg
---

# A stream names a session of its own for a kind's agents, which a person opens by its name

Status: accepted on the user's word of 2026-11-06, who approved the design of `where-an-agent-opens-is-given-by-kind-and-by-stage`, this record's decision among it, and named each partition's research-and-design session `<partition>-design`. The need is the user's, on 2026-11-05: "I'm realizing that mixing construction sessions from Flume with research and design sessions makes it hard to find and keep track of the research and design sessions. We probably want to put those into different Herdr sessions. I don't know if that is possible, whether that has to be explicit at the stream level (so we need different streams), or whether we can do it below the stream level". The user ruled on 2026-11-06, in the Plannotator Inbox thread msg_01M6Q2N04BS0RPG16TA76W67W6 (their reply msg_01M6Q4DZM9GFJJFVDX9JE7G1N9), which part of the terminal, where the host says it, what opens there and how a person opens it; their words are quoted below as the decision.

Deciders: the user, on the need, on the rulings below, on the session's name and on the whole, approving it on 2026-11-06; the construct stage of `where-an-agent-opens-is-given-by-kind-and-by-stage`, on 2026-11-06, on the rest.
Sources: `agentplot/blueprints:design/flume/requirements.md` at 7bd1891, clauses 32, 70, 107, 143, 180 to 182, 186, 187, 190, 200, 201, C.5's herdr row and C.7, and its amendment, on main at f676e8f, which adds 199, S106 and S107; `design/weir/requirements.md` at 7bd1891, clause 55 and S11, the elaboration kind; records 0018, 0020, 0031, 0032, 0033 and 0037; rules core.1, core.4, core.29, core.44, core.46, core.48 and core.49; the changes `an-agents-program-settings-come-from-flumes-layers`, `an-agents-effort-is-a-setting-of-flumes-layers`, `an-answer-reaches-the-open-session-within-an-hour` (`keep_open`), `every-question-a-place-asks-reaches-someone-who-answers-it`, `flume-list-shows-each-place-and-stream-on-one-line` (record 0049) and `flume-opens-a-streams-multiplexer` (landed as #109); afterthought/swancloud's change `each-stream-opens-in-a-session-the-host-chooses-and-gets-its-labels` (its record 0053); `crates/flume-core/src/room.rs` (`StreamRoom::session`, `Queue::session_of`), `crates/flume-core/src/open.rs` (`Engine::open_stream`), `crates/flume-core/src/settings.rs` (`placeless`, `sessionless`), `crates/flume-seams/src/terminal.rs` (`OpenIn`, `SessionFrom`, `SessionHeld`), `crates/flume-seams/src/host.rs` (`Stream`), `crates/flume/src/host/config.rs` (`streams_declared`, `LOCAL_KEYS`) and `crates/flume/src/profiles/terminal/herdr/mod.rs`.

## Context and problem statement

Record 0031 opens each stream's agents in a session of its terminal, the one the host names for the stream, else the stream's name. The fleet names each stream by its partition, so a partition's agents open in the partition's own herdr session. There they mix: research and design places beside every change, fix and chore. A person looking for the design work reads every workspace's tabs.

A place runs in exactly one stream (143). A stream of its own for research would split the partition's one stream, its bound, its pool and its planning label's share, which the fleet keeps one per partition on the user's words ("Put planning in a label of each work stream instead of its own stream").

Where an agent opens is resolved today from the launch's settings (a pane beside, or a session), then the stream (107; `Queue::session_of`). No host's word names a session by kind: every layer but the launch's has `session` taken away (`sessionless`). And `flume open [<stream>]` shows a stream's own session alone (186; record 0037).

## Considered options

For which part of the terminal:

1. A session of the stream's terminal: a herdr session of its own, a server of its own, under herdr.
2. A workspace within the stream's session, holding the research and design places of every repository and base.

For where the host says it:

3. A stream per sort of work.
4. One of clause 32's settings, `session`, given by the launch and by the host's agents tables for a repository and kind, a kind, a stream and every agent, as `keep_open` is.
5. The stream's own word for its kinds: `session` in `[streams.<stream>.kinds.<kind>]`.
6. The stream's own word for its kinds and for one stage of a kind, `.stages.<stage>` in a kind's table.
7. One word of the stream's for every elaboration kind at once, flume reading whether a kind has weir's elaboration section.
8. A session per label.

For how a person opens it:

9. `flume open <stream> --session <name>`.
10. `flume open <name>`, the name a stream's or a session's a stream names for a kind.
11. `flume open <stream> <kind>`, resolving the kind's session.

## Decision outcome

Options 1, 5 and 10, on the user's rulings and their approval.

1. **A session of its own (190, 199).** The user: "A session of its own beside the stream's, e.g. `madswan-design`". A stream may name, for a kind, the session of its terminal that kind's agents open in. Under herdr that is herdr's session of that name, started headless where it does not run, laid out as every session is (record 0018): one workspace per repository and base, one tab per place.
2. **The stream's table for a kind (143, 199, C.7).** The user: "The stream's own table, `[streams.<stream>.kinds.<kind>]`", and, of `[streams.madswan.kinds.change.stages.construct]`: "not this one. just any elaboration types". `session` in `[streams.<stream>.kinds.<kind>]`, every stage of the kind opening there; the table holds nothing else. A kind is named as `[kinds.<kind>]` names it, by its name whatever source and version it resolves to. The elaboration kinds are the kinds whose whole work is research and design, `research` on this host today and crew's design roles once they run as kinds, each named by the host in a table of its own. It is not one of clause 32's layered settings, and no agents table, kind or repository gives a session.
3. **The order (107).** A pane the settings name; then a session the launch's settings name, the stage's own first; then the session the stream names for the place's kind; then the stream's own, the one it names, else its name, and flume's own for the default stream naming none.
4. **Read at each start (32, 190).** From the host's configuration as it stands, the local layer over the managed one. A change takes the next start, and an agent already started stays where it opened. A table for a kind the place does not run is ignored.
5. **What refuses (4).** A start that reads its stream's table for its kind holding any key but `session`, `stages` among them, is refused naming the key until the configuration is mended, as a label's table is (182). A session a stream names for a kind is that stream's alone: one that another stream's agents open in by that stream's word, its own session, its name where it names none, or one it names for a kind, refuses the starts that read it, naming both streams. A session name the pinned herdr refuses refuses the open in herdr's words (core.29).
6. **The place keeps its stream (70, 143, 180).** Its bound, priority, pool, points, pace and its label's share are its stream's as before. Only where its agents open differs.
7. **The local layer (C.7; record 0032).** The local layer may set a kind's `session`, and the managed layer may pin it.
8. **Status (56).** Each agent's session with where it came from: `launch`, `stream-kind`, `stream`, `stream-name` or `own`, none beside a pane. Each stream gives its own session as before, and each session it names for its kinds, with those kinds and their layers, and each fault of its tables.
9. **The view (186; record 0037).** The user, of `flume open madswan --session madswan-design`: "This confuses me. Why wouldn't it just be madswan design? Why do you need the flag? Why wouldn't you just open madswan design?" `flume open [<name>]` takes one name: a stream's, which shows the stream's own session as before, or a session a stream names for a kind, shown in that stream's view and terminal app under the session's name, or under the stream's title where it is the stream's own. A name that is neither is refused naming the streams and their sessions before anything is started, so a mistyped name starts no session. Each such session is one stream's (5), so its name alone finds its stream. A person finds madswan's research and design agents by `flume open madswan-design`.
10. **The terminal profile (190; C.5).** Nothing new is asked of herdr. The start records the session and the pane by its id and server, every later call goes to that pane (core.49), a place has a tab in each session its agents opened in, its tidy closes each, and the base's count shows on each of its workspaces, as record 0031 built. The printed command names no session.

### Why not the others

Option 2 puts the design work in the window a person already has open, but a workspace holding places of several repositories and bases breaks the layout of record 0018, which the base's count clue (139) and the tidy of a base's workspace rest on, and it needs a part within a part that the terminal seam does not have. The user chose option 1.

Option 3 splits the partition's stream, which the user ruled out for planning.

Option 4 gives a session for a kind in every stream at once: the host's agents tables for a kind and for a repository and kind name no stream, so two partitions' research would open in one session, or the host would write a session for every repository. The user chose option 5.

Option 6's one use was a change's construct stage, which the user ruled "not this one"; nothing else asks for it. A change's design stays in the change's own tab beside its code. A stage's table adds later without breaking a host, since `stages` in a kind's table is refused today. The user approved the kind's table alone.

Option 7 opens a new elaboration kind apart without the host's word, but flume would read whether a kind has weir's elaboration section, which weir 55 gives to whoever runs elaborations ("Whoever runs places reads the kind's stages and nothing of the section") and core.4 keeps from flume; weir's statement would change first. The user approved the kinds the host names, one table each.

Option 8 gives a label a meaning beyond its share, which clause 180 and core.46 rule out.

Two streams naming one session for their kinds is not taken: a session two streams' agents open in has two views, two terminal apps and two titles to choose from, a person could not open it by its name, and two partitions' research would share it.

Option 9 names one session twice and asks the person which stream owns it; the user asked why it needs the flag, and it does not. Option 11 reads the same table a second way on the command line, when the person thinks of the session they want. The user approved `flume open <session>` and named each partition's session `<partition>-design`, one word, so `flume open madswan-design`.

### Consequences

- Clauses 32, 107, 143, 186, 190 and 200, C.5's herdr row and C.7 are amended, and 199, S106 and S107 added, on blueprints main at f676e8f (core.19). Clause 200's list names each stream's sessions for its kinds, built by whichever of this change and record 0049's lands on flume's main second.
- Rules core.44 and core.48 are amended with this record.
- `SessionFrom` gains `stream-kind`; a start written before reads as it did.
- `flume open`'s one argument is a stream's name or a session's a stream names for a kind; every `flume open <stream>` opens as before.
- A host that names no session for a kind runs as before. One that names `work-design` for research in `work` finds research's next starts there, and every agent running stays where it opened.
- Each session's server runs the herdr plugin's watch, as each stream's does (record 0031), and settles are safe to repeat (core.17).
- The fleet would write `[streams.<partition>.kinds.research] session = "<partition>-design"` for each partition stream, the name the user gave, and the same for each elaboration kind it runs; that is its own change.
- The change `where-an-agent-opens-is-given-by-kind-and-by-stage` builds it.
