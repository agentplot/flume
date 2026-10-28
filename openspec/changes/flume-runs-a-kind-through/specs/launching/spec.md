## MODIFIED Requirements

### Requirement: A launch makes one place and answers

A launch SHALL name a repository, a kind, a name and a base, and a brief
where a stage asks for one, and may carry the place's priority; it SHALL
resolve its kind to one declaration, the launch's over the repository's
over flume's own (19). It SHALL make one place on `<prefix>/<name>` at
the base's current revision, record it with its priority, ready it, and
answer once its first stage is started, on hold, waiting on readiness or
waiting for room, never waiting on an agent (2, 6, 7, 9, 70).

#### Scenario: A fix launched

- **WHEN** a fix is launched on a repository whose fakes bind a merge queue, with a brief
- **THEN** one place is made on `fix/<name>` at the merge queue's base revision, its readiness runs, its code stage's agent is started, and the answer gives the place with the stage started and its agent state unknown (S1, 2, 6, 35)

#### Scenario: The first stage on hold

- **WHEN** a launch puts its first stage on hold
- **THEN** the place is made and readied, no agent is started, and the stage reads on hold (7, 47)

#### Scenario: A launch on a full host

- **WHEN** a launch with priority 5 finds the host at its bound
- **THEN** the place is made and readied, its first stage waits for room, and the answer gives its priority and its place in the queue (70)

### Requirement: A launch is refused whole

A launch SHALL be refused, naming why, and SHALL leave no worktree,
branch or held state, when its place, branch or location already
stands, when its kind reads a change whose name its base or a live
place already holds, when its kind resolves to no declaration, asks for
more than flume runs or lands where no merge queue is bound, when the
binding names a place profile flume does not hold, when a stage asks for
a brief it lacks, or when its base's revision cannot be read. No launch
SHALL be refused for want of room (3, 4, 9, 70, C.1).

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

#### Scenario: A place profile flume does not hold

- **WHEN** a repository's binding names a place profile the resolver does not hold
- **THEN** the launch is refused naming the place profiles the resolver holds, and nothing is made (C.1, core.3)
