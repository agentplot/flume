## Purpose

The host's configuration in two layers: the managed `config.toml` and
the local `config.local.toml` read over it key by key at each reading,
the local layer setting only C.7's keys and none the managed layer
pins, a local layer at fault not taken, flume writing neither file, and
status giving every value with its layer (32, C.7).

## ADDED Requirements

### Requirement: A managed layer and a local layer over it

The host's configuration SHALL be read from its managed layer,
`$XDG_CONFIG_HOME/flume/config.toml`, and its local layer,
`config.local.toml` beside it, at each reading, a key the local layer
sets replacing the managed layer's whole; a host with no local file
SHALL read the managed layer alone, and no command of flume's SHALL
write either file (32, C.7; core.16).

#### Scenario: A stream's bound and priority set locally

- **WHEN** the managed layer gives `[streams.work]` six agents and the pool `work`, and the local layer gives it three agents and a priority of 30
- **THEN** the host reads `work` at three agents and priority 30, each from the local layer, and its pool from the managed one, and the next start finds room for three (32, C.7, S83)

#### Scenario: A kind's settings key by key

- **WHEN** the managed layer gives `[kinds.change.agents]` the model `opus` and instructions, and the local layer gives it the model `sonnet`
- **THEN** a change's agents take `sonnet` from the local layer and the instructions from the managed one (32, C.7)

#### Scenario: No local file

- **WHEN** the host has no `config.local.toml`
- **THEN** it reads the managed layer alone, and status says the local layer has no file (C.7)

#### Scenario: flume writes neither file

- **WHEN** a launch, its settles, status and its tidy run on a host with both files
- **THEN** neither file's content or write time changes (C.7; core.16)

### Requirement: The local layer sets only C.7's keys

The local layer SHALL set only a stream's `agents`, `priority`,
`points`, `pace` and `session`, a label's `reserve` and `keep_free`,
the `stream` and `label` a repository's kind or a kind is given, the
`stream` a repository is given, and a kind's agents table; a local file
setting any other key, unreadable, or setting a value its key refuses,
SHALL not be taken at all, the managed layer read alone (C.7).

#### Scenario: A local file that sets a pool

- **WHEN** the local layer of a host whose managed layer gives `work` six agents gives it three, and also gives `[pools.work]` a strategy and an account
- **THEN** no key of it is taken, `work` reads six agents from the managed layer, and status names `pools.work` (C.7, S83)

#### Scenario: A stream's pool set locally

- **WHEN** the local layer gives `[streams.work] pool = "personal"`
- **THEN** no key of it is taken, and status names `streams.work.pool` (C.7)

#### Scenario: A local file at fault

- **WHEN** the local layer is not TOML, or gives `[streams.work] agents = "three"`
- **THEN** no key of it is taken, status gives its fault, and every command runs on the managed layer alone (C.7)

#### Scenario: A stream only the local layer names

- **WHEN** the local layer gives `[streams.nightly]` one agent and `[kinds.chore] stream = "nightly"`, and the managed layer gives `[host] pool = "personal"`
- **THEN** the host declares `nightly` with one agent from the local layer and the pool `personal` from the host's, and a chore runs in it (143, C.7)

### Requirement: The managed layer pins keys

The managed layer SHALL pin keys by `pin` under `[host]`, a list of
dotted keys in TOML's spelling, each naming a key the local layer may
set or a table holding some; a local key a pin holds SHALL be passed
over, status naming it with its pin, and the rest taken; a pin naming
nothing the local layer may set SHALL refuse the configuration naming
the pin, as any fault of the managed layer does (C.7).

#### Scenario: One key pinned

- **WHEN** the managed layer gives `work` six agents and pins `"streams.work.agents"`, and the local layer gives `work` three agents and a priority of 30
- **THEN** the local priority is taken, its agents are passed over, `work` reads six agents from the managed layer, and status names `streams.work.agents` with its pin (C.7, S83)

#### Scenario: A table pinned

- **WHEN** the managed layer pins `"streams.work"`, and the local layer gives `work` three agents and the session `office`, and `[kinds.fix] stream = "work"`
- **THEN** both of work's keys are passed over and the fix's stream is taken (C.7)

#### Scenario: A pin of nothing the local layer sets

- **WHEN** the managed layer pins `"pools.work"`
- **THEN** reading the configuration is refused naming the pin (C.7)

### Requirement: Status gives every value with its layer

Status's JSON form SHALL give every value of the host's configuration
with the layer it came from, `managed` or `local`, an account's
variables by name and `<set>` and never a value; and both forms SHALL
give whether the local layer was taken, and where not, each key or the
fault, each value it set, and each key passed over with its pin (56,
C.7; core.30).

#### Scenario: Values from both layers

- **WHEN** status is asked of a host whose local layer gives `work` three agents and a priority of 30
- **THEN** its JSON form gives `streams.work.agents` 3 and `streams.work.priority` 30 from `local`, `streams.work.pool` from `managed`, and each account's variables by name and `<set>`, and the person's form gives the two local values (56, S83; core.30)

#### Scenario: A local layer not taken

- **WHEN** status is asked of a host whose local layer sets `[pools.work]`
- **THEN** both forms give the local layer not taken, naming `pools.work`, and the JSON form every value from `managed` (56, S83)
