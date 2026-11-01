## ADDED Requirements

### Requirement: A declaration gives no keep_open

A declaration's agents table, a kind's, a stage's or the repository's
for a kind it does not declare, that gives `keep_open` SHALL fail to
read, naming the field and that the host or the launch gives it, so the
launch is refused whole (4, 32, C.2).

#### Scenario: A repository's keep_open

- **WHEN** a repository's `.config/flume.toml` gives `[kinds.fix.agents] keep_open = "2h"`
- **THEN** a launch of a fix is refused naming `kinds.fix.agents.keep_open`, and nothing is made (4, 32)
