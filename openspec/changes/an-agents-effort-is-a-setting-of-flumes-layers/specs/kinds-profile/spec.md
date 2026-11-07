## ADDED Requirements

### Requirement: A declaration's agents tables give an effort

The kinds profile SHALL read `effort` from every agents table a
declaration holds, a kind's, a stage's, the repository's for a kind it
does not declare and the repository's `[agents]` for every kind, from
the table weir's crate returns as written: a
string, kept as written; any other, a table of programs among it until
the model's is built with it, SHALL fail the declaration naming the
field (23, 158, 172, 197, C.2; core.4).

#### Scenario: An effort for every stage and one for a stage

- **WHEN** a scratch repository's `.config/flume.toml` gives `[kinds.bump.agents] effort = "high"` and a stage's `agents = [{ effort = "xhigh" }]`
- **THEN** the kind's settings read `high` for every stage and `xhigh` for that stage (23, 197, C.2)

#### Scenario: An effort for every kind of a repository

- **WHEN** a scratch repository's `.config/flume.toml` gives `[agents] effort = "medium"` beside a `[kinds.fix.agents]` table giving `effort = "high"`
- **THEN** the kinds read give the repository's settings for every kind with `medium`, and its settings for fix with `high` (32, 197, C.2)

#### Scenario: An effort that is no word

- **WHEN** a declaration's agents table gives `effort = 3` or `effort = { claude = "high" }`
- **THEN** the declaration cannot be read, naming the table's `effort` (4, 197, C.2)

## MODIFIED Requirements

### Requirement: A repository's settings for every kind

The kinds profile SHALL read a repository's `[agents]`, flume's own
table of `.config/flume.toml` beside `[profiles]` and no part of any
kind, from the sections weir's crate returns unnamed, at the revision
the kinds are read at, giving `program`, `model`, `effort`,
`mode`, `instructions` and `programs` as the repository's kind tables
do (21, 23, 32, 195, 197, C.2; core.4).

#### Scenario: A repository's agents for every kind

- **WHEN** a scratch repository's `.config/flume.toml` at the revision gives `[agents.programs.claude.settings] attribution = { commit = "Assisted-by: Claude Code", pr = "" }` beside a `[kinds.fix.agents]` table
- **THEN** the kinds read give the repository's settings for every kind with that `attribution` as written, apart from its settings for fix (S102, 32, C.2)
