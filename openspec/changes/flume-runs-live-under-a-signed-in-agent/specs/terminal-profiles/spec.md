## MODIFIED Requirements

### Requirement: herdr reads a start held or never come up

The herdr profile SHALL read the folder-trust question off the pane's
screen whatever `agent start` answered, `agent_started`,
`agent_not_ready` or `timeout`, since herdr may read the dialog ready,
held or neither and may answer before it is drawn; where the screen
shows it, the profile SHALL send the agent profile's next key by `pane
send-keys` while another choice holds the focus and its confirming key
only while the yes choice holds it, each key once the screen shows what
the last did, and read again until the question is gone, within 30
seconds, never confirming another choice. Where `agent start` answered
`agent_not_ready`, it SHALL then wait within its bound until herdr reads
the agent idle, and a screen that shows no folder-trust question by
five seconds SHALL be held with the screen; a `timeout` with no
folder-trust question SHALL be not come up with the screen (34, 35,
94, C.5; record 0018).

#### Scenario: The folder-trust question at the start

- **WHEN** `agent start` answers `agent_not_ready` and the pane shows the folder-trust question
- **THEN** the profile moves the focus to the yes choice, confirms it, waits within its bound until herdr reads the agent idle, and the prompt is sent only after the started ack (S39)

#### Scenario: The folder-trust question drawn late

- **WHEN** `agent start` answers `agent_not_ready` while the pane shows only the dialog's first line, and the choices are drawn a second later
- **THEN** the profile waits for the choices, answers yes from the screen, and the agent comes up (S39)

#### Scenario: A folder-trust dialog herdr does not see

- **WHEN** the program shows the folder-trust question and herdr reads it neither ready nor held, so `agent start` answers `timeout`
- **THEN** the profile answers yes from the screen and the start is opened, its started ack deciding it (S39)

#### Scenario: The bypass warning at the start

- **WHEN** `agent start` answers `agent_not_ready` and the pane shows the warning about bypassing permissions
- **THEN** nothing is answered, and the core reads the start never begun with the warning's lines (S40)

#### Scenario: A program that never comes up

- **WHEN** the program the stage names is not on the pane's `PATH` and `agent start` answers `timeout`
- **THEN** the core reads the start never begun with the pane's visible lines, and sends no prompt (S45)

### Requirement: The printed command

The printed-command profile SHALL answer open with the command, its
prompt carried in it, for a person to run in the place, its environment
kept in a file beside its mark in the place's folder that the host's
user alone may read and that the command names, so the command holds no
value of it; refuse send and close; tidy nothing and show nothing; read
the agent stopped only once the agent's own record says its session
ended (134); give no screen (136); and keep no watch, so flume runs
attended (32, 38, 119, C.5).

#### Scenario: A tell under the printed command

- **WHEN** a caller tells a place's agent under the printed command
- **THEN** the tell is refused with nothing typed (53)

#### Scenario: Seam scenarios over the printed command

- **WHEN** the terminal seam's scenarios run against the profile
- **THEN** each passes, its refusals among them

#### Scenario: The environment beside the command

- **WHEN** the host's configuration gives the agent a variable and the command is printed
- **THEN** the command names its environment file and not the variable, and the file, readable by the host's user alone, holds the variable read
