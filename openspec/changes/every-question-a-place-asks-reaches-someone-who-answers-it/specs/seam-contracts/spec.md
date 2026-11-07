## MODIFIED Requirements

### Requirement: The host gives the addresses standing agents hold

The host SHALL give, as its sources stand at each call, each address a
source gives a standing agent, with the source, the agent's name, the
`session` the entry gives where it gives one, the words of what the
role answers and whether it stands behind its address's stream; its
stream SHALL be its address's; and it SHALL give no run for such an
address (132, 193, C.7).

#### Scenario: Standing and run apart

- **WHEN** the fake host is given a standing agent for `madswan/planner` and a run for `madswan/ops`
- **THEN** its standing entries hold `madswan/planner` alone, and its role for `madswan/planner` is none (132)

## ADDED Requirements

### Requirement: The host gives what its sources declare of roles

The host SHALL give, as its sources stand at each call and keeping
nothing between calls, each address one entry declares with that
entry, a run's, a standing agent's or a carried one's; each entry that
declares nothing for its address, with its source and why; and each
source whose roles file cannot be read, with the file and why (132,
191, 193, C.7).

#### Scenario: What a source declares round-trips

- **WHEN** a run's entry with every key and with none it may leave out, a standing agent's, a carried address's, an entry refused and a roles file not read are written and read
- **THEN** each reads back unchanged, and a run written before sources reads with no source (core.14)

### Requirement: The host answers from what its sources declare

From what its sources declare, the host SHALL give what is declared for
an address or why nothing is, the answerer behind a stream, and the
answering roles, each type serializing to JSON and back unchanged (28,
132, 143, 193; core.14).

#### Scenario: An address's answer, a stream's answerer and the answering roles

- **WHEN** the seams' roles hold a standing agent, a carried address and a run standing behind `east`, and an entry refused for `east/ops`
- **THEN** `east/ops` is answered with the entry's reason, an address no entry names with no source declaring it, `east`'s answerer is the run's address with its source, `north` has none, and the answering roles give each address with words and its stream

#### Scenario: An address's stream

- **WHEN** the seams read `madswan/planner` and `slc-2/ops.1`
- **THEN** each is an address, its stream `madswan` and `slc-2` and its role `planner` and `ops.1`, and a name of any other form is a session's (132, 143, 193)
