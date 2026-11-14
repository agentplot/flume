## REMOVED Requirements

### Requirement: A label's reserve and the slots it keeps free

**Reason**: A label's table holds `owed` and `reserved`, and `reserve`
and `keep_free` are no words of it (181, 182; record 0063).

**Migration**: "What a label is owed and holds reserved" below, which
holds its scenarios in 181's words.

### Requirement: What a stream's labels refuse

**Reason**: Its refusals are worded in `owed` and `reserved`, and a
label's table naming `reserve` or `keep_free` refuses as any other key
does (181, 182; record 0063).

**Migration**: "A stream whose labels are at fault starts no stage"
below, which holds its scenarios, "Reserves past the bound" as "Owed
past the bound" and "More kept free than owed" as "More reserved than
owed".

### Requirement: Status gives labels and the room they keep

**Reason**: Status gives each label's `owed` and `reserved`, keeping
`reserve` and `keep_free` in the JSON form (61, 181; record 0063).

**Migration**: "Status gives what each label is owed and holds
reserved" below, which holds its scenarios, "Waiting on a slot kept
free" as "Waiting on an agent held reserved".

## ADDED Requirements

### Requirement: What a label is owed and holds reserved

A stream SHALL give a label, in `[streams.<stream>.labels.<label>]`,
`owed`, the stream's agents its places are first in line for, and
`reserved`, those held for it alone, each a whole number of agents
from 0, 0 where none is given. A stage whose label runs fewer than it
is owed SHALL start before its stream's stages whose labels run what
they are owed or more, whatever their priorities, and only while the
stream's room passes what its other labels hold reserved and do not
use (31, 70, 181).

#### Scenario: An owed agent lent and taken back, a reserved agent held empty

- **WHEN** `work` has five agents and owes `operation` two, one of them reserved, and `construction` two; four construction places and a research place of no label are launched, then an operation place, then a second operation place
- **THEN** four construction agents start and the research place waits on the agent held reserved for operation; the first operation place starts at once in it; the second waits until an agent ends, then starts before the research place and every construction place waiting, whatever their priorities, and no agent is ended for it (70, 180, 181, S84)

#### Scenario: One settle ranks again as it starts

- **WHEN** `work` has room for two, `operation` runs one agent of the two it is owed and `construction` three of the two it is owed, and two operation places at priority zero and a construction place at priority 10 wait
- **THEN** one settle starts an operation place, then, operation running what it is owed, the construction place by its priority, and the second operation place waits (70, 181)

#### Scenario: What a label is owed where the host has no room

- **WHEN** the host's bound is reached and an operation place below what it is owed waits in `work`, which has room
- **THEN** it waits on the host's room, as any stage does (70, 181)

#### Scenario: A stream with no bound

- **WHEN** `personal` states no bound and `[streams.personal.labels.operation]` gives `owed = 1`
- **THEN** nothing is owed in `personal`, and status names what its labels are owed passed over (181)

#### Scenario: What a label is owed set locally

- **WHEN** the local layer gives `streams.work.labels.operation.owed = 3`
- **THEN** the next start in `work` owes `operation` three, from the local layer (181, C.7, S85)

### Requirement: A stream whose labels are at fault starts no stage

A stream whose labels are owed more together than its bound, whose
label holds more reserved than it is owed, or whose label's table holds
any key but `owed` and `reserved`, `reserve` and `keep_free` among
them, SHALL start no stage, each refused naming the stream and why,
until the configuration is mended; no key of a label's table SHALL be
read as another's; and a label SHALL have no pool, account, points
bound or pace of its own (4, 181, 182).

#### Scenario: A pool in a label's table

- **WHEN** the managed layer's `[streams.work.labels.operation]` gives `pool = "ops"` and a fix is launched on `work`
- **THEN** its start is refused naming `streams.work.labels.operation.pool` (182, S85)

#### Scenario: A label's table naming reserve

- **WHEN** the managed layer's `[streams.work.labels.operation]` gives `reserve = 2` and a fix is launched on `work`
- **THEN** its start is refused naming `streams.work.labels.operation.reserve`, and `operation` is owed nothing (181, 182)

#### Scenario: A label's table naming keep_free

- **WHEN** the managed layer's `[streams.work.labels.operation]` gives `owed = 2` and `keep_free = 1`, and a fix is launched on `work`
- **THEN** its start is refused naming `streams.work.labels.operation.keep_free`, and `operation` holds nothing reserved (181, 182)

#### Scenario: Owed past the bound

- **WHEN** `work` has five agents, owes `operation` three and `construction` three, and a fix is launched on `work`
- **THEN** its start is refused naming `work`, what its labels are owed, six in all, and its bound of five (4, 181)

#### Scenario: More reserved than owed

- **WHEN** `[streams.work.labels.operation]` gives `owed = 1` and `reserved = 2`, and a fix is launched on `work`
- **THEN** its start is refused naming `operation`'s two reserved of the one it is owed (4, 181)

#### Scenario: The configuration mended

- **WHEN** the configuration of a refused start is mended and its stage retried
- **THEN** it starts (4, 181)

### Requirement: Status gives what each label is owed and holds reserved

Status SHALL give each place's label with where it came from, and each
stream's labels, each with what it is owed, what it holds reserved,
its agents running and its stages waiting, whether what they are owed
is held, and what they refuse. Each label's JSON SHALL give `owed` and
`reserved` and keep `reserve` and `keep_free`, the same numbers (61).
A stage waiting on room another label holds reserved SHALL say so, its
JSON want kept `kept-free` (56, 180, 181).

#### Scenario: The labels in status

- **WHEN** status is asked of a host where research runs as `inception` and deploy as `construction`, and `work` owes `operation` two, one of them reserved
- **THEN** each place gives its label and where it came from, and `work` gives `operation` and `construction`, each with what it is owed, what it holds reserved, its agents running and its stages waiting: in the JSON form `operation`'s `owed` 2 and `reserved` 1 beside its `reserve` 2 and `keep_free` 1, and in the person's `label operation: owed 2, reserved 1` (56, 61, S85)

#### Scenario: Waiting on an agent held reserved

- **WHEN** status is asked while a research place waits as the owed scenario has it
- **THEN** its entry in the queue wants `work`'s room held reserved for `operation`, `kept-free` naming `operation` in JSON and the room held reserved in words, and it waits on the host, never on its caller (56, 70, 181)
