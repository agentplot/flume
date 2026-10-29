# seam-contracts Specification

## Purpose
The seven seams of Part B as one interface each, with the types they
pass, so every profile of Part C codes against them and the core reaches
no tool but through them (core.1, core.2).

## Requirements

### Requirement: One interface per seam

There SHALL be exactly one interface for each of the seven seams of Part
B: the place (78 to 83), the kinds (84, 85), the change (86, 87, 127),
the agent (88 to 93, 115, 116, 121, 134, 135), the terminal (94 to 98,
122, 136 to 139), the merge queue (99, 101, 117, 125, 126, 133) and the
store (102 to 104, 128). Each SHALL offer the operations its section
names and no operation of the core's, and no interface SHALL name a tool
(core.1, core.2).

#### Scenario: Each seam's operations

- **WHEN** each seam's scenarios run against its fake
- **THEN** each operation its section of Part B names is exercised, and each scenario cites the clause of its operation

#### Scenario: No tool named outside a profile

- **WHEN** the seams', core's, fakes', suite's and commands' sources are searched for `wt`, `worktrunk`, `openspec`, `claude`, `herdr` and `sluice` as words
- **THEN** none is found outside a `profiles` directory

### Requirement: The merge queue's submit holds no wait

The merge queue's submit SHALL answer once the merge queue holds the
submission, with no wait on its verdict, and its inbox SHALL give each
pending outcome of the place once (117, 133).

#### Scenario: Submit holds no wait

- **WHEN** the fake merge queue holds a submission undecided
- **THEN** submit answers with the submission at once, and a later inbox gives its outcome once and then nothing (117, 133)

### Requirement: What the agent and terminal seams read

The agent seam SHALL say whether the agent's record shows its session
ended (134) and whether the agent has said it started under its start's
identity (135); the terminal seam SHALL name the session a command runs
in, for a launch's default recipient (7), and give what it shows of an
agent (136).

#### Scenario: Started and screen

- **WHEN** the fake agent holds no started word for a start, and the fake terminal shows lines for its agent
- **THEN** `started` reads no and `screen` gives those lines (135, 136)

### Requirement: The terminal seam's open, tidy and show

The terminal seam's open SHALL answer, beside opened and refused, a
start held at a question its program asks before it starts and a program
that did not come up within its bound, each with the screen (94); it
SHALL tidy what it made for a place gone (138) and show a place's clues
(139); and every terminal operation SHALL answer within a bound the
profile states (98).

#### Scenario: Held, tidied and shown

- **WHEN** the fake terminal holds a start at a question, makes a place's tab in its base's workspace, and has a person's pane in that workspace
- **THEN** open answers held with the screen, tidy closes the place's tab and keeps the workspace naming the person's pane, and show takes the place's clues (94, 138, 139)

### Requirement: The base, a change's location and a place's folder

The merge queue seam SHALL answer the revision the base stands at where
its profile says the base lives, after that profile's fetch, or a
refusal naming why (126; 9, core.7); the change seam where a change
lies in a place, for a prompt's change part (127); and the store the
place's own folder beside its held state, for a report kept there (128,
123).

#### Scenario: The base where the merge queue says it lives

- **WHEN** the fake merge queue says the base lives at a revision other than the local base's
- **THEN** the place is cut at that revision

#### Scenario: No base revision readable

- **WHEN** the fake merge queue refuses to give the base's revision, naming a failed fetch
- **THEN** the launch is refused naming the fetch, and nothing is made

### Requirement: The types the seams pass

The seams SHALL pass one type for each of: a launch with its priority,
its recipients and each hold's words (2, 3, 7, 8, 70, 130); a place and
its held state, its priority and each stage's wait for room among it
(62, 70, 73 to 76, 102 to 104); a kind with its stages as read-kinds
returns them (18, 23); a deliverable in one of the six forms (24); and a
prompt and its parts (27, 28). Each SHALL serialize to JSON and back
unchanged.

#### Scenario: A type round-trips

- **WHEN** a value of each type is written to JSON and read back
- **THEN** it equals the value written

#### Scenario: A kind with two agents or two deliverables

- **WHEN** a kind's stage carries two agents or two deliverables as read-kinds returns them
- **THEN** the type keeps both, so the core can refuse the kind (4, 23)

### Requirement: The types of an agent, the terminal and a submission

The seams SHALL pass one type for each of: an agent's identity, its
start's secret and a stop-short that needs an answer or found work (29,
30, 33, 41, 92); an agent's command with the program's folder-trust
question, its answer and the program's kind (34, 88, C.4); the
terminal's clues and what its tidy closed and kept (138, 139); and a
submission with its outcomes, landed, failed, in conflict or withdrawn
(99, 111 to 113, 117, 125, 133). Each SHALL serialize to JSON and back
unchanged.

#### Scenario: An agent's command and a withdrawn submission round-trip

- **WHEN** an agent's command carrying its folder-trust question, its answer and its kind, and a submission withdrawn, are each written to JSON and read back
- **THEN** each equals the value written (34, 88, 125)

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

### Requirement: The terminal reaches a recipient's session by its name

The terminal seam SHALL send a text to, and read the state of, a
recipient's session named as the terminal names sessions, the same
guarantee as a send to an agent: delivered as input or refused, never
left typed; and a recipient the terminal cannot find SHALL be answered
as not found, never guessed (43, 95, 96, 98).

#### Scenario: A session by its name

- **WHEN** the terminal scenarios send a notice to a recipient's session by its name, then to a name no session holds
- **THEN** the first is sent and the session's state is read, and the second is answered not found with nothing sent

### Requirement: An agent opened under a session name

The terminal seam's open SHALL take the session name the agent is to
carry, a role run's address where the place is one, and a send or a
state read by that name SHALL reach that agent (94, 107, 132).

#### Scenario: A role run's agent

- **WHEN** an agent is opened under the session name `madswan/planner`
- **THEN** a send to the recipient `madswan/planner` reaches that agent, and a second open under that name while it is live is refused (98)
