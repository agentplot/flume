## MODIFIED Requirements

### Requirement: A notice names the place's brief and the end's key

A notice SHALL stay one line, the end's message as its delivery
recorded it followed by the way its recipient confirms it, and SHALL
name the place, the first line of its brief cut at 60 characters, the
end, its key, the command that reads it whole, `flume inbox` for an end
the place waits on a move for and `flume status <place>` for any other,
and `flume confirm --recipient <recipient>` with the delivery's key (43;
core.11, core.25).

#### Scenario: A stop-short's notice

- **WHEN** a change whose brief begins "Every question a place asks reaches someone who answers it" ends short needing an answer at construct, and the settle hands it to a pane
- **THEN** the fake terminal records one line naming the place, "Every question a place asks reaches someone who answers it" cut at 60 characters, "construct needs an answer", `stage/construct/1/ended-short`, `flume inbox` and `flume confirm --recipient ops` with the end's delivery key (S94)

#### Scenario: A delivery's notice

- **WHEN** the same place's construct delivers instead
- **THEN** the notice names `flume status`, the delivery's key and `flume confirm --recipient ops` with its delivery key

## ADDED Requirements

### Requirement: A notice confirms as its recipient

A notice's `flume confirm` SHALL name the recipient it is sent to,
since a caller confirms as its own name where it names none, and an
agent in a pane runs under the pane's. The delivery's message, every
reader's, SHALL hold all of the notice but its way to confirm, and a
message an earlier flume recorded with `flume confirm <key>` in it SHALL
be sent with that taken off and the recipient's in its stead (43; weir
100, 102; core.25).

#### Scenario: A notice into a standing agent

- **WHEN** a fix for `madswan/planner`, which the host gives the standing agent `madswan-planner` in the session `madswan`, delivers, and the settle hands its end to that agent
- **THEN** the notice names `flume confirm --recipient madswan/planner` with the end's delivery key; a confirm of that key as the agent's own name in its pane is refused, and the command the notice names delivers the end (43, 132; weir 102)
