## ADDED Requirements

### Requirement: launch gives own settings by --agent

`flume launch --agent [<stage>.]programs.<program>.<key>=<value>` SHALL
give one key of a program's own settings for every stage or for one,
its value read as JSON where it parses and as the text written
otherwise, several building one entry; one key given twice for the
same stage SHALL be refused naming it, and `programs` SHALL be named
among the settings in every refusal (7, 175, C.2).

#### Scenario: Own settings at the launch

- **WHEN** `flume launch --kind fix --agent programs.claude.outputStyle=Explanatory --agent code.programs.claude.spinnerTipsEnabled=false` runs over a scratch repository
- **THEN** the launch's settings give every stage `outputStyle` the string `Explanatory` and code `spinnerTipsEnabled` the boolean `false`, each at the `launch` layer (7, 32, 175)

#### Scenario: A key given twice

- **WHEN** `flume launch --kind fix --agent programs.claude.outputStyle=a --agent programs.claude.outputStyle=b` runs
- **THEN** it is refused naming `programs.claude.outputStyle`, and nothing is launched (7)
