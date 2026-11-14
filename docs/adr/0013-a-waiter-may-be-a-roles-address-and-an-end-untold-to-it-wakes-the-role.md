---
number: 13
title: A waiter may be a role's address, and an end untold to an address with no live run wakes the role
status: accepted
date: 2026-10-28
decision-makers:
- Chuck Swanberg
---

# A waiter may be a role's address, and an end untold to an address with no live run wakes the role

Status: accepted by the user on 2026-10-28, in their design session ("i agree with these changes"), on the recommendations drawn from reading crew records 0045 to 0048, sluice records 0056, 0065 and 0068, and flume records 0002 to 0011 together.

Amended 2026-10-28 by record 0014, accepted by the user: a waiter is a recipient, and an end untold to an address is pending for it, collected by the run's start.

Amended 2026-11-06 by madswan-design, below: an address may be held by a standing agent the host's configuration names, whose ends go into it and wake no run.

The construct stage of `a-roles-address-may-be-held-by-a-standing-agent` read it into the code's terms on 2026-11-06, below the amendment.

Amended 2026-11-06 by record 0039, its design approved by the user the same day: an address is `<stream>/<role>`, a role of a stream the host declares, where this record wrote `<partition>/<role>`; a role is declared by a source of the host in the roles file its declaration names, never by the host's `[roles]`, and so is the standing agent the amendment below names (record 0039's own amendment); a run is named per address, by its entry's name or `<stream>-<role>`, runs in its address's stream, carries its entry's settings and intent, and is over at its delivery, flume declaring it done with no caller's word; and a stop-short may name the role it is for, of any stream.

Amended 2026-11-14 by boomdrive-design, below: a command that names no recipient, run in a standing agent's pane, acts as the address its source declares the agent holds.

Deciders: the user, accepting recommendation R1 of the design review of 2026-10-28; the shape is the reviewing agent's.
Sources: `boomdrivedev/blueprints:design/flume/requirements.md` clauses 3, 43, 69, 70, 72, 132, C.7 and S38, as amended with this record, and the callers table's crew row; records 0005, 0009, 0010, 0011; `afterthought/crew:docs/adr/0047` move 6 (no crew role stands; each is a kind Flume launches on a nudge naming a record) and `0048` patterns 3 and 4; rules core.13, core.25, core.26, core.27.

## Context and problem statement

Crew record 0047's move 6 has no crew role stand: the planner, ops, design, the operator and the reviewer each run on demand, launched by Flume on a nudge, reading the records, acting and ending with their turn. Every push in the design lands in a session that is already running: flume tells the waiter's own pane (record 0009), sluice pushes into the submitter's session (sluice 0068), and Pending You's wake mod wakes the card owner's session. When the planner run that launched a place has ended, the place's end, a stop-short's question, a left place and a card's answer all stand untold, and no record says what starts the next planner run. flume cannot decide to start it (rule core.27: no agent of flume's decides the next step), and crew runs no service (crew `state.4`). Something must turn "untold for this role" into "launch this role", or move 6 never wakes.

## Considered options

1. A role is addressed, not a session: a waiter is named by `<partition>/<role>`, a run of the role takes that address as its session's name, and an end untold to an address with no live run, or a carrier's wake, launches the kind the host's configuration names for it, one run per address at a time.
2. A host job of crew's or swancloud's that polls each tool's untold lists and launches roles.
3. Pending You, sluice and flume each launch the role themselves when their push finds no pane.

## Decision outcome

Option 1.

**The address is the waiter.** A caller names the waiter of a place by a role's address rather than by a session. A run of the role is a place of the kind the host's configuration names for the address (`[roles."<partition>/<role>"]`, C.7), named by the role, so clause 3 already keeps one run of an address live at once; its agent takes the address as its session's name, so the terminal finds it by the address as it finds any session.

**Flume wakes on its own facts, and on a carrier's word.** Where a settle finds an end standing untold to an address with no run live or waiting for room, it launches the address's kind at the priority the configuration gives it, inside the settle, so no process of flume's is added (core.13). A carrier of another tool, the card wake among them, may ask flume to wake an address (`flume wake <address>`); a wake of an address whose run is live or waiting does nothing. The run's start asks every tool for what is untold to the address (43; crew 0048 pattern 4), so a burst of events is one run that reads them all.

**Flume decides nothing.** Which address waits on a place, what the role's kind does and what its run decides are the caller's (69, 70, 72). flume delivers to an address as it delivers to a session, and launching the address's kind when nobody is there to be told is how the delivery is made. An address the configuration names no kind for is woken by nothing, and status names its untold ends.

**Why not the others.** Option 2 is a standing poller, the daemon the family refuses, and a second reader of every tool's untold state. Option 3 has sluice name flume (sluice core.1) and puts three launchers on one role.

### Consequences

- Clause 132 is added to the statement, 43 names the role's address, C.7 gains `[roles]`, the callers table's crew row names its waiters by address, and S38 shows a burst of ends woken into one run. Rule core.28 is drawn from it.
- Rule core.25's details name a role run's start as the caller's ask, in place of "crew's session-start hook".
- Units: the address as a waiter; the wake at a settle; `flume wake`; `[roles]` read from the host's configuration; status naming ends untold to an address with no kind.
- crew's side, the role kinds and the waiter named by address, is crew record 0049.

## Amendment, 2026-11-06: an address a standing agent holds

Status: decided on 2026-11-06 by madswan-design, the design agent, on swancloud-design's ask relayed by madswan-operator. Nothing in it is the user's alone: it holds to clauses 3, 43 and 132 and rule core.28 as they stand, and each choice in it is cheap to change.

Deciders: madswan-design.
Sources: `boomdrivedev/blueprints:design/flume/requirements.md` on main at 6d322be, clauses 3, 43, 132, 143 and C.7 and the vocabulary's recipient; `crates/flume/src/profiles/terminal/herdr/mod.rs` `reach` on main at 78ea62ea, read 2026-11-06; `herdr --session madswan agent list` on deneb, read 2026-11-06, which names madswan's standing agents `madswan-planner`, `madswan-ops` and the rest in that session; `afterthought/swancloud:docs/adr/` record 0042 as amended at f424dd86; crew's runner, which names every place's recipient by a role's address (`afterthought/crew:plugin/lib/runner.py` on `teamless/deneb`, as swancloud's record reads it); `afterthought/crew:docs/adr/0047` move 6.

### Context and problem statement

This record has a role run only as a place flume opens, as crew's record 0047 move 6 had no role stand. Every main-level role now stands. Each partition's crew teams file has `flume` false throughout, so each role is a herdr agent crew keeps under a name of its own, `madswan-planner` among them. crew still names every place's recipient by the role's address, `madswan/planner`. flume reaches an address only in the pane of a run it opened (`reach` finds a `herdr:<pane>@<socket>` or a name its own open recorded, and nothing else). An entry under `[roles]` would wake a second agent of the role beside the standing one, two agents acting on one plan. So every end for a standing role pends: 22 for `madswan/planner` on deneb on 2026-11-06, read by nobody until the agent runs `flume inbox` by hand.

### Considered options

1. **The host's configuration names a standing agent for an address,** by the name its terminal knows it by and the terminal session it lives in. flume delivers the address's ends into it as notices at the settles that read them, opens nothing and wakes no run, and status says where the address lives.
2. **crew names each launch's recipient by the standing agent's pane,** `herdr:<pane>@<socket>`, in place of the address.
3. **flume takes the address as the agent's terminal name,** finding `madswan/planner` among the terminal's agents with no entry.
4. **Each standing agent collects with `flume inbox --recipient <address>` at each wake,** and flume changes nothing.

### Decision outcome

Option 1.

**The address stays the recipient.** A caller names the address as before, so crew's launches, the places already launched and `flume inbox` change nothing. Which agent holds an address is the host's configuration's alone, written by whoever administers the host: `[roles."<partition>/<role>"]` with `agent`, the terminal's name for the agent, and an optional `session`, else the session the host names for the entry's `stream`, else its default stream's (143). When the roles crew keeps become kinds, the entry trades `agent` for `kind`, and no launch changes.

**One holder for an address.** An entry giving `agent` beside `kind` is refused, naming the address, since a standing agent and a woken run are two agents of one role (3).

**Delivered as any end is, and nothing more.** An end for a standing address goes into the agent's pane as a notice at a settle that reads it, only while the agent asks no question (43), and counts delivered on the terminal's answer. flume opens nothing for the address, starts no run and keeps no watch over it (core.13). The terminal finds the agent by its name in its session at each delivery, so a revive that keeps the name keeps the address. Where it finds none, or more than one, the end stays pending, and status says where the address lives and why nothing went (core.15).

**Why not the others.** Option 2 puts the terminal's pane ids into crew's launches. A pane's id changes when crew revives the agent, and the places already launched name the address. A pane that is gone takes the end, and nobody reads it. Option 3 couples flume to crew's naming: the address `madswan/planner` is not the agent's herdr name, `madswan-planner`. A standing agent of a caller that names none would take ends meant for a role. Option 4 is today's stopgap: an end waits until something else wakes the agent, and every caller of a standing role must remember to collect.

### Consequences

- Good: an end reaches a standing role's agent as it reaches a role's run, with no second agent and no process of flume's.
- Good: the crossing from standing roles to roles that run as kinds changes one entry per address and no launch.
- Bad: a burst of ends while the agent is away is a burst of notices when it returns, one for each end, as for any recipient (43).
- Bad: flume trusts the configuration's name. An agent that takes a standing agent's name in that session takes its ends, which no identity of flume's guards, since flume did not start it.
- The statement's vocabulary, 43, 132 and C.7 say so (blueprints 6d322be). Rule core.28 takes it. The roadmap lists it as decided, not built. The herdr profile's reading of an agent by its name is the change's construct to read and state in C.5's herdr row.
- Unit: a role's address held by a standing agent, its `[roles]` entry read with `agent` and `session`, its ends delivered at a settle, and status naming where each address lives. swancloud's `lib/flume-config.nix` writes the entries from `lib/crew-teams.nix` once a release reads them (swancloud record 0042 as amended).

### The construct's reading, 2026-11-06

Status: decided on 2026-11-06 by the construct stage of flume's change
`a-roles-address-may-be-held-by-a-standing-agent`. Each choice is cheap
to change and none is the user's alone: the rules decide it or leave it
open, and the change's design gives each in full.

Deciders: the construct stage of `a-roles-address-may-be-held-by-a-standing-agent`.
Sources: the amendment above; `herdr --session madswan agent list` under
herdr 0.9.5 on deneb, read 2026-11-06, answering `result.agents[]` with
`name`, `pane_id` and `agent_status`; the statement's amendment on
boomdrivedev/blueprints main at 1c89abc, its C.5 approved by the user on
2026-11-06 (C.5, C.7); rules core.11, core.15, core.28, core.44 and core.49; the change
`every-question-a-place-asks-reaches-someone-who-answers-it`, held for
the user's read.

- **C.5's standing row.** herdr finds the session's socket by one
  `session list --json`, starting no server, and reads `agent list` there
  by `name`, `pane_id` and `agent_status`; an agent of the name is
  reached at `herdr:<pane_id>@<socket>` by the send and state rows. The
  printed command finds none. The statement says so on the branch above.
- **The terminal is the entry's stream's** (190; core.49), as its session
  is, and C.7 says so on that branch. Not taken: every terminal the host
  binds, as a session's recipient is found, which would read two
  terminals' agents of one name as more than one.
- **The terminal seam gives a part's agents by name,** each with the
  session name `send_to` already reaches, and the core counts them. Not
  taken: a lookup by one name, which status would ask once per address
  rather than once per session. No clause of B.5 is added, since 132
  says what the terminal answers.
- **A standing entry holds `agent`, `session` and `stream` alone.** A
  run's other keys beside `agent` make the configuration unreadable,
  naming the address and the key, as any key a role's table does not
  know does. Not taken: keeping them and reading them for nothing, so
  the crossing to a kind swaps one key; a key read for nothing tells a
  reader the agent runs in that repository at that priority.
- **`flume wake` of a standing address launches nothing** and answers
  that a standing agent holds it, with what status says of it. Not
  taken: a refusal, since the carrier asked nothing wrong.
- **The configuration decides between a standing agent and a run** at
  each delivery, so an entry that changes holder moves the next end and
  a run still live of an earlier entry takes none.
- **Status gives every address a standing agent holds,** pending or not,
  read at the ask and once for each terminal and session (core.15).
- **How it meets every-question's design.** That design takes `[roles]`
  out of C.7 and reads the roles from crew's source, and its moves leave
  a role standing until the user moves it. The standing entry is read
  apart from the run's, so that design decides whether crew's file may
  give `agent` for a role not yet moved, or whether such roles go back
  to the door at the stall, and either keeps or drops one reader and one
  step of the delivery.

## Amendment, 2026-11-14: a standing agent's own commands act as the address it holds

Status: decided on 2026-11-14 by boomdrive-design, the design agent of the boomdrive drive. Nothing in it is the user's alone: it holds to clauses 7 and 132, rule core.28 and principle 14, and each choice is cheap to change.

Deciders: boomdrive-design.

Sources: signal `2026-11-14-boomdrive-ops-bca216e0/01` on afterthought/crew-state's `boomdrive/main`, where boomdrive-ops' `flume confirm` in its standing pane acted as `herdr:w2:p3@…/boomdrive/herdr.sock`, which is no reader of the landing's end owed to `boomdrive/ops`, so the end stayed pending; `crates/flume/src/commands/mod.rs` `recipient_of` on main at ae188b74, which takes a command's recipient from `--recipient`, then `$FLUME_RECIPIENT`, then the address of the role's run `$FLUME_PLACE` names, and else the terminal's session; crew record 0057 (crew writes its roles file, each standing role its agent); clauses 7, 43, 132 and 193; principle 14 (boomdrivedev/blueprints `design/principles.md`).

### Context

The amendment of 2026-11-06 delivers an address's ends into the standing agent its source names. What the agent then runs, a `flume confirm`, `flume inbox` or `flume wait`, or a launch of its own, names no recipient, and flume reads the agent as the terminal's session: its pane. A pane is no reader of an end owed to the address, so the agent cannot confirm the end it was handed, and each one stays pending until someone confirms it with `--recipient` written out. A place the agent launches takes its pane as recipient, which a revive changes.

### Considered options

1. **flume reads a standing agent as the address its source declares it holds.**
2. **Whoever starts a standing agent sets `$FLUME_RECIPIENT` to its address in the agent's environment.**
3. **Every standing agent writes `--recipient <address>` on each command.**

### Decision outcome

Option 1.

- **The agent acts as its address.** Where a command names no recipient, `$FLUME_RECIPIENT` names none and the command runs in no role's run, flume asks the terminal which agent the command runs in. Where a source declares that agent, by its name in that session, standing for an address (132), the command acts as that address: a confirm, a collect and a wait as that recipient, and a launch with it as the place's recipient (7). Anywhere else it acts as the terminal's session.
- **One address, or the command asks for one.** An agent its sources declare standing for more than one address acts as none by itself: its command is refused, naming each address, and asks for `--recipient`.
- **What the command names comes first.** `--recipient` and `$FLUME_RECIPIENT` keep their place ahead of this reading, so a person or a tool may still act as another recipient (core.28).

Not taken: option 2, which keeps a second record of which agent holds an address, written by whoever starts the agent, where principle 14 has a tool read what it needs from its own source and never from an environment a host sets for it; a restart by anything else loses it. Option 3, the stopgap the 2026-11-06 amendment refused for delivery: every standing role's brief must carry the flag, and a launch from the pane still names the pane.

### Consequences

- Good: the agent an end was handed to confirms it with no flag, and a place it launches delivers to its address across a revive.
- Bad: a command that names no recipient reads the terminal once more, in a pane flume did not open.
- Bad: as for delivery, flume trusts the name, so an agent that takes a standing agent's name in that session acts as its address.
- Rule core.28 says so. Clause 132 says so on boomdrivedev/blueprints' branch `design/a-standing-agent-acts-as-its-address`.
- Unit: a command that names no recipient, run in a standing agent's pane, acts as the address its source declares the agent holds.
