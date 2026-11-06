## ADDED Requirements

### Requirement: A caller keeps watch over the host

A settle of every place a caller asks with a bound SHALL record, in the
host's state and before it settles, when it began and the bound it
stated, written by compare-and-set as every host document is. A caller
SHALL keep watch while the newest such settle began less than twice its
bound ago, and SHALL decide nothing of what the settle does; no terminal
SHALL keep watch (46, 119, 120, 207; core.13, core.16).

#### Scenario: A watch kept, then lapsed

- **WHEN** a caller settles every place with a bound of two minutes, and the fake clock then passes three minutes, then two more
- **THEN** the host's watch holds when that settle began and two minutes; flume runs unattended at three minutes, and attended at five, with no other settle between (119, 207, S113)

#### Scenario: A terminal's nudge keeps no watch

- **WHEN** the fake terminal nudges on every agent's change and no caller has settled with a bound
- **THEN** flume runs attended, and a caller's drop settles every place first (119, 122, 207)

### Requirement: A watch's settle moves every place, an agent in it or none

A settle of every place, as a caller keeping watch runs it, SHALL
settle each place the store holds whether or not an agent is in it,
and SHALL drive one round of each open submission's base once per
command where the merge queue moves only while something drives it, a
base place's onto its base and the places on its branch onto it alike,
so a submitted place moves on within the watch's bound with no agent
and no other caller (46, 120, 124, 125, 207, I8, S109).

#### Scenario: A submitted chore with no agent

- **WHEN** a caller keeps watch with a bound of two minutes, a chore's code stage delivers, its agent ends, the place is submitted, and the fake merge queue lands it only after three rounds
- **THEN** each of the caller's settles logs one `step main` of the fake merge queue's, with no agent of the place live and no other caller's act; after the third the next settle reads the landing, delivers it and tidies the place (46, 124, 207, S109)

#### Scenario: A queue that moves on its own

- **WHEN** the same chore is submitted to a fake merge queue whose `main` moves on its own, as beside a live driver, and it lands there
- **THEN** each settle's step answers that the queue moves on its own, nothing else of the settle changes, and the next settle after the landing tidies the place (124, 125; sluice 135)

#### Scenario: Two depths in one settle

- **WHEN** a base place's submission to `main` and a change's submission to the base place's branch are open together, and every place is settled once
- **THEN** the fake merge queue logs one `step main` and one `step <the base place's branch>`, each once in the command (124, 125, 202)

## MODIFIED Requirements

### Requirement: Attended and watched

While a caller keeps watch over the host, its settles SHALL settle each
place on its stated bound, and a terminal's nudge MAY settle sooner.
While no caller keeps watch, whatever terminals the host binds, every
command of flume's that acts on places SHALL settle every place on the
host before it acts. Status and a prompt's rendering SHALL settle
nothing (119, 122, 190, 207).

#### Scenario: Killed with nobody watching

- **WHEN** no caller keeps watch, the fake terminal nudges nothing, a code stage's agent stops with nothing delivered, and later a caller drops another place
- **THEN** nothing reads the stage until the drop, which settles every place first, and status then reports the stage lost (S27)

#### Scenario: Crashed under a watch

- **WHEN** a caller keeps watch, the fake terminal nudges nothing, and a code stage's agent stops with nothing delivered and its on-quiet never runs
- **THEN** the caller's next settle reads the stage lost within its bound, with no other caller asking, and starts no agent (S28)

#### Scenario: A watched host with one stream unwatched

- **WHEN** a caller keeps watch, the host's own fake terminal nudges, its `trial` names a fake terminal that nudges nothing, and a caller drops a place on `work`
- **THEN** the drop settles nothing first, and `trial`'s places move at the caller's settles (119, 190, 207, S91)

#### Scenario: Crashed under a nudge

- **WHEN** a caller keeps watch, the fake terminal nudges, and a code stage's agent stops with nothing delivered and its on-quiet never runs
- **THEN** the nudge's settle reads the stage lost before the caller's next settle, and starts no agent (122, S28)
