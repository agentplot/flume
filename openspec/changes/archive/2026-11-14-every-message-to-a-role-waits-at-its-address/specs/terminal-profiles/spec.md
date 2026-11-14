## MODIFIED Requirements

### Requirement: Rex reaches a session by its name

The Rex profile SHALL name a recipient's session `rex:<block>@<socket>`
and reach that block on that server: a block that waits has its held
input cleared first, one working, asking or unread is refused with
nothing typed, and one gone is not found. A role run's address given to
open SHALL reach the block its live start holds. A name in any other
form SHALL be no session of Rex's, `send_to` answering not found and
`session_state` none (7, 43, 54, 132, 190, 214, C.5).

#### Scenario: A recipient's block

- **WHEN** a block the test opened, running no agent of flume's and waiting, is sent a notice by `rex:<block>@<socket>`
- **THEN** it takes the notice, and `session_state` reads it waiting

#### Scenario: A recipient's block at work

- **WHEN** a block the test opened reports working, and a notice is sent to it by `rex:<block>@<socket>`
- **THEN** it is refused naming that the session works, and nothing is typed into the block (54, 214)

#### Scenario: A role run's address

- **WHEN** a role run opens under the address `east/ops`, and a notice is sent to `east/ops`
- **THEN** the notice reaches the run's block, and no second open carries `east/ops` while it lives (132)

#### Scenario: A herdr session's name

- **WHEN** `send_to` and `session_state` are asked of `herdr:w1-p2@/tmp/s`
- **THEN** the first answers not found and the second none, and no Rex call is made

## ADDED Requirements

### Requirement: herdr types only into a pane that waits

The herdr terminal profile SHALL type into a recipient's pane only
while herdr reads it waiting, its held input cleared first, and SHALL
refuse one it reads working, asking or unknown with nothing typed,
calling neither `agent send-keys` nor `agent prompt` (54, 214, C.5;
core.11).

#### Scenario: A recipient at work

- **WHEN** `send_to` names a pane whose agent herdr reads working
- **THEN** it is refused naming that the session works, and no `agent send-keys` and no `agent prompt` is called (54, 214)
