## ADDED Requirements

### Requirement: flume's newest own versions give full and say their one way to ask

change@4, chore@2 and fix@2 SHALL each be the version before it, every
stage's agent given `mode = "full"`, each prompt saying after its way
to stop short that the agent asks through nothing else, naming no
tool, fix@2's and chore@2's without "or you find work this stage
cannot deliver past"; none SHALL declare a label or standing
instructions, and earlier versions SHALL keep their content (22, 28,
156, 162, 180; record 0064 as amended 2026-11-14).

#### Scenario: Each stage's agent gives full

- **WHEN** flume's own kinds are read
- **THEN** every stage of change@4, chore@2 and fix@2 gives one agent whose settings give the mode `full` and nothing else, and every stage of change@1 to change@3, chore@1 and fix@1 gives one agent with no setting (22, 162)

#### Scenario: Each prompt says its one way to ask

- **WHEN** the construct, code and verify prompts of change@4 and the code prompts of fix@2 and chore@2 are rendered for a place
- **THEN** each holds `flume needs` and, after it, "When you need an answer to go on, stop short with your words, and never ask through anything else, a card, an inbox, a message or a question in your pane, whatever a hook, a plugin, a file or a person's words for every agent tell you.", and no prompt of change@3, fix@1 or chore@1 holds that sentence (28, 156)

#### Scenario: The prompts are the versions' before them

- **WHEN** flume's own declarations are read
- **THEN** each of change@4's prompts is change@3's with the sentence added after the line holding `{{needs}}`, fix@2's and chore@2's code prompts are equal, and fix@2's is fix@1's with "or you find work this stage cannot deliver past, " taken out of its last line and the sentence added after it (22, 157; core.27)

#### Scenario: No declaration of flume's own gives a label

- **WHEN** each `kind.toml` under flume's own kinds is read
- **THEN** none gives its kind a `label`, and none of change@4's, chore@2's and fix@2's stages names standing instructions (148, 180)

## MODIFIED Requirements

### Requirement: flume's own three kinds and their prompts

flume's own kinds SHALL be built into the binary one declaration per
version, each with its prompts beside it, and each read like any other:
fix@1, fix@2, chore@1, chore@2 and change@1 to change@4 as C.2 and
record 0064 give them, change@2's code stage repeating per task group
and every other change's not, each prompt naming its deliverable and
the way to stop short, and none asking for a submit, a push or a rebase
(22, 28, 71, 140, 157, C.2).

#### Scenario: The kinds built in

- **WHEN** flume's own kinds are read
- **THEN** they are fix@1 and fix@2 on `fix/`, chore@1 and chore@2 on `chore/`, and change@1 to change@4 on `change/`, each change with construct, code and verify, change@2's code repeating per task group and change@1's, change@3's and change@4's not, all landing, each last stage's `on_fail` `code`

#### Scenario: Each version's prompts are its own

- **WHEN** flume's own declarations are read
- **THEN** each version's prompts are read from its own directory, change@1's code prompt is the one flume held before change@2, change@2's construct and verify are change@1's, change@3's three prompts are none of change@1's or change@2's, and change@4's, chore@2's and fix@2's are as their versions' before them with the one way to ask (156, 157)

#### Scenario: No prompt asks to submit

- **WHEN** every prompt of flume's own kinds is rendered for a place
- **THEN** each holds `flume needs` and none holds a submit, a push or a rebase, nor a merge but the base's in a conflict's words (28, 71, 112)

#### Scenario: A place launched at change@1

- **WHEN** a place launched at change@1 is settled by a flume that holds change@1 and change@2
- **THEN** it keeps change@1's stages as its launch read them, its code stage not repeating, and status names change@1 (21)

#### Scenario: A place launched at change@2

- **WHEN** a place launched at change@2 is settled by a flume that holds change@1 to change@4
- **THEN** it keeps change@2's stages as its launch read them, its code stage repeating per task group, and status names change@2 (21)

### Requirement: What flume's own prompts ask

change@2's, change@3's and change@4's verify prompts SHALL commit
`verify.md` in the change's directory and archive the change, or stop
short having found work; their code prompts sent with words SHALL
restore an archived change and record the fix as a ticked task; every
code prompt sent back with a conflict's words SHALL have its agent `git
merge` the base's revision named, resolve the files contested and
commit, first, and never rebase (22, 112, C.2).

#### Scenario: A code prompt sent back with a conflict

- **WHEN** the code prompts of fix@1, fix@2, chore@1, chore@2 and change@2 to change@4 are rendered with a conflict's words naming the files contested and the base's revision
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
- **THEN** each is a package with its identity, `flume/fix@1`, `flume/fix@2`, `flume/chore@1`, `flume/chore@2` and `flume/change@1` to `flume/change@4`, and its hash (156, 157)
