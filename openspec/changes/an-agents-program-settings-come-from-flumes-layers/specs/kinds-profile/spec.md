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
