## ADDED Requirements

### Requirement: The fake agent scripts its own settings

The fake agent SHALL script whether it takes own settings, the keys it
reserves with their owners, and its way, a file under the start's
folder its command names by `--own <file>`, each passing the agent
seam's scenarios; its log SHALL name the entry it was given, so a test
reads what the core passed (106, 175, 176).

#### Scenario: A scripted way and a reserved key

- **WHEN** a test scripts the fake agent to take own settings by a file and to reserve `env`, and a fix's layers give `line` and `tips`
- **THEN** the core writes the file the fake's command lists before the open, the fake's log names `line` and `tips`, and a launch giving `env` is refused naming the fake's owner (175, 176)
