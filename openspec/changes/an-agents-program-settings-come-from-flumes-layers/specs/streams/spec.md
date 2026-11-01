## ADDED Requirements

### Requirement: A stream's agents take the host's settings for the stream

A place's agents SHALL take the settings `[agents.streams.<stream>]`
gives for its stream, read as it stands at each start, after the host's
for the repository and kind and for the kind and before the host's for
every agent; `[streams.<stream>]` SHALL keep its bound, priority and
pool, and give no setting (32, 143).

#### Scenario: Two streams' status lines

- **WHEN** the host gives `[agents.programs.agent] line = "host"` and `[agents.streams.work.programs.agent] line = "work"`, and two fixes are launched, one in `work` and one in `default`
- **THEN** the fix in `work` is given `line = "work"` from `host-stream` and the one in `default` `line = "host"` from `host` (S76, 143)
