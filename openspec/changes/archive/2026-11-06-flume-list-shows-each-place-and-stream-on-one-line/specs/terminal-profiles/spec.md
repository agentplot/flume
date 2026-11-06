## ADDED Requirements

### Requirement: herdr's runs reads one session listing

The herdr profile's runs SHALL read one `herdr session list --json`,
with the caller's `HERDR_*` stripped, and find the session by its
`name`, its own being `flume`: listed running, it runs; listed not
running or not listed, it does not. It SHALL start no server (201,
C.5; core.13).

#### Scenario: A session not listed stays so

- **WHEN** the profile, under a scratch session home, is asked runs of `w`, which `herdr session list --json` does not list
- **THEN** it answers `w` not running, and `herdr session list --json` still lists no `w` (201)

#### Scenario: A running session and the own session

- **WHEN** the profile has started its own session `flume` and the session `w` by opening an agent in each, and is asked runs of `w` and of its own
- **THEN** it answers `w` running and its own running as `flume` (107, 201)

### Requirement: herdr's runs is bounded at one second

The herdr profile's runs SHALL bound its listing at one second, a
listing not read within it, refused or not parsed being unread in
herdr's words; and a profile whose own server is held by its socket
alone SHALL find its own part by `socket_path`, unread naming the
socket where none lists it (201, C.5; core.29).

#### Scenario: A herdr that does not answer

- **WHEN** the profile's herdr is a stand-in of the test's own that sleeps ten seconds before it answers, and the profile is asked runs of `w`
- **THEN** it answers unread within two seconds, saying herdr did not answer within its bound (201; core.29)

#### Scenario: An own server held by its socket

- **WHEN** a profile whose own server is held by the socket of the session `w` it started is asked runs of its own part, and then one held by a socket no session lists
- **THEN** the first answers running as `w`, and the second unread naming the socket (201)

### Requirement: The printed command's runs

The printed command's runs SHALL answer that it holds no part, saying
it holds no session a person can be shown, since its agents run in the
commands it printed, and run nothing (187, 201).

#### Scenario: Runs under the printed command

- **WHEN** the printed command is asked runs of `work` and of its own part
- **THEN** it answers each that it holds no part, with those words, and runs nothing (201)
