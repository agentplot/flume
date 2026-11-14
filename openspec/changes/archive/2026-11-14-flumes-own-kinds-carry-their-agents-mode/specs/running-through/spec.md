## MODIFIED Requirements

### Requirement: change@3 codes its change with one agent

A place of flume's own change@3, or of change@4 after it, SHALL run its
construct, then its code stage as one start of one agent for every task
of the change, whatever its task groups, then its verify, and SHALL be
submitted once verify delivers the change archived; status SHALL give
its code stage as `code` with no group (22, 30, 154, S119; record 0064).

#### Scenario: A bare change runs change@3 through

- **WHEN** a repository whose lock records `flume/change@3` launches `change`, its construct writes a change of four task groups, its code agent ticks every task and goes quiet, and its verify archives the change
- **THEN** the place runs `flume/change@3`, its code stage has one start, status gives the stage as `code` with no group count while it runs, and the place is submitted once verify delivers (22, 154, 155, S119)

#### Scenario: A bare change runs change@4 through

- **WHEN** a repository that locks nothing launches `change`, its construct writes a change of four task groups, its code agent ticks every task and goes quiet, and its verify archives the change
- **THEN** the place runs `flume/change@4`, each stage's agent in `full` from `kind`, its code stage has one start, status gives the stage as `code` with no group count while it runs, and the place is submitted once verify delivers (22, 154, 162, S119)
