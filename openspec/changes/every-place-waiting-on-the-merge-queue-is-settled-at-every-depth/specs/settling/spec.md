## ADDED Requirements

### Requirement: The watch's settle moves every place, an agent in it or none

A settle of every place, as a terminal's watch runs it, SHALL settle
each place the store holds whether or not an agent is in it, and SHALL
drive one round of each open submission's base once per command where
the merge queue moves only while something drives it, a base place's
onto its base and the places on its branch onto it alike, so a
submitted place moves on within the watch's bound with no agent and no
caller (46, 120, 122, 124, 125, I8, S109).

#### Scenario: A submitted chore with no agent

- **WHEN** under a fake terminal with a watch a chore's code stage delivers, its agent ends, and the place is submitted, and the fake merge queue lands it only after three rounds
- **THEN** each settle of every place after the watch's bound logs one `step main` of the fake merge queue's, with no agent of the place live and no caller's act; after the third the next settle reads the landing, delivers it and tidies the place (46, 122, 124, S109)

#### Scenario: A queue that moves on its own

- **WHEN** the same chore is submitted to a fake merge queue whose `main` moves on its own, as beside a live driver, and it lands there
- **THEN** each settle's step answers that the queue moves on its own, nothing else of the settle changes, and the next settle after the landing tidies the place (124, 125; sluice 135)

#### Scenario: Two depths in one settle

- **WHEN** a base place's submission to `main` and a change's submission to the base place's branch are open together, and every place is settled once
- **THEN** the fake merge queue logs one `step main` and one `step <the base place's branch>`, each once in the command (124, 125, 202)
