## ADDED Requirements

### Requirement: A declaration's agents tables give own settings

The kinds profile SHALL read `programs` from every agents table a
declaration holds, a kind's, a stage's and the repository's for a kind
it does not declare, from the table weir's crate returns as written:
each entry's `file` and `settings`, kept as JSON values. Any other
shape or a date SHALL fail the declaration naming the field (23, 158,
175, C.2; core.4).

#### Scenario: Own settings for every stage and for one

- **WHEN** a scratch repository's `.config/flume.toml` gives `[kinds.bump.agents.programs.claude.settings] tui = "fullscreen"` and a stage's `agents = [{ programs = { claude = { file = "claude-bump.json" } } }]`, and `.config/claude-bump.json` at the revision gives a `statusLine`
- **THEN** the kind's settings read `tui` for every stage, and for that stage the file's `statusLine` with the file's path, read at the revision (23, 157, 175, C.2)

#### Scenario: An entry flume cannot take

- **WHEN** a declaration's agents table gives `programs = { claude = "x" }`, or `programs = { claude = { tui = "fullscreen" } }`
- **THEN** the declaration cannot be read, naming the field, and nothing is launched (4, 175, C.2)

### Requirement: A declaration's settings file is read beside it

A file an agents table of a declaration names SHALL be read by a path
relative to the declaration's file, as its prompts are, at the same
revision for a repository's and on disk for a launch's; one that cannot
be read, does not parse or lies outside the repository SHALL fail the
declaration naming the field (157, 175, C.2; core.34).

#### Scenario: A file outside the repository or not at the revision

- **WHEN** a declaration's agents table gives `programs = { claude = { file = "../claude.json" } }`, or names a file the revision does not hold
- **THEN** the declaration cannot be read, naming the field, and nothing is launched (4, 157; core.34)

### Requirement: A repository's settings for every kind

The kinds profile SHALL read a repository's `[agents]`, flume's own
table of `.config/flume.toml` beside `[profiles]` and no part of any
kind, from the sections weir's crate returns unnamed, at the revision
the kinds are read at, giving `program`, `model`, `mode`,
`instructions` and `programs` as the repository's kind tables do (21,
23, 32, 175, C.2; core.4).

#### Scenario: A repository's agents for every kind

- **WHEN** a scratch repository's `.config/flume.toml` at the revision gives `[agents.programs.claude.settings] attribution = { commit = "Assisted-by: Claude Code", pr = "" }` beside a `[kinds.fix.agents]` table
- **THEN** the kinds read give the repository's settings for every kind with that `attribution` as written, apart from its settings for fix (S78, 32, C.2)

### Requirement: A repository's agents table gives no stage, stream or environment

A `stages`, `streams`, `env` or `beside` in a repository's `[agents]`
SHALL fail the declaration naming the field, and a launch's declaration
and a package's SHALL not be read for the table (4, 32, C.2).

#### Scenario: A repository's agents table that names a stage

- **WHEN** a repository's `[agents]` gives `stages = { code = { model = "opus" } }`, or `env = { X = "1" }`
- **THEN** the declaration cannot be read, naming `agents.stages` or `agents.env`, and nothing is launched (4, C.2)
