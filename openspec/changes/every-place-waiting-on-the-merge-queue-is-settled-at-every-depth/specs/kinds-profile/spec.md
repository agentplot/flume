## ADDED Requirements

### Requirement: A kind that lands with no stages read through weir's crate

The kinds profile SHALL read a kind declaring `stages = []` and
`lands = true` through `weir-kinds`, at the revision that reads it, as
a kind with no stages; a kind declaring no stages that does not land
SHALL be refused in weir's words, naming the field; and a table that
leaves `stages` out SHALL read as settings for a kind declared
elsewhere, as before (4, 18, 23, 202; weir 51, 57; core.4).

#### Scenario: Three tables over a scratch repository

- **WHEN** a scratch repository's `.config/flume.toml` declares `train` with `lands = true` and `stages = []`, `idle` with `lands = false` and `stages = []`, and a `[kinds.fix.agents]` table with no stages
- **THEN** `train` reads as a kind that lands with no stages, `idle` is refused naming its stages, and `fix`'s table is read as settings for flume's own fix (18, 57)
