## MODIFIED Requirements

### Requirement: The terminal's clues follow each stage

The core SHALL call the terminal's `show` at each settle that changes
the place's current stage's state, the short form of its line, or the
count of the places of its repository on its base, with the stage, its
state, that short form, read from the facts the settle read, and that
count of places running, asking, waiting on their caller and stalled;
a show that fails SHALL be logged on the host and fail nothing, and
nothing the core decides SHALL read a clue back (139; record 0018).

#### Scenario: Titles follow a stage

- **WHEN** a code stage's agent starts, waits, works and asks, a settle between each
- **THEN** show is called once at each, its state starting, waiting, working and asking, and a show that fails is logged while the settle goes on (S49)

#### Scenario: The line alone changes

- **WHEN** a change place's code stage keeps working while the change at its head goes from 2 tasks ticked of 5 to 3, a settle running at each
- **THEN** show is called once more, with the line's short form ending `tasks 3/5 · working`, and not again at a settle that changes nothing (139)
