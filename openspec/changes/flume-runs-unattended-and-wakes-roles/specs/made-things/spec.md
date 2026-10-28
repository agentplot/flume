# made-things

## Purpose

What a stage made besides its deliverable, recorded by the run that made
it and carried by flume as written, so a caller's card can link a spike,
a page or a revision to bring up without building an address of its own
(23, 43, 128, 129; record 0007; core.23).

## ADDED Requirements

### Requirement: A run records what it made in the place's folder

A stage whose declaration names what it makes SHALL have `{{made}}`
render the path of `made.jsonl` in the place's own folder, and a prompt
of a stage that declares nothing SHALL be refused naming `{{made}}` when
it asks for it. Each line SHALL name one made thing: its name and what
it is, with its address, or the branch and revision that bring it up
(27, 128, 129, C.2).

#### Scenario: The path rendered

- **WHEN** a code stage declares it makes `preview` and its prompt asks for `{{made}}`
- **THEN** the rendered prompt carries the path of `made.jsonl` in the place's folder in the store

#### Scenario: Asked for and not declared

- **WHEN** a stage that declares nothing it makes has a prompt asking for `{{made}}`
- **THEN** the stage stands refused naming `{{made}}`, and starts no agent (27)

### Requirement: A declared thing with no line is not yet made

A stage SHALL NOT be read delivered while a thing its declaration says
it makes has no line, and the stop check SHALL send the agent back
naming each such thing beside the deliverable (44, 110, 129).

#### Scenario: Commits and no line

- **WHEN** a code stage that declares `preview` commits and goes quiet with no line for `preview`
- **THEN** the stage is not delivered, and the stop check's words name `preview` and the way to stop short

#### Scenario: The line written

- **WHEN** the agent then writes the line for `preview` and goes quiet
- **THEN** the stage is delivered at its head

### Requirement: The lines are carried as written

flume SHALL read the lines when the stage ends and carry them in the
stage's end, in status, and in a wait's and the inbox's answer, each as
the run wrote it, a thing the kind did not declare among them, building,
checking or rewriting no address (43, 56, 129, core.23).

#### Scenario: An address carried whole

- **WHEN** the run records a spike at `https://forge.example/o/r/tree/spike/x?ref=a%2Fb` and a thing it did not declare
- **THEN** the stage's end, status and the inbox carry both lines byte for byte

#### Scenario: A thing that must run

- **WHEN** the run records `preview` as the branch and revision that bring it up
- **THEN** the end carries that branch and revision and no address, and flume starts nothing for it

### Requirement: The lines go with the place

The lines SHALL go with the place's folder when the place is tidied, and
flume SHALL keep none of them after (62, 123, 129).

#### Scenario: After the tidy

- **WHEN** a place whose stage recorded made lines lands and is tidied
- **THEN** the store holds no folder and no line of it
