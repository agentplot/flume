## MODIFIED Requirements

### Requirement: A stream's agents open in its own session

Where the launch's settings and the stage's declaration name no place
in the terminal, a stage's agent SHALL open in the session its place's
stream names for the place's kind, else in the stream's own session:
the one `session` in `[streams.<stream>]` names, else the one named by
the stream's name, and, for the host's default stream naming none, the
terminal's own. It SHALL be read at each start, and an agent already
started SHALL stay where it opened (32, 107, 143, 199).

#### Scenario: Two streams, each in its own session

- **WHEN** the host declares `work` and `personal`, naming no session for either, and a fix is launched on each and one on the default stream
- **THEN** the work fix's agent opens in the session `work`, the personal fix's in `personal`, and the default fix's in the terminal's own (107, 143, S82)

#### Scenario: A stream that names its session

- **WHEN** the host's `[streams.work]` gives `session = "office"` and a fix is launched on `work`
- **THEN** its agent opens in the session `office` (107, 143, C.7)

#### Scenario: The default stream that names one

- **WHEN** the host's `[streams.default]` gives `session = "misc"` and a fix is launched naming no stream
- **THEN** its agent opens in the session `misc` (107, 143)

#### Scenario: A session changed between starts

- **WHEN** the host's `[streams.work] session` is set to `office` while a change on `work` runs its construct stage
- **THEN** the construct agent stays where it opened, and the change's code stage opens in `office` (32, 107)

#### Scenario: A kind's session

- **WHEN** the host's `[streams.work.kinds.research]` gives `session = "work-design"`, and a research place, a change and a fix are launched on `work`
- **THEN** every agent of the research place opens in `work-design`, and every agent of the change and of the fix in `work` (107, 199, S106)

#### Scenario: Another stream's word

- **WHEN** the host's `work` names `work-design` for research, and a research place is launched on `personal`
- **THEN** its agent opens in `personal`, which names no session for research (143, 199)

#### Scenario: A kind's session changed between starts

- **WHEN** the host's local layer names `work-research` for research in `work` while a research place's first stage runs in `work-design`
- **THEN** that agent stays in `work-design`, and the place's next stage opens in `work-research` (32, 190, 199, S107)

### Requirement: A launch names its place's session

A launch SHALL name a session for every stage or one by its settings'
`session`, which SHALL come before every session its stream names, for
the place's kind or for itself; no layer of clause 32 but the launch's
SHALL give one, a kind's or a host's agents table naming `session`
giving nothing; and a stage whose settings name a pane SHALL open beside
it, whatever session is named (32, 107, 158, 199).

#### Scenario: A launch's session over its stream's

- **WHEN** a fix is launched on `work` with `--agent session=review`
- **THEN** its agents open in the session `review`, whatever its stream (107, S82)

#### Scenario: One stage's session

- **WHEN** a change is launched on `work` with `--agent verify.session=review`
- **THEN** its construct and code agents open in `work` and its verify agent in `review` (32, 107)

#### Scenario: A pane before a session

- **WHEN** a launch names the pane `operator` for a conversation stage and the session `review` for every stage
- **THEN** the conversation's agent opens beside `operator` (107)

#### Scenario: A session no other layer gives

- **WHEN** a launch's declaration gives a stage's agent `session = "x"` and the host's `[kinds.fix.agents]` gives `session = "y"`
- **THEN** neither is read as a setting, and the fix's agents open in its stream's session (32, 158)

#### Scenario: A launch's session over the stream's word for a kind

- **WHEN** the host's `work` names `work-design` for research, and a research place is launched on `work` with `--agent session=review`
- **THEN** every one of its agents opens in `review` (107, 199, S106)

#### Scenario: A pane before the stream's word for a kind

- **WHEN** the host's `work` names `work-design` for a kind with a conversation stage, and a launch of that kind on `work` names the pane `operator` for that stage
- **THEN** the conversation's agent opens beside `operator`, and the kind's other stages in `work-design` (107, 199, S106)

### Requirement: Status gives each agent's session and each stream's

Status SHALL give each agent's session with where it came from,
`launch`, `stream-kind`, `stream`, `stream-name` or `own`, none for an
agent opened beside a pane; each stream's own session the same way; and
each session a stream names for its kinds, with the kinds it names it
for, each with its layer, `managed` or `local`, and each fault of its
tables with the refusal it causes, in the JSON form and the person's
(56, 107, 143, 199, C.7).

#### Scenario: The sessions in status

- **WHEN** status is asked of a host running a work fix, a fix launched with `--agent session=review` and a default fix, `work` naming no session
- **THEN** the agents give `work` from `stream-name`, `review` from `launch`, and the terminal's own from `own`, and the stream `work` gives `work` from `stream-name` (56, S82)

#### Scenario: The sessions a stream names for its kinds

- **WHEN** status is asked of a host whose managed layer names `work-design` for research in `work` and whose local layer names it for `plan` there, while a research place and a fix run on `work`
- **THEN** the research agent gives `work-design` from `stream-kind`, the fix's agent `work` from `stream-name`, and the stream `work` gives its own `work` from `stream-name` and `work-design` for `research` from `managed` and for `plan` from `local`, in the JSON form and in the person's as `session work-design (stream work, kind research)` (56, 199, S106, C.7)

## ADDED Requirements

### Requirement: A stream's table for a kind holds its session alone

The host's `[streams.<stream>.kinds.<kind>]` SHALL be read with
`session`, the kind named by its name whatever source and version it
resolves to; a `session` that is not a string SHALL refuse the
configuration naming the table and the key (143, 199, C.7).

#### Scenario: A session that is no word

- **WHEN** the host's `[streams.work.kinds.research]` gives `session = 3`
- **THEN** the configuration is refused naming `streams.work.kinds.research` and `session` (C.7)

### Requirement: A table for a kind the place does not run is ignored

A stream's table for a kind the place does not run SHALL be ignored, as
the host's settings for a kind the place does not run are (199, C.7).

#### Scenario: A kind no place runs

- **WHEN** the host's `[streams.work.kinds.writeback]` gives `session = "x"`, and a change is launched on `work`
- **THEN** its agents open in `work`, and no start is refused (199, C.7)

### Requirement: A table's other key refuses the starts that read it

Any key of a stream's table for a kind but `session` SHALL be kept, and
SHALL refuse every start that reads that table, naming the key and the
table, until the configuration is mended and the stage retried, every
other start running (4, 27, 182, 199).

#### Scenario: A table's other key

- **WHEN** the host's managed `[streams.work.kinds.research]` gives `session = "work-design"` and `label = "inception"`, and a research place and a change are launched on `work` and a research place on `personal`
- **THEN** the research stage on `work` is refused naming `streams.work.kinds.research` and `label`, the change on `work` and the research place on `personal` start, and, once `label` is gone, a retry starts the research stage in `work-design` (4, 199, S107)

#### Scenario: A stage's table

- **WHEN** the host's `[streams.work.kinds.change.stages.construct]` gives `session = "work-design"`, and a change is launched on `work`
- **THEN** its construct stage is refused naming `streams.work.kinds.change` and `stages` (4, 199)

### Requirement: A session a stream names for a kind is its alone

A session a stream names for a kind that is a session another stream's
agents open in by that stream's word, its own session, its name where it
names none, or one it names for a kind, SHALL refuse every start that
reads it, naming both streams and the session, until the configuration
is mended, every other start running; a stream naming its own session
for a kind SHALL open that kind's agents there from `stream-kind` (4,
143, 199).

#### Scenario: Another stream's own session

- **WHEN** the host declares `work` and `personal`, naming no session for either, and `work` names `personal` for research, and a research place and a fix are launched on `work`
- **THEN** the research stage is refused naming `work`, `personal` and the session `personal`, and the fix starts in `work` (4, 199, S107)

#### Scenario: Two streams naming one session for a kind

- **WHEN** the host's `work` and `personal` each name `design` for research, and a research place is launched on each
- **THEN** each research stage is refused naming `work`, `personal` and `design` (4, 199)

#### Scenario: A stream's own session named for a kind

- **WHEN** the host's `[streams.work]` gives `session = "office"` and names `office` for research, and a research place is launched on `work`
- **THEN** its agent opens in `office` from `stream-kind` (107, 199)

### Requirement: A place whose agents open apart keeps its stream

A place whose agents open in a session its stream names for its kind
SHALL keep its stream, its bound, its priority, its pool, its points and
pace, and its label's share, every one of its agents counted as any
agent of its stream is (70, 143, 180, 199).

#### Scenario: Room and shares unchanged

- **WHEN** the host's `work` states two agents and the pool `work`, names `work-design` for research, and a research place, a fix and then a chore are launched on `work`
- **THEN** the research place and the fix run, each counted in work's two and each on an account of `work`'s pool, the research agent in `work-design`, and the chore waits for work's room (70, 143, 199, S106)
