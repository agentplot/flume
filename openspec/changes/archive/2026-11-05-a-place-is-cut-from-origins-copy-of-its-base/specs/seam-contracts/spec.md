## MODIFIED Requirements

### Requirement: The base, a change's location and a place's folder

The merge queue seam SHALL answer the revision the base stands at where
its profile says the base lives, after that profile's fetch, or a
refusal naming why, or that the merge queue lands no such base, which
the core then reads at origin (126; 9, C.1, core.7); the change seam
where a change lies in a place, for a prompt's change part (127); and
the store the place's own folder beside its held state, for a report
kept there (128, 123).

#### Scenario: The base where the merge queue says it lives

- **WHEN** the fake merge queue says the base lives at a revision other than the local base's
- **THEN** the place is cut at that revision

#### Scenario: No base revision readable

- **WHEN** the fake merge queue refuses to give the base's revision, naming a failed fetch
- **THEN** the launch is refused naming the fetch, and nothing is made

#### Scenario: A base the merge queue lands not

- **WHEN** the fake merge queue is asked for a base it lands nothing on
- **THEN** it answers that it lands no such base, and the place's own base at a revision (126)
