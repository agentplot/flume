# Design

## Context

### What is decided

Record 0013's amendment of 2026-11-06, the design agent's, is the
design. It holds to clauses 3, 43 and 132 and rule core.28 as they
stand. The host's configuration names a standing agent for a role's
address, by the name the terminal knows it by and the session it lives
in. flume delivers the address's ends into that agent as notices at the
settles that read them. It opens nothing, wakes no run and keeps no
watch, and status says where the address lives. Its rejected options
were crew naming each launch's recipient by a pane, flume taking the
address as the agent's terminal name, and each standing agent collecting
by hand.

The statement on agentplot/blueprints main at 6d322be says it: the
vocabulary's recipient, held by a run of its kind or by a standing
agent; 43, a recipient named by an address delivered to in the pane of
its live run or of the standing agent the configuration names; 132, the
standing agent found by its name in its session at each delivery, none
or more than one leaving the end pending, and both a standing agent and
a kind refused; C.7, the entry's `agent` and `session`, its session
falling back to the one the host names for the entry's `stream`, else
the default stream's (143). Record 0013's sources call that commit
e6ab2a0, its name before its message was reworded. This branch cites
6d322be.

### What this change adds to the statement

Record 0013 leaves the herdr profile's reading of an agent by its name
to this change, to read and state in C.5's herdr row. The construct read
`herdr --session madswan agent list` under herdr 0.9.5, the pinned
release, on 2026-11-06. It answers `result.agents[]`, each with `name`
(null for an agent started with none), `pane_id`, `agent_status`
(`working`, `idle`, `done`, `blocked`, `unknown`), `workspace_id`,
`tab_id` and more. madswan's standing agents were there by name:
`madswan-design`, `madswan-planner`, `madswan-ops`, `madswan-operator`.
herdr refuses a second agent of one name on a server as
`agent_name_taken` (C.5's open row), and each session is a server of its
own, so herdr lists at most one agent of a name in a session.

The amendment is on agentplot/blueprints main at 1c89abc, its C.5
approved by the user on 2026-11-06:

- **C.5's herdr table gains a standing row.** One `herdr session list
  --json` finds the session's socket by `name` and `running`, flume's
  own being `flume`. A session not listed running holds no agent, and
  nothing is started. Then `agent list` on that socket, read by `name`,
  `pane_id` and `agent_status`. An agent of the name is reached at
  `herdr:<pane_id>@<socket>` by the send and state rows. Read at each
  delivery and each status, kept by nothing.
- **C.5's paragraph on sessions** says a standing agent is reached in
  the pane herdr lists under its name, found again at each delivery, so
  an agent revived under the name is found where it is.
- **C.5's printed command** finds no standing agent.
- **C.7's entry** finds its agent in the terminal of the stream its
  session comes from (190).

No clause is added. The terminal's new operation rests on 132's words,
"where the terminal finds no agent of that name in that session", and
C.5's row. A clause of B.5 for it was the path not taken: 132 already
says what the terminal answers, and a new number would be one more to
reconcile with the designs in flight, which take 193, 198 and 202 to
208.

### What stands in the code

On flume's main at 4b3d2d12:

- **`[roles]`** is read by `role_runs` in `crates/flume/src/host/config.rs`
  into `RoleRun` (`repository`, `kind`, `priority`, `base`, `brief`,
  `stream`). An entry naming no `repository` or `kind`, or any other
  key, makes the configuration unreadable, naming it.
  `Host::role(address)` gives the entry as the configuration stands.
- **Delivery.** `Engine::deliver_ends` (`crates/flume-core/src/ends.rs`)
  finds each pending recipient's terminal by `recipient_terminal`, the
  first terminal the host binds whose `session_state(recipient)` answers,
  claims the delivery in held state, then `send_to(recipient, notice)`.
  A send not made releases the claim and is logged.
- **The herdr profile's reach** (`crates/flume/src/profiles/terminal/herdr/mod.rs`)
  answers `herdr:<pane>@<socket>` by its pane, and a name an open gave a
  role run's agent by the pane its live start holds. Any other name is
  gone.
- **The wake.** `Engine::wake_roles` (`crates/flume-core/src/wake.rs`),
  run at each settle after the delivery, calls `wake_address` for each
  pending address. With no `RoleRun`, it records why on the place, in
  `unwoken`, for status. `Engine::wake` answers `Answer::Woken`.
- **Status.** `HostStatus::addresses` gives each address an end stands
  pending for, its `RoleRun`, its run and why it was not woken.

### Rules that bind

core.md binds the change; no area page exists. It relies on core.1 and
core.2 (the core names no terminal; the terminal is reached through its
seam alone), core.3 (the terminal from the stream's configuration
alone), core.11 (a notice only into an agent asking no question, its
held input cleared first, never left typed), core.13 (no process kept
for the address), core.14 (fields only gained), core.15 (status reads
the terminal at the ask and keeps nothing), core.16 and core.17 (the
delivery's claim written by compare-and-set, safe to repeat), core.25
(an end delivered once to each recipient, by the settle that reads it),
core.28 (the rule this builds), core.29 (each herdr call bounded,
`HERDR_*` stripped, read against 0.9.5), core.35 (the site names no
host of ours), core.45 (`[roles]` the managed layer's) and core.49 (an
agent's terminal is its stream's).

## Goals / Non-Goals

**Goals:**

- An end for an address a standing agent holds reaches that agent at
  the settle that reads it, with no run woken and nothing opened.
- Whatever the agent's pane, it is found by its name at each delivery.
- Status says where each such address lives and why an end stays
  pending.
- One holder per address, refused otherwise.
- The crossing from a standing agent to a run of a kind is one entry
  per address, and no launch changes.

**Non-Goals:**

- Moving `[roles]` out of the host's configuration, or roles onto
  kinds. That is `every-question-a-place-asks-reaches-someone-who-answers-it`'s
  (decision 11).
- Changing `flume inbox`, a launch's recipients or the notice's words.
- An identity guarding a standing agent's name. flume did not start it
  and trusts the configuration's name (record 0013, Consequences).
- Anything a crew role does with a notice.

## Decisions

### 1. The entry

`[roles."<partition>/<role>"]` takes, beside a run's keys, a standing
agent's: `agent`, a string, and an optional `session` and `stream`, each
a string. An entry giving `agent` is a standing agent's. One giving none
is a run's, as today.

The configuration is unreadable, naming the address, where an entry
gives:

- `agent` beside `kind`: "gives both a standing agent and a kind, two
  agents of one role" (3, 132, C.7);
- `agent` beside `repository`, `priority`, `base` or `brief`, naming the
  key as no word of a standing agent's entry;
- `session` with no `agent`, naming it as no word of a role's run, as
  any key the run does not know is today;
- `agent` or `session` empty or not a string.

flume checks no rule of herdr's on the name (core.44's details rule out
a name checked by a rule of flume's). A name herdr cannot hold is one
it never lists, and status says none was found. A local layer's `[roles]`
is not taken, as today (core.45).

The configuration stays unreadable for a faulty entry, as for any
faulty entry today. Keeping a fault to its entry is the path not taken
here. `every-question-a-place-asks-reaches-someone-who-answers-it` keeps
a fault to its file, its entry or its address once a source declares
the roles.

A standing entry's other run keys refused, rather than kept and read
for nothing, was weighed against a crossing that swaps one key. A key
read for nothing would tell a reader the agent runs in that repository
at that priority. A host's configuration is written by a program, which
rewrites an entry as cheaply as a key.

### 2. Where the agent is found

The session is the entry's `session`, else the stream's own session as
a start resolves it (107, 143): `[streams.<stream>] session`, else the
stream's name, and the terminal's own for the default stream naming
none. The stream is the entry's `stream`, else `default`. A stream the
host does not declare reaches nothing, and status says the host declares
no such stream.

The terminal is the stream's, as a start reads it (190; core.49): the
one the host's configuration names for the stream, else the host's. A
stream whose terminal names no profile flume carries reaches nothing,
and status names it. The path not taken was every terminal the host
binds, as a session's recipient is found today. The entry's session is
the stream's, so its terminal is too. Asking every terminal would read
two terminals' agents of one name as more than one.

Each is read as the configuration stands at each delivery and each
status (32, 132). Nothing is recorded.

### 3. The terminal seam's agents in a session

`Terminal` gains one operation:

```rust
/// The agents the part `session` holds, the terminal's own where none,
/// each that carries a name, by that name, with the session name
/// `send_to` and `session_state` reach it by and its state as the
/// terminal reads it; or why the part could not be read. A part that
/// does not run holds none, and nothing is started (132, C.5). The
/// default answers not read: the terminal holds no agent a name finds.
fn agents_in(&self, _session: Option<&str>) -> AgentsIn { … }

pub enum AgentsIn {
    Read { agents: Vec<NamedAgent> },
    Unread { reason: String },
}

pub struct NamedAgent {
    pub name: String,
    pub at: String,
    pub state: AgentState,
}
```

The core counts the agents of the entry's name: one is found, none and
more than one are said so. The terminal decides nothing of names. Its
`at` is a session name the terminal already reaches, so the notice goes
through `send_to` as any recipient's does, with its held input cleared,
its state read again and nothing left typed (95; core.11). The core
holds `at` only for the settle that read it.

Asking by one name was the path not taken. Giving the session's agents
lets status read once for every address whose agent lives in one
session, and keeps counting in the core.

### 4. Delivery at a settle

`deliver_ends` reads the host's standing entries once. For a recipient
the entries name, the standing reach replaces `recipient_terminal`:

1. The stream, its terminal and the session, as decision 2 resolves
   them.
2. `agents_in(session)` on that terminal, filtered by the entry's
   `agent`.
3. One agent working or waiting: the delivery is claimed as today, then
   `send_to(at, notice)` on that terminal. Asking or unknown: the end
   stays pending. Sent counts delivered on the terminal's answer (43).
4. None, more than one, unread, or no stream or terminal: the end stays
   pending, nothing is claimed and nothing is logged. Status reads why
   at its ask.

A refused send releases its claim and is logged, as today. The delivery
is recorded to the address, by notice, with when. Nothing records the
agent or its pane, so the next settle finds the agent afresh.

The reach is read once per recipient in one place's delivery, as a
session's recipient is today. A read shared across one settle of every
place is left out. A standing address's ends are pending only while its
agent is away or asking, and herdr answers its two calls in
milliseconds.

### 5. No run

- **The settle's wake.** `wake_roles` passes over each address the
  configuration gives a standing agent. Nothing is launched and nothing
  is recorded in `unwoken` (132; core.13, core.28).
- **`flume wake <address>`** of a standing address settles as today
  where flume runs attended, launches nothing and answers
  `Answer::Held`: the address, its `StandingStatus` and its pending
  ends. A wake is a carrier's word that the role should look, and the
  standing agent's own pane is where it would look. What wakes it is its
  keeper's. A refusal was the path not taken: the carrier asked nothing
  wrong.
- **Nothing opened.** No terminal part is started for a standing
  address: herdr's `agents_in` starts no server, and the core calls no
  open, attach or show for it.

### 6. The configuration decides between a standing agent and a run

Which agent holds an address is the host's configuration's alone (132).
Where the configuration gives a standing agent for an address whose run,
launched under an earlier entry with a kind, is still live, the end goes
to the standing agent, and the run's pane takes none. Where it gives a
kind for an address a standing agent held, the next end wakes a run,
and the standing agent takes none. So the crossing is one entry, and
the answer the configuration gives at each delivery decides.

### 7. Status

`HostStatus::addresses` gives each address an end stands pending for, as
today, and also every address the configuration gives a standing agent,
pending or not. A standing address with nothing pending is how a host's
operator sees the entry read and the agent found. Each `AddressStatus`
gains `standing`, none for a run's address:

```rust
pub struct StandingStatus {
    pub agent: String,
    pub stream: String,
    pub session: SessionHeld,      // its name and SessionFrom
    pub terminal: Option<String>,  // the profile the stream's terminal names
    pub found: StandingFound,
}

pub enum StandingFound {
    One { at: String, state: AgentState },
    None,
    Several { count: usize },
    Unread { reason: String },
}
```

`SessionFrom` gains `Role`, serialized `role`, for the entry's own
`session`; the stream's `session`, its name and the terminal's own keep
`stream`, `stream-name` and `own`. The terminal is read at the ask, once
for each terminal and session one status asks of (core.15), and nothing
is kept. A standing address's `unwoken` is none.

The person's form gives, for a standing address:

```text
  madswan/planner: the standing agent madswan-planner in the session madswan (the entry's), found waiting; 3 ends pending
  madswan/ops: the standing agent madswan-ops in the session madswan (the stream madswan's name), none of that name there; 1 end pending
```

with each pending end beneath, as today.

### 8. The terminal profiles

- **herdr** (`crates/flume/src/profiles/terminal/herdr/mod.rs`), as C.5's
  standing row reads it. `agents_in(session)` lists the sessions once,
  bounded, with `HERDR_*` stripped (core.29). A session not listed
  running answers `Read` with no agents, and `socket_of` is not called,
  so no server starts. Otherwise `agent list` on that socket, each entry
  with a string `name` read with its `pane_id` and `agent_status`,
  mapped as `pane_state` maps it (`working`; `idle` and `done` waiting;
  `blocked` asking; anything else unknown), its `at` the pane's
  `herdr:<pane_id>@<socket>`. A refusal or an unanswered call answers
  `Unread` in herdr's words. `send_to(at)` then reaches the pane by its
  id, as for any recipient's pane.
- **The printed command** keeps the seam's default, `Unread`: it holds
  no agent a name finds (C.5). herdr and the fake terminal answer their
  own.
- The core names neither (core.1).

### 9. The fakes and the suite

- **The fake host** scripts each address's standing entry,
  `set_standing(address, Option<StandingAgent>)`, and `standing()` gives
  them.
- **The fake terminal** scripts named agents:
  `add_agent(session, name, state) -> String` holds a recipient's
  session under a key of its own, `at`, and lists the agent under
  `session` by its name. `remove_agent(at)` takes it away, and
  `refuse_agents_in(session, reason)` makes the part unread.
  `notices(at)` gives what reached it. Two agents of one name may be
  scripted in one part, which herdr never lists.
- **The terminal seam's scenarios** gain one: an agent the harness
  starts under a name in a session is listed by that name with an `at`
  that `send_to` reaches, and a name no agent carries is not listed. The
  herdr harness starts its stand-in under `herdr agent start <name>` on
  its own scratch server; the fake's scripts it.
- **The engine's scenarios** run every scenario of the deltas over the
  fakes, in `engine/role_waking.rs`, citing 3, 43, 132, C.7 and S38
  where they bear on it.
- **The walkthroughs** gain one under the pinned herdr and the
  stand-in: a fix launched with the recipient `w/planner`, its host's
  configuration giving that address the agent `w-planner` in the session
  `w`, a stand-in started there under that name. The fix's end reaches
  that pane, and no planner place is made.

### 10. What the host writes

Once a release reads them, the host's managed configuration writes one
entry per standing role, whatever program writes it. Each entry names
the agent and, where the agent's session is not its stream's own, the
session:

```toml
# $XDG_CONFIG_HOME/flume/config.toml, the managed layer
[roles."madswan/planner"]
agent = "madswan-planner"
session = "madswan"

[roles."madswan/ops"]
agent = "madswan-ops"
session = "madswan"
```

or, where the partition's stream opens in the agent's session:

```toml
[roles."madswan/planner"]
agent = "madswan-planner"
stream = "madswan"
```

An entry carries no `kind`, `repository`, `priority`, `base` or `brief`.
When the role moves onto a kind, its entry becomes a run's:

```toml
[roles."madswan/planner"]
repository = "~/Code/…"
kind = "…"
priority = 5
```

and no launch changes.

### 11. With the changes in flight

- **`every-question-a-place-asks-reaches-someone-who-answers-it`**, held
  for the user's read. It makes each main-level role a Flume kind at a
  `<stream>/<role>` address crew declares, reads the roles from crew's
  source, and takes `[roles]` out of C.7, a configuration that gives it
  unreadable. It leaves out a session serving an address, since only a
  standing role would need it (its decision 11.5), and has a standing
  role's questions reach the stream's door at the stall until the role
  moves (its decisions 3 and 13). This change makes nothing of it
  harder:
  - The standing reach is apart from the run's. `Host::standing` and
    `StandingAgent` stand beside `Host::role` and `RoleRun`, which that
    change rewrites, and the delivery reads them in one step of its own.
    Its decision 2 changes where an address's entry comes from, and
    this change reads only what an entry says.
  - The address, the recipient, the notice and `flume inbox` are as
    they stand, so its keyed notices and its inbox of waiting places
    build on them unchanged.
  - Its `<stream>/<role>` address names the stream this change's entry
    gives in `stream`, so the session's fallback reads the address's
    stream where it lands.
  - Its decision 13 moves the roles one at a time. Between moves, the
    roles not yet moved stand. With this change their ends reach them at
    the settle. Without it they reach the door at the stall. Where that
    change takes `[roles]` out of the host's configuration, it decides
    whether crew's roles file may give `agent` for a role not yet moved,
    so an entry trades `agent` for `kind` at the move (record 0013's
    amendment), or whether the roles not yet moved go back to the door.
    Either way, what this change builds is a reader of the entry and
    one step of the delivery. The first keeps both, the second drops
    both.
  - It does not add `flume serve`. A standing agent is named by the
    configuration, not served by a command, and needs no process.
- **`a-lost-conversation-resumes-its-session`**, in code. It touches the
  settle's starts, `what_ended` in `ends.rs`, status and the agent seam's
  `live`. It changes neither the herdr profile's `reach` nor the
  terminal seam. The two meet in `ends.rs` and `answer.rs`, each adding
  lines of its own.
- **`every-place-waiting-on-the-merge-queue-is-settled-at-every-depth`**,
  held. It has a caller keep the host's watch by `flume settle --every`,
  and the terminal's watch becomes its nudge (122, 208). A standing
  address is delivered to at whichever settle reads its end, so the
  delivery moves with the settle and needs nothing of the watch. Its C.7
  amendment touches no line of `[roles]`.
- **`an-answer-reaches-the-open-session-within-an-hour`** (record 0046)
  and **`an-agents-effort-is-a-setting-of-flumes-layers`**, held. Neither
  touches a recipient, an address or the terminal's reach.

### 12. The record and the rule

- **Record 0013** cites blueprints main at 6d322be, in its amendment's
  sources and its consequences, where it named e6ab2a0. It gains a
  section of this change's construct choices, decided by the construct
  stage on 2026-11-06, each cheap to change: decision 1's refusals,
  decision 2's terminal, decision 3's operation, decision 5's wake,
  decision 6's precedence and decision 7's status, each with the path
  not taken, and the statement's amendment at 1c89abc.
- **Rule core.28's details** rule out a standing agent's pane kept
  between deliveries, and the agent sought in a terminal other than its
  stream's. They name C.5 and 190 among their sources. The rule's
  sentence stands as written.

### 13. The site

To any host's operator, naming no host, team or word of ours (core.35):

- `guides/roles-and-wake.md`: "An address an agent already holds", the
  entry with `agent`, `session` and `stream`, where the agent is found,
  what goes in and when, why an end stays pending, the wake's answer,
  and the crossing to a kind.
- `reference/configuration.md`: `[roles."<group>/<role>"]`'s `agent` and
  `session`, and what makes the file unreadable.
- `reference/commands.md`: `flume wake`'s answer for a standing address.
- `reference/state.md`: `addresses[].standing`, `session.from`'s `role`,
  and the wake's `held` answer.
- `roadmap.md`: the item "A standing agent holds a role's address" moves
  from planned to what flume does.

## Risks / Trade-offs

- **A burst of notices on return.** An agent away for a day gets one
  notice for each end at the settle that finds it (record 0013,
  Consequences).
- **The name is trusted.** An agent that takes a standing agent's name in
  that session takes its ends. flume did not start it and holds no
  identity for it (record 0013, Consequences).
- **herdr drops a name when its agent restarts within the pane**, as a
  `/clear` does for a recipient's session (`terminal-profiles`, "A
  recipient whose agent herdr renamed"). Its ends then stay pending, and
  status says no agent of the name is there, until its keeper names it
  again.
- **A held input cleared.** A notice to a waiting agent clears what it
  holds typed, as for any recipient (core.11). A person typing into a
  standing agent's pane loses that text when a notice arrives, as a
  person does in any recipient's pane today.
- **Each place's delivery reads the terminal.** A settle of every place
  reads the standing agent's session once for each place with an end
  pending for it, while the agent is away or asking. That is two bounded
  herdr calls each, as a session's recipient costs one or two today.

## Migration Plan

- **A host's configuration** with no `agent` reads as before. An entry
  with `agent` makes an earlier release's configuration unreadable,
  since an earlier release refuses a key a role's table does not know.
  So the host writes its entries once it runs a release that reads them.
- **The 22 ends pending for `madswan/planner`** on deneb, and the others
  pending for standing roles, are delivered at the first settle that
  finds the agent waiting, one notice each.
- **Rollback** is the earlier release with the entries taken out.
  Nothing in held state changes: a delivery to an address is recorded as
  any delivery is.

## Open Questions

None.
