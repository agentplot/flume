# A role's address may be held by a standing agent

## Why

Every main-level role of crew's now stands as an agent crew keeps in a
herdr session, under a name of its own, `madswan-planner` among them.
crew still names every place's recipient by the role's address,
`madswan/planner`. flume reaches an address only in the pane of a run it
opened: the herdr profile's `reach` finds a `herdr:<pane>@<socket>`, or a
name its own open recorded, and nothing else. A `[roles]` entry with a
kind would wake a second agent of the role beside the standing one. So
every end for a standing role stays pending: 22 for `madswan/planner` on
deneb on 2026-11-06, read by nobody until the agent runs `flume inbox`
by hand.

Record 0013's amendment of 2026-11-06 decides it, and rule core.28 takes
it. The statement says it in the vocabulary's recipient, 43, 132 and
C.7, on agentplot/blueprints main at 6d322be. This change's own reading
of herdr, C.5's standing row, and the terminal C.7's entry finds its
agent in, are on blueprints' branch
`design/flume-a-standing-agent-is-found-by-its-name` at b8abde8, cut
from main at 51b6316 (core.19).

## What Changes

- **A `[roles]` entry may name a standing agent (C.7).**
  `[roles."<partition>/<role>"]` with `agent`, the terminal's name for
  an agent a caller keeps standing, and an optional `session` and
  `stream`. With no `session`, the agent is found in the session the
  host names for the entry's `stream`, else the default stream's (143).
  It is found in that stream's terminal (190). The table stays the
  managed layer's.
- **One holder for an address (3).** An entry giving `agent` beside
  `kind` makes the host's configuration unreadable, naming the address.
  So does `agent` beside a run's other keys, naming the address and the
  key, as any key a role's table does not know does today.
- **Its ends go into the agent (43, 132).** At each settle that reads an
  end pending for the address, the terminal finds the agent by its name
  in its session, and the end goes in as a notice, only while the agent
  asks no question. It counts delivered on the terminal's answer.
  Nothing is kept of where the agent was, so a revive that keeps the
  name keeps the address. No agent of the name, or more than one, leaves
  the end pending.
- **It wakes no run (132; core.13, core.28).** The settle launches
  nothing for a standing address. `flume wake <address>` launches
  nothing and answers that a standing agent holds it. flume opens
  nothing for the address and keeps no watch over it.
- **Status says where each address lives (56, 132; core.15).** The
  host's status gives every address a standing agent holds, pending or
  not: its agent, its session with where that came from, its stream and
  terminal, and what the terminal found at the ask, one agent with its
  state, none, more than one, or why it could not read.
- **Unchanged.** The address stays every launch's recipient, so crew's
  launches and the places already launched change nothing. `flume
  inbox` collects for the address as before.
- **The terminal seam (C.5).** A new operation gives a session's agents
  by name, each with the session name `send_to` reaches it by. herdr
  reads one `session list --json` and one `agent list`. The printed
  command finds none. The fake terminal scripts named agents.
- **Record 0013** cites blueprints main at 6d322be, the commit its
  sources named before the message was reworded, and gains this
  change's construct choices. **Rule core.28's** details name the
  terminal it is found in and what is never kept.
- **The site** says how a host names a standing agent for an address,
  the status it gives and the wake's answer, and the roadmap moves the
  item from planned to built.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `role-waking`: an address the host's configuration gives a standing
  agent, its entry's keys and refusals, no run woken at a settle or by
  `flume wake`, and the configuration deciding between a standing agent
  and a run.
- `end-delivery`: an end for a standing address delivered into the agent
  the terminal finds by its name in its session at each settle.
- `place-status`: where each address lives, read at the ask.
- `seam-contracts`: the host's standing entries, and the terminal seam's
  agents of a session by name.
- `seam-fakes`: the fake host's standing entries, and the fake
  terminal's named agents.
- `terminal-profiles`: herdr's agents of a session by name, and the
  printed command's none.

## Impact

- `crates/flume-seams`: `StandingAgent` and `Host::standing`;
  `Terminal::agents_in`, `AgentsIn` and `NamedAgent`; `SessionFrom::Role`;
  their round trips.
- `crates/flume-core`: a module for the standing reach; `ends.rs`
  delivering through it; `wake.rs` waking no run for a standing address
  and listing every standing address; `answer.rs`'s `StandingStatus`,
  `AddressStatus::standing` and `Answer::Held`.
- `crates/flume`: the host configuration's reader of `[roles]`; the herdr
  profile's and the printed command's `agents_in`; the person's form of
  status and of `flume wake`.
- `crates/flume-fakes`: the fake host's standing entries and the fake
  terminal's named agents.
- `crates/flume-conformance`: the terminal seam's scenario of a named
  agent, and the engine's scenarios of a standing address.
- `site/`: `guides/roles-and-wake.md`, `reference/configuration.md`,
  `reference/commands.md`, `reference/state.md` and `roadmap.md`.
- `docs/adr/0013-…` and `docs/architecture/core.md`'s core.28 details.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency.
- The host's managed configuration writes its entries once a release
  reads them, design 10 giving the TOML.
