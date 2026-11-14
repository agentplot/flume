## MODIFIED Requirements

### Requirement: The host declares each source in a file of its own

flume SHALL read the host's sources from
`$XDG_CONFIG_HOME/flume/sources/`, each regular file named
`<source>.toml` and beginning with no `.` declaring the source of that
name, its `kinds_dir` an absolute path or one under `~`, and SHALL ignore
every other entry; the sources SHALL be the managed layer's, the local
layer declaring none; it SHALL read them as they stand at each launch,
wake, lock and status, and write nothing there or in a `kinds_dir` (154,
159, 191, C.7).

#### Scenario: A source written beside and renamed

- **WHEN** a scratch host's sources directory gains `.acme.toml.tmp` giving `kinds_dir = "~/kinds/acme"`, which is then renamed to `acme.toml`
- **THEN** a read before the rename holds no source, and one after holds the source `acme` at the host user's home's `kinds/acme` (159, 191, C.7)

#### Scenario: Entries that are no source

- **WHEN** the sources directory holds `README`, `acme.toml~` and a directory `beta.toml/` beside `acme.toml`
- **THEN** the host declares the source `acme` alone (191, C.7)

#### Scenario: No sources directory

- **WHEN** the host's configuration directory holds no `sources/`
- **THEN** the host declares no source, and a launch of `fix` runs flume's own newest fix, fix@2, as before (154)

### Requirement: A bare name keeps its meaning

A kind named with no source SHALL resolve among the launch's
declaration, the repository's own and flume's own alone, and never to a
kind of a source the host declares; where none of the three holds it,
the refusal SHALL name each identity of that name the host's sources
hold (19, 154).

#### Scenario: A source that holds fix

- **WHEN** the host's source acme holds `fix@1`, locked by the host, and launches name `fix` and `acme/fix`
- **THEN** the first runs flume's own fix@2, its newest, and the second acme's, and status names `flume/fix@2` and `acme/fix@1` (S97, 154)

#### Scenario: A name only the host's sources hold

- **WHEN** acme holds `review@1` and beta holds `review@3`, both locked, the repository declaring no `review`, and launches name `review` and `beta/review`
- **THEN** the first is refused naming acme/review@1 and beta/review@3 as the host's, which a launch names by source, and makes nothing, and the second runs beta/review@3 (S62, 154)

### Requirement: flume's own kinds are packages whose content never changes

Each version of flume's own kinds SHALL be a package with its content
hash, as weir's crate computes it over its declaration and prompts, and
a check SHALL fail naming the version where a version's hash is not the
one this release records, so a change to what a version holds is a new
version (156, 157).

#### Scenario: The hashes recorded

- **WHEN** the check reads flume's own kinds
- **THEN** fix@1, fix@2, chore@1, chore@2 and change@1 to change@4 each hash to the value the check records, fix@1, chore@1 and change@1 to change@3 to the values recorded before change@4, chore@2 and fix@2 (156)

#### Scenario: A lock of flume's own at another hash

- **WHEN** a repository's lock records `flume/change@1` at a hash that is not flume's, and a launch names `change`
- **THEN** it is refused naming flume/change@1 and both hashes, and makes nothing (156)
