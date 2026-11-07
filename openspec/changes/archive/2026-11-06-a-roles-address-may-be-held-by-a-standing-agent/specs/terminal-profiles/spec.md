## ADDED Requirements

### Requirement: herdr gives a session's agents by name

The herdr profile SHALL find the session's socket by one bounded
`session list --json`, starting none, a session not running holding no
agent; then read `agent list` there, each agent with a string `name` by
its `pane_id` and `agent_status`, the status read as state reads it, at
`herdr:<pane_id>@<socket>`; and answer a call refused or unanswered as
not read, in herdr's words (132, C.5; core.29).

#### Scenario: A standing agent found and reached

- **WHEN** under the pinned herdr on a scratch server, a stand-in is started in the session `w` by `herdr agent start w-planner`, and the profile is asked for `w`'s agents
- **THEN** it lists `w-planner` waiting at `herdr:<pane>@<socket>`, and `send_to` that name delivers a notice the stand-in receives (95, 132, C.5)

#### Scenario: A session that does not run

- **WHEN** the profile is asked for the agents of `x`, which `herdr session list --json` does not list running
- **THEN** it lists none, and `x` is still not listed running (132; core.13)

#### Scenario: An agent with no name

- **WHEN** a session holds an agent herdr lists with no name beside `w-planner`
- **THEN** only `w-planner` is listed (C.5)

#### Scenario: The agent gone

- **WHEN** the stand-in under `w-planner` is closed and the profile is asked again
- **THEN** it lists no `w-planner` (132)

### Requirement: The printed command finds no named agent

The printed command SHALL answer a part's agents as not read, saying it
holds no agent a name finds, and run nothing (132, C.5).

#### Scenario: Agents under the printed command

- **WHEN** the printed command is asked for the agents of `madswan` and of its own part
- **THEN** it answers each not read, with those words, and runs nothing (132)
