# launching Specification

## Purpose
A caller's launch making one place of a repository on the host, cut
from its base where the merge queue says the base lives, and starting,
holding or leaving waiting its first stage; or refused whole, naming
why, with nothing it made left behind (1 to 17, 19 to 23).

## Requirements

### Requirement: A launch makes one place and answers

A launch SHALL name a repository, a kind, a name and a base, a brief
where a stage asks for one, and may carry a priority, recipients and
holds with their words; it SHALL resolve its kind to one declaration,
the launch's over the repository's over flume's (19). It SHALL make one
place on `<prefix>/<name>` at the base's current revision, record it,
ready it, and answer once its first stage is started, on hold, waiting
on readiness or waiting for room, never on an agent (2, 6, 8, 9, 70).

#### Scenario: A fix launched

- **WHEN** a fix is launched on a repository whose fakes bind a merge queue, with a brief
- **THEN** one place is made on `fix/<name>` at the merge queue's base revision, its readiness runs, its code stage's agent is started, and the answer gives the place with the stage started and its agent state unknown (S1, 2, 6, 35)

#### Scenario: The first stage on hold

- **WHEN** a launch puts its first stage on hold
- **THEN** the place is made and readied, no agent is started, and the stage reads on hold with its words (7, 47, 130)

#### Scenario: A launch on a full host

- **WHEN** a launch with priority 5 finds the host at its bound
- **THEN** the place is made and readied, its first stage waits for room, and the answer gives its priority and its place in the queue (70)

### Requirement: A launch's recipients

A launch naming no recipient SHALL take `$FLUME_RECIPIENT`, or else the
session that launched it as the terminal names it, or none, and SHALL
record the place with its priority, its recipients and its holds' words,
which nothing after the launch changes (7, 8, 43, 70).

#### Scenario: Recipients at the launch

- **WHEN** one launch names two recipients, another names none with `FLUME_RECIPIENT` set, and a third names none from a session the fake terminal names
- **THEN** status gives the first place both, the second the variable's, the third that session, and nothing after the launch changes them (7, 8)

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
branch or held state, when its place, branch or location already stands,
when its kind reads a change whose name its base or a live place already
holds, or when its base's revision cannot be read. No launch SHALL be
refused for want of room (3, 9, 70).

#### Scenario: Two launches of one place at once

- **WHEN** two launches of one repository, kind and name run at once over one fake store
- **THEN** one place is made and the other launch is refused naming it (S11, 3, 75)

### Requirement: A launch of a kind flume cannot run is refused whole

A launch SHALL be refused, naming the kind, the stage or the merge
queue, and SHALL leave nothing made, when its kind resolves to no
declaration, asks for more than flume runs or lands where no merge queue
is bound, or when a stage asks for a brief the launch lacks (4, 9, C.2).

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

### Requirement: A launch whose binding or settings flume cannot run is refused whole

A launch SHALL be refused, naming the program, field or stage and its
layer, and SHALL leave nothing made, when the binding names a place
profile flume does not hold, when a stage's agent resolves to a program
no agent profile binds or the host's terminal cannot run, when the
repository's `.config/flume.toml` gives an agent an `env`, or when
settings name a stage the kind does not hold (4, 32, C.1, C.2).

#### Scenario: A program no agent profile binds

- **WHEN** the host's configuration gives change@1's code stage `program = "nothing-here"` and a change is launched
- **THEN** the launch is refused naming the program, the stage `code` and the layer `host-kind`, and nothing is made (4, 32)

#### Scenario: An environment in a repository's file

- **WHEN** a repository's `.config/flume.toml` gives `[kinds.change.agents] env = { A = "b" }` and a change is launched
- **THEN** the launch is refused naming the field, and nothing is made (C.2, core.30)

#### Scenario: A place profile flume does not hold

- **WHEN** a repository's binding names a place profile the resolver does not hold
- **THEN** the launch is refused naming the place profiles the resolver holds, and nothing is made (C.1, core.3)

### Requirement: A place keeps what its launch resolved

A place SHALL run its kind and keep its repository's binding as each
resolved at its launch, and nothing after the launch SHALL change its
kind, brief, base, the settings its launch carries, the repository's and
the kind's settings as the launch read them, holds or submission's
hold; the host's configuration of its agents' settings SHALL be read at
each start (8, 21, 32).

#### Scenario: A kind's version changes after a launch

- **WHEN** a place is launched with a repository's kind at version 1, the fake kinds then give version 2 at the base's new revision, and a second place is launched
- **THEN** the first place runs version 1's stages and the second version 2's, and status names each place's kind at its version (S13)

#### Scenario: A repository's settings change after a launch

- **WHEN** a change is launched with the repository giving change@1's agents `model = "opus"`, and the base's next revision gives `model = "sonnet"`
- **THEN** the place's later stages start with `opus`, read from its held state (21, 32)

### Requirement: An agent's settings are resolved from six layers at each start

At each start an agent's program, model, instructions and environment
SHALL each come from the first that gives it of: the launch's settings;
the host's configuration for the repository and kind; the host's for the
kind; the repository's for a kind it does not declare; the kind's
declaration; flume's default. Within each, a stage's own setting SHALL
come before the kind's (32; record 0017).

#### Scenario: Settings from four layers

- **WHEN** the host's configuration gives change@1's agents `env = { CLAUDE_CONFIG_DIR = "~/.claude-work" }`, the repository gives change@1's code stage `model = "opus"` without declaring it, and the launch gives verify `instructions`
- **THEN** construct's agent starts with `CLAUDE_CONFIG_DIR` under the host user's home, code's with the repository's model as well, and verify's with the launch's instructions (S44)

### Requirement: The environment merges by variable and the host's values are read against the login environment

The environment SHALL merge variable by variable across the layers, and
the program, model and instructions SHALL each be taken whole. A
host-layer `env` value SHALL read a leading `~` as the host user's home
and `$VAR` or `${VAR}` from the login environment, never the caller's
session; no other layer's value SHALL be expanded (29, 32).

#### Scenario: The environment merged by variable

- **WHEN** the host gives change@1's agents `env = { A = "1", B = "2" }` and the launch gives `env.B = "3"`
- **THEN** the agent starts with `A=1` and `B=3`, status naming each one's layer (32)

### Requirement: The host's configuration is read at each start

The host's layers SHALL be read as they stand at each start, and an
agent already started SHALL keep what it started with. A start whose
host layers give a program no agent profile binds, or a variable the
login environment lacks, SHALL leave the stage refused, naming it and
its layer, starting no agent, until a retry (8, 27, 32).

#### Scenario: The host's configuration changes while a stage works

- **WHEN** the host's configuration changes `CLAUDE_CONFIG_DIR` while code's agent works
- **THEN** code's agent keeps the value it started with, and verify's starts with the new one (S44, 8)

#### Scenario: A later start refused

- **WHEN** after a launch the host's configuration gives verify `env = { X = "$NOT_SET" }` and the login environment lacks `NOT_SET`
- **THEN** verify is refused naming the variable and the layer, no agent starts, and a retry after the host's configuration is mended starts it (27, 32)
