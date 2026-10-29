## ADDED Requirements

### Requirement: herdr reaches a recipient by pane and socket

The herdr terminal profile SHALL name a session `herdr:<pane>@<socket>`
and send to, and read the state of, a recipient by that pane on that
server; a role run's session SHALL be the pane its live start holds,
found again by that start's herdr name where the id no longer answers
and taken only once herdr lists it under that name; and each call SHALL
run under the profile's bound with the caller's `HERDR_*` stripped and
the socket passed (43, 95, 96, 98, 132, C.5; records 0016, 0018;
core.29).

#### Scenario: A recipient whose agent herdr renamed

- **WHEN** a recipient's session `/clear`s, so herdr drops its agent's name, and an end is delivered to it
- **THEN** the notice reaches the pane by its id on its server

#### Scenario: A recipient on another server

- **WHEN** a settle runs from a pane of one herdr server and the recipient's pane is on another
- **THEN** the notice is sent through the recipient's own socket, none of the settle's `HERDR_*` reaching herdr

#### Scenario: A role run's pane moved to another workspace

- **WHEN** a role run's pane is moved into another workspace, so its id no longer answers, and an end is delivered to the role
- **THEN** the profile finds the pane by the run's herdr name, records its new id, and the notice reaches it there
