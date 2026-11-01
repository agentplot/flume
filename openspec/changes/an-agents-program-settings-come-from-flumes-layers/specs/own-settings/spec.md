## Purpose

An agent program's own settings, its status line, its renderer, hooks
or tools' permissions among them, given through flume's layers as a
table of programs, each entry a settings file the layer names and keys
it gives inline: each top-level key taken whole from the first layer
that gives it, a layer's file beneath its own keys, only the agent's
program's keys passed, as its profile takes them, over its account's
own configuration; nothing passed where no layer gives one; a reserved
key refused where a layer writes it and left out of a named file; and
a credential-shaped value or a program that takes none refused.

## ADDED Requirements

### Requirement: Own settings are a table of programs

An agents table's `programs` SHALL be a table of programs, each entry
giving `file`, the path of a settings file, `settings`, a table of the
program's own keys as written, or both, kept as JSON values. Only the
entry for the agent's program SHALL be given to its profile, and no
other program's SHALL reach it (175, C.2, C.7).

#### Scenario: Two programs' entries

- **WHEN** the host's `[agents.programs.agent.settings]` gives `line = "a"` and `[agents.programs.other.settings]` gives `line = "b"`, and a fix's agent runs the fake program `agent`
- **THEN** its profile is given `{ line = "a" }` alone, and status names `programs.agent.settings.line` from `host` and nothing of `other` (175)

### Requirement: An entry flume cannot read fails its table

An entry's key other than `file` and `settings` SHALL fail its table,
naming the key and saying a program's own settings go under
`settings`; a TOML date or time SHALL fail naming its key; and a
`programs` that is not a table of entries SHALL fail naming the field
(175, C.2, C.7).

#### Scenario: A program's key beside the file

- **WHEN** the host's `[agents.programs.agent]` gives `line = "a"`
- **THEN** every launch and start is refused naming `agents.programs.agent.line`, and saying a program's own settings go under `settings` (C.7)

#### Scenario: A date in a program's settings

- **WHEN** the host's `[agents.programs.agent.settings]` gives `since = 2026-11-01`
- **THEN** every launch and start is refused naming `agents.programs.agent.settings.since`, and saying to give it as a string (175, C.7)

### Requirement: A layer names a settings file beneath its own keys

An entry's `file` SHALL name a file whose top level is a table of the
program's own settings, read as JSON where its name ends `.json` and as
TOML where it ends `.toml`, its top-level keys beneath the entry's
`settings`. Within a layer that names stages, a stage's table, its
`settings` over its file, SHALL come before the table for every stage,
its `settings` over its file (32, 175).

#### Scenario: A file beneath its layer's own keys

- **WHEN** the host's `[agents.programs.agent]` names `agent.json`, beside the host's configuration, giving `{ "tui": "fullscreen", "line": "file" }`, and its `settings` gives `line = "inline"`
- **THEN** a fix's agent is given `tui = "fullscreen"` and `line = "inline"`, and status names `programs.agent.settings.tui` from `host` with the file and `programs.agent.settings.line` from `host` with none (S76, 175)

#### Scenario: A stage's file before every stage's own keys

- **WHEN** the kind's declaration gives every stage `settings = { tui = "every" }` and code's agents table names a file giving `tui = "code"`
- **THEN** code's agent is given `tui = "code"` and verify's `tui = "every"`, each from `kind` (32, 175)

### Requirement: A settings file is read where its layer lives

A host's file SHALL be read on the host at each start, absolute, under
`~/` read as the home, or relative to the configuration's directory; a
repository's or a kind's within its repository or package at the
launch, beside the declaration as its prompts are; and a launch's at
the launch. A file read at the launch SHALL be kept as read and not
read again (21, 32, 157, 175, C.7).

#### Scenario: A host's file read at each start

- **WHEN** the host's file changes `tui` while code's agent works
- **THEN** code's agent keeps what it started with, and verify's is given the new `tui` (8, 32, 175)

#### Scenario: A launch's file read at the launch

- **WHEN** a launch gives `--agent programs.agent.file=agent.json` from a directory holding it, and the file changes after the launch
- **THEN** every stage is given the keys the launch read, and status names the file by its absolute path (21, 175)

#### Scenario: A repository's file at the launch's revision

- **WHEN** the repository's `.config/flume.toml` gives `[kinds.fix.agents.programs.agent] file = "agent-fix.json"`, and a commit to the base after the launch changes `.config/agent-fix.json`
- **THEN** every stage of the fix is given the keys at the revision the launch read, from `repository` with the file (21, 157)

### Requirement: A settings file flume cannot take fails its table

A file that cannot be read or parsed, whose top level is no table,
whose name ends neither `.json` nor `.toml`, or that lies outside its
repository or package SHALL fail its table naming the file (4, 157,
175, C.7; core.34).

#### Scenario: A file outside its repository

- **WHEN** the repository's `[kinds.fix.agents.programs.agent]` gives `file = "../agent.json"`
- **THEN** the declaration cannot be read, naming the field and saying the file lies outside the repository, and nothing is launched (4, 157; core.34)

#### Scenario: A host's file that cannot be read

- **WHEN** the host's `[agents.programs.agent]` names a file that does not exist, or one ending `.yaml`
- **THEN** every launch and start is refused naming `agents.programs.agent.file` and the file (C.7)

### Requirement: Every layer gives own settings key by key

Each top-level key of a program's own settings SHALL come from the
first of clause 32's eight layers that gives it, taken whole, never
merged with another layer's value of that key; within a layer, a
stage's own SHALL come first, and a table's `settings` before its file.
The host's layers SHALL be read as they stand at each start, the others
as the launch read them (8, 21, 32, 175).

#### Scenario: Keys from three layers

- **WHEN** the host's `[agents.programs.agent.settings]` gives `line = "host"` and `tips = true`, its `[agents.streams.work.programs.agent.settings]` gives `line = "work"`, and the repository's file gives `[kinds.fix.agents.programs.agent.settings] style = "plain"`
- **THEN** a fix in `work` is given `line = "work"` from `host-stream`, `tips` from `host` and `style` from `repository`, and a fix in `default` `line = "host"` from `host` (S76, 32, 175)

#### Scenario: A key taken whole

- **WHEN** the host's file gives `status = { command = "a", padding = 1 }` and its `[agents.streams.work.programs.agent.settings]` gives `status = { command = "b" }`
- **THEN** a fix in `work` is given `status = { command = "b" }` from `host-stream`, with no `padding` (175)

#### Scenario: A file's table replaced by an empty one

- **WHEN** the host's file gives `hooks` with a hook on each of two events, and its `settings` gives `hooks = {}`
- **THEN** a fix's agent is given `hooks = {}` from `host` with no file (175)

#### Scenario: The host's table changes while a stage works

- **WHEN** the host's `[agents.programs.agent.settings]` changes `line` while code's agent works
- **THEN** code's agent keeps what it started with, and verify's is given the new `line` (8, 32)

### Requirement: Nothing passed where no layer gives own settings

Where no layer gives the agent's program a key, its profile SHALL be
given none, its command SHALL pass nothing for them and write no file,
and status SHALL give the agent no own settings, so the account's own
configuration decides (144, 162, 175).

#### Scenario: A host and a repository that give none

- **WHEN** a fix is launched where no layer gives `programs`
- **THEN** the fake agent's command passes no own settings and lists no file, and status gives no own settings (S76, 175)

#### Scenario: A file that gives no key

- **WHEN** the host's `[agents.programs.agent]` names a file holding `{}` and no layer gives a key
- **THEN** the fake agent's command passes no own settings and lists no file, and status names the file with no key (175)

### Requirement: A reserved key a layer writes refuses the launch or the stage

A key the agent's profile names as another setting's or the account's,
given by a table's `settings` or the launch, SHALL refuse the launch,
or at a later start the stage, naming the key, the program, the stage,
the layer and what owns it, and starting no agent until a retry (4,
27, 32, 175).

#### Scenario: A reserved key at the launch

- **WHEN** the fake agent's profile reserves `env` for the account, and the launch gives `--agent programs.agent.settings.env={"X":"1"}`
- **THEN** the launch is refused naming `programs.agent.settings.env`, `agent`, every stage, `launch` and the account, and nothing is made (S77, 4)

#### Scenario: A reserved key after the launch

- **WHEN** after a fix's launch the host's `[agents.programs.agent.settings]` gives `env`
- **THEN** its next stage is refused naming the key and `host`, no agent starts, and a retry after the table is mended starts it (27, 32)

### Requirement: A reserved key of a named file is left out

A key the agent's profile reserves, given by a named file, SHALL be
left out unread, a nested key out of the table that holds it with the
rest of that table kept as the file gives it, and status SHALL name it
with its file, its layer and its owner; the launch and the stage SHALL
go on (175).

#### Scenario: A people's file with keys another setting owns

- **WHEN** the fake agent's profile reserves `env` for the account and `perms.mode` for flume's mode, and the host's file gives `env = { PEOPLE = "1" }`, `perms = { mode = "full", allow = ["a"] }` and `line = "people"`
- **THEN** a fix launches, its agent is given `perms = { allow = ["a"] }` and `line = "people"` and no `env`, and status names `env` and `perms.mode` left out of the file, from `host`, with the account and the mode as their owners (S76, 175)

#### Scenario: A credential under a key left out

- **WHEN** the host's file gives `env = { TOKEN = "ghp_" }` followed by twenty letters and digits, and the profile reserves `env`
- **THEN** the fix launches with `env` left out, and no part of the value is in status, a log line or the place's held state (29, 175)

### Requirement: A program whose profile takes no own settings refuses them

Keys resolved for the agent's program that its profile gives no way to
take SHALL refuse the launch, or at a later start the stage, naming the
program, the key and the layer; an entry for a program that no stage
of the place runs SHALL refuse nothing but a value shaped as a
credential (4, 32, 170, 175).

#### Scenario: A profile that keeps the seam's default

- **WHEN** the fake agent is scripted to take no own settings, and the host's `[agents.programs.agent.settings]` gives `line`
- **THEN** a fix's launch is refused naming `agent`, `line` and `host`, and saying its profile passes no settings of its program's own (4, 175)

#### Scenario: An entry for a program no stage runs

- **WHEN** the host's `[agents.programs.other.settings]` gives `line` and every stage runs `agent`
- **THEN** the fix launches and lands, and `other`'s entry reaches nothing (175)

### Requirement: A value shaped as a credential is refused

Once a named file's reserved keys are out, a string in an own setting,
inline or from a file, SHALL refuse the launch, or at a later start the
stage, naming its key's path, the layer and the file where it came from
one and never the value, when it holds a token's known prefix, a
private key or a JSON web token, a long run mixing cases and digits, or
is a secret-named key's value naming no variable (4, 29, 175).

#### Scenario: A token in a launch

- **WHEN** a launch gives a key whose value opens `ghp_` and twenty letters and digits
- **THEN** it is refused naming the key and `launch`, saying a credential goes in the account's `env`, its words holding no part of the value, and nothing is made (S77, 29)

#### Scenario: A token in a host's file

- **WHEN** the host's file gives `line = { command = "run --key sk-" }` followed by twenty letters and digits
- **THEN** every launch, and every later start, is refused naming `programs.agent.settings.line.command`, `host` and the file, its words holding no part of the value (S77, 29)

#### Scenario: A secret-named key in the host's configuration

- **WHEN** the host's `[agents.programs.agent.settings]` gives `headers = { Authorization = "Bearer abc" }`
- **THEN** every launch, and every later start, is refused naming `programs.agent.settings.headers.Authorization` and `host`, its words holding no part of the value (29)

#### Scenario: A variable named rather than a value

- **WHEN** the host's `[agents.programs.agent.settings]` gives `headers = { Authorization = "Bearer ${API_TOKEN}" }`
- **THEN** the value is taken as written (175)

### Requirement: Own settings are checked before anything holds them

The core SHALL check every entry of every layer at the launch, each
program's once its own profile's reserved keys are out of its files,
before anything is made, and the host's layers at each later start;
nothing unchecked SHALL be held, logged or answered (29, 175, C.7;
core.8).

#### Scenario: A credential in an entry no stage runs

- **WHEN** a launch gives `--agent programs.other.settings.line=` followed by `ghp_` and twenty letters and digits, and every stage runs `agent`
- **THEN** the launch is refused naming `programs.other.settings.line` and `launch`, and nothing is made or held (29; core.8)

### Requirement: flume reads nothing inside an own setting's value

flume SHALL pass each value as its layer or its file gave it, reading
no `~` and no `$VAR` in it and changing nothing inside it but for
taking out a reserved key of a named file; and it SHALL never pass a
named file to its program as it stands (175; core.30).

#### Scenario: A home and a variable in a command

- **WHEN** the host's `[agents.programs.agent.settings]` gives `line = "~/bin/line $HOME"`
- **THEN** the fake agent's profile is given `~/bin/line $HOME` as written (175)

#### Scenario: A named file not passed

- **WHEN** the host's `[agents.programs.agent]` names a file and the fake takes own settings by a file
- **THEN** the fake's command lists its own file under the start's folder and names no path of the host's file (175)

### Requirement: An account is usable only where its program takes what is given

Where no layer names a stage's program, an account of its pool SHALL
be usable for the stage only where its program's profile takes the
stage's own settings for that program (144, 173, 175).

#### Scenario: A pool of two programs

- **WHEN** a stage's layers name no program, its pool holds an `other` account and then an `agent` account, fill-first, the host gives `[agents.programs.other.settings] line`, and `other`'s profile takes no own settings
- **THEN** the stage starts on the `agent` account, and status says the account came from the strategy (145, 173)
