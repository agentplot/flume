## MODIFIED Requirements

### Requirement: The resolver gives a terminal and a window by name

The resolver SHALL give a terminal profile and a window profile by
name, none for a name it carries none for, and list the names it
carries, the one the host's multiplexer names being the terminal the
core is built with (105, 190).

#### Scenario: Named terminals and windows

- **WHEN** the fake resolver is given a terminal `other` and a window `other-window`, and asked for `fake`, `other`, `cmux`, `fake-window` and `kitty`
- **THEN** it gives the core's own terminal, `other`, none, the fake window and none, and lists `fake` and `other`, and `fake-window` and `other-window` (105, 190)

#### Scenario: The terminals built in

- **WHEN** the built-in resolver is asked for `herdr` twice, `rex` and `cmux`
- **THEN** it gives one herdr for the command, the Rex profile, and none for `cmux`, and lists `printed`, `herdr` and `rex`, building each without starting a server (105, 190)
