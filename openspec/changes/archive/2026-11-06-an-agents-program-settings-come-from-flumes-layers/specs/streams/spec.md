## ADDED Requirements

### Requirement: A stream's agents take the host's settings for the stream

A place's agents SHALL take the settings `[agents.streams.<stream>]`
gives for its stream, its own keys and the file it names, read as it
stands at each start, after the host's for the repository and kind and
for the kind and before the host's for every agent;
`[streams.<stream>]` SHALL keep its bound, priority and pool, and give
no setting (32, 143).

#### Scenario: Two streams' status lines

- **WHEN** the host's `[agents.programs.agent]` names a file giving `line = "host"` and `tui = "fullscreen"`, its `[agents.streams.work.programs.agent.settings]` gives `line = "work"`, and two fixes are launched, one in `work` and one in `default`
- **THEN** the fix in `work` is given `line = "work"` from `host-stream` and the file's `tui` from `host`, and the one in `default` the file's `line = "host"` and `tui`, each from `host` (S100, 143)
