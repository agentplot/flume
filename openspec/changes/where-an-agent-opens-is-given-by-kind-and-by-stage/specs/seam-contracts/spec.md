## MODIFIED Requirements

### Requirement: The host's facts give sessions, labels and the layers

The host's facts SHALL give, as its configuration stands when asked,
each stream's `session`, the session it names for each kind and each
kind's stage with any other key their tables hold, and its labels' shares, each with its
`reserve`, `keep_free` and any other key its table holds; the label the
configuration gives a repository's kind or a kind, with where from; and
every value of the configuration with its layer, whether its local layer
was taken, and each key passed over with its pin (143, 180, 181, 199,
C.7).

#### Scenario: The fake host's sessions, labels and layers

- **WHEN** a test scripts the fake host's `work` with the session `office` and `operation`'s reserve of two keeping one free, `[kinds.research] label = "inception"`, and `streams.work.agents` from the local layer
- **THEN** the core reads `office` for `work`, `operation`'s share, `inception` from `host-kind` for research in any repository, and `streams.work.agents` from `local` (143, 180, 181, C.7)

#### Scenario: The fake host's sessions for its kinds and stages

- **WHEN** a test scripts the fake host's `work` naming `work-design` for research and for release's review, and `work-plan` for plan with the other key `label`
- **THEN** the core reads `work-design` for research, `work-design` for release's review, and `work-plan` for plan with `label` named as its table's other key (199, C.7)

### Requirement: The types of a session, a label and the layers

The seams SHALL pass one type each for where an agent opens, a session
and where it came from, `launch`, `stream-stage`, `stream-kind`,
`stream`, `stream-name` or `own`, the sessions a stream names for its
kinds and stages, a label and where it came from, a label's share of its stream, and the
configuration's values with their layers; each SHALL serialize to JSON
and back unchanged, and a held state, a kind, a stream or settings
written before them SHALL read with none (107, 143, 180, 181, 199, C.7).

#### Scenario: A held state with a session and a label

- **WHEN** a held state whose kind declares a label, whose launch's settings name a session, and whose live start holds its session and label, is written and read
- **THEN** it reads back unchanged, and one written before reads with no label and no session (107, 180)

#### Scenario: A start's session from its stream's word for its stage

- **WHEN** a held state whose live start holds the session `work-review` from `stream-stage`, and a stream naming `work-design` for research and `work-review` for release's review, are written and read
- **THEN** each reads back unchanged, and a stream written before reads with no session for any kind or stage (199)
