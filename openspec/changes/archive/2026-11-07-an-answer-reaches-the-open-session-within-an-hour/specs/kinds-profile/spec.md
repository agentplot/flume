## ADDED Requirements

### Requirement: A declaration gives no keep_open

A declaration's agents table, a kind's, a stage's, or a repository's
for every kind or for a kind it does not declare, that gives
`keep_open` SHALL fail to read, naming the field and that the host or
the launch gives it, so the launch is refused whole (4, 158, 198, C.2).

#### Scenario: A repository's keep_open

- **WHEN** a repository's `.config/flume.toml` gives `[kinds.fix.agents] keep_open = "2h"`
- **THEN** a launch of a fix is refused naming `kinds.fix.agents.keep_open`, and nothing is made (4, 198)

#### Scenario: A kind's keep_open

- **WHEN** a package's kind declares a stage whose `agents` entry gives `keep_open = "5m"`
- **THEN** a launch of that kind is refused naming the stage's `keep_open`, and nothing is made (4, 158)
