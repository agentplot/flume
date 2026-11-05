## ADDED Requirements

### Requirement: The answerer behind a stream

The host's facts SHALL give, for a stream, the role its sources declare
to stand behind it, with the source that declares it; none where no
role stands behind it; and, where two do, none, naming both. They SHALL
read the sources as they stand when asked and keep nothing between asks
(143, 191, 193, C.7).

#### Scenario: Each partition's door

- **WHEN** the scratch source `acme`'s roles file declares `east/operator` with `stream = "east"` and `answerer = true`, and `west/operator` likewise for `west`
- **THEN** the host names `east/operator` for `east` and `west/operator` for `west`, each from `acme`, and none for `default`

#### Scenario: No answerer

- **WHEN** no source declares a role standing behind any stream
- **THEN** the host names none, for every stream

#### Scenario: Two behind one stream

- **WHEN** the sources `acme` and `beta` each declare a role standing behind `east`
- **THEN** the host names none for `east`, naming both roles and their sources

### Requirement: The roles a partition answers with

The host's facts SHALL give, for a partition, every address of it that
a source declares with `answers`, a run's or a carried one's, each with
its words, as the sources stand when asked (28, 41, 132, 193).

#### Scenario: A partition's roles

- **WHEN** `acme`'s roles file declares runs for `east/planner` and `east/ops`, each with its `answers`, carries `east/design` with its `answers`, declares `east/reviewer` with no `answers`, and declares `west/ops`
- **THEN** the host gives `east`'s roles as `east/design`, `east/ops` and `east/planner`, each with its words, and neither `east/reviewer` nor `west/ops`
