## MODIFIED Requirements

### Requirement: The host's facts give sessions, labels and the layers

The host's facts SHALL give, as its configuration stands when asked,
each stream's `session`, the sessions it names for its kinds and their
stages, and its labels' shares with their `reserve` and `keep_free`,
each table with any other key it holds; the label the configuration
gives a repository's kind or a kind, with where from; and every value
of the configuration with its layer, whether its local layer was taken,
and each key passed over with its pin (143, 180, 181, 199, C.7).

#### Scenario: The fake host's sessions, labels and layers

- **WHEN** a test scripts the fake host's `work` with the session `office` and `operation`'s reserve of two keeping one free, `[kinds.research] label = "inception"`, and `streams.work.agents` from the local layer
- **THEN** the core reads `office` for `work`, `operation`'s share, `inception` from `host-kind` for research in any repository, and `streams.work.agents` from `local` (143, 180, 181, C.7)

#### Scenario: The fake host's sessions for its kinds and stages

- **WHEN** a test scripts the fake host's `work` naming `work-design` for research and for release's review, and `work-plan` for plan with the other key `label`
- **THEN** the core reads `work-design` for research, `work-design` for release's review, and `work-plan` for plan with `label` named as its table's other key (199, C.7)

### Requirement: The types of a session, a label and the layers

The seams SHALL pass one type each for where an agent opens, a session
and where it came from, `launch`, `stream-stage`, `stream-kind`,
`stream`, `stream-name` or `own`, a stream's sessions for its kinds and
stages, a label, where it came from and its share of its stream, and
the configuration's values with their layers; each SHALL serialize to
JSON and back unchanged, and a held state, a kind, a stream or settings
written before them SHALL read with none (107, 143, 180, 181, 199, C.7).

#### Scenario: A held state with a session and a label

- **WHEN** a held state whose kind declares a label, whose launch's settings name a session, and whose live start holds its session and label, is written and read
- **THEN** it reads back unchanged, and one written before reads with no label and no session (107, 180)

#### Scenario: A start's session from its stream's word for its stage

- **WHEN** a held state whose live start holds the session `work-review` from `stream-stage`, and a stream naming `work-design` for research and `work-review` for release's review, are written and read
- **THEN** each reads back unchanged, and a stream written before reads with no session for any kind or stage (199)

### Requirement: The terminal seam's runs

The terminal seam SHALL offer runs: given one or more parts, each by
its name or the terminal's own, read together in one reading of the
terminal, for each in the order asked, that it runs or does not, with
the name the terminal knows it by; or that the terminal holds no part a
person can be shown, or that it could not read the part, in the
terminal's words; starting nothing, within a bound the profile states.
No terminal SHALL take a default answer for it (98, 200, 201; core.2).

#### Scenario: Runs over the fake

- **WHEN** the fake terminal is scripted to read `work` running and `desk` unread with `no listing`, and is asked runs of `work`, `desk`, `office` and its own part, one at a time
- **THEN** it answers `work` running, `desk` unread with `no listing`, `office` not running and its own part not running as `own`, logs `runs work`, `runs desk`, `runs office` and `runs own`, and opens nothing (201)

#### Scenario: Runs of several parts over the fake

- **WHEN** the same fake terminal is asked runs of `work`, `desk`, `office` and its own part in one call
- **THEN** it answers each as it answered alone, in that order, and logs the one `runs work desk office own` (201)
