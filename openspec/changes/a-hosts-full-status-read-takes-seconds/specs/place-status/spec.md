## MODIFIED Requirements

### Requirement: Status reads where each fact lives and writes nothing

Status SHALL read every fact of a deliverable, a change or a submission
from its seam when asked, at one head per place, SHALL keep none of
them between asks, and SHALL start no agent, send nothing and write
nothing (57, 58). It SHALL read its places side by side, ask each
repository's merge queue once for all the places of that repository it
gives, and read each live agent once, for its place and its base's
count (58, 99, 139, C.6).

#### Scenario: A change ticked between two asks

- **WHEN** status is asked, the fake change then ticks a task at a new head, and status is asked again
- **THEN** the second answer gives the new count, and neither ask wrote to the fake store or called the fake terminal's open or send

#### Scenario: Three places of one repository

- **WHEN** status is asked of every place while three fixes of one repository each run a live agent, and a hand has submitted the second's branch
- **THEN** the fake merge queue is read once, naming the three branches, the second place gives the hand's submission and the others none, and the fake terminal reads each agent's state once (58, 99, 139)
