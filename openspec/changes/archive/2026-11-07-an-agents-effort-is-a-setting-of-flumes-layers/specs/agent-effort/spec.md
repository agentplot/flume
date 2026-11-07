## Purpose

An agent's effort, how hard its program's model works on each turn,
given as the program's own word in every layer of clause 32 and taken
whole as the model is; nothing passed where no layer gives one; each
profile taking its program's words, read from the release it pins, or
refusing; refused at the launch or a later start.

## ADDED Requirements

### Requirement: An effort is its program's own word

An agent's effort SHALL be one string, its program's own word, kept as
written; flume SHALL map no effort from one program to another and
SHALL define no scale of its own. A table of programs SHALL be refused
naming its table and `effort` until the model's table is built with
it (172, 197), and a number or any other value SHALL be refused naming
its table and `effort` (197, C.2).

#### Scenario: A word kept as written

- **WHEN** the kind's agents table gives `effort = "xhigh"` and a fix's agent runs the fake program `agent`
- **THEN** the agent's resolved settings give the effort `xhigh` as written (197)

#### Scenario: A table refused

- **WHEN** the kind's agents table gives `effort = { claude = "xhigh" }`
- **THEN** the declaration cannot be read, naming the table's `effort` (4, 172, 197)

### Requirement: Every layer may give the effort, taken whole

The effort SHALL come from the first layer of clause 32 that gives it,
a stage's own before every stage's, as the model does, and SHALL be
taken whole from that layer; the launch's, the repository's and the
kind's SHALL be those the launch read, and the host's as they stand at
each start (21, 32, 197).

#### Scenario: The launch's over the kind's

- **WHEN** a kind gives `[kinds.bump.agents] effort = "high"`, and its launch gives `--agent code.effort=max`
- **THEN** code's agent starts with `max` from `launch`, and every other stage's with `high` from `kind` (32, S103)

#### Scenario: The host's over the repository's

- **WHEN** the host gives `[kinds.fix.agents] effort = "medium"` and the repository's own file gives `[kinds.fix.agents] effort = "xhigh"`
- **THEN** a fix's agent starts with `medium` from `host-kind` (32, 197)

#### Scenario: An effort for a repository, a stream and every agent

- **WHEN** the host gives `[agents] effort = "medium"`, `[agents.streams.work] effort = "high"` and `[repos."/r".agents] effort = "xhigh"`, and the repository `/s`'s file gives `[agents] effort = "low"`
- **THEN** a fix's agent in `/r` starts with `xhigh` from `host-repository`, one in `/s` in `work` with `high` from `host-stream`, and one in `/s` in `default` with `medium` from `host`, the host's layers coming before the repository's (32, 143, 197)

#### Scenario: The host's table changed after the launch

- **WHEN** the host's `[kinds.change.agents]` gives `effort = "high"` at a change's launch and `effort = "xhigh"` before its verify starts
- **THEN** construct's agent starts with `high` and verify's with `xhigh`, each from `host-kind`, and construct's keeps `high` (32)

### Requirement: No effort where no layer gives one

Where no layer gives an effort, flume SHALL ask nothing of the agent's
profile, its command SHALL pass no effort, and status SHALL give none,
the account's own configuration deciding (144, 162, 197).

#### Scenario: No effort in any layer

- **WHEN** a fix launches with no effort in any layer
- **THEN** its agent's command passes no effort, the fake's log shows no `effort` asked, and status gives the agent no effort (197)

### Requirement: A profile takes its program's words or refuses

Each agent profile SHALL take the words its program takes, read from the
release it pins, and SHALL refuse any other, and a profile that names no
way to give its program an effort SHALL refuse every word; the core
SHALL ask the profile before any command is built, so no command holds
a word its profile refused (88, 170, 197).

#### Scenario: A word the fake takes and one it does not

- **WHEN** the fake agent is scripted to take `low` and `high`, and two fixes launch with `--agent effort=high` and `--agent effort=med`
- **THEN** the first's agent starts with `high` passed, and the second is refused naming `med`, `agent`, the stage and the layer `launch`, and the words `low` and `high` (4, 197)

### Requirement: An effort its profile refuses refuses the launch or the stage

`Engine::launch` SHALL resolve every stage's effort and ask its agent's
profile before anything is made, a refusal refusing the launch whole;
a word the profile refuses at a later start, from the host's table as
it stands then, SHALL leave the stage refused in the same words,
starting no agent, until a retry (4, 8, 27, 32, 197).

#### Scenario: A refusal at the launch

- **WHEN** a change launches with `--agent verify.effort=ultracode` and the fake takes `low` and `high`
- **THEN** the launch is refused naming `ultracode`, `agent`, the stage verify and `launch`, and no worktree, branch or held state is left (4, 8)

#### Scenario: A refusal at a later start

- **WHEN** the host's `[kinds.change.agents]` gives no effort at a change's launch and `effort = "turbo"` before its code starts
- **THEN** code is refused naming `turbo`, `agent`, the stage and `host-kind`, no agent starts, and a retry after the host's table is put right starts it (27, 32)
