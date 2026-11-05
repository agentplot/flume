## ADDED Requirements

### Requirement: herdr's attach

The herdr profile's attach SHALL answer `<herdr> session attach
<name>`, `<herdr>` the full path of the herdr its calls run, `flume` for
its own session; SHALL first find or start the session's server as open
does; SHALL give `XDG_CONFIG_HOME` where its calls run under one and no
`HERDR_*`; and SHALL refuse a session herdr refuses in herdr's words
(107, 187, C.5; core.29).

#### Scenario: A stream's session not yet running

- **WHEN** the profile, under a scratch session home, is asked to attach the session `w`, which `herdr session list --json` lists not running
- **THEN** herdr lists `w` running, and the command is the pinned herdr's full path with `session attach w` and `XDG_CONFIG_HOME` the scratch home (187, S90)

#### Scenario: A client the command runs

- **WHEN** the command attach gave for `w` is run under a pseudo-terminal for two seconds
- **THEN** it is still running at their end, and `herdr session list --json` lists `w` running at the socket attach started, no other server of `w` started (187)

#### Scenario: The own session

- **WHEN** a profile whose own server is the session `flume` is asked to attach its own, and a profile whose own server is held by its socket alone is asked the same
- **THEN** the first answers `session attach flume`, and the second refuses, saying herdr attaches by a session's name (187)

#### Scenario: A name herdr refuses

- **WHEN** the profile is asked to attach a session whose name the pinned herdr refuses
- **THEN** attach is refused with herdr's words, as open is for that name (187; core.29)

### Requirement: The printed command refuses attach

The printed command's attach SHALL refuse, saying it holds no session a
person can be shown, since its agents run in the commands it printed
(187).

#### Scenario: Attach under the printed command

- **WHEN** the printed command is asked to attach `work`
- **THEN** it refuses with those words, and runs nothing (187)
