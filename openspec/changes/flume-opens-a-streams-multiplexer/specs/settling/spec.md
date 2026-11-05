## MODIFIED Requirements

### Requirement: Attended and watched

Where a terminal the host binds keeps a watch, its watch SHALL settle
each place on its stated bound. Where any terminal the host binds, its
own or a stream's, keeps none, every command of flume's that acts on
places SHALL settle every place on the host before it acts, and status
and a prompt's rendering SHALL settle nothing (119, 122, 190).

#### Scenario: Killed with nobody watching

- **WHEN** under a fake terminal with no watch a code stage's agent stops with nothing delivered, and later a caller drops another place
- **THEN** nothing reads the stage until the drop, which settles every place first, and status then reports the stage lost (S27)

#### Scenario: Crashed under a watch

- **WHEN** under a fake terminal with a watch a code stage's agent stops with nothing delivered and its on-quiet never runs
- **THEN** the watch's settle reads the stage lost within its bound, with no caller asking, and starts no agent (S28)

#### Scenario: A watched host with one stream unwatched

- **WHEN** the host's own fake terminal keeps a watch, its `trial` names a fake terminal that keeps none, and a caller drops a place on `work`
- **THEN** the drop settles every place first (119, 190, S91)
