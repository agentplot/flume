# host-configuration Specification

## Purpose
The host's configuration in two layers: the managed `config.toml` and
the local `config.local.toml` read over it key by key at each reading,
the local layer setting only C.7's keys and none the managed layer
pins, a local layer at fault not taken, flume writing neither file, and
status giving every value with its layer (32, C.7).

## Requirements

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
`points`, `pace`, `session` and `multiplexer`, the `session` a stream
names for a kind and for a kind's stage, a label's `reserve` and `keep_free`, the `stream` and
`label` a repository's kind or a kind is given, the `stream` a
repository is given, and a kind's agents table; a local file setting any
other key, unreadable, or setting a value its key refuses, SHALL not be
taken at all, the managed layer read alone (C.7, 199).

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

#### Scenario: A stream's multiplexer set locally

- **WHEN** the managed layer gives `[host] multiplexer = "herdr"` and `[streams.trial] multiplexer = "printed"`, and the local layer gives `[streams.trial] multiplexer = "herdr"`
- **THEN** the local layer is taken, `trial` reads `herdr` from `stream` in the local layer, and status gives it with that layer (190, C.7, S91)

#### Scenario: A kind's session and a stage's set locally

- **WHEN** the managed layer gives `[streams.work.kinds.research] session = "work-design"`, and the local layer gives it `session = "work-research"` and `[streams.work.kinds.release.stages.review] session = "work-review"`
- **THEN** the local layer is taken, research reads `work-research` and release's review `work-review` in `work`, each from the local layer, and status gives each with that layer (199, C.7, S107)

#### Scenario: A kind's table key the local layer cannot set

- **WHEN** the local layer gives `[streams.work.kinds.research] label = "inception"`
- **THEN** no key of it is taken, and status names `streams.work.kinds.research.label` (C.7)

#### Scenario: A kind's session pinned

- **WHEN** the managed layer gives `[streams.work.kinds.research] session = "work-design"` and pins `"streams.work.kinds"`, and the local layer gives research `session = "work-research"` and `work` three agents
- **THEN** `work`'s three agents are taken, research's session is passed over and reads `work-design` from the managed layer, and status names `streams.work.kinds.research.session` with its pin (199, C.7)

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

### Requirement: A stream's view, terminal app and title are the managed layer's

A stream's `view`, `terminal_app` and `title`, and every key of
`[host]`, SHALL be the managed layer's alone: a local file that sets one
SHALL not be taken at all, status naming the key, the managed layer read
alone (186, C.7).

#### Scenario: A stream's view set locally

- **WHEN** the local layer gives `[streams.work] view = "app"`
- **THEN** no key of it is taken, and status names `streams.work.view` (186, C.7)

#### Scenario: A stream's title or the host's multiplexer set locally

- **WHEN** the local layer gives `[streams.work] title = "Mine"`, or `[host] multiplexer = "herdr"`
- **THEN** no key of it is taken, and status names `streams.work.title`, or `host.multiplexer` (186, C.7)

### Requirement: A repository's tables are named by its address, else its folder

The host's configuration SHALL name a repository, in
`[repos."<repository>"]` and every table under it, by its address,
`<owner>/<name>` as its clone's `origin` gives it on its forge, and by
its folder, a leading `~` in the key the home, where its clone has no
forge origin. A key SHALL be matched exactly, with no star (143, 180,
C.7; record 0055).

#### Scenario: The address's table

- **WHEN** the managed layer gives `[repos."acme/acme"] stream = "work"`, `[repos."acme/acme".kinds.fix] label = "operation"`, `[repos."acme/acme".kinds.fix.agents] model = "haiku"` and `[repos."acme/acme".agents] keep_open = "2h"`, and the repository at `/r` has the address `acme/acme`
- **THEN** the host names `work` for a fix in `/r` from `host-repository`, gives it the label `operation` from `host-repository-kind`, and its settings give the kind's model `haiku` and every kind's `keep_open` `2h` (32, 143, 180)

#### Scenario: A repository with no address

- **WHEN** the managed layer gives `[repos."/r"] stream = "work"` and `[repos."acme/acme"] stream = "personal"`, and the repository at `/r` has no address
- **THEN** the host names `work` for it from `host-repository` (143)

#### Scenario: Matched exactly

- **WHEN** the managed layer gives `[repos."Acme/acme"] stream = "work"` and `[repos."acme/*"] stream = "personal"`, and the repository at `/r` has the address `acme/acme`
- **THEN** the host names no stream for it (143)

### Requirement: The address's table stands whole, else the folder's

Where the configuration has a table under a repository's address, that
table SHALL be the repository's, whole, and its folder's SHALL not be
read; otherwise its folder's SHALL be, so a file written by folder
reads as it did (143; record 0055).

#### Scenario: A folder's table where no table names the address

- **WHEN** the managed layer gives `[repos."~/Code/acme"] stream = "work"` and no table under `acme/acme`, and the repository in `~/Code/acme` has the address `acme/acme`
- **THEN** the host names `work` for it from `host-repository`, so a file written by folder reads as it did (143; record 0055)

#### Scenario: The address's table whole over the folder's

- **WHEN** the managed layer gives `[repos."acme/acme"] stream = "work"` and `[repos."/r".kinds.fix] stream = "personal"`, and the repository at `/r` has the address `acme/acme`
- **THEN** the host names `work` for a fix in `/r` from `host-repository`, the folder's table not read (143)

### Requirement: The local layer finds an address's table by its key

The local layer SHALL set a key of a managed table named by an address
under the same key, and a pin SHALL name such a key as it names any
other (C.7; core.45).

#### Scenario: An address's stream set locally

- **WHEN** the managed layer gives `[repos."acme/acme"] stream = "work"` and the local layer gives `[repos."acme/acme"] stream = "personal"`, and then the managed layer also gives `[host] pin = ["repos.\"acme/acme\".stream"]`
- **THEN** the host names `personal` from the local layer for the repository whose address is `acme/acme`, and with the pin `work` from the managed layer, status naming the key passed over (C.7; core.45)
