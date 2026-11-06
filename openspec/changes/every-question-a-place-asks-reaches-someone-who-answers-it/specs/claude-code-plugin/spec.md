## MODIFIED Requirements

### Requirement: The plugin's hooks

flume's Claude Code plugin SHALL hold SessionStart, UserPromptSubmit,
PreToolUse, PostToolUse, Stop and SessionEnd hooks, and StopFailure and
the permission-request or permission-prompt notification hook where the
pinned Claude Code has them, each running `flume hook <event>` by the
binary's absolute path; every hook SHALL exit 0 and never hold up a
start or a turn, and each but SessionStart SHALL do nothing in a session
flume did not start (C.4, section 9; record 0016).

#### Scenario: A session the operator started

- **WHEN** each hook but SessionStart runs with no `FLUME_AGENT` in its environment
- **THEN** each allows what it was asked and writes nothing

## ADDED Requirements

### Requirement: SessionStart gives a caller's session what waits on it

In a session flume did not start, SessionStart SHALL run `flume inbox`
as the session the terminal names, never as a role's address, and SHALL
give the agent, as added context, each end and each waiting place it
answers with the key, whether its agent is kept open and until when,
and how to answer, hold or leave it; or nothing
where it answers nothing, no session is named, or flume does not answer
(43, C.4).

#### Scenario: A restarted pane

- **WHEN** a session flume did not start begins in the pane `ops`, a place whose recipient is `ops` waits on a retry of construct with a stop-short delivered before, and SessionStart runs
- **THEN** the hook's output gives the agent the place, the stop-short's words, its key and the three ways on, and exits 0 (S94)

#### Scenario: Nothing waits

- **WHEN** the same hook runs for a session nothing is pending for and nothing waits on
- **THEN** it gives no output and exits 0

#### Scenario: An address in the environment

- **WHEN** SessionStart runs in a session flume did not start with `FLUME_RECIPIENT=east/planner`, and an end stands pending for `east/planner`
- **THEN** the hook collects as the session the terminal names, and the end stays pending for `east/planner`

#### Scenario: flume unreachable

- **WHEN** the configuration cannot be read and SessionStart runs in a session flume did not start
- **THEN** the hook gives no output and exits 0
