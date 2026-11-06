## MODIFIED Requirements

### Requirement: A wait drives no settle where the terminal keeps watch

While a caller keeps watch over the host, a wait SHALL settle nothing
and read only the host's state. While none does, it SHALL settle every
place before each reading, as every acting command does, whatever any
terminal nudges (119, 122, 207; core.13).

#### Scenario: Under a watch

- **WHEN** a wait follows a place while a caller keeps watch
- **THEN** the wait makes no call to any seam but the store, and the place moves on only by the caller's settles

#### Scenario: Attended under a nudge

- **WHEN** a wait follows a place while no caller keeps watch and the fake terminal nudges
- **THEN** the wait settles every place before each reading (119, 207)

### Requirement: The printed command delivers nothing

Where the terminal has no pane to deliver to, no notice SHALL be sent,
every end SHALL stay pending until a wait answers it or the inbox
collects it, and status SHALL give it (43, C.5).

#### Scenario: Under the printed command

- **WHEN** a stage ends under a fake terminal that nudges nothing and refuses send
- **THEN** status gives the end pending for each recipient and the fake terminal records no send
