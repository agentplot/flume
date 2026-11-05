## MODIFIED Requirements

### Requirement: The terminal seam's open, tidy and show

The terminal seam's open SHALL take where the agent runs, beside a
pane, in a session by its name or in the terminal's own, and SHALL
answer, beside opened and refused, a start held at a question
its program asks before it starts and a program that did not come up
within its bound, each with the screen (94, 107); it SHALL tidy what it
made for a place gone (138) and show a place's clues (139); and every
terminal operation SHALL answer within a bound the profile states (98).

#### Scenario: Held, tidied and shown

- **WHEN** the fake terminal holds a start at a question, makes a place's tab in its base's workspace, and has a person's pane in that workspace
- **THEN** open answers held with the screen, tidy closes the place's tab and keeps the workspace naming the person's pane, and show takes the place's clues (94, 138, 139)

#### Scenario: Where an agent opens

- **WHEN** the fake terminal is asked to open one agent beside the pane `operator`, one in the session `work` and one in its own
- **THEN** its log names each where it was asked, and the open's answer is the same for each (94, 107)

## ADDED Requirements

### Requirement: The host's facts give sessions, labels and the layers

The host's facts SHALL give, as its configuration stands when asked,
each stream's `session` and its labels' shares, each with its
`reserve`, `keep_free` and any other key its table holds; the label
the configuration gives a repository's kind or a kind, with where from;
and every value of the configuration with its layer, whether its local
layer was taken, and each key passed over with its pin (143, 180, 181,
C.7).

#### Scenario: The fake host's sessions, labels and layers

- **WHEN** a test scripts the fake host's `work` with the session `office` and `operation`'s reserve of two keeping one free, `[kinds.research] label = "inception"`, and `streams.work.agents` from the local layer
- **THEN** the core reads `office` for `work`, `operation`'s share, `inception` from `host-kind` for research in any repository, and `streams.work.agents` from `local` (143, 180, 181, C.7)

### Requirement: The types of a session, a label and the layers

The seams SHALL pass one type each for where an agent opens, a session
and where it came from, a label and where it came from, a label's
share of its stream, and the configuration's values with their layers;
each SHALL serialize to JSON and back unchanged, and a held state, a
kind or settings written before them SHALL read with none (107, 143,
180, 181, C.7).

#### Scenario: A held state with a session and a label

- **WHEN** a held state whose kind declares a label, whose launch's settings name a session, and whose live start holds its session and label, is written and read
- **THEN** it reads back unchanged, and one written before reads with no label and no session (107, 180)
