## ADDED Requirements

### Requirement: launch gives a mode by --agent

`flume launch --agent [<stage>.]mode=<mode>` SHALL give one of flume's
four for every stage or for one, and `--agent
[<stage>.]mode.<program>=<value>` one entry of a table of programs,
several building one table; a word and a table for the same stage, or
for every stage, SHALL be refused naming both, and a bare value that is
not one of the four SHALL be refused naming it (7, 162, 169, C.2).

#### Scenario: A mode and a table at the launch

- **WHEN** `flume launch --kind fix --agent mode=edits --agent code.mode.claude=auto --agent code.mode.other=ask` runs over a scratch repository
- **THEN** the launch's settings give every stage `edits` and code the table `{ claude = "auto", other = "ask" }`, each at the `launch` layer (7, 32)

#### Scenario: A word and a table for one stage

- **WHEN** `flume launch --kind fix --agent code.mode=edits --agent code.mode.claude=auto` runs
- **THEN** it is refused naming both, and nothing is launched (7, 169)

#### Scenario: A program's own value given bare

- **WHEN** `flume launch --kind fix --agent mode=auto` runs
- **THEN** it is refused naming `auto` and saying it goes under its program's name, as `--agent mode.<program>=auto`, and nothing is launched (169)
