## Purpose

An agent's mode, how far it may act in its place without asking a
person: flume's four, plan, ask, edits and full, or a table of programs
each giving its program's own value; given by any of the six layers and
taken whole from the first; mapped by the agent's profile to its
program's setting that means it, or refused naming the mode, the
program, the stage and the layer; and none passed where no layer gives
one, so the account's own configuration decides.

## ADDED Requirements

### Requirement: A mode is one of flume's four or a table of programs

A mode SHALL be one of `plan`, `ask`, `edits` and `full`, or a table of
programs, each entry one of the four or a value of that program's own,
kept as written. A value given bare that is not one of the four SHALL
be refused where it is read, naming it and saying that a program's own
value is given under its program's name; a table that names no program
SHALL be refused too (162, 169, C.2).

#### Scenario: A program's own value given bare

- **WHEN** a launch's declaration gives `[kinds.fix.agents] mode = "auto"` and a fix is launched from it
- **THEN** the launch is refused naming `kinds.fix.agents.mode` and `auto`, saying it goes under its program's name, and nothing is made (4, 169)

#### Scenario: A table of programs

- **WHEN** a launch's declaration gives `[kinds.fix.agents] mode = { agent = "careful", other = "edits" }` and the fake agent's profile names `careful` as a value of its own
- **THEN** the fix's agent starts with the value `careful` passed, status giving the mode as the table was written and the entry `agent` (162, 169)

### Requirement: Every layer may give the mode, taken whole

An agent's mode SHALL come from the first of the six layers that gives
one: the launch's, the host's for the repository and kind, the host's
for the kind, the repository's for a kind it does not declare, the
kind's, and flume's default, which gives none; within each a stage's
own SHALL come before one for every stage. The host's layers SHALL be
read as they stand at each start, and the others as the launch read
them (8, 21, 32, 162).

#### Scenario: The host's table over the repository's

- **WHEN** the host's `[kinds.fix.agents]` gives `mode = "edits"`, the repository's `[kinds.fix.agents]` gives `mode = "full"`, and a fix is launched
- **THEN** its agent starts with the value the agent's profile maps `edits` to, status naming `host-kind` as where its mode came from (S66, 32, 162)

#### Scenario: A stage's own before every stage's

- **WHEN** the kind's declaration gives `mode = "edits"` for every stage and `mode = "ask"` for verify, and the launch gives `--agent mode=full`
- **THEN** every stage's agent starts in `full` from `launch`; launched again with no `--agent`, construct's and code's start in `edits` and verify's in `ask`, each from `kind` (32)

#### Scenario: The host's table changes while a stage works

- **WHEN** the host's `[kinds.change.agents]` changes from `mode = "edits"` to `mode = "full"` while code's agent works
- **THEN** code's agent keeps the mode it started with, and verify's starts in `full` (8, 32)

### Requirement: No mode where no layer gives one

Where no layer gives a mode, flume SHALL give the agent none: its
profile is passed no mode, the start's record SHALL name none, and its
word that it started SHALL be compared with no mode, so the account's
own configuration decides (32, 161, 162).

#### Scenario: A host and a repository that say nothing

- **WHEN** a fix is launched on a host whose configuration, repository, kind and launch give no mode
- **THEN** its agent's command carries no mode, its start's record names none, its started ack is taken whatever mode it reports, and status gives no mode with the agent (S66, 161, 162)

### Requirement: A profile maps a mode exactly or refuses it

The agent's profile SHALL map each of the four to the one setting of
its program that means it, and each own value it names to itself, and
SHALL refuse any other, naming the mode and the program; it SHALL never
pass a wider or a narrower setting (88, 169).

#### Scenario: Each mode mapped as the profile says

- **WHEN** the fake agent's profile maps `plan`, `ask`, `edits` and `full` to `look`, `ask-each`, `edit-files` and `all`, and a fix's code stage is launched once in each
- **THEN** each start's agent is passed the value its mode maps to, and no other (88, 169)

### Requirement: A mode its profile refuses refuses the launch or the stage

A launch where a stage's agent resolves to a mode its profile refuses
SHALL be refused naming the mode, the program, the stage and the
layer, leaving nothing made; a later start so resolved, after the
host's configuration changed, SHALL leave the stage refused in the same
words, starting no agent, until a retry (4, 27, 32, 169).

#### Scenario: A mode the profile cannot honour

- **WHEN** the fake agent's profile maps no setting for `edits`, and a fix is launched with `--agent code.mode=edits`
- **THEN** the launch is refused naming `edits`, the program, the stage `code` and the layer `launch`, and nothing is made (4, 169)

#### Scenario: A later start refused

- **WHEN** after a change is launched the host's `[kinds.change.agents.stages.verify]` gives `mode = { agent = "unheard" }`, which the agent's profile does not name
- **THEN** verify is refused naming the mode as written, the program, verify and `host-kind`, no agent starts, and a retry after the host's table is mended starts it (27, 32, 169)

### Requirement: A table's entry is the agent's program's, never a later layer's

The agent's entry of a table of programs SHALL be the entry for its
program's name. A table that names no entry for the agent's program
SHALL refuse as a mode its profile cannot honour does, naming the table
as written, the program, the stage and the layer, and SHALL never pass
to a later layer's mode (169).

#### Scenario: No entry for the agent's program

- **WHEN** the kind's declaration gives `mode = "edits"` and the launch gives `--agent mode.other=edits`, naming no entry for the agent's program
- **THEN** the launch is refused naming `{ other = "edits" }`, the program, the stage and the layer `launch`, and the kind's `edits` is not taken (4, 169)

### Requirement: plan only where its stage can deliver in it

A mode that resolves to `plan`, one of the four or an own value its
profile maps `plan` to, given to a stage that is not a conversation,
SHALL refuse the launch naming the stage, its deliverable and the
layer; a conversation SHALL take it (4, 109, 169).

#### Scenario: plan for every stage of a kind

- **WHEN** a change is launched with `--agent mode=plan`
- **THEN** the launch is refused naming construct, its deliverable and the layer `launch`, and nothing is made (4, 169)

#### Scenario: plan for a conversation

- **WHEN** a launch's declaration gives a conversation stage `mode = "plan"` and its kind is launched
- **THEN** the launch is taken, and the conversation's agent starts with the value its profile maps `plan` to (109, 169)

### Requirement: A mode widens nothing flume answers

An agent in any mode SHALL be answered nothing flume would not answer
for it in another: where it asks a person, it SHALL wait asking and
status SHALL say so (34, 90; core.12).

#### Scenario: An agent in ask asks a person

- **WHEN** a code stage's agent started in `ask` has begun and asks a person's permission past the stage's quiet limit
- **THEN** status reads it asking, the terminal is sent nothing for it, and the stage is neither ended nor stuck while it asks (34, 44, 90)
