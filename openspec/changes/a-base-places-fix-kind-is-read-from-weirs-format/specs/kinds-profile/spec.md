## ADDED Requirements

### Requirement: A kind with no stages names its landing fix through weir's crate

The kinds profile SHALL map a kind's `landing_fix`, as `weir-kinds`
reads it, onto the seam's kind of its fixes, spelled as weir's crate
spells the identity, and its `max_landing_fixes` onto the seam's bound,
taking neither from anything else, in a repository's declaration, a
launch's and a package's alike; a kind that gives neither SHALL name no
fix and no bound (23, 157; weir 52, 89; weir record 0005; core.4).

#### Scenario: A landing fix and its bound over a scratch repository

- **WHEN** a scratch repository's `.config/flume.toml` declares `train`, landing with `stages = []`, `landing_fix = "repo/mend@1"` and `max_landing_fixes = 2`; `yard`, landing with `stages = []` and neither field; and `mend`, landing with one code stage delivering commits
- **THEN** `train` reads naming `repo/mend@1` the kind of its fixes with a bound of 2, and `yard` and `mend` name no fix and no bound (23; weir 52)

#### Scenario: A package's landing fix

- **WHEN** a scratch source's `kinds_dir` holds `train@1/kind.toml`, declaring `train` landing with `stages = []`, `landing_fix = "flume/fix@1"` and `max_landing_fixes = 1`, and the package is read at `acme/train@1`
- **THEN** its kind names `flume/fix@1` the kind of its fixes with a bound of 1 (157; weir 89)

### Requirement: A landing fix weir or flume refuses

A `landing_fix` weir refuses, a bare name or a name with no source or
no version among them, and either field on a kind with stages, SHALL be
answered as could not be read in weir's words, naming the field; a
bound past what flume counts SHALL be refused naming the field; and
`fix` and `max_fixes` SHALL be read as the sections weir returns them
as, and ignored (4, 23, 84; weir 57; weir record 0005; core.4).

#### Scenario: A landing fix weir refuses

- **WHEN** the scratch repository's `train` gives `landing_fix = "mend"`, then `landing_fix = "repo/mend"`, each at a revision of its own, and then a one-stage kind `bump` gives `landing_fix = "flume/fix@1"` and `max_landing_fixes = 1`
- **THEN** each revision's kinds are answered as could not be read: the first two naming `kinds.train.landing_fix` and that it is no identity, the third naming `kinds.bump.landing_fix` and `kinds.bump.max_landing_fixes` and that each is on a kind with stages, all in weir's words (57; weir record 0005)

#### Scenario: A bound past what flume counts

- **WHEN** the scratch repository's `train` gives `landing_fix = "repo/mend@1"` and `max_landing_fixes = 4294967296`
- **THEN** its kinds are answered as could not be read, naming `kinds.train.max_landing_fixes` and that it is more than flume counts (4)

#### Scenario: The fields' old spellings

- **WHEN** the scratch repository's `train`, landing with `stages = []`, gives `fix = "repo/mend@1"` and `max_fixes = 2`
- **THEN** `train` reads naming no fix and no bound (23; weir record 0005)

### Requirement: A base place's launch resolves the landing fix the profile reads

A base place launched under a kind the kinds profile read SHALL resolve
the landing fix its kind names as a launch's whole identity resolves,
at the version the identity names, and hold it with where it came from;
a landing fix that resolves to no kind SHALL refuse the launch, naming
it, before the merge queue opens anything (4, 21, 23, 154, 203; core.8,
core.34).

#### Scenario: The landing fix held through the profile

- **WHEN** a scratch repository binds the fakes' merge queue and declares `train`, landing with `stages = []`, `landing_fix = "repo/mend@1"` and `max_landing_fixes = 2`, and `mend` at version 1, landing with one code stage delivering commits, and `train--spring` is launched on `main`
- **THEN** the launch is taken, the place holds `repo/mend@1` from the repository's declaration as the kind its fixes run as, and status gives `train--spring` that fix kind and a bound of 2 (21, 56, 203)

#### Scenario: A landing fix at a version the repository does not declare

- **WHEN** the scratch repository's `train` gives `landing_fix = "repo/mend@2"` while it declares `mend` at version 1, and `train--spring` is launched on `main`
- **THEN** the launch is refused naming `repo/mend@2` and that the repository declares `mend` at version 1, the fake merge queue logs no open, and the store holds no document of the place (4, 154; core.8, core.34)
