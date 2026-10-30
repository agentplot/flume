## ADDED Requirements

### Requirement: A role's run names its stream

A role's `[roles]` table SHALL take an optional `stream`, given to its
woken run's launch as the launch's stream, so the run is placed and
refused as any launch naming that stream is (132, 143, C.7).

#### Scenario: A role in its stream

- **WHEN** the host's configuration names a role's run with `stream = "work"` and an end pending for its address wakes it
- **THEN** the woken place runs in `work`, from `launch` (132, 143)
