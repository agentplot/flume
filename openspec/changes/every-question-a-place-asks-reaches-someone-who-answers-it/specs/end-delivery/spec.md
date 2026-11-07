## MODIFIED Requirements

### Requirement: The inbox collects what is pending

`flume inbox` SHALL give every end pending for a recipient, of every
place on the host, with what 43 names and its key, and SHALL record each
delivered to that recipient. Status, a wait's read and an inbox for
another recipient SHALL mark nothing delivered (43, core.25).

#### Scenario: Collected after a restart

- **WHEN** two ends of two places are pending for `ops` while its session is gone, and its restarted session runs `flume inbox` as `ops`
- **THEN** the answer holds both ends whole, each with its key, both read delivered to `ops`, and a second `flume inbox` gives nothing

### Requirement: An end for a standing address goes into the agent found by its name

At each settle that reads an end pending for a standing address, the
terminal of its address's stream SHALL be asked for the session's
agents, and the end sent through it as a notice to the one agent of the
entry's name while it asks no question; none, more than one, or a part
not read SHALL leave it pending; nothing of where the agent was SHALL be
kept (43, 132, 190, 193, C.7; core.11, core.28).

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

- **WHEN** the entry gives `w/planner` `agent = "w-planner"` and no session, the stream `w` names `session = "w-main"`, and the fake terminal holds `w-planner` in `w-main`
- **THEN** the notice reaches it, asked of `w`'s terminal in `w-main` (143, 190, 193)

#### Scenario: The default stream's own session

- **WHEN** the entry gives `default/planner` only `agent = "w-planner"`, the default stream names no session, and the fake terminal holds `w-planner` in its own part
- **THEN** the notice reaches it, asked of the terminal's own part (107, 143, 193)

#### Scenario: A stream the host does not declare

- **WHEN** the entry gives `gone/planner` an agent, and the host declares no stream `gone`
- **THEN** no terminal is asked, and the end stays pending (143, 193)

#### Scenario: The inbox collects for a standing address

- **WHEN** an end is pending for `madswan/planner`, whose agent is away, and `flume inbox --recipient madswan/planner` runs
- **THEN** it collects the end as for any recipient, marked delivered to the address, and the next settle sends no notice of it (43)

#### Scenario: A standing agent under herdr through the binary

- **WHEN** under the pinned herdr and the stand-in, a fix names `w/planner` as its recipient, a scratch source's roles file gives it `agent = "w-planner"` and `session = "w"`, the host declares the stream `w`, and a stand-in runs in `w` under the herdr name `w-planner`
- **THEN** the fix's end reaches that pane as one notice, and no place of a planner is made (43, 132, 193, C.5)

## ADDED Requirements

### Requirement: The inbox gives what waits on its recipient

`flume inbox` SHALL also give, as `waiting`, every place that waits on
its caller and that the recipient is a recipient of, was named by the
stop-short the place waits on, or was given a stall of as the answerer,
delivered or not, with its kind, name, stream and brief's first line,
the move and since when, a stop-short's words, needs, key and the
address it named, and a hold's words, since and bound. It SHALL mark
nothing for them (43, 130, 193; core.25).

#### Scenario: A question collected, then a restart

- **WHEN** `ops` collected a construct's stop-short by `flume inbox`, did nothing with it, and its restarted session runs `flume inbox` again
- **THEN** `ends` is empty, and `waiting` gives the place with the retry of construct as its move, since the stop-short, the agent's words and the key `stage/construct/1/ended-short`

#### Scenario: A question addressed to a role

- **WHEN** a place whose recipient is `east/planner` waits on a stop-short that named `east/ops`, and `east/ops`'s run runs `flume inbox`
- **THEN** `waiting` gives that place, naming `east/ops` as the stop-short's addressee

#### Scenario: The answerer's waiting places

- **WHEN** a pane-launched place stalled and its stall was delivered to the answerer `east/operator`, and the answerer's run runs `flume inbox`
- **THEN** `waiting` gives that place, though `east/operator` is not among its recipients (193)

#### Scenario: A held question

- **WHEN** a recipient holds the waiting move with words and a bound of one hour
- **THEN** `waiting` gives the place with the hold's words, since when and its bound, and the next `flume inbox` gives it the same

### Requirement: A notice names the place's brief and the end's key

A notice SHALL stay one line, and SHALL name the place, the first line
of its brief cut at 60 characters, the end, its key, and the command
that reads it whole: `flume inbox` for an end the place waits on a move
for, and `flume status <place>` for any other (43; core.11, core.25).

#### Scenario: A stop-short's notice

- **WHEN** a change whose brief begins "Every question a place asks reaches someone who answers it" ends short needing an answer at construct, and the settle delivers it to a pane
- **THEN** the fake terminal records one line naming the place, "Every question a place asks reaches someone who answers it" cut at 60 characters, "construct needs an answer", `stage/construct/1/ended-short` and `flume inbox` (S94)

#### Scenario: A delivery's notice

- **WHEN** the same place's construct delivers instead
- **THEN** the notice names `flume status` and the delivery's key
