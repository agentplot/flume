## ADDED Requirements

### Requirement: Status gives where a place's base was read

Status SHALL give, for each place, where its launch read the base's
revision: `merge-queue`, `origin`, `no-origin` or `origin-lacks`, as
`base_from` in the JSON form and beside the base in the person's form,
and none for a place launched before flume said (56, C.1).

#### Scenario: Where the base was read

- **WHEN** status is asked of a place cut at origin's base, and of one cut at the host's own where origin holds none
- **THEN** the JSON form gives `base_from` `origin` and `origin-lacks`, and the person's form `from main (at origin)` and `from trial (the host's own, origin holding none)` (56, C.1)
