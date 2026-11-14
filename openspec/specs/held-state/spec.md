# held-state Specification

## Purpose
flume's live state of each place, one document per place in the store
seam, written by compare-and-set on its version, gone when the place is
tidied, and enough on its own that a restart loses nothing (62, 73 to
77, 102 to 104).

## Requirements

### Requirement: One document per place, written by compare-and-set

Every act SHALL read a place's held state with its version and write
its next state at that version through the store seam; a write refused
because the version moved SHALL be read again and decided again, so two
acts on one place leave it as one or the other would have (75, 103).

#### Scenario: Two acts on one place at once

- **WHEN** a stop and a retry act on one place at once over one fake store
- **THEN** the held state ends as one of them alone would leave it, and the place has at most one agent (75, 30)

### Requirement: No worktree or branch the held state does not name

The core SHALL write a place's held state before the place seam makes
its worktree, and remove it only after the place and its branch are
gone or named kept, so no act leaves a worktree or branch of flume's
that its held state does not name (17, 76).

#### Scenario: A make that is refused

- **WHEN** the fake place refuses make because the location already exists
- **THEN** the launch is refused naming the location, and the fake store holds no document for the place (2, 76)

### Requirement: Every act is safe to repeat

A repeat of an act already done SHALL change nothing (74).

#### Scenario: A drop repeated

- **WHEN** a place is dropped and the drop is repeated
- **THEN** the second answers as the first did, and the fakes record no second end or remove

### Requirement: A restart loses nothing

A core built anew over the same store and seams SHALL read every place
again from its held state, its repository, its terminal and the merge
queue, report each agent that did not survive as lost, and take an
agent the terminal resumed as the stage's only when its identity and
its own record both match its start's (77).

#### Scenario: The host restarts while code works

- **WHEN** the core is rebuilt over the same fake store while a code stage works, and the fake terminal no longer holds its agent
- **THEN** the held state is whole, the stage reads lost, and no agent starts until a caller retries (S12, 45, 48)

#### Scenario: A resumed agent with another record

- **WHEN** after a rebuild the fake terminal holds an agent under the stage's identity whose own record differs from its start's
- **THEN** the stage reads lost

### Requirement: An agent brought back in a conversation's pane is left for a person

After a restart, an agent the terminal brought back that is not the
stage's SHALL be ended once quiet, but one in a conversation's pane,
which SHALL be left in its pane, sent nothing, and ended only by its
caller's stop, drop or done; its start SHALL go not live once the
terminal reads it stopped (31, 49, 51, 52, 77, 114).

#### Scenario: A bare agent in a conversation's pane

- **WHEN** after a rebuild the fake terminal brings a talk's agent back bare and quiet, and the place is settled three times
- **THEN** talk reads lost after the first, the fake terminal's log shows no close and no send to that agent, and the start stays live; once the fake terminal reads the agent stopped, the next settle reads the start not live (31, 77, S113)

#### Scenario: A bare agent in a code stage's pane

- **WHEN** after a rebuild the fake terminal brings a code stage's agent back bare and quiet
- **THEN** code reads lost and the settle closes that agent (31, 77, S41, S116)

#### Scenario: A stop ends it

- **WHEN** the caller stops a place whose lost talk's pane holds an agent the fake terminal brought back bare, quiet
- **THEN** the stop closes it (51, 77)

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
