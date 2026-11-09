## MODIFIED Requirements

### Requirement: SessionStart gives a caller's session what waits on it

In a session flume did not start, SessionStart SHALL run `flume inbox`
as the session the terminal names, never as a role's address,
confirming on receipt each end it is handed, and SHALL give the agent,
as added context, each end and each waiting place it answers with the
key, whether its agent is kept open and until when, and how to answer,
hold or leave it; or nothing where it answers nothing, no session is
named, or flume does not answer (43, C.4; weir 103).

#### Scenario: A restarted pane

- **WHEN** a session flume did not start begins in the pane `ops`, a place whose recipient is `ops` waits on a retry of construct with a stop-short handed over before, and SessionStart runs
- **THEN** the hook's output gives the agent the place, the stop-short's words, its key and the three ways on, the stop-short reads handed to `ops` by `session_start` and confirmed by it, and the hook exits 0 (S94)

#### Scenario: Nothing waits

- **WHEN** the same hook runs for a session nothing is pending for and nothing waits on
- **THEN** it gives no output and exits 0

#### Scenario: An address in the environment

- **WHEN** SessionStart runs in a session flume did not start with `FLUME_RECIPIENT=east/planner`, and an end stands pending for `east/planner`
- **THEN** the hook collects as the session the terminal names, and the end stays pending for `east/planner`, handed to it by nothing

#### Scenario: flume unreachable

- **WHEN** the configuration cannot be read and SessionStart runs in a session flume did not start
- **THEN** the hook gives no output and exits 0
