# launching Specification

## Purpose
A caller's launch making one place of a repository on the host, cut
from its base where the merge queue says the base lives, and starting,
holding or leaving waiting its first stage; or refused whole, naming
why, with nothing it made left behind (1 to 17, 19 to 23).

## Requirements

### Requirement: A launch makes one place and answers

A launch SHALL name a repository, a kind, a name and a base, and a brief
where a stage asks for one, and SHALL resolve its kind to one
declaration, the launch's over the repository's over flume's own (19).
It SHALL make one place on `<prefix>/<name>` at the base's current
revision, record it, ready it, and answer once its first stage is
started, on hold or waiting, never waiting on an agent (2, 6, 9).

#### Scenario: A fix launched

- **WHEN** a fix is launched on a repository whose fakes bind a merge queue, with a brief
- **THEN** one place is made on `fix/<name>` at the merge queue's base revision, its readiness runs, its code stage's agent is started, and the answer gives the place with the stage started and its agent state unknown (S1, 2, 6, 35)

#### Scenario: The first stage on hold

- **WHEN** a launch puts its first stage on hold
- **THEN** the place is made and readied, no agent is started, and the stage reads on hold (7, 47)

### Requirement: Readiness that fails

When a place's readiness fails, its first stage SHALL wait and status
SHALL name the failed step and its words, unless the kind starts its
first stage in a place that could not be made ready, when the stage
SHALL start with the step and its words in its prompt (12).

#### Scenario: A change's place waits, a fix's starts

- **WHEN** the fake place's readiness fails at its first step for a change's launch and for a fix's launch
- **THEN** the change's construct stage waits on readiness with the step and words in status, and the fix's code stage starts with them in its prompt (S16)

### Requirement: A launch is refused whole

A launch SHALL be refused, naming why, and SHALL leave no worktree,
branch or held state, when its place, branch or location already
stands, when its kind reads a change whose name its base or a live
place already holds, when its kind resolves to no declaration, asks for
more than flume runs or lands where no merge queue is bound, when a
stage asks for a brief it lacks, or when its base's revision cannot be
read (3, 4, 9).

#### Scenario: A kind no declaration holds

- **WHEN** a launch names a kind none of the three declarations holds
- **THEN** it is refused naming the kind, and the fakes record no make and no held state (S15)

#### Scenario: A stage with two agents

- **WHEN** a repository's kind gives one stage two agents
- **THEN** a launch of it is refused naming the kind and makes nothing (S31)

#### Scenario: A brief a later stage asks for

- **WHEN** a kind's second stage asks for the brief and the launch gives none
- **THEN** it is refused naming the stage and the brief, nothing is made, and no agent receives a prompt (S22)

#### Scenario: No binding

- **WHEN** a repository binds no profile and fix is launched in it, then a repository kind that does not land
- **THEN** the fix is refused naming the kind and the merge queue it needs, and the other makes its place with the place seam's no-tool profile (S33, 105)

#### Scenario: Two launches of one place at once

- **WHEN** two launches of one repository, kind and name run at once over one fake store
- **THEN** one place is made and the other launch is refused naming it (S11, 3, 75)

### Requirement: A place keeps what its launch resolved

A place SHALL run its kind and keep its repository's binding as each
resolved at its launch, and nothing after the launch SHALL change its
kind, brief, base, settings, holds or submission's hold (8, 21).

#### Scenario: A kind's version changes after a launch

- **WHEN** a place is launched with a repository's kind at version 1, the fake kinds then give version 2 at the base's new revision, and a second place is launched
- **THEN** the first place runs version 1's stages and the second version 2's, and status names each place's kind at its version (S13)
