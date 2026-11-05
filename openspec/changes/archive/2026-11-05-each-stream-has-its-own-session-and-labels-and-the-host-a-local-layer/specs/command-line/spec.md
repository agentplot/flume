## ADDED Requirements

### Requirement: launch names a session by --agent

`flume launch --agent [<stage>.]session=<name>` SHALL name the session
its place's agents open in, for every stage or for one, at the
`launch` layer, and `--agent`'s help and its refusals SHALL name
`session` among the settings it gives (7, 32, 107).

#### Scenario: A session for every stage and for one

- **WHEN** `flume launch --kind change --agent session=review --agent verify.session=late` runs over a scratch repository
- **THEN** the launch's settings give every stage `review` and verify `late`, each at the `launch` layer (7, 107)

#### Scenario: The settings named in a refusal

- **WHEN** `flume launch --kind fix --agent colour=blue` runs
- **THEN** it is refused naming `colour`, and its words name `session` among the settings `--agent` gives, and nothing is launched (7, 32)
