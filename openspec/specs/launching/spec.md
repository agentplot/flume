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
SHALL start with the step and its words in its prompt; a retry of that
first stage SHALL ready the place again at the revision its launch
read (12, 20).

#### Scenario: A change's place waits, a fix's starts

- **WHEN** the fake place's readiness fails at its first step for a change's launch and for a fix's launch
- **THEN** the change's construct stage waits on readiness with the step and words in status, and the fix's code stage starts with them in its prompt (S16)

#### Scenario: A retry readies at the launch's revision

- **WHEN** a change's readiness failed at its launch from main at R, main's revision moves on, the fake place's readiness is mended, and construct is retried
- **THEN** the fake place is readied at R, and construct starts (12, 20)

#### Scenario: Each step named as it passed

- **WHEN** the fake place's readiness passes `toolchain`, `proto install`, `moon --version` and `moon setup` for a change's launch
- **THEN** construct starts, and status names each step passed, in that order (S89, 12)

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

A launch SHALL be refused, naming the program, field, stage or stream
and its layer, and SHALL leave nothing made, when the binding names a
place profile flume does not hold, when a stage's agent resolves to a
program no agent profile binds or its stream's terminal cannot run,
when its stream's multiplexer names a terminal profile flume does not
carry, when a declaration gives an agent an `env`, or when settings
name a stage the kind does not hold (4, 32, 158, 190, C.1, C.2).

#### Scenario: A program no agent profile binds

- **WHEN** a change is launched with `--agent code.program=nothing-here`
- **THEN** the launch is refused naming the program, the stage `code` and the layer `launch`, and nothing is made (4, 32)

#### Scenario: An environment in a repository's file

- **WHEN** a repository's `.config/flume.toml` gives `[kinds.change.agents] env = { A = "b" }` and a change is launched
- **THEN** the launch is refused naming the field, and nothing is made (C.2, core.30)

#### Scenario: A place profile flume does not hold

- **WHEN** a repository's binding names a place profile the resolver does not hold
- **THEN** the launch is refused naming the place profiles the resolver holds, and nothing is made (C.1, core.3)

#### Scenario: A stream's multiplexer flume does not carry

- **WHEN** the host's `[streams.trial]` gives `multiplexer = "rex"` and a fix is launched on `trial`
- **THEN** the launch is refused naming `trial`, `rex` and the terminal profiles the resolver carries, and nothing is made (4, 190; core.8)

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

### Requirement: An agent's settings are resolved from eight layers at each start

At each start an agent's program, model, mode, instructions and own
settings SHALL each come from the first of eight layers that gives it:
the launch's; the host's for the repository, for its kind then for
every kind; the host's for the kind, for the stream, for every agent;
the repository's, for a kind it does not declare then for every kind;
the kind's; flume's default. The host's four SHALL give no program.
Within each, a stage's own SHALL come first, each setting taken whole
(32, 195).

#### Scenario: Settings from four layers

- **WHEN** the host's pool has one account giving `CLAUDE_CONFIG_DIR = "~/.claude-work"`, the repository gives change@1's code stage `model = "opus"` without declaring it, and the launch gives verify `instructions`
- **THEN** construct's agent starts with `CLAUDE_CONFIG_DIR` under the host user's home from `account`, code's with the repository's model as well, and verify's with the launch's instructions (S44)

#### Scenario: A model for each kind

- **WHEN** the host gives `[kinds.change.agents] model = "opus"`, `[kinds.fix.agents] model = "sonnet"` and `[repos."/r".kinds.fix.agents] model = "haiku"`
- **THEN** a change place's agents start with `opus` and a fix place's of `/s` with `sonnet`, each from `host-kind`, and a fix place's of `/r` with `haiku` from `host-repository` (32)

#### Scenario: A mode beside a model

- **WHEN** the host gives `[repos."/r".kinds.fix.agents] mode = "edits"` and `[kinds.fix.agents] model = "sonnet"`, and the repositories `/r` and `/s` each give `[kinds.fix.agents] mode = "ask"` in their own file
- **THEN** a fix place's agent of `/r` starts in `edits` from `host-repository` with `sonnet` from `host-kind`, and one of `/s` in `ask` from `repository` (32, 162)

#### Scenario: A model for a stream and for every agent

- **WHEN** the host gives `[agents] model = "sonnet"`, `[agents.streams.work] model = "opus"` and `[kinds.fix.agents] model = "haiku"`, and the repository's file gives `[kinds.change.agents] model = "other"`
- **THEN** a change place's agents start with `opus` from `host-stream` in `work` and with `sonnet` from `host` in `default`, and a fix place's with `haiku` from `host-kind` in either (32, 143)

#### Scenario: A model for every kind of a repository

- **WHEN** the host gives `[repos."/r".agents] model = "opus"` and `[kinds.fix.agents] model = "haiku"`, and the repository `/s`'s file gives `[agents] model = "sonnet"`
- **THEN** a fix's and a chore's agents in `/r` start with `opus` from `host-repository`, a fix's in `/s` with `haiku` from `host-kind`, and a chore's in `/s` with `sonnet` from `repository` (32)

### Requirement: The host's configuration is read at each start

The host's layers, its streams', its pools' and their accounts' SHALL
be read as they stand at each start, and an agent already started SHALL
keep what it started with. A start whose account names a variable the
login environment lacks SHALL leave the stage refused, naming it and the
account, starting no agent, until a retry (8, 27, 32).

#### Scenario: The host's configuration changes while a stage works

- **WHEN** the host's configuration changes the account's `CLAUDE_CONFIG_DIR` while code's agent works under it
- **THEN** code's agent keeps the value it started with, and verify's starts with the new one (S44, 8)

#### Scenario: A later start refused

- **WHEN** after a launch the host's configuration gives the stage's account `env = { X = "$NOT_SET" }` and the login environment lacks `NOT_SET`
- **THEN** verify is refused naming the variable and the account, no agent starts, and a retry after the host's configuration is mended starts it (27, 32)

#### Scenario: An account added to a pool

- **WHEN** an account is added to a stage's pool while the place's code agent works under another
- **THEN** code's agent is unchanged, and verify's start may take the new account (32, 145)

### Requirement: A launch may name its kind's version

A launch's kind SHALL be a name, a name and a version, or an identity,
`<source>/<name>` with or without `@<version>`, resolved as the
kind-sources capability says: a name over the launch's declaration, the
repository's and flume's own, and a version the one named, else the
newest the locks record, else the newest its source holds; a version its
source does not hold SHALL refuse the launch naming it and the versions
held, and leave nothing made (4, 19, 22, 154).

#### Scenario: No version named

- **WHEN** a repository that declares no `change` and locks nothing launches `change`
- **THEN** the place runs change@2 and status names `flume/change@2` (22, 154)

#### Scenario: change@1 named

- **WHEN** the same repository launches `change@1`
- **THEN** the place runs change@1, its code stage one agent for the whole change, and status names `flume/change@1` (22, 154)

#### Scenario: A version flume does not hold

- **WHEN** a launch names `change@3`
- **THEN** it is refused naming change@3 and the versions held, 1 and 2, and nothing is made (4)

#### Scenario: A version the repository's declaration does not hold

- **WHEN** a repository declares `change` at version 1 and a launch names `change@2`
- **THEN** the launch is refused naming the version, since the repository's declaration resolves the name, and nothing is made (4, 19, 154)

#### Scenario: A kind of a host's source named whole

- **WHEN** the host's source acme holds `review@2`, locked by the host, and a launch names `acme/review@2`
- **THEN** the place runs acme's review@2, and status names `acme/review@2` (154, 155)

### Requirement: The environment is the account's, its values read against the login environment

An agent's environment SHALL be the variables of the account chosen
for it and nothing else's, over the login environment; no launch, host
table, repository or kind SHALL give one. An account's value SHALL read
a leading `~` as the host user's home and `$VAR` or `${VAR}` from the
login environment, never the caller's session (29, 32, 144).

#### Scenario: The account's variables

- **WHEN** a stage's account gives `env = { CLAUDE_CONFIG_DIR = "~/.claude-work-1", TOOL_HOME = "$HOME/tool" }`
- **THEN** the agent starts with both read against the host user's home, status naming each from `account`, and no other variable of its settings (29, 32)

### Requirement: A place is cut where its base's current revision lives

A launch SHALL cut its place at the base's current revision: where a
merge queue lands the base, where the merge queue's profile says it
lives; otherwise at `origin/<base>` after flume's own fetch; and at the
host's own branch only where the repository has no `origin` or origin
holds no such branch. A fetch that fails otherwise SHALL refuse the
launch, nothing made. The place SHALL keep where its base was read (9,
56, 126, C.1, S92).

#### Scenario: A base no merge queue lands, its local copy lagging origin

- **WHEN** a fix is launched in a clone bound to no merge queue whose own `main` lags origin's by two commits, or under a merge queue that lands no `main`
- **THEN** the place is cut at origin's `main` after flume's fetch, the clone's own `main` is left where it was, and status gives `base_from` `origin` (9, 126, C.1)

#### Scenario: A base origin lacks

- **WHEN** a fix is launched on `trial`, a branch the clone holds and origin does not, or in a repository with no `origin`
- **THEN** the place is cut at the host's own branch, and status gives `base_from` `origin-lacks` or `no-origin` (9, 56, C.1)

#### Scenario: A fetch from origin that fails

- **WHEN** a fix is launched on `main` with origin out of reach
- **THEN** the launch is refused naming the fetch of `main` from origin, and nothing is made (2, 9)

#### Scenario: A base the merge queue lands

- **WHEN** a fix is launched in a clone whose merge queue lands `main` at a revision other than origin's
- **THEN** the place is cut where the merge queue says, origin's `main` never read for it, and status gives `base_from` `merge-queue` (9, 126)

### Requirement: How long an agent is kept open is the launch's or the host's

An agent's `keep_open` SHALL be resolved at its start as clause 32
resolves every setting, taken whole from the first that gives it of the
launch's settings, the host's for the repository and kind, then for the
repository's every kind, the host's for the kind, the host's for the
place's stream, and the host's for every agent, a stage's own first in
each, else flume's own one hour; a repository's settings and a kind's
declaration SHALL give none (32, 158, 198).

#### Scenario: From the host's kind

- **WHEN** the host gives `[kinds.fix.agents] keep_open = "10m"`
- **THEN** a fix place's code agent starts with `keep_open` `10m` from `host-kind` (32)

#### Scenario: The host's kind over its stream

- **WHEN** the host gives `[kinds.fix.agents] keep_open = "10m"` and `[agents.streams.work] keep_open = "2h"`, and a fix runs in the stream `work`
- **THEN** its code agent starts with `10m` from `host-kind` (32)

#### Scenario: The launch over the host

- **WHEN** a launch gives `--agent code.keep_open=0` over the host's `10m`
- **THEN** code's agent starts with `0` from `launch`, and once it stops short needing an answer it is ended once quiet (32, 198)

#### Scenario: No layer gives one

- **WHEN** no layer gives `keep_open`
- **THEN** the agent starts with `1h` from `default` (32, 198)

#### Scenario: A host change after the launch

- **WHEN** the host's `[agents] keep_open` changes from `1h` to `15m` while a place's first stage runs
- **THEN** that stage's agent keeps `1h`, and the place's next stage's agent starts with `15m` (8, 32)
