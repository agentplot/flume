## MODIFIED Requirements

### Requirement: The host's facts give sessions, labels and the layers

The host's facts SHALL give, as its configuration stands when asked,
each stream's `session`, the sessions it names for its kinds and their
stages, and its labels' shares with their `owed` and `reserved`,
each table with any other key it holds; the label the configuration
gives a repository's kind or a kind, with where from; and every value
of the configuration with its layer, whether its local layer was taken,
and each key passed over with its pin (143, 180, 181, 199, C.7).

#### Scenario: The fake host's sessions, labels and layers

- **WHEN** a test scripts the fake host's `work` with the session `office` and `operation` owed two, one of them reserved, `[kinds.research] label = "inception"`, and `streams.work.agents` from the local layer
- **THEN** the core reads `office` for `work`, `operation`'s share, `inception` from `host-kind` for research in any repository, and `streams.work.agents` from `local` (143, 180, 181, C.7)

#### Scenario: The fake host's sessions for its kinds and stages

- **WHEN** a test scripts the fake host's `work` naming `work-design` for research and for release's review, and `work-plan` for plan with the other key `label`
- **THEN** the core reads `work-design` for research, `work-design` for release's review, and `work-plan` for plan with `label` named as its table's other key (199, C.7)
