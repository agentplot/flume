## ADDED Requirements

### Requirement: A kind with no stages names its fix kind through weir's crate

The kinds profile SHALL read, through `weir-kinds` at the revision that
reads them, the kind a kind that lands with no stages names for its
fixes and its bound, onto flume's kind as written, the bound none where
the declaration leaves it out; and SHALL take either on a kind with
stages only as weir's crate takes it, refused in weir's words naming
the field. flume's own parser SHALL read neither (4, 23, 157; weir 52,
57, 89; core.4).

#### Scenario: A train naming mend over a scratch repository

- **WHEN** a scratch repository's `.config/flume.toml` declares `train` with `lands = true`, `stages = []`, `mend` as the kind of its fixes and a bound of 2, `yard` with `lands = true`, `stages = []` and neither, and `bump`, a kind with one stage, naming `mend` as the kind of its fixes
- **THEN** `train` reads naming `mend` with a bound of 2, `yard` reads naming none, and `bump` is refused in weir's words naming the field (23, 57)
