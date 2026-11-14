---
number: 67
title: Rex is flume's second terminal, its calls bounded, stripped of the caller's REX_* and checked against the Rex the tree pins, as herdr's are
status: proposed
date: 2026-11-13
decision-makers:
- the construction agent of a-stream-runs-its-agents-in-rex
- the construction agent of a-person-sees-a-rex-place-in-the-app
---

# Rex is flume's second terminal, its calls bounded, stripped of the caller's REX_* and checked against the Rex the tree pins, as herdr's are

Status: proposed. The operator asked for Rex as flume's second terminal beside herdr, "I'm ready to land the Rex research and get a bolt queued for that research/rex-is-flumes-second-terminal" (2026-11-12, signal 2026-11-12-agentplot-planner-c2cd5624/01-rex-bolt-queued), and settled with the research how flume drives it: "Given this exploration and prototype, we don't need to have a server running. We can do it all through the CLI, right?", and "then we'd have an afterthought, agentplot, madswan, and wldn server (the streams). each base is a session". Those rulings are the statement's, C.5's Rex rows on boomdrivedev/blueprints main at 786d040, and its rows for attach and show at 6557c33, read with the Rex app's Remote Control on. What this record decides beyond them is the construction agent's, made on the rules written and cheap to change, and stays proposed until the operator's yes.

Deciders: the operator, whose rulings C.5 states; the change's design, `openspec/changes/a-stream-runs-its-agents-in-rex/design.md`, for the choices it names as its own (sections 3, 4 and 13); the design of the bolt's next unit, `openspec/changes/a-person-sees-a-rex-place-in-the-app/design.md`, for the choices of its sections 3, 4, 6 and 7; the construction agents, for the choices listed last below.
Sources: `boomdrivedev/blueprints:design/flume/requirements.md` at 786d040, clauses 7, 32, 34, 35, 43, 94 to 98, 107, 132, 136 to 139, 187, 190, 199, 201, C.5's Rex rows and the paragraphs beneath them, C.6's stripped variables and S91, and at 6557c33 C.5's attach and show rows and the paragraph on the app's Remote Hosts; `design/research/2026-11-11-rex-as-flumes-second-terminal.md`, "Showing a session to a person" among it; records 0016, 0018, 0026 and 0037; rules core.1, core.2, core.13, core.22, core.29, core.30 and core.51; Rex 0.1.0-preview.14 (commit 7e460d0, API revision 12) as Rex's flake packages it, read on deneb on 2026-11-13 by its `--help`, `api describe` and calls on servers of the tests' own.

## Context and problem statement

Record 0037 made a stream's multiplexer a setting of its own and named Rex among the multiplexers a profile could be written for; S91 refused a stream naming `rex` while none was. C.5 now states the Rex profile's rows: each of the host's terminal sessions a Rex server of flume's own at `<state>/terminal/rex/<session>.sock`, each base a Rex session, each place a window, each agent a block started by a self-removing script, Rex's ready the program's first Program Status report, and every call `rex --autostart=false -S unix://<socket> <words>` under a bound. Rule core.29 binds herdr's calls and names no other terminal, and core.1 names `herdr` among the strings no module outside a profile holds and not `rex`.

## Considered options

1. **Rex bound as herdr is (chosen).** core.29 binds every call to herdr or Rex: a bound on each, an unanswered call read as unknown or a refusal in its tool's words, and every command and field read checked against the release the tree pins; core.1 names `rex` beside `herdr`.
2. A rule of Rex's own beside core.29, saying the same of Rex. Two rules for one discipline would drift apart, as the family's CLAUDE.md warns of two tools doing one thing.
3. No rule, the statement's C.5 alone. A reviewer pointing at a Rex call with no bound would have no rule to name.

## Decision outcome

Option 1.

1. **Rex is flume's second terminal profile**, `crates/flume/src/profiles/terminal/rex`, named `rex` by `[host] multiplexer` and a stream's `multiplexer`, listed third among the terminals, built over `<state>/terminal/rex` running nothing (105, 190).
2. **Every Rex call is bounded and names its server**: `rex --autostart=false -S unix://<socket> <words>`, ten seconds where the call states none, one second for runs' `server.status`, a call past its bound killed with its process group and read unknown or refused naming the bound, a nonzero exit a refusal in Rex's words (98, C.5; core.29).
3. **The caller's `REX_*` are stripped from every call**, the design's choice (section 3): the pinned Rex reads `REX_BLOCK` where a block call names none, so a command run in a Rex block would otherwise act on the caller's block. Not taken: passing them on, as a herdr call passing `HERDR_*` would.
4. **A session's name is refused where its socket would leave `<state>/terminal/rex`**, the design's choice (section 4): empty, `.` or `..`, or holding `/` or a NUL, beside C.5's refusal of a socket past 104 bytes. Not taken: leaving it to Rex's bind, which would let `..` reach a folder of the host's.
5. **Checked against the pinned Rex**: Rex is pinned from Rex's flake by the URL its release 0.1.0-preview.14 keeps for good, locked in devenv.lock, on the platforms that flake packages, aarch64-darwin and Linux; each command, field and refusal the profile reads is checked against it by the profile's tools tests (core.22).
6. **attach tells the Rex app to show a session, and show labels a place's window and its base's Rex session**, as C.5's attach and show rows say at blueprints 6557c33, built by the bolt's next unit, `a-person-sees-a-rex-place-in-the-app`. attach for the app finds or starts the part's server as the server row says, takes the app among `client.list`'s clients, and selects for it, by `-C <client> do session.select session_id=<id>`, the newest Rex session the profile made in that server; it refuses naming the socket a person adds under the Rex app's Remote Hosts where the app is no client of the server, and in Rex's words where the app refuses, as it does until a person turns on its Remote Control. attach for a window is refused, since Rex 0.1.0-preview.14 has no terminal client. show labels each window of the place `<kind>--<name> · <line>` by `session.set_window_label` and each Rex session of its base `<repository> <base> · <count>` by `session.set_label`, on the recorded ids, each label cut to 128 characters, a block keeping its agent's Rex name (139, 187).

The construction agent's choices, each with the path not taken:

- **The pinned release is 0.1.0-preview.14 itself**, the one C.5 and the research read, though the preview channel served 0.1.0-preview.19 on 2026-11-13. Not taken: the channel's newest, whose readings no one has checked against C.5's table.
- **open claims the identity by linking a record written whole**, so no second open reads a claim half written and takes it for none; a claim that names no block yet reads live while the process that made it lives, and stopped once it is gone. Not taken: a record created empty and written after, which two opens at once both passed in the tests.
- **A start never begun closes its block**, as every failure of open after the block was made does (design section 6), the screen given with the answer. Not taken: leaving a program that never reported running in a block no record names.
- **A window gone at a tidy reads closed**: Rex closes a window with its last block, so a place whose agents were closed has its window gone before its tidy, and the tidy names it among what it closed. Not taken: naming only windows the tidy's own call closed, which would report nothing of a place's window though it is gone.
- **The calling command's session is read from `REX_SERVER` naming a Unix socket**, `unix://<path>` or a path, and none for a server Rex reaches over another address, whose blocks no call of flume's, all by socket, could reach.
- **Rex's words are given without its `Error: ` lead**, the rest as Rex wrote it.
- **The agents of a part are every labelled block of its server**, flume's own agents' among them, for the core to count by name (132, C.5's standing row). Not taken: leaving out blocks flume opened, which would hide a role run's own block from its address.
- **The app is the last client listed of `info.kind` `app` whose `connection_state` is not `lost`**, a missing or unknown state read as connected as the pinned Rex's `api describe client.list` says, and attach refuses saying the app's connection is lost where every app listed is (design section 3): a lost client takes no action, and the newest connection is the app a person is looking at. Not taken: the first app listed, which may be a connection the app dropped and has not yet taken up again.
- **The Rex session the app is shown is the newest of the profile's base records for the part's server that `session.list` still lists**, newest by its record's file time, since a base's record is written once, when open makes its Rex session, and nothing rewrites it; and attach refuses saying the server holds no Rex session of flume's yet where none stands (design section 4). Not taken: selecting the newest record whatever the server holds, which Rex would refuse in words that say nothing of why; and answering shown with nothing selected, which would say the app was told to show what it was told nothing of.
- **show passes over a window or Rex session Rex answers not found, or whose server does not run**, neither labelled nor failed, and answers failed naming each window and Rex session another call did not label, with Rex's words or the bound (design section 6): Rex closes a window with its last block, so a place between stages has no window to label. Not taken: a failure logged at each settle while the place waits for room.
- **shown_at names a place's window, its live agent's block and its base's Rex sessions by id and socket**, `rex:<id>@<socket>`, from the records alone, the window on the live agent's server else the newest and the base's Rex session on that window's server else the newest, a window a tidy kept for a block flume did not open none of the place's (design section 7), as herdr's shown_at names its tabs and workspaces: each id is Rex's own and found again whatever label a person or Rex gives it. Not taken: the labels, which do not find a window again.

### Consequences

- Good: a stream naming Rex runs its places in a Rex server of flume's own, under the one discipline herdr's calls keep.
- Good: a reviewer can point at a Rex call and name the rule it breaks.
- Good: a person sees a Rex stream's session in the Rex app, and each place's line and its base's count on its window and Rex session, once they add the server's socket under the app's Remote Hosts and turn on its Remote Control.
- Bad: the app's own answer to `session.select` is outside the gate, since no app runs on a gate's host; a regression in the app shows only to a person, as `flume open` answering shown while the app shows nothing new.
- Bad: the tools tests of the Rex profile run only where Rex's flake packages Rex; elsewhere they fail naming it, and never pass without it (core.51).
- Bad: Rex is in tester beta and its preview channel moves weekly; a later release that reads otherwise is a finding for the statement, never a reading changed here (core.19).
