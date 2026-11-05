## MODIFIED Requirements

### Requirement: Readiness that fails

When a place's readiness fails, its first stage SHALL wait and status
SHALL name the failed step and its words, unless the kind starts its
first stage in a place that could not be made ready, when the stage
SHALL start with the step and its words in its prompt; a retry of that
first stage SHALL ready the place again at the revision its launch
read (12, 20).

#### Scenario: A change's place waits, a fix's starts

- **WHEN** the fake place's readiness fails at its first step for a change's launch and for a fix's launch
- **THEN** the change's construct stage waits on readiness with the step and words in status, and the fix's code stage starts with them in its prompt (S16)

#### Scenario: A retry readies at the launch's revision

- **WHEN** a change's readiness failed at its launch from main at R, main's revision moves on, the fake place's readiness is mended, and construct is retried
- **THEN** the fake place is readied at R, and construct starts (12, 20)

#### Scenario: Each step named as it passed

- **WHEN** the fake place's readiness passes `toolchain`, `proto install`, `moon --version` and `moon setup` for a change's launch
- **THEN** construct starts, and status names each step passed, in that order (S89, 12)
