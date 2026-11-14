---
number: 67
title: Rex is flume's second terminal, its calls bounded, stripped of the caller's REX_* and checked against the Rex the tree pins, as herdr's are
status: proposed
date: 2026-11-13
decision-makers:
- the construction agent of a-stream-runs-its-agents-in-rex
---

# Rex is flume's second terminal, its calls bounded, stripped of the caller's REX_* and checked against the Rex the tree pins, as herdr's are

Status: proposed. The operator asked for Rex as flume's second terminal beside herdr, "I'm ready to land the Rex research and get a bolt queued for that research/rex-is-flumes-second-terminal" (2026-11-12, signal 2026-11-12-agentplot-planner-c2cd5624/01-rex-bolt-queued), and settled with the research how flume drives it: "Given this exploration and prototype, we don't need to have a server running. We can do it all through the CLI, right?", and "then we'd have an afterthought, agentplot, madswan, and wldn server (the streams). each base is a session". Those rulings are the statement's, C.5's Rex rows on boomdrivedev/blueprints main at 786d040. What this record decides beyond them is the construction agent's, made on the rules written and cheap to change, and stays proposed until the operator's yes.

Deciders: the operator, whose rulings C.5 states; the change's design, `openspec/changes/a-stream-runs-its-agents-in-rex/design.md`, for the choices it names as its own (sections 3, 4 and 13); the construction agent, for the choices listed last below.
Sources: `boomdrivedev/blueprints:design/flume/requirements.md` at 786d040, clauses 7, 32, 34, 35, 43, 94 to 98, 107, 132, 136 to 139, 187, 190, 199, 201, C.5's Rex rows and the paragraphs beneath them, C.6's stripped variables and S91; `design/research/2026-11-11-rex-as-flumes-second-terminal.md`; records 0016, 0018, 0026 and 0037; rules core.1, core.2, core.13, core.22, core.29, core.30 and core.51; Rex 0.1.0-preview.14 (commit 7e460d0, API revision 12) as Rex's flake packages it, read on deneb on 2026-11-13 by its `--help`, `api describe` and calls on servers of the tests' own.

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
6. **attach for the app and show are the bolt's next unit**, `a-person-sees-a-rex-place-in-the-app`: until it lands, attach refuses both views in the profile's words and show shows none, so a stream on Rex runs its places and a person is not yet shown them (139, 187).

The construction agent's choices, each with the path not taken:

- **The pinned release is 0.1.0-preview.14 itself**, the one C.5 and the research read, though the preview channel served 0.1.0-preview.19 on 2026-11-13. Not taken: the channel's newest, whose readings no one has checked against C.5's table.
- **open claims the identity by linking a record written whole**, so no second open reads a claim half written and takes it for none; a claim that names no block yet reads live while the process that made it lives, and stopped once it is gone. Not taken: a record created empty and written after, which two opens at once both passed in the tests.
- **A start never begun closes its block**, as every failure of open after the block was made does (design section 6), the screen given with the answer. Not taken: leaving a program that never reported running in a block no record names.
- **A window gone at a tidy reads closed**: Rex closes a window with its last block, so a place whose agents were closed has its window gone before its tidy, and the tidy names it among what it closed. Not taken: naming only windows the tidy's own call closed, which would report nothing of a place's window though it is gone.
- **The calling command's session is read from `REX_SERVER` naming a Unix socket**, `unix://<path>` or a path, and none for a server Rex reaches over another address, whose blocks no call of flume's, all by socket, could reach.
- **Rex's words are given without its `Error: ` lead**, the rest as Rex wrote it.
- **The agents of a part are every labelled block of its server**, flume's own agents' among them, for the core to count by name (132, C.5's standing row). Not taken: leaving out blocks flume opened, which would hide a role run's own block from its address.

### Consequences

- Good: a stream naming Rex runs its places in a Rex server of flume's own, under the one discipline herdr's calls keep.
- Good: a reviewer can point at a Rex call and name the rule it breaks.
- Bad: the tools tests of the Rex profile run only where Rex's flake packages Rex; elsewhere they fail naming it, and never pass without it (core.51).
- Bad: Rex is in tester beta and its preview channel moves weekly; a later release that reads otherwise is a finding for the statement, never a reading changed here (core.19).
