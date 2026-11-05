## ADDED Requirements

### Requirement: The answerer behind a stream

The host's facts SHALL give, for a stream, the role its sources declare
to stand behind it, an address of that stream, with the source that
declares it; none where no role stands behind it; and, where two do,
none, naming both. They SHALL read the sources as they stand when asked
and keep nothing between asks (143, 191, 193, C.7).

#### Scenario: Each stream's door

- **WHEN** the scratch host declares the streams `east` and `west`, and the scratch source `acme`'s roles file declares `east/operator` with `answerer = true`, and `west/operator` likewise
- **THEN** the host names `east/operator` for `east` and `west/operator` for `west`, each from `acme`, and none for `default`

#### Scenario: No answerer

- **WHEN** no source declares a role standing behind any stream
- **THEN** the host names none, for every stream

#### Scenario: Two behind one stream

- **WHEN** the sources `acme` and `beta` each declare a role of `east` standing behind it, `east/operator` and `east/ops`
- **THEN** the host names none for `east`, naming both roles and their sources

### Requirement: The roles a host answers with

The host's facts SHALL give every address a source declares with
`answers`, a run's or a carried one's, each with its stream and its
words, as the sources stand when asked (28, 41, 132, 193).

#### Scenario: The host's roles

- **WHEN** `acme`'s roles file declares runs for `east/planner`, `east/ops` and `west/ops`, each with its `answers`, carries `north/design` with its `answers`, and declares `east/reviewer` with no `answers`
- **THEN** the host gives `east/ops` and `east/planner` for `east`, `west/ops` for `west` and `north/design` for `north`, each with its words, and not `east/reviewer`
