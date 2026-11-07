## ADDED Requirements

### Requirement: launch gives an effort by --agent

`flume launch --agent [<stage>.]effort=<word>` SHALL give the effort for
every stage or for one, at the `launch` layer, kept as written, and
`--agent`'s help and its refusals SHALL name `effort` among the
settings it gives (7, 32, 197).

#### Scenario: An effort for every stage and for one

- **WHEN** `flume launch --kind change --agent effort=high --agent verify.effort=max` runs over a scratch repository
- **THEN** the launch's settings give every stage `high` and verify `max`, each at the `launch` layer (7, 197)

#### Scenario: The settings named in a refusal

- **WHEN** `flume launch --kind fix --agent colour=blue` runs
- **THEN** it is refused naming `colour`, and its words name `effort` among the settings `--agent` gives, and nothing is launched (7, 32)
