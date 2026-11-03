## ADDED Requirements

### Requirement: A stream's agents open in its own session

Where the launch's settings and the stage's declaration name no place
in the terminal, a stage's agent SHALL open in its place's stream's
session: the one `session` in `[streams.<stream>]` names, else the one
named by the stream's name, and, for the host's default stream naming
none, the terminal's own. It SHALL be read at each start, and an agent
already started SHALL stay where it opened (32, 107, 143).

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

### Requirement: A launch names its place's session

A launch SHALL name a session for every stage or one by its settings'
`session`, which SHALL come before its stream's; no other layer SHALL
give one, a kind's or a host's agents table naming `session` giving
nothing; and a stage whose settings name a pane SHALL open beside it,
whatever session is named (32, 107, 158).

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

### Requirement: Status gives each agent's session and each stream's

Status SHALL give each agent's session with where it came from,
`launch`, `stream`, `stream-name` or `own`, none for an agent opened
beside a pane, and each stream's session the same way, in the JSON form
and the person's (56, 107, 143).

#### Scenario: The sessions in status

- **WHEN** status is asked of a host running a work fix, a fix launched with `--agent session=review` and a default fix, `work` naming no session
- **THEN** the agents give `work` from `stream-name`, `review` from `launch`, and the terminal's own from `own`, and the stream `work` gives `work` from `stream-name` (56, S82)
