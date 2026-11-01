## ADDED Requirements

### Requirement: A declaration's agents tables give own settings

The kinds profile SHALL read `programs` from every agents table a
declaration holds, a kind's, a stage's and the repository's for a kind
it does not declare, from the table weir's crate returns as written: a
table of programs each a table, kept as JSON values; any other shape, a
date or a credential-shaped value SHALL fail the declaration naming the
field (23, 158, 175, C.2, core.4).

#### Scenario: Own settings for every stage and for one

- **WHEN** a scratch repository's `.config/flume.toml` gives `[kinds.bump.agents.programs.claude] outputStyle = "Explanatory"` and a stage's `agents = [{ programs = { claude = { spinnerTipsEnabled = false } } }]`
- **THEN** the kind's settings read the output style for every stage and the boolean for that stage (23, 175, C.2)

#### Scenario: Own settings flume cannot take

- **WHEN** a declaration's agents table gives `programs = { claude = "x" }`, or a value opening `sk-ant-` and twenty letters
- **THEN** the declaration cannot be read, naming the field and never the value (4, 175, C.2)
