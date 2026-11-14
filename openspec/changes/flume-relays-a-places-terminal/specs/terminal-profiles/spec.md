## ADDED Requirements

### Requirement: herdr relays an agent's pane

The herdr profile's relay SHALL find the agent's pane as its other
operations do, refusing where herdr holds none of the agent. Its handle
SHALL state 250 milliseconds, read by `pane read <pane> --source
visible --format ansi`, type text by `pane send-text <pane> <text>` and
keys by one `pane send-keys <pane> <key>…` as herdr 0.9.5 names them,
each call with `HERDR_SOCKET_PATH` the pane's server and the caller's
`HERDR_*` stripped (107, 221, 222, C.5; core.29).

#### Scenario: Reads and types over a stand-in herdr

- **WHEN** the profile relays an agent whose pane is `w1:p2` at the socket `S`, over a stand-in `herdr` that logs its arguments and environment, reads twice, types the text `yes`, then the keys `ctrl+c` and `enter`
- **THEN** the handle states 250 milliseconds, and the stand-in ran `pane read w1:p2 --source visible --format ansi` twice, `pane send-text w1:p2 yes` and `pane send-keys w1:p2 ctrl+c enter`, each with `HERDR_SOCKET_PATH=S` (222, C.5)

#### Scenario: The terminal's scenarios against the pinned herdr

- **WHEN** the terminal seam's scenarios run against the herdr profile and the pinned herdr, its harness saying it relays
- **THEN** each passes, the relay's among them (221, 222, C.5)

#### Scenario: Every key name against the pinned herdr

- **WHEN** each of flume's key names, bare and led by `ctrl+`, `alt+` and `shift+` where the name allows, is typed by the profile's relay into a pane of the pinned herdr running `cat`
- **THEN** herdr takes each, refusing none (222; core.29)

### Requirement: herdr's relay finds a moved pane again and reads a gone one gone

The herdr profile's relay handle SHALL, where a call answers
`pane_not_found`, find the pane again by the agent's herdr name, as its
other operations do, and read or type there, and SHALL read gone where
the pane is not found again or herdr answers `server_not_running`
(222, C.5; record 0018).

#### Scenario: A pane moved, then gone

- **WHEN** the stand-in `herdr` answers the relay's read of `w1:p2` with `pane_not_found` and `agent get` with the pane `w3:p1`, then answers a read of `w3:p1` with `pane_not_found` and `agent get` with `agent_not_found`
- **THEN** the first read gives the screen read at `w3:p1`, and the second reads gone (222)

#### Scenario: A server not running

- **WHEN** the stand-in `herdr` answers the relay's read with `server_not_running`
- **THEN** the read is gone (222)

### Requirement: herdr's relay calls are bounded at one second

Each call of the herdr profile's relay handle SHALL be bounded at one
second, a read past it unread and an input past it refused, each naming
the bound (98, 222; core.29).

#### Scenario: A read that does not answer

- **WHEN** the stand-in `herdr` sleeps five seconds before answering a relay's read
- **THEN** the read is unread naming one second, within two seconds of being asked (98)

### Requirement: The printed command refuses relay

The printed command's relay SHALL be refused, saying it holds no part of
a terminal a person can be shown, with nothing run (222, C.5).

#### Scenario: A relay under the printed command

- **WHEN** the printed command's profile is asked to relay an agent it printed
- **THEN** it is refused saying it holds no part of a terminal a person can be shown, and nothing is run or written (222, C.5)

### Requirement: Rex refuses relay

The Rex profile's relay SHALL be refused before any call, saying
flume's Rex profile offers no relay, since C.5 gives Rex no relay row
(222, C.5; core.19).

#### Scenario: A relay under Rex

- **WHEN** the Rex profile is asked to relay an agent
- **THEN** it is refused saying flume's Rex profile offers no relay, and no Rex call is made (222)

#### Scenario: The terminal's scenarios against the pinned Rex

- **WHEN** the terminal seam's scenarios run against the Rex profile, its harness saying it offers no relay
- **THEN** each passes, the relay's refusal among them (222)
