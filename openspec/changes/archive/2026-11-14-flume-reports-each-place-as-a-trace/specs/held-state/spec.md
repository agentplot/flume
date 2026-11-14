## ADDED Requirements

### Requirement: A launch names the place's trace

A launch SHALL name, in the place's first document, its trace, its
place's span and the launch's time, the ids from the host's source of
secrets, and the place's span SHALL be marked sent once it went. A
document written before SHALL take its trace, its place's span from
that write's time, at its next write (217; core.16, core.17).

#### Scenario: A launch names the trace

- **WHEN** a place is launched over the fakes
- **THEN** its document names a trace of 32 hex digits, a place's span of 16 and the launch's time, and every later write keeps them (217)

#### Scenario: A document written before

- **WHEN** a held state written before this change, with a stage running, is read and settled
- **THEN** it reads with no trace, and the settle's write gives it a trace, a place's span from that write's time, and the running start a span from its `started_at` (217, 77)

### Requirement: Each start and each wait is held with its span

Each start SHALL name its span in its `StartHeld` as the write that
claims it is made, before its agent's command is built, and SHALL hold
since when a settle read its agent asking a person. Each wait of the
place SHALL be held with its span, whom it waits on, its move, its
stage and since when, from the write that first reads it to the one
that reads it ended (217; core.16).

#### Scenario: A start's span before its command

- **WHEN** a stage starts over the fakes
- **THEN** the start's span is in the held state before the agent seam is asked for its command (217, 219)

#### Scenario: The trace round trips

- **WHEN** a held state with a trace, a start's span, a start asking since a time and two open waits is written and read back
- **THEN** it reads the same, and one with none of them is written with no key for them (core.16)
