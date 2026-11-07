## ADDED Requirements

### Requirement: An end for a standing address goes into the agent found by its name

At each settle that reads an end pending for a standing address, the
terminal of the entry's stream SHALL be asked for the session's agents,
and the end sent through it as a notice to the one agent of the entry's
name while it asks no question; none, more than one, or a part not read
SHALL leave it pending; nothing of where the agent was SHALL be kept
(43, 132, 190, C.7; core.11, core.28).

#### Scenario: A standing agent waiting

- **WHEN** a fix names `madswan/planner` as its recipient, the entry gives `agent = "madswan-planner"` and `session = "madswan"`, the fake terminal holds that agent waiting in `madswan`, and the fix's code stage delivers
- **THEN** the agent receives one notice naming the place and the end, the end is recorded delivered to `madswan/planner`, and no place is made (43, 132)

#### Scenario: A standing agent asking

- **WHEN** the standing agent is asking when the end is read, and later reads waiting
- **THEN** no notice is sent while it asks, the end stays pending, and the first settle that reads it waiting delivers it once (43; core.11)

#### Scenario: No agent of the name

- **WHEN** the fake terminal holds no agent named `madswan-planner` in `madswan` when the end is read
- **THEN** the end stays pending, nothing is claimed, and the settle goes on with the place's other steps (132)

#### Scenario: Two agents of the name

- **WHEN** the fake terminal holds two agents named `madswan-planner` in `madswan`
- **THEN** neither receives a notice, and the end stays pending (132)

#### Scenario: An agent revived under its name

- **WHEN** the standing agent takes one end, is removed, and another agent named `madswan-planner` is added in `madswan` under a new session key before the next end is read
- **THEN** the next notice reaches the new agent, and the removed one receives no second notice (132)

#### Scenario: The session from the entry's stream

- **WHEN** the entry gives `agent = "w-planner"` and `stream = "w"`, the stream `w` names `session = "w-main"`, and the fake terminal holds `w-planner` in `w-main`
- **THEN** the notice reaches it, asked of `w`'s terminal in `w-main` (143, 190, C.7)

#### Scenario: The default stream's own session

- **WHEN** the entry gives only `agent = "w-planner"`, the default stream names no session, and the fake terminal holds `w-planner` in its own part
- **THEN** the notice reaches it, asked of the terminal's own part (107, 143, C.7)

#### Scenario: A stream the host does not declare

- **WHEN** the entry gives `stream = "gone"`, which the host does not declare
- **THEN** no terminal is asked, and the end stays pending (143, C.7)

#### Scenario: The inbox collects for a standing address

- **WHEN** an end is pending for `madswan/planner`, whose agent is away, and `flume inbox --recipient madswan/planner` runs
- **THEN** it collects the end as for any recipient, marked delivered to the address, and the next settle sends no notice of it (43)

#### Scenario: A standing agent under herdr through the binary

- **WHEN** under the pinned herdr and the stand-in, a fix names `w/planner` as its recipient, the host's configuration gives it `agent = "w-planner"` and `session = "w"`, and a stand-in runs in `w` under the herdr name `w-planner`
- **THEN** the fix's end reaches that pane as one notice, and no place of a planner is made (43, 132, C.5)
