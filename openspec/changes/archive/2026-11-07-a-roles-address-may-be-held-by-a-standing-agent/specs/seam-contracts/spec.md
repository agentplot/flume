## ADDED Requirements

### Requirement: The host gives the addresses standing agents hold

The host SHALL give, as its configuration stands at each call, each
address the configuration gives a standing agent, with the agent's
name, and the `session` and `stream` the entry gives where it gives
them; and SHALL give no run for such an address (132, C.7).

#### Scenario: Standing and run apart

- **WHEN** the fake host is given a standing agent for `madswan/planner` and a run for `madswan/ops`
- **THEN** its standing entries hold `madswan/planner` alone, and its role for `madswan/planner` is none (132)

### Requirement: The terminal seam gives a session's agents by name

The terminal seam SHALL give the agents a part holds, by part name or
its own, each carrying a name, with that name, the session name a send
reaches it by, and its state; a part that does not run holds none and
is not started; a part it cannot read is answered so, in its words
(132, C.5; core.13).

#### Scenario: An agent by its name

- **WHEN** the terminal scenarios start an agent under a name in a part, then ask for that part's agents
- **THEN** the agent is listed by its name with its state, a send to its session name reaches it, and a name no agent carries is not listed (95, 132)

### Requirement: The types of a standing agent

The seams SHALL pass one type each for a standing agent's entry, a
part's named agents, and an address's standing as status gives it with
what was found, and `SessionFrom` SHALL take `role`; each SHALL
serialize to JSON and back unchanged (56, 132; core.14).

#### Scenario: A standing address round-trips

- **WHEN** a standing entry, a part's agents, and an address's standing found one, none, several and unread, with a session from `role`, are written and read
- **THEN** each reads back unchanged, and an address written before this change reads with no standing (core.14)
