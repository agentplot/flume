## Purpose

A stream's agents shared among labels: a place's label from the host's
configuration or its kind, a label below its reserve started first
within its stream and the slots it keeps free held from the others, a
stream whose labels are at fault starting no stage, and status giving
each place's label and each stream's labels (180 to 182).

## ADDED Requirements

### Requirement: A place's label

A place's label SHALL be the one `label` in the host's
`[repos."<repository>".kinds.<kind>]` gives, else in `[kinds.<kind>]`,
else its kind's own, else none, with where it came from, read at each
start; an agent SHALL count toward the label it started under until it
ends; a label SHALL be one word of letters, digits, `-` and `_`; and
flume's own kinds SHALL declare none (180, C.2, C.7).

#### Scenario: Each source of a label

- **WHEN** the host gives flume's `research` `label = "inception"` in `[kinds.research]`, the kind `deploy` that the repositories `/r` and `/s` declare gives `label = "operation"`, and the host's `[repos."/r".kinds.deploy]` gives `label = "construction"`
- **THEN** a research place's label is `inception` from `host-kind`, a deploy place in `/r` is `construction` from `host-repository-kind`, and one in `/s` is `operation` from `kind` (180, S85)

#### Scenario: A label changed between starts

- **WHEN** the host's `[kinds.fix] label` is changed from `construction` to `operation` while a fix's code agent runs
- **THEN** that agent counts toward `construction` until it ends, and the fix's verify starts as `operation`'s (32, 180)

#### Scenario: flume's own kinds

- **WHEN** flume's own kinds are read
- **THEN** none declares a label (180)

#### Scenario: A label that is no word

- **WHEN** the managed layer gives `[kinds.fix] label = "two words"`
- **THEN** reading the configuration is refused naming `kinds.fix.label` (180)

### Requirement: A label's reserve and the slots it keeps free

A stream SHALL give a label, in `[streams.<stream>.labels.<label>]`,
a `reserve` and a `keep_free`, zero where it gives none. In the stream
a stage of a label running fewer agents than its reserve SHALL start
before every stage of a label running its reserve or more, whatever
their priorities; a stage SHALL start only while its stream's room
passes what its other labels keep free and do not use; and no agent
SHALL be ended for a reserve (31, 70, 181).

#### Scenario: A reserve lent and taken back, a slot kept free

- **WHEN** `work` has five agents and owes `operation` two, keeping one free, and `construction` two; four construction places and a research place of no label are launched, then an operation place, then a second operation place
- **THEN** four construction agents start and the research place waits on the slot kept free for operation; the first operation place starts at once in it; the second waits until an agent ends, then starts before the research place and every construction place waiting, whatever their priorities, and no agent is ended for it (70, 180, 181, S84)

#### Scenario: One settle ranks again as it starts

- **WHEN** `work` has room for two, `operation` runs one agent of its reserve of two and `construction` three of its reserve of two, and two operation places at priority zero and a construction place at priority 10 wait
- **THEN** one settle starts an operation place, then, operation at its reserve, the construction place by its priority, and the second operation place waits (70, 181)

#### Scenario: A reserve where the host has no room

- **WHEN** the host's bound is reached and an operation place below its reserve waits in `work`, which has room
- **THEN** it waits on the host's room, as any stage does (70, 181)

#### Scenario: A stream with no bound

- **WHEN** `personal` states no bound and `[streams.personal.labels.operation]` gives `reserve = 1`
- **THEN** no reserve is held in `personal`, and status names its reserves passed over (181)

#### Scenario: A reserve set locally

- **WHEN** the local layer gives `streams.work.labels.operation.reserve = 3`
- **THEN** the next start in `work` holds three for `operation`, from the local layer (181, C.7, S85)

### Requirement: What a stream's labels refuse

A stream whose reserves together pass its bound, whose label keeps
more free than its reserve, or whose label's table holds any key but
`reserve` and `keep_free` SHALL start no stage, each refused naming the
stream and why, until the configuration is mended; a label SHALL have
no pool, account, points bound or pace of its own (4, 181, 182).

#### Scenario: A pool in a label's table

- **WHEN** the managed layer's `[streams.work.labels.operation]` gives `pool = "ops"` and a fix is launched on `work`
- **THEN** its start is refused naming `streams.work.labels.operation.pool` (182, S85)

#### Scenario: Reserves past the bound

- **WHEN** `work` has five agents, owes `operation` three and `construction` three, and a fix is launched on `work`
- **THEN** its start is refused naming `work`, its reserves of six and its bound of five (4, 181)

#### Scenario: More kept free than owed

- **WHEN** `[streams.work.labels.operation]` gives `reserve = 1` and `keep_free = 2`, and a fix is launched on `work`
- **THEN** its start is refused naming `operation`'s two kept free of a reserve of one (4, 181)

#### Scenario: The configuration mended

- **WHEN** the configuration of a refused start is mended and its stage retried
- **THEN** it starts (4, 181)

### Requirement: Status gives labels and the room they keep

Status SHALL give each place's label with where it came from, and for
each stream its labels, each with its reserve, what it keeps free, its
agents running and its stages waiting, whether its reserves are held
or passed over, and what it refuses; a stage waiting on room another
label keeps free SHALL say so (56, 180, 181).

#### Scenario: The labels in status

- **WHEN** status is asked of a host where research runs as `inception` and deploy as `construction`, and `work` owes `operation` two, keeping one free
- **THEN** each place gives its label and where it came from, and `work` gives `operation` and `construction`, each with its reserve, what it keeps free, its agents running and its stages waiting, in the JSON form and the person's (56, S85)

#### Scenario: Waiting on a slot kept free

- **WHEN** status is asked while a research place waits as the reserve scenario has it
- **THEN** its entry in the queue wants `work`'s room kept free for `operation`, in JSON and in words, and it waits on the host, never on its caller (56, 70, 181)
