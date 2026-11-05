## ADDED Requirements

### Requirement: A declaration's agents tables give an effort

The kinds profile SHALL read `effort` from every agents table a
declaration holds, a kind's, a stage's and the repository's for a kind
it does not declare, from the table weir's crate returns as written: a
string, kept as written; any other, a table of programs among it until
the model's is built with it, SHALL fail the declaration naming the
field (23, 158, 172, 197, C.2; core.4).

#### Scenario: An effort for every stage and one for a stage

- **WHEN** a scratch repository's `.config/flume.toml` gives `[kinds.bump.agents] effort = "high"` and a stage's `agents = [{ effort = "xhigh" }]`
- **THEN** the kind's settings read `high` for every stage and `xhigh` for that stage (23, 197, C.2)

#### Scenario: An effort that is no word

- **WHEN** a declaration's agents table gives `effort = 3` or `effort = { claude = "high" }`
- **THEN** the declaration cannot be read, naming the table's `effort` (4, 197, C.2)
