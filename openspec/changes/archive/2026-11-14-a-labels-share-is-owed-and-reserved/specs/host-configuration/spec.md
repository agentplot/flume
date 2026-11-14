## MODIFIED Requirements

### Requirement: The local layer sets only C.7's keys

The local layer SHALL set only a stream's `agents`, `priority`,
`points`, `pace`, `session` and `multiplexer`, the `session` a stream
names for a kind and for a kind's stage, a label's `owed` and `reserved`, the `stream` and
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

#### Scenario: A label's share set locally

- **WHEN** the local layer gives `streams.work.labels.operation.owed = 3` and `streams.work.labels.operation.reserved = 1`
- **THEN** the local layer is taken, `operation` is owed three in `work` and holds one reserved, each from the local layer (181, C.7, S85)

#### Scenario: A label's old key set locally

- **WHEN** the local layer gives `streams.work.labels.operation.reserve = 3`
- **THEN** no key of it is taken, and status names `streams.work.labels.operation.reserve` (181, C.7)
