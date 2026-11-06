## Purpose

Where a host's kinds come from beyond flume's own, the repository's and
a launch's: each source the host declares in a file of its own beside
its configuration, holding packages side by side; a kind of one named
by its identity and run only as a lock records its content's hash;
`flume kinds lock` recording a kind's content; and a place keeping the
kind its launch read (19, 21, 154 to 160, 191, 192).

## ADDED Requirements

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
- **THEN** the host declares no source, and a launch of `fix` runs flume's own fix@1 as before (154)

### Requirement: A source's fault refuses only the launches naming it

A source's file that is not TOML, gives no `kinds_dir`, gives one
neither absolute nor under `~`, gives a key flume does not know, or is
named `flume`, `repo`, `launch` or a name no identity allows, SHALL
refuse each launch naming its source, naming the file and the fault, and
no other launch; a `kinds_dir` that does not exist SHALL refuse such a
launch naming the directory (4, 154, 159, 191).

#### Scenario: A file with no kinds_dir

- **WHEN** the sources directory holds `bad.toml` with no `kinds_dir`, and launches name `bad/x@1`, `fix` and `acme/review@1`, acme's review@1 locked by the host
- **THEN** the first is refused naming `bad.toml` and `kinds_dir`, and makes nothing, and the other two are made (S96, 4, 159, 191)

#### Scenario: A file taking flume's own source's name

- **WHEN** the sources directory holds `flume.toml` giving a `kinds_dir` that holds `change@9`, and a launch names `flume/change@2`
- **THEN** the launch runs flume's own change@2, and a launch of `flume/change@9` is refused naming the versions flume holds (154)

#### Scenario: A kinds_dir not yet written

- **WHEN** `acme.toml` names a `kinds_dir` that does not exist, and a launch names `acme/review@1`
- **THEN** it is refused naming the directory as missing, and makes nothing (4, 159, 191)

### Requirement: A source holds packages side by side

A source SHALL hold its kinds as packages at
`<kinds_dir>/<name>@<version>/`, each read through weir's crate with its
content hash, several versions of one name side by side; a package whose
kind is not the identity it lies at, that declares other than one kind,
names a part outside itself, or gives an `env` or a program by a path
SHALL refuse the launch naming the field (157, 158; weir 89, 90, 92).

#### Scenario: Two versions side by side

- **WHEN** acme's `kinds_dir` holds `review@1/` and `review@2/`, each locked by the host, and launches name `acme/review@1` and `acme/review@2`
- **THEN** each place runs its own version's stages and prompts, and status names each identity with its own hash (154, 159)

#### Scenario: A part outside the package and an environment

- **WHEN** acme's `review@1` names its standing instructions at `../shared/standing.md`, and `lint@1` gives its agents `env = { A = "b" }`, each locked
- **THEN** a launch of each is refused, the first naming the path outside its package and the second the field, and neither makes anything (S63, 157, 158)

#### Scenario: A package at another identity

- **WHEN** acme's `review@2/kind.toml` declares `review` at version 1
- **THEN** a launch of `acme/review@2` is refused naming both versions, and makes nothing (157)

### Requirement: A launch names a kind by its identity

A launch's kind SHALL be read through weir's crate as `<name>` or
`<source>/<name>`, each with or without `@<version>`; a source named
SHALL be the only one searched, `flume`, `repo` and `launch` naming
flume's own, the repository's and the launch's declaration, any other a
source the host declares; one no source holds SHALL refuse the launch
naming it and what the source holds (4, 154).

#### Scenario: A source the host does not declare

- **WHEN** the host declares the source `crew` alone, and a launch names `acme/review@2`
- **THEN** it is refused naming the source `acme` as one the host does not declare and `crew` as the one it does, and makes nothing (4, 154)

#### Scenario: A version the source does not hold

- **WHEN** acme holds `review@1` and `review@2`, and a launch names `acme/review@3`
- **THEN** it is refused naming review@3 and the versions acme holds, 1 and 2, and makes nothing (4, 154)

#### Scenario: The repository's and the launch's named by source

- **WHEN** a repository declares `bump` at version 2, and launches name `repo/bump`, `repo/bump@1` and `launch/bump`, the last carrying no `--kinds`
- **THEN** the first runs the repository's bump@2, the second is refused naming version 1 as not held, and the third is refused naming the declaration the launch does not carry (4, 154)

### Requirement: A bare name keeps its meaning

A kind named with no source SHALL resolve among the launch's
declaration, the repository's own and flume's own alone, and never to a
kind of a source the host declares; where none of the three holds it,
the refusal SHALL name each identity of that name the host's sources
hold (19, 154).

#### Scenario: A source that holds fix

- **WHEN** the host's source acme holds `fix@1`, locked by the host, and launches name `fix` and `acme/fix`
- **THEN** the first runs flume's own fix@1 and the second acme's, and status names `flume/fix@1` and `acme/fix@1` (S97, 154)

#### Scenario: A name only the host's sources hold

- **WHEN** acme holds `review@1` and beta holds `review@3`, both locked, the repository declaring no `review`, and launches name `review` and `beta/review`
- **THEN** the first is refused naming acme/review@1 and beta/review@3 as the host's, which a launch names by source, and makes nothing, and the second runs beta/review@3 (S62, 154)

### Requirement: A kind from a source runs only as a lock records it

A kind of a host's source SHALL run only where the repository's lock at
the base's revision or the host's lock records its identity; its content
SHALL be hashed at each launch, and the launch refused where the hash is
not a lock's, where the two locks give two hashes, or where a lock it
reads cannot be read, naming the kind, the lock and the hashes (155,
156).

#### Scenario: Unlocked, then locked by the repository

- **WHEN** acme holds `review@1`, the repository locks nothing, and a launch names `acme/review`; then `flume kinds lock acme/review@1` runs in the repository, the lock is committed on the base, and the launch runs again
- **THEN** the first is refused naming acme/review@1 as recorded in no lock and both ways to lock it, and makes nothing, and the second runs it, status naming its hash and `.config/flume.lock` (S60, 155)

#### Scenario: An edited prompt

- **WHEN** a place of acme/review@1 is launched under the host's lock, and a prompt of `review@1/` is then edited
- **THEN** the next launch of `acme/review@1` is refused naming the kind and both hashes, and the first place's next stage runs with the prompt it launched with (S61, 21, 156)

#### Scenario: Two locks that disagree

- **WHEN** the repository's lock and the host's lock each record acme/review@1, with two hashes
- **THEN** a launch of `acme/review@1` is refused naming both locks and both hashes, and makes nothing (155)

#### Scenario: A lock that cannot be read

- **WHEN** the host's lock is not TOML, and launches name `acme/review@1`, `change` and the repository's own `bump`
- **THEN** the first two are refused naming the host's lock and its fault, and the third is made (155)

### Requirement: A launch naming no version takes the locks' version

A launch naming no version SHALL take the newest version of its kind the
repository's lock records, else the newest the host's lock records, else
the newest its source holds, which a kind of a host's source then runs
only where locked; for flume's own this SHALL pick the version a bare
name runs (154, 155).

#### Scenario: The repository locks change@1

- **WHEN** a repository's lock records `flume/change@1`, and launches name `change` and `change@2`
- **THEN** the first runs change@1, its code stage one agent for the whole change, and the second change@2, status naming `flume/change@1` with its hash locked in `.config/flume.lock` and `flume/change@2` with its hash and no lock (S59, 154)

#### Scenario: A version installed and not yet locked

- **WHEN** acme holds `review@1` and `review@2`, the host's lock recording review@1 alone, and a launch names `acme/review`
- **THEN** it runs review@1 (154, 155)

#### Scenario: The newest the source holds, unlocked

- **WHEN** acme holds `review@1` and `review@2`, no lock records either, and a launch names `acme/review`
- **THEN** it is refused naming acme/review@2 as recorded in no lock, and makes nothing (154, 155)

### Requirement: flume's own kinds are packages whose content never changes

Each version of flume's own kinds SHALL be a package with its content
hash, as weir's crate computes it over its declaration and prompts, and
a check SHALL fail naming the version where a version's hash is not the
one this release records, so a change to what a version holds is a new
version (156, 157).

#### Scenario: The hashes recorded

- **WHEN** the check reads flume's own kinds
- **THEN** fix@1, chore@1, change@1 and change@2 each hash to the value the check records (156)

#### Scenario: A lock of flume's own at another hash

- **WHEN** a repository's lock records `flume/change@1` at a hash that is not flume's, and a launch names `change`
- **THEN** it is refused naming flume/change@1 and both hashes, and makes nothing (156)

### Requirement: flume kinds lock records a kind's content

`flume kinds lock <identity>…` SHALL record each whole identity at the
hash of its content as the host holds it or flume carries it, in the
working copy's `.config/flume.lock` or with `--host` the host's lock;
`--remove` SHALL take entries out, and `--hash` with one identity SHALL
refuse where its content hashes otherwise; it SHALL write the lock whole
by rename, fetch nothing and commit nothing (155).

#### Scenario: The host's lock written

- **WHEN** `flume kinds lock --host acme/review@1 flume/change@1` runs on a scratch host with no lock
- **THEN** the host's lock records both, each at its content's hash, and its answer names the lock's file and each entry (155)

#### Scenario: A working copy's lock written

- **WHEN** `flume kinds lock acme/review@1` runs in a scratch repository
- **THEN** its `.config/flume.lock` records it, nothing is committed, and nothing else is written in the working copy (155, core.16)

#### Scenario: A hash the content does not have

- **WHEN** `flume kinds lock --host --hash sha256:00 acme/review@1` runs
- **THEN** it is refused naming both hashes, and the host's lock is as it was (155, 156)

#### Scenario: An identity no lock records

- **WHEN** `flume kinds lock --host repo/bump@1`, then `acme/review`, runs
- **THEN** the first is refused naming the repository's kinds as needing no lock, the second naming the whole identity as required, and the host's lock is as it was (155)

#### Scenario: Two writers at once, and a removal

- **WHEN** `flume kinds lock --host acme/review@1` and `flume kinds lock --host acme/lint@1` run at once, and then `flume kinds lock --host --remove acme/lint@1` runs twice
- **THEN** the lock records both after the first two, and acme/review@1 alone after the removals, the second removal changing nothing (74, 155)

### Requirement: A place keeps the kind its source gave at its launch

A place SHALL keep its kind's identity, content and where it came from as
its launch read them; a source's file renamed over, a package edited or
removed, or a lock entry moved or removed SHALL change no place already
launched, and status SHALL go on naming the source's file and the kind's
directory as the launch read them (21).

#### Scenario: The source moved under a running place

- **WHEN** a place of acme/review@1 works its first stage, and `acme.toml` is renamed over by one naming a directory without `review@1/`, and the host's lock's entry for it is removed
- **THEN** the place's next stage starts with review@1's prompt as launched, status names acme/review@1 from the source's file and the kind's directory as launched, and a new launch of `acme/review@1` is refused naming it as one acme does not hold (S95, 21, 191)
