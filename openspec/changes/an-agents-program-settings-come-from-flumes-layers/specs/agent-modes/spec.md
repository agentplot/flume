## MODIFIED Requirements

### Requirement: Every layer may give the mode, taken whole

An agent's mode SHALL come from the first of the eight layers that
gives one: the launch's, the host's for the repository, for its kind
and then for every kind, the host's for the kind, for the stream and
for every agent, the repository's for a kind it does not declare and
then for every kind, the kind's, and flume's default, which gives
none; within each that names stages a stage's own SHALL come before one
for every stage, and that before a repository's for every kind. The host's layers SHALL be read as they stand at each
start, and the others as the launch read them (8, 21, 32, 162).

#### Scenario: The host's table over the repository's

- **WHEN** the host's `[kinds.fix.agents]` gives `mode = "edits"`, the repository's `[kinds.fix.agents]` gives `mode = "full"`, and a fix is launched
- **THEN** its agent starts with the value the agent's profile maps `edits` to, status naming `host-kind` as where its mode came from (S66, 32, 162)

#### Scenario: A stage's own before every stage's

- **WHEN** the kind's declaration gives `mode = "edits"` for every stage and `mode = "ask"` for verify, and the launch gives `--agent mode=full`
- **THEN** every stage's agent starts in `full` from `launch`; launched again with no `--agent`, construct's and code's start in `edits` and verify's in `ask`, each from `kind` (32)

#### Scenario: The host's table changes while a stage works

- **WHEN** the host's `[kinds.change.agents]` changes from `mode = "edits"` to `mode = "full"` while code's agent works
- **THEN** code's agent keeps the mode it started with, and verify's starts in `full` (8, 32)

#### Scenario: A stream's mode over the kind's

- **WHEN** the host's `[agents.streams.watched]` gives `mode = "edits"` and the kind's declaration gives `mode = "full"`
- **THEN** a place in `watched` starts its agents in `edits` from `host-stream`, and one in `default` in `full` from `kind` (32, 143, 162)
