## ADDED Requirements

### Requirement: change@3's construct gives each task its own check

change@3's construct prompt SHALL ask each task to name the narrowest
check that shows it done, one test by name, one package's tests or a
lint of the files it touches, and SHALL rule out a task that runs the
repository's whole checks or checks a task group (22, C.2, S119).

#### Scenario: change@3's construct prompt rendered

- **WHEN** change@3's construct prompt is rendered for a place
- **THEN** it asks each task for the narrowest check that shows it done, and rules out a task running the repository's whole checks or a task group's (22, C.2)

### Requirement: change@3's code runs each task's own check alone

change@3's code prompt SHALL have its one agent work through every task
of the change in order, run each task's own check and tick the task once
that check passes, run no other check, and put a retry's words last
(22, 131, C.2; core.33).

#### Scenario: change@3's code prompt rendered

- **WHEN** change@3's code prompt is rendered for a place
- **THEN** it asks for each task's own check before its tick and rules out the repository's whole checks and a task group's (22, C.2)

#### Scenario: A retry's words come last

- **WHEN** change@3's code prompt is rendered for one place without words and with a verify's words
- **THEN** the two texts are equal up to the first character of the words' section, and the second holds the words and asks the fix be recorded as a task naming its own check (131; core.33)

### Requirement: change@3's verify runs the whole checks once

change@3's verify prompt SHALL run the repository's whole checks once,
as the repository's instructions name them, before it checks the work
against the change, and SHALL stop short having found work where the
checks fail or the work falls short (22, 131, C.2, S119).

#### Scenario: change@3's verify prompt rendered

- **WHEN** change@3's verify prompt is rendered for a place
- **THEN** it runs the repository's whole checks once before it checks the specs and the tasks, names their outcome in `verify.md`, and archives the change only where both hold (22, 131, C.2)

## MODIFIED Requirements

### Requirement: flume's own three kinds and their prompts

flume's own kinds SHALL be built into the binary one declaration per
version, each with its prompts beside it, and each read like any other:
fix@1, chore@1, change@1, change@2 and change@3 as C.2 gives them,
change@2's code stage repeating per task group and change@1's and
change@3's not, each prompt naming its deliverable and the way to stop
short, and none asking for a submit, a push or a rebase (22, 28, 71,
140, 157, C.2).

#### Scenario: The kinds built in

- **WHEN** flume's own kinds are read
- **THEN** they are fix@1 on `fix/`, chore@1 on `chore/`, and change@1, change@2 and change@3 on `change/`, each change with construct, code and verify, change@2's code repeating per task group and change@1's and change@3's not, all landing, each last stage's `on_fail` `code`

#### Scenario: Each version's prompts are its own

- **WHEN** flume's own declarations are read
- **THEN** each version's prompts are read from its own directory, change@1's code prompt is the one flume held before change@2, change@2's construct and verify are change@1's, and change@3's three prompts are none of change@1's or change@2's (156, 157)

#### Scenario: No prompt asks to submit

- **WHEN** every prompt of flume's own kinds is rendered for a place
- **THEN** each holds `flume needs` and none holds a submit, a push or a rebase, nor a merge but the base's in a conflict's words (28, 71, 112)

#### Scenario: A place launched at change@1

- **WHEN** a place launched at change@1 is settled by a flume that holds change@1 and change@2
- **THEN** it keeps change@1's stages as its launch read them, its code stage not repeating, and status names change@1 (21)

#### Scenario: A place launched at change@2

- **WHEN** a place launched at change@2 is settled by a flume that holds change@1, change@2 and change@3
- **THEN** it keeps change@2's stages as its launch read them, its code stage repeating per task group, and status names change@2 (21)

### Requirement: What flume's own prompts ask

change@2's and change@3's verify prompts SHALL commit `verify.md` in the
change's directory and archive the change, or stop short having found
work; their code prompts sent with words SHALL restore an archived
change and record the fix as a ticked task; every code prompt sent back
with a conflict's words SHALL have its agent `git merge` the base's
revision named, resolve the files contested and commit, first, and
never rebase (22, 112, C.2).

#### Scenario: A code prompt sent back with a conflict

- **WHEN** fix@1's, chore@1's, change@2's and change@3's code prompts are rendered with a conflict's words naming the files contested and the base's revision
- **THEN** each tells the agent to `git merge` that revision into the place's branch, resolve those files and commit, before anything else, and never to rebase (112)

### Requirement: Packages, identities and locks through weir's crate

The kinds profile SHALL read a kind as a launch writes it, a package with
its content hash, a source's listing, the repository's lock at a revision
and a lock on disk, and SHALL write a lock, each through `weir-kinds` at
its pinned revision alone; it SHALL spell every identity it answers as
weir's crate spells it, and flume's own code SHALL parse, spell or hash
none (core.4; weir 88 to 93, C.4).

#### Scenario: A package's hash wherever it lies

- **WHEN** one package is copied to two scratch directories, one under a link, and each is read at its identity
- **THEN** both read the same kind and the same hash, and the hash changes when one prompt's byte does (156; weir 90)

#### Scenario: A source's listing

- **WHEN** a scratch `kinds_dir` holds `review@1/` and `review@2/`, each with a `kind.toml`, and `notes/`, `lint@x/` and `lint@1/` with no `kind.toml`
- **THEN** the listing answers acme/review@1 and acme/review@2 alone (159)

#### Scenario: The repository's lock at a revision

- **WHEN** a scratch repository commits `.config/flume.lock` recording `flume/change@1`, then a second revision recording nothing
- **THEN** the lock at the first revision records flume/change@1 with its hash, the one at the second records nothing, and one at a revision with no lock is answered as none (20, 155)

#### Scenario: A lock written and read back

- **WHEN** the profile writes a lock of acme/review@1 and acme/review@2 to a scratch file and reads it
- **THEN** it reads both entries as written, and a write that fails leaves the file as it was (155; weir 91)

#### Scenario: Flume's own as packages

- **WHEN** flume's own kinds are read
- **THEN** each is a package with its identity, `flume/fix@1`, `flume/chore@1`, `flume/change@1`, `flume/change@2` and `flume/change@3`, and its hash (156, 157)
