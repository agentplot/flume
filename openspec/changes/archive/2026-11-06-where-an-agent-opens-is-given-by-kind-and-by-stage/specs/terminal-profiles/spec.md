## MODIFIED Requirements

### Requirement: herdr's runs reads one session listing

The herdr profile's runs SHALL read one `herdr session list --json` for
every part asked, with the caller's `HERDR_*` stripped, and find each
session by its `name`, its own being `flume`: listed running, it runs;
listed not running or not listed, it does not. It SHALL start no server
(201, C.5; core.13).

#### Scenario: A session not listed stays so

- **WHEN** the profile, under a scratch session home, is asked runs of `w`, which `herdr session list --json` does not list
- **THEN** it answers `w` not running, and `herdr session list --json` still lists no `w` (201)

#### Scenario: A running session and the own session

- **WHEN** the profile has started its own session `flume` and the session `w` by opening an agent in each, and is asked runs of `w` and of its own
- **THEN** it answers `w` running and its own running as `flume` (107, 201)

#### Scenario: Several sessions in one listing

- **WHEN** the profile, its herdr a stand-in of the test's own that logs each call and runs the pinned herdr, has started its own session `flume` and the session `w`, and is asked runs of `w`, of `x`, which no listing names, and of its own in one call
- **THEN** it answers `w` running, `x` not running and its own running as `flume`, in that order, the stand-in logging the one `session list --json`, and `x` is still not listed (200, 201)

### Requirement: The printed command's runs

The printed command's runs SHALL answer, for every part asked, that it
holds no part, saying it holds no session a person can be shown, since
its agents run in the commands it printed, and run nothing (187, 201).

#### Scenario: Runs under the printed command

- **WHEN** the printed command is asked runs of `work` and of its own part
- **THEN** it answers each that it holds no part, with those words, and runs nothing (201)

#### Scenario: Several parts under the printed command

- **WHEN** the printed command is asked runs of `work`, `work-design` and its own part in one call
- **THEN** it answers each of the three that it holds no part, with those words, in that order (201)
