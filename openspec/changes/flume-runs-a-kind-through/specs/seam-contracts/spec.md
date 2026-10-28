## MODIFIED Requirements

### Requirement: One interface per seam

There SHALL be exactly one interface for each of the seven seams of
Part B: the place (78 to 83), the kinds (84, 85), the change (86, 87, 127),
the agent (88 to 93, 115, 116, 121, 134), the terminal (94 to 98, 122), the
merge queue (99, 101, 117, 125, 126, 133) and the store (102 to 104,
128). Each SHALL offer the operations its section names and no
operation of the core's, and no interface SHALL name a tool (core.1,
core.2). The merge queue's submit SHALL answer once the merge queue
holds the submission, with no wait on its verdict, and its inbox SHALL
give each pending outcome of the place once (117, 133). The terminal
seam SHALL name the session a command runs in, for a launch's default
recipient (7), and the agent seam SHALL say whether the agent's record
shows its session ended (134).

#### Scenario: Each seam's operations

- **WHEN** each seam's scenarios run against its fake
- **THEN** each operation its section of Part B names is exercised, and each scenario cites the clause of its operation

#### Scenario: No tool named outside a profile

- **WHEN** the seams', core's, fakes', suite's and commands' sources are searched for `wt`, `worktrunk`, `openspec`, `claude`, `herdr` and `sluice` as words
- **THEN** none is found outside a `profiles` directory

#### Scenario: Submit holds no wait

- **WHEN** the fake merge queue holds a submission undecided
- **THEN** submit answers with the submission at once, and a later inbox gives its outcome once and then nothing (117, 133)

### Requirement: The types the seams pass

The seams SHALL pass one type for each of: a launch with its priority,
its recipients and each hold's words (2, 3, 7, 8, 70, 130); a place and its held state, its priority and each
stage's wait for room among it (62, 70, 73 to 76, 102 to 104); a kind
with its stages as read-kinds returns them (18, 23); a deliverable in
one of the six forms (24); a prompt and its parts (27, 28); an agent's
identity, its start's secret and a stop-short that needs an answer or
found work (29, 30, 33, 41, 92); an agent's command with the program's
folder-trust question and its answer (34, 88); and a submission with its
outcomes, landed, failed, in conflict or withdrawn (99, 111 to 113, 117,
125, 133). Each SHALL serialize to JSON and back unchanged.

#### Scenario: A type round-trips

- **WHEN** a value of each type is written to JSON and read back
- **THEN** it equals the value written

#### Scenario: A kind with two agents or two deliverables

- **WHEN** a kind's stage carries two agents or two deliverables as read-kinds returns them
- **THEN** the type keeps both, so the core can refuse the kind (4, 23)

### Requirement: Git's history and the host's facts beside the seams

The core SHALL read git's history and the host's facts through two
interfaces of their own, which are no seam of Part B (section 9): a
branch's head, whether a head holds commits another does not, the
newest commit at a head touching a path, whether a branch name is
valid; and the host's clock, a new secret, a file's write time, whether
a command of flume's is still live, the bound on agents the host's
configuration states, and a host-wide lock over the room (70).

#### Scenario: A commit since the stage started

- **WHEN** the fake history holds a head that reaches a commit the stage's start head does not
- **THEN** the history answers that the head holds a commit since the start (25)

#### Scenario: An agent reset to an older commit

- **WHEN** the head is moved back to an ancestor of the start head
- **THEN** the history answers that the head holds no commit since the start

#### Scenario: The fake host's bound

- **WHEN** a test sets the fake host's bound to two
- **THEN** the core reads a bound of two, and none where the test sets none
