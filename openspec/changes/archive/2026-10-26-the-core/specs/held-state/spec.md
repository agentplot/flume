# Held state

## Purpose

flume's live state of each place, one document per place in the store
seam, written by compare-and-set on its version, gone when the place is
tidied, and enough on its own that a restart loses nothing (62, 73 to
77, 102 to 104).

## ADDED Requirements

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
