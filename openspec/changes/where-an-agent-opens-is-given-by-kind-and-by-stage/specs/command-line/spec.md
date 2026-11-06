## ADDED Requirements

### Requirement: open takes a session

`flume open [<stream>] --session <name>` SHALL ask the core to open that
session of the stream, `default`'s where no stream is named, and print
its answer, or its JSON form with `--json`; `open`'s help SHALL name
`--session` and say it opens a session the stream names for a kind or a
stage (186, 199).

#### Scenario: A stream's design session

- **WHEN** `flume open work --session work-design --json` runs over a scratch host whose `work` names `work-design` for research and names no terminal app
- **THEN** its JSON answer gives the stream `work`, the session `work-design` from `stream-kind` named for research, the title `work-design`, and the command that attaches `work-design`, printed for the person to run (186, 199)

#### Scenario: A session the stream does not name

- **WHEN** `flume open work --session nightly` runs over the same host
- **THEN** it exits refused naming `work`, `work` and `work-design`, and starts no session (186, 199)
