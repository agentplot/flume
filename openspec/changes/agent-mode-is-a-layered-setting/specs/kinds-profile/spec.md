## ADDED Requirements

### Requirement: A declaration's agents tables give a mode

The kinds profile SHALL read `mode` from every agents table a
declaration holds, a kind's, a stage's and the repository's for a kind
it does not declare, from the table weir's crate returns as written: a
word, one of flume's four, or a table of programs whose entries are
strings, each kept as written; any other SHALL fail the declaration
naming the field (23, 162, 169, C.2, core.4).

#### Scenario: A mode for every stage and a table for one

- **WHEN** a scratch repository's `.config/flume.toml` gives `[kinds.bump.agents] mode = "edits"` and a stage's `agents = [{ mode = { claude = "auto" } }]`
- **THEN** the kind's settings read `edits` for every stage and the table for that stage (23, 162, C.2)

#### Scenario: A mode that is neither

- **WHEN** a declaration's agents table gives `mode = "auto"`, `mode = ["edits"]` or `mode = {}`
- **THEN** the declaration cannot be read, naming the table's `mode` (4, 169, C.2)
