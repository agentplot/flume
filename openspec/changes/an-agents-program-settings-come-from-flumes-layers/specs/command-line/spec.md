## ADDED Requirements

### Requirement: launch gives own settings by --agent

`flume launch --agent [<stage>.]programs.<program>.settings.<key>=<value>`
SHALL give one key of a program's own settings for every stage or for
one, its value read as JSON where it parses and as the text written
otherwise, several building one entry; one key given twice for the
same stage SHALL be refused naming it, and `programs` SHALL be named
among the settings in every refusal (7, 175, C.2).

#### Scenario: Own settings at the launch

- **WHEN** `flume launch --kind fix --agent 'programs.claude.settings.statusLine={"type":"command","command":"line"}' --agent code.programs.claude.settings.tui=fullscreen` runs over a scratch repository
- **THEN** the launch's settings give every stage `statusLine` the table and code `tui` the string `fullscreen`, each at the `launch` layer (7, 32, 175)

#### Scenario: A key given twice

- **WHEN** `flume launch --kind fix --agent programs.claude.settings.tui=fullscreen --agent programs.claude.settings.tui=default` runs
- **THEN** it is refused naming `programs.claude.settings.tui`, and nothing is launched (7)

### Requirement: launch names a settings file by --agent

`flume launch --agent [<stage>.]programs.<program>.file=<path>` SHALL
name a program's settings file for every stage or for one, made
absolute against the command's working directory and read at the
launch (7, 21, 175, C.2).

#### Scenario: A file at the launch

- **WHEN** `flume launch --kind fix --agent programs.claude.file=claude.json` runs in a scratch directory holding `claude.json`, which gives `tui = "fullscreen"`
- **THEN** the launch's settings name the file by its absolute path with `tui` as read at the launch, at the `launch` layer (7, 21, 175)
