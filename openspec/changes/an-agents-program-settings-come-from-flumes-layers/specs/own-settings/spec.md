## Purpose

An agent program's own settings, its status line, hooks or tools'
permissions among them, given through flume's layers as a table of
programs: each top-level key taken whole from the first layer that
gives it, only the agent's program's entry passed, as its profile takes
them, over its account's own configuration; nothing passed where no
layer gives one; and a reserved key, a credential-shaped value or a
program that takes none refused.

## ADDED Requirements

### Requirement: Own settings are a table of programs

An agents table's `programs` SHALL be a table of programs, each entry
that program's own settings as written, kept as JSON values. Only the
entry for the agent's program SHALL be given to its profile, and no
other program's SHALL reach it. A TOML date or time SHALL be refused
naming its key, and a `programs` that is not a table of tables SHALL be
refused naming the field (175, C.2, C.7).

#### Scenario: Two programs' entries

- **WHEN** the host's `[agents.programs.agent]` gives `line = "a"` and `[agents.programs.other]` gives `line = "b"`, and a fix's agent runs the fake program `agent`
- **THEN** its profile is given `{ line = "a" }` alone, and status names `programs.agent.line` from `host` and nothing of `other` (175)

#### Scenario: A date in a program's settings

- **WHEN** the host's `[agents.programs.agent]` gives `since = 2026-11-01`
- **THEN** every launch and start is refused naming `agents.programs.agent.since`, and saying to give it as a string (175, C.7)

### Requirement: Every layer gives own settings key by key

Each top-level key of a program's own settings SHALL come from the
first of clause 32's eight layers that gives it, taken whole, never
merged with another layer's value of that key; within a layer that
names stages, a stage's own SHALL come first. The host's layers SHALL
be read as they stand at each start, the others as the launch read
them (8, 21, 32, 175).

#### Scenario: Keys from three layers

- **WHEN** the host's `[agents.programs.agent]` gives `line = "host"` and `tips = true`, its `[agents.streams.work.programs.agent]` gives `line = "work"`, and the repository's file gives `[kinds.fix.agents.programs.agent] style = "plain"`
- **THEN** a fix in `work` is given `line = "work"` from `host-stream`, `tips` from `host` and `style` from `repository`, and a fix in `default` `line = "host"` from `host` (S76, 32, 175)

#### Scenario: A key taken whole

- **WHEN** the host's `[agents.programs.agent]` gives `status = { command = "a", padding = 1 }` and its `[agents.streams.work.programs.agent]` gives `status = { command = "b" }`
- **THEN** a fix in `work` is given `status = { command = "b" }` from `host-stream`, with no `padding` (175)

#### Scenario: The host's table changes while a stage works

- **WHEN** the host's `[agents.programs.agent]` changes `line` while code's agent works
- **THEN** code's agent keeps what it started with, and verify's is given the new `line` (8, 32)

### Requirement: Nothing passed where no layer gives own settings

Where no layer gives the agent's program an entry, its profile SHALL be
given none, its command SHALL pass nothing for them and write no file,
and status SHALL give the agent no own settings, so the account's own
configuration decides (144, 162, 175).

#### Scenario: A host and a repository that give none

- **WHEN** a fix is launched where no layer gives `programs`
- **THEN** the fake agent's command passes no own settings and lists no file, and status gives no own settings (S76, 175)

### Requirement: A reserved key refuses the launch or the stage

A key the agent's profile names as another setting's or the account's
SHALL refuse the launch, or at a later start the stage, naming the
key, the program, the stage, the layer and what owns it, and starting
no agent until a retry (4, 27, 32, 175).

#### Scenario: A reserved key at the launch

- **WHEN** the fake agent's profile reserves `env` for the account, and the launch gives `--agent programs.agent.env={"X":"1"}`
- **THEN** the launch is refused naming `programs.agent.env`, `agent`, every stage, `launch` and the account, and nothing is made (S77, 4)

#### Scenario: A reserved key after the launch

- **WHEN** after a fix's launch the host's `[agents.programs.agent]` gives `env`
- **THEN** its next stage is refused naming the key and `host`, no agent starts, and a retry after the table is mended starts it (27, 32)

### Requirement: A program whose profile takes no own settings refuses them

An entry for the agent's program that its profile gives no way to take
SHALL refuse the launch, or at a later start the stage, naming the
program, the key and the layer; an entry for a program that no stage
of the place runs SHALL refuse nothing (4, 32, 170, 175).

#### Scenario: A profile that keeps the seam's default

- **WHEN** the fake agent is scripted to take no own settings, and the host's `[agents.programs.agent]` gives `line`
- **THEN** a fix's launch is refused naming `agent`, `line` and `host`, and saying its profile passes no settings of its program's own (4, 175)

#### Scenario: An entry for a program no stage runs

- **WHEN** the host's `[agents.programs.other]` gives `line` and every stage runs `agent`
- **THEN** the fix launches and lands, and `other`'s entry reaches nothing (175)

### Requirement: A value shaped as a credential is refused

A string in an own setting SHALL be refused where its layer is read,
naming its key's path and the layer and never the value, when it holds
a token's known prefix, a private key or a JSON web token, a long run
mixing cases and digits, or is a secret-named key's value naming no
variable. The host's configuration then refuses every launch and
start, and a declaration, a repository's file or a launch the launch
(4, 29, 175, C.7).

#### Scenario: A token in a launch

- **WHEN** a launch gives a key whose value opens `ghp_` and twenty letters and digits
- **THEN** it is refused naming the key and `launch`, saying a credential goes in the account's `env`, its words holding no part of the value, and nothing is made (S77, 29)

#### Scenario: A secret-named key in the host's configuration

- **WHEN** the host's `[agents.programs.agent]` gives `headers = { Authorization = "Bearer abc" }`
- **THEN** every launch and start is refused naming `agents.programs.agent.headers.Authorization`, and status says the configuration cannot be read without the value (C.7)

#### Scenario: A variable named rather than a value

- **WHEN** the host's `[agents.programs.agent]` gives `headers = { Authorization = "Bearer ${API_TOKEN}" }`
- **THEN** the value is taken as written (175)

### Requirement: flume reads nothing inside an own setting's value

flume SHALL pass each value as its layer wrote it, reading no `~` and
no `$VAR` in it and changing nothing inside it (175; core.30).

#### Scenario: A home and a variable in a command

- **WHEN** the host's `[agents.programs.agent]` gives `line = "~/bin/line $HOME"`
- **THEN** the fake agent's profile is given `~/bin/line $HOME` as written (175)

### Requirement: An account is usable only where its program takes what is given

Where no layer names a stage's program, an account of its pool SHALL
be usable for the stage only where its program's profile takes the
stage's own settings for that program (144, 173, 175).

#### Scenario: A pool of two programs

- **WHEN** a stage's layers name no program, its pool holds an `other` account and then an `agent` account, fill-first, the host gives `[agents.programs.other] line`, and `other`'s profile takes no own settings
- **THEN** the stage starts on the `agent` account, and status says the account came from the strategy (145, 173)
