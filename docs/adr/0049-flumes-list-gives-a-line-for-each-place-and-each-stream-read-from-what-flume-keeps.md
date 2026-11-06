---
number: 49
title: flume's list gives a line for each place and each stream, read from what flume keeps
status: accepted
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# flume's list gives a line for each place and each stream, read from what flume keeps

Status: accepted by the user on 2026-11-05, on their yes in the Plannotator Inbox. Its statement amendment is on agentplot/blueprints main at 7bd1891 (57, 62, 64 to 66, 68, 70, 107, 111, 130, 186, 200, 201, C.5, section 9, S108). Proposed earlier that day by the construct stage of the change `flume-list-shows-each-place-and-stream-on-one-line`, from the user's words of the same day, quoted below. The command, what it reads and what it leaves out are the user's. How each line is worded, the terminal's new operation and its bound are the stage's. Amended 2026-11-06, the last section below, accepted on the user's yes the same day: the list leaves out a place that landed or was dropped, and gives it only when every place is asked, `flume list --all`.

Amended 2026-11-06, the first section below, accepted on the user's Pending You card of 2026-11-06, its statement amendment on agentplot/blueprints main at 12866a8: runs reads one or more parts of the terminal together, in one reading, so each stream's line gives each of its sessions, its own and each it names for a kind or a stage, its terminal still read once.

Deciders: the user, who ran `flume status` with no place on deneb, under flume 0.9.1 and 53 places, and said: "It was extremely slow and returns a bunch of errors too. It's completely unusable." Then: "We need a short name because I need a short list because I don't know what's available to open." The brief that carried their words asked for one short line per place, with its kind and name, its stage and its state, and one line per stream with its session and whether it runs. It asked that the list read what flume keeps, settle nothing, answer within a second on a host with fifty places, and leave a place's long words to `flume status <place>`. "list" is the user's word. The construct stage chose each line's words, the grouping by repository, the terminal seam's `runs` and its bound of one second.
Sources: `agentplot/blueprints:design/flume/requirements.md` on main at 7bd1891, clauses 56, 57, 58, 60 to 62, 64 to 66, 68, 70, 107, 111, 130, 139, 186, 200 and 201, B.5, C.5 and section 9, and S108; records 0005, 0006, 0031 and 0037; rules core.1, core.13 to core.17, core.29, core.35 and core.48; `crates/flume-core/src/status.rs` (`Engine::status`), `crates/flume-core/src/line.rs` (`Engine::lines`, `state_words`), `crates/flume-core/src/open.rs` (`Engine::open_stream`), `crates/flume-seams/src/terminal.rs` (`Terminal`), `crates/flume-seams/src/held.rs` (`Phase`, `Kept`) and `crates/flume/src/profiles/terminal/herdr/mod.rs` (`running`); flume 0.9.1's status and `status --line` timed on deneb on 2026-11-05.

## Context and problem statement

On deneb on 2026-11-05, flume 0.9.1 held 42 documents among 54 place folders. `flume status` with no place took 75 seconds and printed 984 lines; its JSON came to 474 KB. Status reads every fact of each place where it lives when asked (58): the change at each head through the change profile, the merge queue's record of each branch, each agent's state in the terminal, and each pool and source. Eleven places' entries repeated sluice's refusal to read old policies, 42 lines between them, and four landed places printed their whole teardown error. Each of those four had failed `sluice release`, which could not read the host's sluice configuration (`unknown field caches`). `flume status --line` took 2.6 seconds. It still reads the change at each live head, names no repository, and says nothing of the streams.

The user's question is what is there and what they can open. The streams are what `flume open` takes (186), and nothing of flume lists them.

## Considered options

For the act:

1. **A new act, `flume list`, with an answer of its own,** read from flume's held state, each live agent's own record and each stream's terminal once.
2. **Shorten status's answer for every place,** reading each fact only when one place is named.
3. **Give the streams to `status --line`,** and leave it reading the change at each head.
4. **Read status's facts side by side or keep them between asks,** so status itself is fast.

For whether a stream's session runs:

5. **A terminal operation of its own, `runs`,** that answers whether a part runs and starts nothing.
6. **The terminal's attach,** which starts the part where it does not run.
7. **Leave it out,** and give only each stream's session.

For a place's state:

8. **The words its line gives,** with whom a waiting place waits on, and, for a place over that still stands, what keeps it, in a word.
9. **A smaller set of its own,** live, waiting, landed, stalled and over.

## Decision outcome

Options 1, 5 and 8.

`flume list` is an act of the core (200), answered as `Answer::List` and printed or given as JSON (60, 61). It needs no repository and refuses nothing. For each place on the host, a tidied place whose document stays among them (62), it gives one line under the place's repository: `<kind>--<name> · <stage> · <state>`. The state is the line's own word for it, as `state_words` gives it (56, 139). A place that waits on its caller adds `waits on its caller` (130). A place waiting for room adds `waits on the host` (70), and a submitted one `waits on the merge queue` (111). A place over that still stands adds what keeps it (64 to 66, 68): `kept by <step>`, the step its teardown failed at, or `kept holding work`, `kept holding a lock`, `kept while its agent runs` or `kept unread`. A tidied place adds `tidied`, and one being removed adds `being tidied`. The stage is the one the line names, without a repeating stage's group, since the group is read from the change at the head. For each stream the host declares it gives `<stream> · session <name> · <runs>`, the session resolved as `flume open` resolves it (107, 186). The word is `runs`, `not running`, `not read`, `no session` under a terminal that holds none, or `no profile for <name>` for a multiplexer flume carries no profile for (190). The places come first, by repository and then by name, and the streams last, in status's order.

The list reads each place's held state and, for a live agent, the asking mark its own record holds, as the line reads it. It reads no worktree, head, deliverable, change or submission, and asks no terminal of a place's agent. It asks each stream's terminal once whether the stream's session runs. It gives none of a place's long words, neither in its print nor in its JSON: a stop-short's, a hold's, a readiness failure's, a submission's failure's or a teardown's. `flume status <place>` gives those. It settles, starts, sends and writes nothing (57).

`runs` is a new operation of the terminal seam (201). Given a part by its name or the terminal's own, it answers that the part runs or does not, with the name the terminal knows it by. Or it answers that the terminal holds no part, or that it could not read the part, in the terminal's words. herdr answers from one `herdr session list --json`, bounded at one second, starting no server. The printed command holds no part.

Option 2 changes the answer every program reads of status, which only gains fields (61). Clause 58 still asks status for every fact of every place. Option 3 keeps the change read that makes `--line` take seconds, and a line that holds kind, version, run and tasks is not short. Option 4 cannot pass core.15: a fact kept between asks is a record flume keeps. Read side by side, status still prints every place's long words. Option 6 would start a stream's session to say whether it runs, and core.48 forbids opening anything but at a person's asking. Option 7 leaves the person not knowing which stream's agents they would see. Option 9 is a second vocabulary for one state, against 60's one answer, and the place line's words already say who waits.

The name is the user's word. `flume places` names half of what it lists, and `flume ls` is not a word of the statement's.

### Consequences

- Good: the list answers in the time it takes to read the host's documents and one session listing per stream, whatever the places hold.
- Good: a place's line uses the words its pane's title, the sidebar and status give it.
- Good: the streams a person can open are named, with which of them run.
- Bad: what the list says is what the last settle left. A place stalls, or is read landed, at the next settle, under the herdr plugin's watch within its bound of thirty seconds.
- Bad: a place's repeating stage shows no group, and its change's tasks are not counted; `status --line` gives both.
- Bad: the terminal seam has one more operation for every terminal profile to answer.

## Amendment, 2026-11-06: runs reads several parts in one reading, and a stream's line gives each of its sessions

Status: accepted on the user's Pending You card of 2026-11-06, which approved "Let Flume ask herdr about all of a stream's sessions in one call, so flume list stays instant?"; its statement amendment, of 201 and C.5's runs, is on agentplot/blueprints main at 12866a8, cut from fa40c94, its message marking this amendment accepted on that card. Proposed the same day by the code stage of the change `where-an-agent-opens-is-given-by-kind-and-by-stage`, on the answer it was started again with, quoted below.

Deciders: the answer to the stage's stop-short, which chose which reading of 200 and 201 holds: "Yes, this change adds the group for the list's sessions: its own statement follow-up (fa40c94) amended 200 to list each session, so the work is this change's, and design 11 and task 5.2 are rewritten to say so. Keep 200 as written, each stream's terminal read once: runs (201) answers several parts in one call, the terminal's sessions listed together, never one call per session, which would read 200's 'once' otherwise than it says." It gave 201's words, "given one or more parts of the terminal … whether each runs now", and asked for the seam's and the herdr profile's matching lines. The stage chose the seam's shape, an answer for each part in the order asked, and each line's words.
Sources: `agentplot/blueprints:design/flume/requirements.md` on main at fa40c94, clauses 199, 200 and 201 and C.5, and on main at 12866a8, 201 and C.5's runs amended; record 0048 and its amendment; rules core.2, core.14, core.29 and core.52; `crates/flume-seams/src/terminal.rs` (`Terminal::runs`, `SessionRuns`), `crates/flume-core/src/list.rs` (`stream_listed`, `stream_line`), `crates/flume-core/src/room.rs` (`StreamRoom::words`) and `crates/flume/src/profiles/terminal/herdr/mod.rs` (`runs`).

### Context and problem statement

Record 0048's amendment, on blueprints main at fa40c94, has 200 give each stream's line each of its sessions, its own and each it names for a kind or a stage, and whether each runs now (201). The list landed on flume's main with each stream's own session alone. 200 still reads each stream's terminal once, and 201 answered one part for each call, so a stream naming a session for a kind could not be listed under both as they were written.

### Considered options

1. **runs takes one or more parts and reads them together,** 201 amended, the list asking each stream's terminal once for all its sessions.
2. **One runs for each session,** 201 as written, and 200's "once" read as once for each session.
3. **The list gives the own session alone,** and 200 is amended back.

### Decision outcome

Option 1, as the answer ruled. The terminal seam's runs is given one or more parts, each by its name or the terminal's own, and answers for each, in the order asked, whether it runs, with the name the terminal knows it by, or that the terminal holds no part, or that it could not read the part. herdr reads one `herdr session list --json`, bounded at a second, for every part asked, and a listing not read is unread for each. A part only the profile's socket names is unread alone where no session is listed at that socket. The printed command holds no part for any. The list asks each stream's terminal once, for its own session and each session it names for a kind or a stage, each once by name. The line gives each session after the own with its word: `work · session work · runs · session work-design · not running`. Where no terminal is asked, the sessions come before the one `no profile for <multiplexer>`. Where the terminal holds no part, the line is `<stream> · no session`. The JSON form gains each stream's `sessions`, each with its `name` and `runs`, written only where the stream names one.

Option 2 asks a stream's terminal once for each of its sessions, which reads 200 otherwise than it says. Option 3 undoes a ruling the user accepted on their card.

### Consequences

- Good: a person sees at once which of a stream's sessions run, and opens each by the name the line gives.
- Good: the list still costs one session listing per stream, however many sessions the stream names.
- Bad: every terminal profile's runs answers a list of parts, the fake and the printed command among them.
- Clause 201 and C.5's runs are amended on agentplot/blueprints main at 12866a8 (core.19). Rule core.52's details say a runs for each session is ruled out.


## Amendment, 2026-11-06: the list leaves out a place that landed or was dropped unless every place is asked

Status: accepted by the user on 2026-11-06, on their yes to the question below, which the fix `flume-list-leaves-finished-places-out` builds. Its statement amendment is on agentplot/blueprints at e4fab7b, on the branch `design/flume-the-list-leaves-finished-places-out` (62, 63, 200, S108).

Deciders: the user, who answered "Should flume list leave finished places out?" on 2026-11-06, once `flume list` had landed (flume 78ea62ea, released in v0.13.0): "Yes: leave out landed and dropped places unless --all, as a follow-up fix after this lands." Which places are left out and the flag's name are theirs. That a place its caller declared done stays listed, that a left-out place is left out whatever keeps it, and that the answer gains no field for what it left out are the fix's code stage's.
Sources: `agentplot/blueprints:design/flume/requirements.md` at e4fab7b, clauses 62, 63 and 200 and S108; rule core.52; `crates/flume-core/src/list.rs` (`Engine::list`, `landed_or_dropped`), `crates/flume/src/commands/mod.rs` (`Command::List`); `flume list` on deneb on 2026-11-06, 16 of its 67 lines places that landed and were tidied or were dropped.

### Context and problem statement

The decision above gives a line for each place flume keeps, a tidied place whose document stays among them. The user asked for the list so they could see what is there to open. A place that landed or was dropped has nothing left to open: its work is over (63), and its agent is gone or going. On deneb on 2026-11-06, 16 of the list's 67 lines were such places, `landed · tidied` and `dropped` among them, between the places the user could open.

### Considered options

1. **Leave out a place that landed or was dropped, whatever keeps it, and give every place under `--all`.**
2. **As 1, and leave out a place its caller declared done too,** every place whose work is over.
3. **As 1, but keep a place that still stands,** one kept by its teardown, its holds, its live agent or its unread state.
4. **As 1, and say how many places were left out,** in a line of the print and a field of the JSON.

### Decision outcome

Option 1.

The list gives a line for each place on the host that has not landed and was not dropped (63, 200). A place whose phase says it landed or was dropped, kept, being tidied or tidied, is left out. `flume list --all` gives every place's line, a tidied place whose document stays among them (62), as the decision above gives them, its words unchanged. A place its caller declared done stays listed, as do a place being made and a live one. The JSON's form is unchanged: `--all` decides which places it holds, and no field says how many were left out (61).

Option 2 reads past the user's words, which name landed and dropped places. A done place's kind does not land, and its agent may be the one its caller still reads: `done · kept while its agent runs` names an agent a person can open. Option 3 keeps the four landed places of deneb's whose `sluice release` failed in the list. Those are a host's operator's to clear, and `flume status` and `--all` give them. Option 4 adds a line to a list the user asked to be short, and `--help` names `--all` beside it.

### Consequences

- Good: the list holds what is there to open, and `--all` gives the rest at the same cost.
- Bad: a place kept by a failed teardown, its holds or its live agent after it landed or was dropped is seen only under `--all` or in `flume status`.
- Bad: a program that read every place from `flume list --json` must now pass `--all`.
- Clause 200 and S108 of the statement are amended at blueprints e4fab7b (core.19). Rule core.52 is amended with it, its rule and its details.
