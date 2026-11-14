## MODIFIED Requirements

### Requirement: herdr reaches a recipient by pane and socket

The herdr terminal profile SHALL name a session `herdr:<pane>@<socket>`
and send to, and read the state of, a recipient by that pane on that
server, typing only into a pane that reads waiting, its held input
cleared first, and refusing one that reads working, asking or unknown
with nothing typed; a role run's session SHALL be the pane its live
start holds, found again by that start's herdr name where the id no
longer answers and taken only once herdr lists it under that name; and
each call SHALL run under the profile's bound with the caller's
`HERDR_*` stripped and the socket passed (43, 54, 95, 96, 98, 132, 214,
C.5; records 0016, 0018; core.29).

#### Scenario: A recipient whose agent herdr renamed

- **WHEN** a recipient's session `/clear`s, so herdr drops its agent's name, and an end is delivered to it
- **THEN** the notice reaches the pane by its id on its server

#### Scenario: A recipient on another server

- **WHEN** a settle runs from a pane of one herdr server and the recipient's pane is on another
- **THEN** the notice is sent through the recipient's own socket, none of the settle's `HERDR_*` reaching herdr

#### Scenario: A role run's pane moved to another workspace

- **WHEN** a role run's pane is moved into another workspace, so its id no longer answers, and an end is delivered to the role
- **THEN** the profile finds the pane by the run's herdr name, records its new id, and the notice reaches it there

#### Scenario: A recipient at work

- **WHEN** `send_to` names a pane whose agent herdr reads working
- **THEN** it is refused naming that the session works, and no `agent send-keys` and no `agent prompt` is called (54, 214)

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
