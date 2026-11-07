## ADDED Requirements

### Requirement: The host's configuration may name a standing agent for an address

A `[roles]` entry SHALL be a standing agent's where it gives `agent`, a
string naming the agent as the terminal knows it, with an optional
string `session` and `stream`; the configuration SHALL be unreadable,
naming the address, where an entry gives `agent` beside `kind`, or
beside `repository`, `priority`, `base` or `brief`, naming the key, or
`session` with no `agent` (3, 132, C.7).

#### Scenario: A standing agent's entry read

- **WHEN** the host's configuration gives `[roles."madswan/planner"]` `agent = "madswan-planner"` and `session = "madswan"`, and `[roles."madswan/ops"]` a repository and a kind
- **THEN** the host gives `madswan/planner` a standing agent `madswan-planner` in `madswan` and no run, and `madswan/ops` its run and no standing agent (132, C.7)

#### Scenario: A standing agent and a kind

- **WHEN** an entry gives `agent = "madswan-planner"` and `kind = "planner"`
- **THEN** the configuration is unreadable, naming `madswan/planner` as given both a standing agent and a kind (3, 132)

#### Scenario: A run's key beside a standing agent

- **WHEN** an entry gives `agent = "madswan-planner"` and `priority = 5`
- **THEN** the configuration is unreadable, naming `madswan/planner` and `priority` (C.7)

#### Scenario: A session with no agent

- **WHEN** an entry gives a repository, a kind and `session = "madswan"`
- **THEN** the configuration is unreadable, naming `session` as no word of a role's run (C.7)

### Requirement: A standing address wakes no run

A settle that finds an end pending for an address the configuration
gives a standing agent SHALL launch nothing for it, record nothing of
it as unwoken, and open, start and watch nothing for it, whatever the
terminal finds (132; core.13, core.28).

#### Scenario: Ends pending while the agent is away

- **WHEN** three ends are pending for `madswan/planner`, whose standing agent the fake terminal holds nowhere, and the place is settled twice
- **THEN** no place is made, the fake terminal records no open, and the place's unwoken names nothing for the address (132, S38)

### Requirement: A wake of a standing address launches nothing

`flume wake <address>` of an address the configuration gives a standing
agent SHALL launch nothing and SHALL answer that a standing agent holds
it, with what status says of the address: its agent, its session, what
the terminal found and its pending ends (132).

#### Scenario: A carrier wakes a standing address

- **WHEN** a carrier runs `flume wake madswan/planner`, whose standing agent the fake terminal holds waiting
- **THEN** the answer is held, naming `madswan-planner`, its session and the agent found waiting, and no place is made (132)

### Requirement: The configuration decides who holds an address

Where the configuration gives an address a standing agent, an end for
it SHALL go to that agent and not to a live run of an earlier entry;
where it gives a kind, an end SHALL wake or reach the run and not a
standing agent; each read as the configuration stands at the delivery
(32, 132).

#### Scenario: A standing agent over a run still live

- **WHEN** `madswan/planner`'s run, woken under an entry with a kind, is live, the entry then gives `agent = "madswan-planner"`, and an end for the address is read
- **THEN** the notice goes to the standing agent, and the run's session receives none (132)

#### Scenario: A standing agent's entry becomes a run's

- **WHEN** `madswan/planner`'s entry gives `agent`, its agent takes one end, the entry then gives a repository and a kind, and another end for the address is read
- **THEN** a run of the kind is woken for the second end, and the standing agent receives no second notice (132)
