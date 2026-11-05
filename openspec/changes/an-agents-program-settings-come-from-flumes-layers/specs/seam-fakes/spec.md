## ADDED Requirements

### Requirement: The fake agent scripts its own settings

The fake agent SHALL script whether it takes own settings, the keys it
reserves with their owners, nested keys by their path among them, and
its way, a file under the start's folder its command names by `--own
<file>`, each passing the agent seam's scenarios; its log SHALL name
the keys it was given, so a test reads what the core passed (106, 195,
196).

#### Scenario: A scripted way and a reserved key

- **WHEN** a test scripts the fake agent to take own settings by a file and to reserve `env`, and a fix's layers give `line` and `tips` inline and `env` by a named file
- **THEN** the core writes the file the fake's command lists before the open, the fake's log names `line` and `tips` and not `env`, and a launch giving `env` inline is refused naming the fake's owner (195, 196)
